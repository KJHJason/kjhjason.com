use crate::database::db;
use crate::model::blog::{BlogError, BlogIdentifier, BlogResponse};
use crate::utils::validations::validate_id;
use actix_web::{get, web::Data, web::Json, web::Path, HttpResponse};
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;

#[get("/api/blog/{id}")]
async fn get_blog(
    client: Data<db::DbClient>,
    blog_identifier: Path<BlogIdentifier>,
) -> Result<Json<BlogResponse>, BlogError> {
    let blog_id = validate_id(&blog_identifier.into_inner().id)?;

    // get api from database
    let blog_col = client.into_inner().get_blog_collection();
    match blog_col.find_one(doc! { "_id": blog_id }, None).await {
        Ok(Some(blog)) => {
            if !blog.is_public {
                return Err(BlogError::BlogNotFound);
            }
            Ok(Json(BlogResponse::from(blog)))
        }
        Ok(None) => Err(BlogError::BlogNotFound),
        Err(err) => {
            log::error!("Failed to get api from database: {:?}", err);
            Err(BlogError::InternalServerError)
        }
    }
}

#[get("/api/blog/exists/{id}")]
async fn blog_exists(
    client: Data<db::DbClient>,
    blog_identifier: Path<BlogIdentifier>,
) -> Result<HttpResponse, BlogError> {
    let blog_id = validate_id(&blog_identifier.into_inner().id)?;

    let options = FindOneOptions::builder()
        .projection(doc! { "_id": 1 })
        .build();
    client.get_blog_post(&blog_id, Some(options)).await?;
    Ok(HttpResponse::Ok().body("api exists".to_string()))
}
