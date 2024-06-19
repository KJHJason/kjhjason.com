use crate::model::blog::{BlogError, BlogIdentifier};
use crate::templates::error::ErrorTemplate;
use crate::utils::html::render_template;
use crate::utils::security::extract_for_template;
use actix_web::http::StatusCode;
use actix_web::web::Path;
use actix_web::{HttpRequest, HttpResponse};
use bson::oid::ObjectId;
use std::str::FromStr;

#[inline]
pub fn validate_id(id: &str) -> Result<ObjectId, BlogError> {
    match ObjectId::from_str(id) {
        Ok(id) => Ok(id),
        Err(_) => Err(BlogError::InvalidObjectId),
    }
}

#[inline]
pub fn get_id_from_path(
    req: &HttpRequest,
    blog_identifier: Path<BlogIdentifier>,
) -> Result<ObjectId, HttpResponse> {
    match validate_id(&blog_identifier.into_inner().id) {
        Ok(blog_id) => Ok(blog_id),
        Err(_) => {
            let template = ErrorTemplate {
                common: extract_for_template(&req),
                status: 400,
                message: "Invalid blog post ID",
            };
            let response = render_template(template, StatusCode::BAD_REQUEST);
            Err(response)
        }
    }
}
