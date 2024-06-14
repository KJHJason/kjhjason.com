use crate::utils::security::{extract_for_template, TemplateValues};
use actix_web::dev::ServiceResponse;
use actix_web::http::header::ContentType;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{HttpResponseBuilder, Result};
use askama_actix::Template;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a> {
    common: TemplateValues,
    pub status: u16,
    pub message: &'a str,
}

// mostly thanks to https://www.reddit.com/r/rust/comments/wu69kt/how_to_display_an_error_page_in_actix_web_using/
pub fn render_error<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let request = res.request(); // Borrow the request part
    if request.path().starts_with("/api") {
        return Ok(ErrorHandlerResponse::Response(res.map_into_left_body()));
    }

    let status = res.status();
    let request = res.into_parts().0;
    let html = ErrorTemplate {
        common: extract_for_template(&request),
        status: status.as_u16(),
        message: status.canonical_reason().unwrap_or("Unknown"),
    }
    .render()
    .unwrap_or(format!("error: {}", status.as_u16()));

    let new_response = HttpResponseBuilder::new(status)
        .insert_header(ContentType::html())
        .body(html);

    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(request, new_response).map_into_right_body(),
    ))
}
