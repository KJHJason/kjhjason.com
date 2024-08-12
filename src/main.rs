mod api;
mod client;
mod constants;
mod database;
mod errors;
mod middleware;
mod models;
mod security;
mod templates;
mod utils;

use actix_web::middleware::Compress;
use actix_web::{
    middleware::{ErrorHandlers, Logger},
    web, App, HttpServer,
};
use api::configure::add_api_routes;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3 as s3;
use client::configure::add_client_routes;
use database::init as db;
use dotenv::dotenv;
use middleware::configure::{
    configure_auth_middleware, configure_cache_control_middleware, configure_csp_middleware,
    configure_csrf_middleware, configure_hsts_middleware,
};
use middleware::errors::render_error;

macro_rules! error_handler_many {
    ($handler:ident, [$($variant:ident),*]) => {
        ErrorHandlers::new()
            $(.handler(actix_web::http::StatusCode::$variant, $handler))+
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    log::info!("Initialising Blog Web App...");

    dotenv().ok();
    if constants::get_debug_mode() {
        log::info!("Debug mode enabled");
    }

    let db_future = async {
        let db_client = db::init_db()
            .await
            .expect("Failed to initialise database client");
        log::info!("Database client initialised");
        db_client
    };
    let aws_future = async {
        let r2_acc_id = constants::get_r2_acc_id();
        let config = aws_config::defaults(BehaviorVersion::latest())
            .endpoint_url(format!("https://{}.r2.cloudflarestorage.com/", r2_acc_id))
            .region(Region::new("auto"))
            .load()
            .await;

        let s3_client = s3::Client::new(&config);
        log::info!("AWS S3 client initialised");
        s3_client
    };
    let (db_client, s3_client) = tokio::join!(db_future, aws_future);

    let address = if constants::get_debug_mode() {
        ("127.0.0.1", 8080)
    } else {
        ("0.0.0.0", 8080)
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .app_data(web::Data::new(s3_client.clone()))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(middleware::host::HostMiddleware)
            .wrap(middleware::content_type::ContentTypeMiddleware)
            .wrap(configure_csrf_middleware())
            .wrap(configure_csp_middleware())
            .wrap(configure_hsts_middleware())
            .wrap(configure_cache_control_middleware())
            .wrap(configure_auth_middleware())
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
    })
    .bind(address)?
    .run()
    .await
}
