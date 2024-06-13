use crate::constants::constants;
use crate::database::db;
use crate::model::blog::{
    Blog, BlogError, BlogIdentifier, BlogPublishOperation, BlogResponse, BlogUpdateOperation,
    UploadedImages,
};
use crate::utils::datetime;
use crate::utils::io::get_temp_file_path;
use crate::utils::storage;
use actix_multipart::Multipart;
use actix_web::http::header::CONTENT_LENGTH;
use actix_web::{
    delete, get, post, put, web::Data, web::Json, web::Path, HttpRequest, HttpResponse,
};
use aws_sdk_s3 as s3;
use futures_util::TryStreamExt;
use mime::{Mime, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG};
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOneOptions;
use std::str::FromStr;
use tokio::fs;
use tokio::io::AsyncWriteExt;

macro_rules! delete_blob {
    ($client:expr, $bucket:expr, $name:expr) => {
        match storage::delete_blob($client, $bucket, $name).await {
            Ok(_) => (),
            Err(err) => {
                log::error!("Failed to delete image from GCP Storage: {}", err);
                return Err(BlogError::InternalServerError);
            }
        }
    };
}

macro_rules! move_blob {
    ($client:expr, $source_bucket:expr, $source_name:expr, $destination_bucket:expr, $destination_name:expr) => {
        match storage::copy_blob(
            $client,
            $source_bucket,
            $source_name,
            $destination_bucket,
            $destination_name,
        )
        .await
        {
            Ok(_) => (),
            Err(err) => {
                log::error!("Failed to move image to GCP Storage: {}", err);
                return Err(BlogError::ImageUploadError);
            }
        }
    };
}

macro_rules! upload_blob {
    ($client:expr, $bucket:expr, $name:expr, $file_name:expr) => {
        match storage::upload_blob($client, $bucket, $name, $file_name).await {
            Ok(_) => (),
            Err(err) => {
                log::error!("Failed to upload image to GCP Storage: {}", err);
                return Err(BlogError::ImageUploadError);
            }
        }
    };
}

fn change_obj_prefix(obj: &str, old_prefix: &str, new_prefix: &str) -> String {
    if obj.len() < old_prefix.len() {
        return obj.to_string();
    }

    let mut obj = obj.to_string();
    if &obj[0..old_prefix.len()] == old_prefix {
        obj.replace_range(0..old_prefix.len(), new_prefix);
    }
    return obj;
}

fn validate_id(id: &str) -> Result<ObjectId, BlogError> {
    match ObjectId::from_str(id) {
        Ok(id) => Ok(id),
        Err(_) => Err(BlogError::InvalidObjectId),
    }
}

#[get("/blog/{id}")]
async fn get_blog(
    client: Data<db::DbClient>,
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

#[get("/blog/exists/{id}")]
async fn blog_exists(
    client: Data<db::DbClient>,
    blog_identifier: Path<BlogIdentifier>,
) -> Result<HttpResponse, BlogError> {
    let blog_id = validate_id(&blog_identifier.into_inner().get_id())?;

    let options = FindOneOptions::builder()
        .projection(doc! { "_id": 1 })
        .build();
    client.get_blog_post(&blog_id, Some(options)).await?;
    Ok(HttpResponse::Ok().body("blog exists".to_string()))
}

#[post("/publish/blog")]
async fn publish_blog(
    client: Data<db::DbClient>,
    s3_client: Data<s3::Client>,
    blog: Json<BlogPublishOperation>,
) -> Result<HttpResponse, BlogError> {
    let blog_op = blog.into_inner();
    let blog_col = client.into_inner().get_blog_collection();

    let title = blog_op.get_title();
    if title.is_empty() {
        return Err(BlogError::EmptyTitle);
    } else if title.len() > constants::TITLE_MAX_LENGTH {
        return Err(BlogError::TitleTooLong);
    }

    let content = blog_op.get_content();
    if content.is_empty() {
        return Err(BlogError::EmptyContent);
    }

    let images = blog_op.get_images();
    for image in images.iter() {
        if image.is_empty() {
            return Err(BlogError::ImageIsEmpty);
        }

        move_blob!(
            &s3_client,
            constants::BUCKET,
            image,
            constants::BUCKET,
            &change_obj_prefix(
                image,
                constants::TEMP_OBJ_PREFIX,
                constants::BLOG_OBJ_PREFIX,
            )
        );
    }

    let blog = Blog::new(
        title.to_string(),
        content.to_string(),
        blog_op.get_tags(),
        images,
        blog_op.get_is_public(),
    );
    match blog_col.insert_one(blog, None).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Blog created successfully".to_string())),
        Err(err) => {
            log::error!("Failed to create blog in database: {}", err);
            Err(BlogError::PublishBlogError)
        }
    }
}

