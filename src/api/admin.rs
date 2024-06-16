use crate::constants::constants;
use crate::database::db;
use crate::model::blog::{
    Blog, BlogError, BlogIdentifier, BlogPreview, BlogProjection, BlogPublishOperation,
    BlogUpdateOperation, FileInfo, UploadedFiles,
};
use crate::templates;
use crate::utils::datetime;
use crate::utils::io::get_temp_file_path;
use crate::utils::md::convert_to_html;
use crate::utils::storage;
use crate::utils::validations::validate_id;
use actix_multipart::Multipart;
use actix_web::http::header::{ContentType, CONTENT_LENGTH};
use actix_web::{
    delete, post, put,
    web::{Data, Form, Json, Path},
    HttpRequest, HttpResponse,
};
use askama::Template;
use futures_util::TryStreamExt;
use google_cloud_storage::client::Client as GcsClient;
use mime::{Mime, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG};
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use std::path::Path as std_Path;
use std::str::FromStr;

macro_rules! delete_blob {
    ($client:expr, $bucket:expr, $name:expr) => {
        if !storage::delete_blob($client, $bucket, $name).await {
            return Err(BlogError::InternalServerError);
        }
    };
}

macro_rules! move_blob {
    ($client:expr, $source_bucket:expr, $source_name:expr, $destination_bucket:expr, $destination_name:expr) => {
        if !storage::copy_blob(
            $client,
            $source_bucket,
            $source_name,
            $destination_bucket,
            $destination_name,
        )
        .await
        {
            return Err(BlogError::FileUploadError);
        }
    };
}

macro_rules! upload_blob {
    ($client:expr, $bucket:expr, $name:expr, $data:expr) => {
        if !storage::upload_blob($client, $bucket, $name, $data).await {
            return Err(BlogError::FileUploadError);
        }
    };
}

fn change_obj_prefix(obj: &str, old_prefix: &str, new_prefix: &str) -> String {
    if obj.len() < old_prefix.len() {
        return obj.to_string();
    }

    let mut obj = obj.to_string();
    log::info!("Changing prefix from {} to {}", old_prefix, new_prefix);
    if &obj[0..old_prefix.len()] == old_prefix {
        log::info!("Prefix found, changing prefix");
        obj.replace_range(0..old_prefix.len(), new_prefix);
    }
    return obj;
}

#[post("/api/admin/ws/blog/preview")]
async fn preview_blog(data: Form<BlogPreview>) -> HttpResponse {
    let content = data.get_content();
    if content.is_empty() {
        return HttpResponse::Ok().body("");
    }
    let preview = convert_to_html(content, None);
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(preview)
}

async fn process_file(
    file: &mut FileInfo,
    content: &str,
    gcs_client: &GcsClient,
) -> Result<String, BlogError> {
    if file.url.is_empty() {
        return Err(BlogError::FileIsEmpty);
    }

    let signed_url = match &file.signed_url {
        Some(url) => &url.clone(),
        None => {
            return Ok("".to_string());
        }
    };

    // check if the signed_url is in the content
    let mut content = content.to_string();
    if !content.contains(signed_url) {
        storage::remove_file_from_md_content(&mut content, signed_url);
        return Ok("".to_string());
    }

    let (bucket, obj_name) = storage::extract_bucket_and_blob_from_url(&file.url);
    // replace the signed url with the actual url
    let obj_name_with_changed_prefix = &change_obj_prefix(
        &obj_name,
        constants::TEMP_OBJ_PREFIX,
        constants::BLOG_OBJ_PREFIX,
    );
    let new_url = format!(
        "https://storage.googleapis.com/{}/{}",
        constants::BUCKET,
        obj_name_with_changed_prefix,
    );
    content = content.replace(signed_url, &new_url);
    file.signed_url = None;
    file.url = new_url;

    move_blob!(
        &gcs_client,
        &bucket,
        &obj_name,
        constants::BUCKET,
        obj_name_with_changed_prefix
    );
    return Ok(content);
}

macro_rules! process_file {
    ($file:expr, $content:expr, $gcs_client:expr) => {
        match process_file($file, &$content, $gcs_client).await {
            Ok(new_content) => {
                if !new_content.is_empty() {
                    $content = new_content;
                }
            }
            Err(err) => {
                return Err(err);
            }
        }
    };
}

