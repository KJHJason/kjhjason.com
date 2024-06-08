mod blog;
mod constants;
mod database;
mod model;
mod utils;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer};
use blog::api::{delete_blog, get_blog, publish_blog, update_blog};
use constants::constants::DEBUG_MODE;
use database::db;
use google_cloud_auth::credentials::CredentialsFile;
use google_cloud_storage::client::{Client, ClientConfig};
use model::index::Index;
use serde_json;

#[get("/")]
async fn hello() -> HttpResponse {
    let serialised = serde_json::to_string_pretty(&Index::new()).unwrap();
    return HttpResponse::Ok()
        .content_type("application/json")
        .body(serialised);
}

async fn get_gcp_cred_file() -> CredentialsFile {
    let mut path = "gcp-storage.json";
    if !DEBUG_MODE {
        path = "/gcp/storage"; // to be mounted as a secret
    }
    match CredentialsFile::new_from_file(path.to_string()).await {
        Ok(cred_file) => cred_file,
        Err(_) => panic!("Failed to get GCP credentials"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let db_client = db::init_db()
        .await
        .unwrap_or_else(|_| panic!("Failed to connect to database"));

    let cred_file = get_gcp_cred_file().await;
    let config = ClientConfig::default()
        .with_credentials(cred_file)
        .await
        .unwrap_or_else(|_| panic!("Failed to parse GCP client config"));
    let gs_client = Client::new(config);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .app_data(web::Data::new(gs_client.clone()))
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
