use crate::constants::constants::{
    GCP_BUCKET, MAX_FILES_ALLOWED, MAX_TAGS, MAX_THUMBNAIL_FILE_SIZE, TITLE_MAX_LENGTH,
};
use crate::database::db::DbClient;
use crate::model::base_msg::Msg;
use crate::model::blog::{
    Blog, BlogError, BlogIdentifier, BlogPublishOperation, BlogResponse, BlogUpdateOperation,
};
use crate::utils::io::get_temp_file_path;
use actix_multipart::Multipart;
use actix_web::http::header::CONTENT_LENGTH;
use actix_web::{delete, get, post, put, web::Data, web::Json, web::Path, HttpRequest};
use chrono::Utc;
use google_cloud_storage::client::Client;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use mime::{Mime, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use std::str::FromStr;
use tokio::fs;
use tokio::io::AsyncWriteExt as _;

fn validate_id(id: &str) -> Result<ObjectId, BlogError> {
    match ObjectId::from_str(id) {
        Ok(id) => Ok(id),
        Err(_) => Err(BlogError::InvalidObjectId),
    }
}

#[get("/blog/{id}")]
async fn get_blog(
    client: Data<DbClient>,
    blog_identifier: Path<BlogIdentifier>,
) -> Result<Json<BlogResponse>, BlogError> {
    let blog_id = validate_id(&blog_identifier.into_inner().get_id())?;

    // get blog from database
    let blog_col = client.into_inner().get_blog_collection();
    match blog_col.find_one(doc! { "_id": blog_id }, None).await {
        Ok(Some(blog)) => {
            if !blog.get_is_public() {
                return Err(BlogError::BlogNotFound);
            }
            Ok(Json(BlogResponse::from(blog)))
        }
        Ok(None) => Err(BlogError::BlogNotFound),
        Err(err) => {
            log::error!("Failed to get blog from database: {}", err);
            Err(BlogError::InternalServerError)
        }
    }
}

#[post("/publish/blog")]
async fn publish_blog(
    client: Data<DbClient>,
    blog: Json<BlogPublishOperation>,
) -> Result<Json<Msg>, BlogError> {
    let blog_op = blog.into_inner();
    let blog_col = client.into_inner().get_blog_collection();

    let title = blog_op.get_title();
    if title.is_empty() {
        return Err(BlogError::EmptyTitle);
    } else if title.len() > TITLE_MAX_LENGTH {
        return Err(BlogError::TitleTooLong);
    }

    let content = blog_op.get_content();
    if content.is_empty() {
        return Err(BlogError::EmptyContent);
    }

    let blog = Blog::new(
        title.to_string(),
        content.to_string(),
        blog_op.get_tags(),
        blog_op.get_is_public(),
    );
    match blog_col.insert_one(blog, None).await {
        Ok(_) => Ok(Json(Msg::new("Blog created successfully".to_string()))),
        Err(err) => {
            log::error!("Failed to create blog in database: {}", err);
            Err(BlogError::PublishBlogError)
        }
    }
}

#[put("/update/blog")]
async fn update_blog(
    client: Data<DbClient>,
    update_blog: Json<BlogUpdateOperation>,
) -> Result<Json<Msg>, BlogError> {
    let blog = update_blog.into_inner();

    let blog_op_id = blog.get_id();
    if blog_op_id == "" {
        return Err(BlogError::InvalidObjectId);
    }

    let new_tags = blog.get_tags();
    if new_tags.len() > MAX_TAGS {
        return Err(BlogError::TooManyTags);
    }

    let blog_id = validate_id(blog_op_id)?;
    let blog_col = client.into_inner().get_blog_collection();

    let query = doc! { "_id": blog_id };
    let blog_in_db = match blog_col.find_one(query.clone(), None).await {
        Ok(Some(blog)) => blog,
        Ok(None) => return Err(BlogError::BlogNotFound),
        Err(err) => {
            log::error!("Failed to get blog from database: {}", err);
            return Err(BlogError::InternalServerError);
        }
    };

    let mut is_updating = false;
    let mut set_doc = doc! {
        "last_modified": Utc::now().timestamp(),
    };

    let title = blog.get_title();
    if !title.is_empty() {
        is_updating = true;
        set_doc.insert("title", title);
    }

    let content = blog.get_content();
    if !content.is_empty() {
        is_updating = true;
        set_doc.insert("content", content);
    }

    if let Some(is_public) = blog.get_is_public() {
        is_updating = true;
        set_doc.insert("is_public", is_public);
    }

    let old_tags = blog_in_db.get_tags();
    // O(n^2) algorithm but the no. of tags must be less than 8 so it's technically O(1)
    if new_tags.len() != old_tags.len() || new_tags.iter().all(|tag| old_tags.contains(tag)) {
        is_updating = true;
        set_doc.insert("tags", new_tags);
    }

    let success_json = Json(Msg::new("Blog updated successfully".to_string()));
    if !is_updating {
        return Ok(success_json);
    }

    let update = doc! { "$set": set_doc };
    match blog_col.update_one(query, update, None).await {
        Ok(_) => Ok(success_json),
        Err(err) => {
            log::error!("Failed to update blog in database: {}", err);
            Err(BlogError::UpdateBlogError)
        }
    }
}

#[delete("/delete/blog")]
async fn delete_blog(
    client: Data<DbClient>,
    blog_identifier: Json<BlogIdentifier>,
) -> Result<Json<Msg>, BlogError> {
    let blog_id = validate_id(&blog_identifier.into_inner().get_id())?;
    let blog_col = client.into_inner().get_blog_collection();

    let query = doc! { "_id": blog_id };
    let _ = match blog_col.find_one(query.clone(), None).await {
        Ok(Some(_)) => (),
        Ok(None) => return Err(BlogError::BlogNotFound),
        Err(err) => {
            log::error!("Failed to get blog from database: {}", err);
            return Err(BlogError::InternalServerError);
        }
    };

    match blog_col.delete_one(query, None).await {
        Ok(_) => Ok(Json(Msg::new("Blog deleted successfully".to_string()))),
        Err(err) => {
            log::error!("Failed to delete blog from database: {}", err);
            Err(BlogError::InternalServerError)
        }
    }
}

#[put("/upload/images")]
async fn upload_blog_thumbnail(
    client: Data<DbClient>,
    gs_client: Data<Client>,
    mut payload: Multipart,
    req: HttpRequest,
) -> Result<Json<Msg>, BlogError> {
    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(v) => v.to_str().unwrap_or("0").parse().unwrap(),
        None => 0,
    };

    if content_length == 0 {
        return Err(BlogError::ImageIsEmpty);
    } else if content_length > MAX_THUMBNAIL_FILE_SIZE {
        return Err(BlogError::ImageTooLarge);
    }

    let allowed_mimetypes: [Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF];
    let mut cur_cnt = 0;
    loop {
        if cur_cnt >= MAX_FILES_ALLOWED {
            return Err(BlogError::TooManyTags);
        }

        if let Ok(Some(mut field)) = payload.try_next().await {
            let content_type: Option<&Mime> = field.content_type();
            if content_type.is_none() {
                break;
            }
            if !allowed_mimetypes.contains(content_type.unwrap()) {
                continue;
            }

            let file_ext = match content_type.unwrap() {
                IMAGE_PNG => "png",
                IMAGE_JPEG => "jpeg",
                IMAGE_GIF => "gif",
                _ => panic!("Invalid image type"), // shouldn't happen
            };
            let destination = format!("{}.{}", get_temp_file_path(), file_ext,);
            let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await {
                saved_file.write_all(&chunk).await.unwrap();
            }

            // save file_data to GCP Storage
            let bytes = fs::read(&destination).await.unwrap();
            fs::remove_file(&destination).await.unwrap();

            let upload_type = UploadType::Simple(Media::new(destination));
            let uploaded = gs_client
                .upload_object(
                    &UploadObjectRequest {
                        bucket: GCP_BUCKET.to_string(),
                        ..Default::default()
                    },
                    bytes,
                    &upload_type,
                )
                .await;
            match uploaded {
                Ok(_) => (),
                Err(err) => {
                    log::error!("Failed to upload image to GCP Storage: {}", err);
                    return Err(BlogError::ImageUploadError);
                }
            };

            // TODO: add images to database
        } else {
            break;
        }

        cur_cnt += 1;
    }

    return Ok(Json(Msg::new("Image uploaded successfully".to_string())));
}
