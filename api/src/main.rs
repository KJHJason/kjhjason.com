mod constants;
mod content;
mod database;
mod model;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer};
use content::api::{delete_blog, get_blog, publish_blog, update_blog};
use database::db;
use model::index::Index;
use serde_json;

#[get("/")]
async fn hello() -> HttpResponse {
    let serialised = serde_json::to_string_pretty(&Index::new()).unwrap();
    return HttpResponse::Ok()
        .content_type("application/json")
        .body(serialised);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let client = db::init_db().await.unwrap_or_else(|_| {
        let error_message = "Failed to connect to database";
        log::error!("{error_message}");
        panic!("{error_message}")
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(Logger::default())
            .service(hello)
            .service(get_blog)
            .service(publish_blog)
            .service(update_blog)
            .service(delete_blog)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
