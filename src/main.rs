mod api;
mod client;
mod constants;
mod database;
mod middleware;
mod model;
mod security;
mod templates;
mod utils;

use actix_files::NamedFile;
use actix_web::{
    get,
    middleware::{ErrorHandlers, Logger},
    web, App, HttpServer,
};
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
use middleware::errors::render_error;

#[macro_export]
macro_rules! error_handler_many {
    ($handler:ident, [$($variant:ident),*]) => {
        ErrorHandlers::new()
            $(.handler(actix_web::http::StatusCode::$variant, $handler))+
    }
}

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
            .wrap(error_handler_many!(
                render_error,
                [
                    BAD_REQUEST,
                    UNAUTHORIZED,
                    FORBIDDEN,
                    NOT_FOUND,
                    METHOD_NOT_ALLOWED,
                    NOT_ACCEPTABLE,
                    REQUEST_TIMEOUT,
                    GONE,
                    LENGTH_REQUIRED,
                    PAYLOAD_TOO_LARGE,
                    URI_TOO_LONG,
                    UNSUPPORTED_MEDIA_TYPE,
                    RANGE_NOT_SATISFIABLE,
                    IM_A_TEAPOT,
                    TOO_MANY_REQUESTS,
                    REQUEST_HEADER_FIELDS_TOO_LARGE,
                    MISDIRECTED_REQUEST,
                    UPGRADE_REQUIRED,
                    INTERNAL_SERVER_ERROR,
                    NOT_IMPLEMENTED,
                    SERVICE_UNAVAILABLE,
                    HTTP_VERSION_NOT_SUPPORTED
                ]
            ))
            .configure(add_client_routes)
            .configure(add_api_routes)
            .service(favicon)
            // Note: due to the error middleware, the 404 html page will
            // be rendered instead of the default actix error text response
            // if the static path is not found. E.g. /static/test.png will
            // return the 404 html page instead of the default error text response.
            .service(actix_files::Files::new("/static", "./static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
