use crate::constants::constants;
use crate::database::db;
use crate::errors::blog::BlogError;
use crate::models::{
    blog::Blog, blog_identifier::BlogIdentifier, blog_preview::BlogPreview, new_blog::NewBlog,
    update_blog::UpdateBlog, uploaded_files::UploadedFiles,
};
use crate::utils::blog::file_utils;
use crate::utils::blog::file_utils::process_file_logic;
use crate::utils::blog::publish_utils;
use crate::utils::datetime;
use crate::utils::html::minify_html;
use crate::utils::io::get_temp_file_path;
use crate::utils::md::convert_to_html;
use crate::utils::storage;
use crate::utils::validations::validate_id;
use actix_multipart::Multipart;
use actix_web::http::header::{ContentType, CONTENT_LENGTH};
use actix_web::{
    delete, patch, post,
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

#[post("/api/new/blog")]
async fn new_blog(
    client: Data<db::DbClient>,
    s3_client: Data<s3::Client>,
    blog: Json<NewBlog>,
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
        file_utils::process_file!(&blog_id, file, &mut blog_op.content, &s3_client);
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

#[patch("/api/blog/update")]
async fn update_blog(
    client: Data<db::DbClient>,
    s3_client: Data<s3::Client>,
    update_blog: Json<UpdateBlog>,
) -> Result<HttpResponse, BlogError> {
    let blog: UpdateBlog = update_blog.into_inner();
    let blog_id = validate_id(&blog.id)?;
    let blog_id_str = blog_id.to_hex();

    let updating_tags = !blog.tags.is_none();
    let new_tags = blog.tags.unwrap_or(vec![]);
    if updating_tags {
        if new_tags.len() > constants::MAX_TAGS {
            return Err(BlogError::TooManyTags);
        }
    }

    let updating_content = !blog.content.is_none();
    let updating_files = updating_content || !blog.new_files.is_none();
    let updating_title = !blog.title.is_none();
    let updating_public = !blog.is_public.is_none();
    let no_changes = !updating_content
        && !updating_files
        && !updating_title
        && !updating_tags
        && !updating_public;
    if no_changes {
        return Ok(HttpResponse::Ok().body("No changes to update".to_string()));
    }

    let mut projection_doc = doc! {};
    if updating_title {
        projection_doc.insert("title", 1);
    }
    if updating_content {
        projection_doc.insert("content", 1);
    }
    if updating_tags {
        projection_doc.insert("tags", 1);
    }
    if updating_files {
        projection_doc.insert("files", 1);
    }
    if updating_public {
        projection_doc.insert("is_public", 1);
    }

    let options = FindOneOptions::builder().projection(projection_doc).build();
    let blog_in_db = client
        .get_projected_blog_post(&blog_id, Some(options))
        .await?;

    let mut is_updating = false;
    let last_modified = bson::DateTime::parse_rfc3339_str(datetime::get_dtnow_str())
        .expect("DateTime shouldn't fail to parse in update_blog");
    let mut set_doc = doc! {
        "last_modified": last_modified,
    };

    let mut blog_content = blog.content.unwrap_or_default();
    if updating_files {
        let mut update_file_flag = false; // initialise it to false as it could be an empty slice.
        let old_files = blog_in_db.files.unwrap_or(vec![]);
        let mut new_files = blog.new_files.unwrap_or(vec![]);
        let mut files_to_put_in_db = Vec::with_capacity(new_files.len() + old_files.len());
        if new_files.len() > 0 {
            update_file_flag = true;
            for file in new_files.iter_mut() {
                file_utils::process_file!(&blog_id_str, file, &mut blog_content, &s3_client);
            }
            files_to_put_in_db = new_files;
        }

        // check if the old_files are in the content
        for file in old_files.into_iter() {
            if blog_content.contains(&file.url) {
                files_to_put_in_db.push(file);
                continue;
            }
            if !update_file_flag {
                update_file_flag = true;
            }
            file_utils::delete_blob!(&s3_client, &file.url);
        }

        if update_file_flag {
            is_updating = true;
            set_doc.insert("files", files_to_put_in_db);
        }
    }

    let old_blog_content = blog_in_db.content.unwrap_or_default();
    if updating_content && !blog_content.is_empty() && blog_content != old_blog_content {
        is_updating = true;
        set_doc.insert("content", &blog_content);
    }

    let title = blog.title.unwrap_or_default();
    if updating_title && !title.is_empty() && title != blog_in_db.title.unwrap_or_default() {
        is_updating = true;
        set_doc.insert("title", title);
    }

    let is_public = blog.is_public.unwrap_or_default();
    if updating_public && is_public != blog_in_db.is_public.unwrap_or(false) {
        is_updating = true;
        set_doc.insert("is_public", is_public);
    }

    let old_tags = blog_in_db.tags.unwrap_or(vec![]);
    if updating_tags && new_tags.len() != old_tags.len() || new_tags != old_tags {
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
        .get_projected_blog_post(&blog_id, Some(options))
        .await?;

    let files = blog_data.files.unwrap_or(vec![]);
    for file in files.iter() {
        file_utils::delete_blob!(&s3_client, &file.url);
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

#[patch("/api/blogs/{id}/publish")]
async fn publish_blog_post(
    client: Data<db::DbClient>,
    blog_identifier: Path<BlogIdentifier>,
) -> Result<HttpResponse, BlogError> {
    publish_utils::configure_blog_post_bool(client, &blog_identifier.into_inner().id, true).await
}

#[patch("/api/blogs/{id}/unpublish")]
async fn unpublish_blog_post(
    client: Data<db::DbClient>,
    blog_identifier: Path<BlogIdentifier>,
) -> Result<HttpResponse, BlogError> {
    publish_utils::configure_blog_post_bool(client, &blog_identifier.into_inner().id, false).await
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
        let content_type_clone = content_type.clone();
        let file_ext = if content_type_clone == image_webp {
            "webp"
        } else if content_type_clone == IMAGE_PNG {
            "png"
        } else if content_type_clone == IMAGE_JPEG {
            "jpeg"
        } else if content_type_clone == IMAGE_GIF {
            "gif"
        } else {
            "mp4"
        };

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
        file_utils::upload_blob!(&s3_client, constants::BUCKET_FOR_TEMP, &destination, data);
        let url = format!(
            "https://{}.{}.r2.cloudflarestorage.com/{}",
            constants::BUCKET_FOR_TEMP,
            constants::get_r2_acc_id(),
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
