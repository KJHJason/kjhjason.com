use actix_files::NamedFile;
use actix_web::{get, Responder};

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    NamedFile::open_async("./static/images/favicon.ico")
        .await
        .expect("should be able to open favicon.ico file")
}

#[get("/robots.txt")]
async fn robots_txt() -> impl Responder {
    NamedFile::open_async("./seo/robots.txt")
        .await
        .expect("should be able to open robots.txt file")
}

#[get("/sitemap.xml")]
async fn sitemap_xml() -> impl Responder {
    NamedFile::open_async("./seo/sitemap.xml")
        .await
        .expect("should be able to open sitemap.xml file")
}

#[get("/static/js/sweetalert2.min.js")]
async fn sweetalert_js() -> impl Responder {
    NamedFile::open_async("./node_modules/sweetalert2/dist/sweetalert2.min.js")
        .await
        .expect("should be able to open sweetalert2.min.js file")
}

#[get("/static/css/sweetalert2.min.css")]
async fn sweetalert_css() -> impl Responder {
    NamedFile::open_async("./node_modules/sweetalert2/dist/sweetalert2.min.css")
        .await
        .expect("should be able to open sweetalert2.min.css file")
}

#[get("/static/js/htmx.org.min.js")]
async fn htmx_org_js() -> impl Responder {
    NamedFile::open_async("./node_modules/htmx.org/dist/htmx.min.js")
        .await
        .expect("should be able to open htmx.min.js file")
}

#[get("/static/js/htmx-ext-response-targets.js")]
async fn htmx_ext_response_targets_js() -> impl Responder {
    NamedFile::open_async("./node_modules/htmx-ext-response-targets/response-targets.js")
        .await
        .expect("should be able to open response-targets.js file")
}