#[post("/api/new/blog")]
async fn new_blog(
    client: Data<db::DbClient>,
    gcs_client: Data<GcsClient>,
    blog: Json<BlogPublishOperation>,
) -> Result<HttpResponse, BlogError> {
    let mut blog_op = blog.into_inner();
    let blog_col = client.into_inner().get_blog_collection();

    let title = blog_op.title;
    if title.is_empty() {
        return Err(BlogError::EmptyTitle);
    } else if title.len() > constants::TITLE_MAX_LENGTH {
        return Err(BlogError::TitleTooLong);
    }

    let mut content = blog_op.content;
    if content.is_empty() {
        return Err(BlogError::EmptyContent);
    }

    for file in blog_op.files.iter_mut() {
        process_file!(file, content, &gcs_client);
    }

    let blog = Blog::new(
        title,
        content,
        &blog_op.tags,
        &blog_op.files,
        blog_op.is_public,
    );
    match blog_col.insert_one(blog, None).await {
        Ok(result) => {
            let id = result.inserted_id.as_object_id().unwrap();
            Ok(HttpResponse::Ok().body(id.to_hex()))
        },
        Err(err) => {
            log::error!("Failed to create api in database: {}", err);
            Err(BlogError::PublishBlogError)
        }
    }
}

#[put("/api/blog/update")]
async fn update_blog(
    client: Data<db::DbClient>,
    gcs_client: Data<GcsClient>,
    update_blog: Json<BlogUpdateOperation>,
) -> Result<HttpResponse, BlogError> {
    let mut blog = update_blog.into_inner();
    let blog_id = validate_id(&blog.id)?;

    let new_tags = blog.tags;
    if new_tags.len() > constants::MAX_TAGS {
        return Err(BlogError::TooManyTags);
    }

    let options = FindOneOptions::builder()
        .projection(doc! { "title": 1, "tags": 1, "files": 1, "is_public": 1 })
        .build();
    let blog_in_db = client
        .get_blog_post_projection(&blog_id, Some(options))
        .await?;

    let mut is_updating = false;
    let last_modified = bson::DateTime::parse_rfc3339_str(datetime::get_dtnow_str())
        .expect("DateTime shouldn't fail to parse in update_blog");
    let mut set_doc = doc! {
        "last_modified": last_modified,
    };

    let old_files = blog_in_db.files.unwrap_or(vec![]);
    let mut files_to_put_in_db = Vec::with_capacity(blog.new_files.len() + old_files.len());
    let mut content = blog.content;
    if blog.new_files.len() > 0 {
        is_updating = true;
        for file in blog.new_files.iter_mut() {
            process_file!(file, content, &gcs_client);
        }
        files_to_put_in_db = blog.new_files;
    }

    // check if the old_files are in the content
    let mut files_to_keep = Vec::with_capacity(old_files.len());
    for file in old_files.into_iter() {
        if !content.contains(&file.url) {
            delete_blob!(&gcs_client, constants::BUCKET, &file.url);
        } else {
            files_to_keep.push(file);
        }
    }

    if !files_to_put_in_db.is_empty() {
        is_updating = true;
        set_doc.insert("files", files_to_put_in_db);
    }

    if !content.is_empty() {
        is_updating = true;
        set_doc.insert("content", content);
    }

    let title = blog.title;
    if !title.is_empty() && title != blog_in_db.title.unwrap_or_default() {
        is_updating = true;
        set_doc.insert("title", title);
    }

    if blog.is_public != blog_in_db.is_public.unwrap_or(false) {
        is_updating = true;
        set_doc.insert("is_public", blog.is_public);
    }

    let old_tags = blog_in_db.tags.unwrap_or(vec![]);
    // O(n^2) algorithm but the no. of tags must be less than 8 so it's technically O(1)
    if new_tags.len() != old_tags.len() || new_tags.iter().all(|tag| old_tags.contains(tag)) {
        is_updating = true;
        set_doc.insert("tags", new_tags);
    }

    let success_json = HttpResponse::Ok().body("Blog updated successfully".to_string());
    if !is_updating {
        return Ok(success_json);
    }

    let query = doc! { "_id": blog_id };
    let update = doc! { "$set": set_doc };
    let blog_col = client.into_inner().get_blog_collection();
    match blog_col.update_one(query, update, None).await {
        Ok(_) => Ok(success_json),
        Err(err) => {
            log::error!("Failed to update api in database: {}", err);
            Err(BlogError::UpdateBlogError)
        }
    }
}

#[delete("/api/blogs/{id}/delete")]
async fn delete_blog(
    client: Data<db::DbClient>,
    gcs_client: Data<GcsClient>,
    blog_identifier: Path<BlogIdentifier>,
) -> Result<HttpResponse, BlogError> {
    let blog_id = validate_id(&blog_identifier.into_inner().id)?;

    let options = FindOneOptions::builder()
        .projection(doc! { "files": 1 })
        .build();
    let blog_data = client
        .get_blog_post_projection(&blog_id, Some(options))
        .await?;

    let files = blog_data.files.unwrap_or(vec![]);
    for file in files.iter() {
        delete_blob!(&gcs_client, constants::BUCKET, &file.url);
    }

    let blog_col = client.into_inner().get_blog_collection();
    match blog_col.delete_one(doc! { "_id": blog_id }, None).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Blog deleted successfully".to_string())),
        Err(err) => {
            log::error!("Failed to delete api from database: {}", err);
            Err(BlogError::InternalServerError)
        }
    }
}

