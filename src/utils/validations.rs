use crate::model::blog::{BlogError, BlogIdentifier};
use crate::templates::error::ErrorTemplate;
use crate::utils::security::extract_for_template;
use actix_web::http::header::ContentType;
use actix_web::web::Path;
use actix_web::{HttpRequest, HttpResponse};
use askama::Template;
use bson::oid::ObjectId;
use std::str::FromStr;

pub fn validate_id(id: &str) -> Result<ObjectId, BlogError> {
    match ObjectId::from_str(id) {
        Ok(id) => Ok(id),
        Err(_) => Err(BlogError::InvalidObjectId),
    }
}

pub fn get_id_from_path(
    req: &HttpRequest,
    blog_identifier: Path<BlogIdentifier>,
) -> Result<ObjectId, HttpResponse> {
    match validate_id(&blog_identifier.into_inner().id) {
        Ok(blog_id) => Ok(blog_id),
        Err(_) => {
            let html = ErrorTemplate {
                common: extract_for_template(&req),
                status: 400,
                message: "Invalid blog post ID",
            }
            .render()
            .unwrap();

            let err_response = HttpResponse::BadRequest()
                .content_type(ContentType::html())
                .body(html);
            Err(err_response)
        }
    }
}