#[put("/update/blog")]
async fn update_blog(
    client: Data<db::DbClient>,
    s3_client: Data<s3::Client>,
    update_blog: Json<BlogUpdateOperation>,
) -> Result<HttpResponse, BlogError> {
    let blog = update_blog.into_inner();

    let blog_op_id = blog.get_id();
    if blog_op_id == "" {
        return Err(BlogError::InvalidObjectId);
    }

    let new_tags = blog.get_tags();
    if new_tags.len() > constants::MAX_TAGS {
        return Err(BlogError::TooManyTags);
    }

    let options = FindOneOptions::builder()
        .projection(doc! { "title": 1, "tags": 1, "images": 1, "is_public": 1 })
        .build();

    let blog_id = validate_id(blog_op_id)?;
    let blog_in_db = client.get_blog_post(&blog_id, Some(options)).await?;

    let mut is_updating = false;
    let last_modified = bson::DateTime::parse_rfc3339_str(datetime::get_dtnow_str())
        .expect("DateTime shouldn't fail to parse in update_blog");
    let mut set_doc = doc! {
        "last_modified": last_modified,
    };

    let old_images = blog_in_db.get_images();
    let new_images = blog.get_images();
    if new_images.iter().all(|image| old_images.contains(image)) {
        is_updating = true;

        // get all images not in old_images for uploading
        let images_to_upload: Vec<String> = new_images
            .iter()
            .filter(|image| !old_images.contains(image))
            .map(|image| image.to_string())
            .collect();
        for image in images_to_upload.iter() {
            if image.is_empty() {
                continue;
            }
            move_blob!(
                &s3_client,
                constants::BUCKET,
                image,
                constants::BUCKET,
                &change_obj_prefix(
                    image,
                    constants::TEMP_OBJ_PREFIX,
                    constants::BLOG_OBJ_PREFIX,
                )
            );
        }

        // get all images not in new_images for deletion
        let images_to_delete: Vec<String> = old_images
            .iter()
            .filter(|image| !new_images.contains(image))
            .map(|image| image.to_string())
            .collect();
        for image in images_to_delete.iter() {
            delete_blob!(&s3_client, constants::BUCKET, image);
        }
    }

    let title = blog.get_title();
    if !title.is_empty() && title != blog_in_db.get_title() {
        is_updating = true;
        set_doc.insert("title", title);
    }

    let content = blog.get_content();
    if !content.is_empty() {
        is_updating = true;
        set_doc.insert("content", content);
    }

    if let Some(is_public) = blog.get_is_public() {
        if is_public != blog_in_db.get_is_public() {
            is_updating = true;
            set_doc.insert("is_public", is_public);
        }
    }

    let old_tags = blog_in_db.get_tags();
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
            log::error!("Failed to update blog in database: {}", err);
            Err(BlogError::UpdateBlogError)
        }
    }
}

#[delete("/delete/blog")]
async fn delete_blog(
    client: Data<db::DbClient>,
    s3_client: Data<s3::Client>,
    blog_identifier: Json<BlogIdentifier>,
) -> Result<HttpResponse, BlogError> {
    let blog_id = validate_id(&blog_identifier.into_inner().get_id())?;

    let options = FindOneOptions::builder()
        .projection(doc! { "images": 1 })
        .build();
    let blog_data = client.get_blog_post(&blog_id, Some(options)).await?;

    for image in blog_data.get_images() {
        delete_blob!(&s3_client, constants::BUCKET, image);
    }

    let blog_col = client.into_inner().get_blog_collection();
    match blog_col.delete_one(doc! { "_id": blog_id }, None).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Blog deleted successfully".to_string())),
        Err(err) => {
            log::error!("Failed to delete blog from database: {}", err);
            Err(BlogError::InternalServerError)
        }
    }
}

#[post("/upload/images")]
async fn upload_blog_images(
    s3_client: Data<s3::Client>,
    mut payload: Multipart,
    req: HttpRequest,
) -> Result<Json<UploadedImages>, BlogError> {
    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(v) => v.to_str().unwrap_or("0").parse().unwrap(),
        None => 0,
    };

    if content_length == 0 {
        return Err(BlogError::ImageIsEmpty);
    } else if content_length > constants::MAX_THUMBNAIL_FILE_SIZE {
        return Err(BlogError::ImageTooLarge);
    }

    let mut images = UploadedImages::new(vec![]);
    let image_webp: Mime = Mime::from_str("image/webp").unwrap();
    let allowed_mimetypes: [Mime; 4] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF, image_webp.clone()];
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type: Option<&Mime> = field.content_type();
        if content_type.is_none() {
            continue;
        }

        let content_type = content_type.unwrap();
        if !allowed_mimetypes.contains(content_type) {
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
        } else {
            file_ext = "gif";
        }

        let destination = format!(
            "{}/{}.{}",
            constants::TEMP_OBJ_PREFIX,
            get_temp_file_path(),
            file_ext
        );
        let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
        while let Ok(Some(chunk)) = field.try_next().await {
            saved_file.write_all(&chunk).await.unwrap();
        }

        upload_blob!(&s3_client, constants::BUCKET, &destination, &destination);
        let url = constants::BUCKET.to_string() + "/" + &destination;
        images.append(url);
    }
    return Ok(Json(images));
}
