use crate::constants::constants;
use crate::database::db;
use crate::model::blog::{
    Blog, BlogError, BlogIdentifier, BlogPreview, BlogPublishOperation, BlogUpdateOperation,
    FileInfo, UploadedFiles,
};
use crate::templates;
use crate::utils::datetime;
use crate::utils::html::{minify_html, render_template};
use crate::utils::io::get_temp_file_path;
use crate::utils::md::convert_to_html;
use crate::utils::storage;
use crate::utils::validations::validate_id;
use actix_multipart::Multipart;
use actix_web::http::header::{ContentType, CONTENT_LENGTH};
use actix_web::http::StatusCode;
use actix_web::{
    delete, post, put,
    web::{Data, Form, Json, Path},
    HttpRequest, HttpResponse,
};
use aws_sdk_s3 as s3;
use futures_util::TryStreamExt;
use mime::{Mime, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG};
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use std::path::Path as std_Path;
use std::str::FromStr;

macro_rules! delete_blob {
    ($client:expr, $file_url:expr) => {
        let (bucket, obj_name) = storage::extract_bucket_and_blob_from_url($file_url);
        if bucket.is_empty() || obj_name.is_empty() {
            return Err(BlogError::InternalServerError);
        }
        if !storage::delete_blob($client, &bucket, &obj_name).await {
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

fn change_obj_prefix(obj: &str, blog_id: &str, old_prefix: &str, new_prefix: &str) -> String {
    if !obj.starts_with(old_prefix) {
        log::error!("Object doesn't start with the old prefix");
        return obj.to_string();
    }
    let obj_without_prefix = &obj[old_prefix.len()..];
    format!("{}/{}{}", new_prefix, blog_id, obj_without_prefix)
}

#[post("/api/admin/ws/blog/preview")]
async fn preview_blog(data: Form<BlogPreview>) -> HttpResponse {
    let content = data.get_content();
    if content.is_empty() {
        return HttpResponse::Ok().body("");
    }
    let preview = convert_to_html(content, None);
    let minified = minify_html(&preview);
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(minified)
}

async fn process_file(
    blog_id: &str,
    file: &mut FileInfo,
    content: &mut String,
    s3_client: &s3::Client,
) -> Result<(), BlogError> {
    if file.url.is_empty() {
        return Err(BlogError::FileIsEmpty);
    }

    let signed_url = match &file.signed_url {
        Some(url) => &url.clone(),
        None => {
            return Ok(());
        }
    };

    // check if the signed_url is in the content
    if !content.contains(signed_url) {
        storage::remove_file_from_md_content(content, signed_url);
        return Ok(());
    }

    let (bucket, obj_name) = storage::extract_bucket_and_blob_from_url(&file.url);
    if bucket.is_empty() || obj_name.is_empty() {
        return Err(BlogError::InternalServerError);
    }

    // replace the signed url with the actual url
    let obj_name_with_changed_prefix = &change_obj_prefix(
        &obj_name,
        blog_id,
        constants::TEMP_OBJ_PREFIX,
        constants::BLOG_OBJ_PREFIX,
    );
    let new_url = format!(
        "{}/{}",
        constants::PUBLIC_S3_URL,
        obj_name_with_changed_prefix,
    );
    let signed_url_idx = match content.find(signed_url) {
        Some(idx) => idx,
        None => {
            log::warn!("Signed url not found in content");
            return Ok(());
        }
    };
    content.replace_range(signed_url_idx..signed_url_idx + signed_url.len(), &new_url);
    file.signed_url = None;
    file.url = new_url;

    move_blob!(
        &s3_client,
        &bucket,
        &obj_name,
        constants::BUCKET,
        obj_name_with_changed_prefix
    );
    return Ok(());
}

macro_rules! process_file {
    ($blog_id:expr, $file:expr, $content:expr, $s3_client:expr) => {
        match process_file($blog_id, $file, $content, $s3_client).await {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }
    };
}

#[post("/api/new/blog")]
async fn new_blog(
    client: Data<db::DbClient>,
    s3_client: Data<s3::Client>,
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

    if blog_op.content.is_empty() {
        return Err(BlogError::EmptyContent);
    }

    let mut blog = Blog::new(
        title,
        String::new(),
        &blog_op.tags,
        &vec![],
        blog_op.is_public,
    );
    let blog_id = blog.get_id_string();

    for file in blog_op.files.iter_mut() {
        process_file!(&blog_id, file, &mut blog_op.content, &s3_client);
    }
    blog.files = blog_op.files;
    blog.content = blog_op.content;

    match blog_col.insert_one(blog, None).await {
        Ok(result) => {
            let id = result.inserted_id.as_object_id().unwrap();
            Ok(HttpResponse::Ok().body(id.to_hex()))
        }
        Err(err) => {
            log::error!("Failed to create api in database: {:?}", err);
            Err(BlogError::PublishBlogError)
        }
    }
}

#[put("/api/blog/update")]
async fn update_blog(
    client: Data<db::DbClient>,
    s3_client: Data<s3::Client>,
    update_blog: Json<BlogUpdateOperation>,
) -> Result<HttpResponse, BlogError> {
    let mut blog = update_blog.into_inner();
    let blog_id = validate_id(&blog.id)?;
    let blog_id_str = blog_id.to_hex();

    let new_tags = blog.tags;
    if new_tags.len() > constants::MAX_TAGS {
        return Err(BlogError::TooManyTags);
    }

    let options = FindOneOptions::builder()
        .projection(doc! { "title": 1, "content": 1, "tags": 1, "files": 1, "is_public": 1 })
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

    let mut update_file_flag = false;
    let old_files = blog_in_db.files.unwrap_or(vec![]);
    let mut files_to_put_in_db = Vec::with_capacity(blog.new_files.len() + old_files.len());
    if blog.new_files.len() > 0 {
        update_file_flag = true;
        for file in blog.new_files.iter_mut() {
            process_file!(&blog_id_str, file, &mut blog.content, &s3_client);
        }
        files_to_put_in_db = blog.new_files;
    }

    // check if the old_files are in the content
    for file in old_files.into_iter() {
        if blog.content.contains(&file.url) {
            files_to_put_in_db.push(file);
            continue;
        }
        if !update_file_flag {
            update_file_flag = true;
        }
        delete_blob!(&s3_client, &file.url);
    }

    if update_file_flag {
        is_updating = true;
        set_doc.insert("files", files_to_put_in_db);
    }

    let old_blog_content = blog_in_db.content.unwrap_or_default();
    let blog_content = blog.content;
    // do an is_empty() check as the content shouldn't be empty
    if !blog_content.is_empty() && blog_content != old_blog_content {
        is_updating = true;
        set_doc.insert("content", &blog_content);
    }

    let title = blog.title;
    // do an is_empty() check as the title shouldn't be empty
    if !title.is_empty() && title != blog_in_db.title.unwrap_or_default() {
        is_updating = true;
        set_doc.insert("title", title);
    }

    if blog.is_public != blog_in_db.is_public.unwrap_or(false) {
        is_updating = true;
        set_doc.insert("is_public", blog.is_public);
    }

    let old_tags = blog_in_db.tags.unwrap_or(vec![]);
    if new_tags.len() != old_tags.len() || new_tags != old_tags {
        is_updating = true;
        set_doc.insert("tags", new_tags);
    }

    if !is_updating {
        return Ok(HttpResponse::Ok().body(old_blog_content));
    }

    log::info!("set doc: {:?}", set_doc);
    let query = doc! { "_id": blog_id };
    let update = doc! { "$set": set_doc };
    let blog_col = client.into_inner().get_blog_collection();
    match blog_col.update_one(query, update, None).await {
        Ok(_) => Ok(HttpResponse::Ok().body(blog_content)),
        Err(err) => {
            log::error!("Failed to update api in database: {:?}", err);
            Err(BlogError::UpdateBlogError)
        }
    }
}

#[delete("/api/blogs/{id}/delete")]
async fn delete_blog(
    client: Data<db::DbClient>,
    s3_client: Data<s3::Client>,
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
        delete_blob!(&s3_client, &file.url);
    }

    let blog_col = client.into_inner().get_blog_collection();
    match blog_col.delete_one(doc! { "_id": blog_id }, None).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Blog deleted successfully".to_string())),
        Err(err) => {
            log::error!("Failed to delete api from database: {:?}", err);
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
            let response = if is_public {
                render_template(templates::admin::Unlocked, StatusCode::OK)
            } else {
                render_template(templates::admin::Locked, StatusCode::OK)
            };
            Ok(response)
        }
        Err(err) => {
            log::error!("Failed to publish api in database: {:?}", err);
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
    s3_client: Data<s3::Client>,
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
        upload_blob!(&s3_client, constants::BUCKET_FOR_TEMP, &destination, data);
        let url = format!(
            "https://{}.{}.r2.cloudflarestorage.com/{}",
            constants::BUCKET_FOR_TEMP,
            std::env::var(constants::R2_ACCOUNT_ID).unwrap(),
            destination
        );
        let file_name = std_Path::new(&destination)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let signed_url =
            storage::get_signed_url(&s3_client, constants::BUCKET_FOR_TEMP, &destination).await;
        files.append(file_name.to_string(), url, signed_url);
    }
    return Ok(Json(files));
}
