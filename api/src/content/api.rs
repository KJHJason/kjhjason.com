use crate::constants::constants::TITLE_MAX_LENGTH;
use crate::database::db::DbClient;
use crate::model::base_msg::Msg;
use crate::model::blog::{
    Blog, BlogError, BlogIdentifier, BlogPublishOperation, BlogResponse, BlogUpdateOperation,
};
use actix_web::{delete, get, post, put, web::Data, web::Json, web::Path};
use chrono::Utc;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use std::str::FromStr;

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

    let blog_id = validate_id(blog_op_id)?;
    let blog_col = client.into_inner().get_blog_collection();

    let query = doc! { "_id": blog_id };
    match blog_col.find_one(query.clone(), None).await {
        Ok(Some(_)) => {
            let mut set_doc = doc! {
                "last_modified": Utc::now().timestamp(),
            };

            let mut is_updating = false;
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
        Ok(None) => Err(BlogError::BlogNotFound),
        Err(err) => {
            log::error!("Failed to get blog from database: {}", err);
            Err(BlogError::InternalServerError)
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
    match blog_col.find_one(query.clone(), None).await {
        Ok(Some(_)) => match blog_col.delete_one(query, None).await {
            Ok(_) => Ok(Json(Msg::new("Blog deleted successfully".to_string()))),
            Err(err) => {
                log::error!("Failed to delete blog from database: {}", err);
                Err(BlogError::InternalServerError)
            }
        },
        Ok(None) => Err(BlogError::BlogNotFound),
        Err(err) => {
            log::error!("Failed to get blog from database: {}", err);
            Err(BlogError::InternalServerError)
        }
    }
}
