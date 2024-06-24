use actix_files::NamedFile;
use actix_web::{get, Responder};

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    NamedFile::open_async("./static/images/favicon.ico")
        .await
        .expect("should be able to open favicon.ico file")
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