async fn configure_blog_post_bool(
    client: Data<db::DbClient>,
    blog_id: &str,
    is_public: bool,
) -> Result<HttpResponse, BlogError> {
    let blog_id = validate_id(blog_id)?;
    let query = doc! { "_id": blog_id };
    let update = doc! { "$set": { "is_public": is_public } };
    let blog_col = client.into_inner().get_blog_collection();
    match blog_col.update_one(query, update, None).await {
        Ok(_) => {
            let html = if is_public {
                templates::admin::Unlocked.render().unwrap()
            } else {
                templates::admin::Locked.render().unwrap()
            };
            Ok(HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(html))
        }
        Err(err) => {
            log::error!("Failed to publish api in database: {}", err);
            Err(BlogError::PublishBlogError)
        }
    }
}

#[put("/api/blogs/{id}/publish")]
async fn publish_blog_post(
    client: Data<db::DbClient>,
    blog_identifier: Path<BlogIdentifier>,
) -> Result<HttpResponse, BlogError> {
    configure_blog_post_bool(client, &blog_identifier.into_inner().id, true).await
}

#[put("/api/blogs/{id}/unpublish")]
async fn unpublish_blog_post(
    client: Data<db::DbClient>,
    blog_identifier: Path<BlogIdentifier>,
) -> Result<HttpResponse, BlogError> {
    configure_blog_post_bool(client, &blog_identifier.into_inner().id, false).await
}

#[post("/api/blog/upload/files")]
async fn upload_blog_files(
    gcs_client: Data<GcsClient>,
    mut payload: Multipart,
    req: HttpRequest,
) -> Result<Json<UploadedFiles>, BlogError> {
    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(v) => v.to_str().unwrap_or("0").parse().unwrap(),
        None => 0,
    };

    if content_length == 0 {
        return Err(BlogError::FileIsEmpty);
    } else if content_length > constants::MAX_FILE_SIZE {
        return Err(BlogError::FileTooLarge);
    }

    let mut files = UploadedFiles::new(vec![]);
    let image_webp: Mime = Mime::from_str("image/webp").unwrap();
    let video_mp4: Mime = Mime::from_str("video/mp4").unwrap();
    let allowed_mimetypes: [Mime; 5] = [
        IMAGE_PNG,
        IMAGE_JPEG,
        IMAGE_GIF,
        image_webp.clone(),
        video_mp4.clone(),
    ];
    while let Ok(Some(mut field)) = payload.try_next().await {
        log::info!("Processing image");

        let content_type: Option<&Mime> = field.content_type();
        if content_type.is_none() {
            log::info!("No content type found for file");
            continue;
        }

        let content_type = content_type.unwrap();
        if !allowed_mimetypes.contains(content_type) {
            log::info!("Invalid content type found for file");
            continue;
        }
        let mut file_ext = "";
        let content_type_clone = content_type.clone();
        if content_type_clone == image_webp {
            file_ext = "webp";
        } else if content_type_clone == IMAGE_PNG {
            file_ext = "png";
        } else if content_type_clone == IMAGE_JPEG {
            file_ext = "jpeg";
        } else if content_type_clone == IMAGE_GIF {
            file_ext = "gif";
        } else {
            // video_mp4
            file_ext = "mp4";
        }

        let destination = format!(
            "{}{}.{}",
            constants::TEMP_OBJ_PREFIX,
            get_temp_file_path(),
            file_ext
        );

        let mut data = Vec::new();
        while let Ok(Some(chunk)) = field.try_next().await {
            data.extend_from_slice(&chunk);
        }

        log::info!("Uploading file, {}", destination);
        upload_blob!(
            &gcs_client,
            constants::BUCKET_FOR_TEMP,
            destination.clone(),
            data
        );
        let url = format!(
            "https://storage.googleapis.com/{}/{}",
            constants::BUCKET_FOR_TEMP,
            destination
        );
        let file_name = std_Path::new(&destination)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let signed_url =
            storage::get_signed_url(&gcs_client, constants::BUCKET_FOR_TEMP, &destination).await;
        files.append(file_name.to_string(), url, signed_url);
    }
    return Ok(Json(files));
}
