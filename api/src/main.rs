mod blog;
mod constants;
mod database;
mod model;
mod utils;
mod security;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer};
use aws_config::BehaviorVersion;
use aws_sdk_s3 as s3;
use blog::api::{delete_blog, get_blog, publish_blog, update_blog, upload_blog_images};
use database::db;
use dotenv::dotenv;
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
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    dotenv().ok();
    let db_client = db::init_db()
        .await
        .unwrap_or_else(|_| panic!("Failed to connect to database"));

    let api_endpoint = std::env::var(constants::constants::AWS_ENDPOINT_URL).unwrap();
    let mut config = aws_config::defaults(BehaviorVersion::latest());
    config = config.endpoint_url(api_endpoint);
    config =
        config.app_name(aws_config::AppName::new("blog".to_string()).expect("Invalid app name"));
    let config = config.load().await;

    let client = s3::Client::new(&config);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .app_data(web::Data::new(client.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(hello)
            .service(get_blog)
            .service(publish_blog)
            .service(update_blog)
            .service(delete_blog)
            .service(upload_blog_images)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
