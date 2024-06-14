mod api;
mod client;
mod constants;
mod database;
mod middleware;
mod model;
mod security;
mod utils;

use actix_files::NamedFile;
use actix_web::{get, middleware::Logger, web, App, HttpServer};
use api::configure::add_api_routes;
use client::configure::add_client_routes;
use database::db;
use dotenv::dotenv;
use google_cloud_auth::credentials::CredentialsFile;
use google_cloud_storage::client::{Client as GcsClient, ClientConfig};
use middleware::configure::{
    configure_auth_middleware, configure_cache_control_middleware, configure_csp_middleware,
    configure_csrf_middleware, configure_hsts_middleware,
};

#[get("/favicon.ico")]
async fn favicon() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./static/images/favicon.ico")?)
}

async fn get_gcp_cred_file() -> CredentialsFile {
    let path = if !constants::constants::DEBUG_MODE {
        "/gcp/storage" // to be mounted as a secret
    } else {
        "gcp-storage.json"
    };
    match CredentialsFile::new_from_file(path.to_string()).await {
        Ok(cred_file) => cred_file,
        Err(_) => panic!("Failed to get GCP credentials"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    log::info!("Initialising Blog Web App...");

    dotenv().ok();
    let db_future = async {
        db::init_db()
            .await
            .unwrap_or_else(|_| panic!("Failed to connect to database"))
    };
    let gcp_future = async {
        let creds = get_gcp_cred_file().await;
        let config = ClientConfig::default()
            .with_credentials(creds)
            .await
            .unwrap_or_else(|_| panic!("Failed to parse GCP client config"));
        GcsClient::new(config)
    };
    let (db_client, client) = tokio::join!(db_future, gcp_future);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .app_data(web::Data::new(client.clone()))
            .wrap(Logger::default())
            .wrap(middleware::content_type::ContentTypeMiddleware)
            .wrap(configure_csrf_middleware())
            .wrap(configure_auth_middleware())
            .wrap(configure_csp_middleware())
            .wrap(configure_hsts_middleware())
            .wrap(configure_cache_control_middleware())
            .configure(add_client_routes)
            .configure(add_api_routes)
            .service(favicon)
            // Note: index file is added here as an error will be thrown if the file in the static path is not found
            // e.g. /static/test.png will return some error text which is not ideal.
            .service(actix_files::Files::new("/static", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
