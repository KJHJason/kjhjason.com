use actix_web::http::StatusCode;
use actix_web::{http::header::ContentType, HttpResponse};
use askama_actix::Template;
use minify_html::{minify, Cfg};

pub fn render_template<T: Template>(template: T, status_code: StatusCode) -> HttpResponse {
    let html = template.render().unwrap_or_else(|_| {
        log::error!("Failed to render template");
        String::new()
    });
    let html_bytes = html.as_bytes().to_vec();
    let minify_cfg = Cfg {
        minify_js: true,
        ..Default::default()
    };
    let minified = minify(&html_bytes, &minify_cfg);
    HttpResponse::build(status_code)
        .content_type(ContentType::html())
        .body(minified)
}
