use actix_web::http::StatusCode;
use actix_web::{http::header::ContentType, HttpResponse};
use askama_actix::Template;
use minify_html::{minify, Cfg};

macro_rules! render_askama_template {
    ($template:expr) => {
        $template.render().unwrap_or_else(|_| {
            log::error!("Failed to render template");
            String::new()
        })
    };
}

#[inline]
pub fn minify_html(html: &str) -> Vec<u8> {
    let html_bytes = html.as_bytes().to_vec();
    let minify_cfg = Cfg {
        minify_js: true,
        ..Default::default()
    };
    minify(&html_bytes, &minify_cfg)
}

#[inline]
pub fn render_template<T: Template>(template: T, status_code: StatusCode) -> HttpResponse {
    let html = render_askama_template!(template);
    let minified = minify_html(&html);
    HttpResponse::build(status_code)
        .content_type(ContentType::html())
        .body(minified)
}
