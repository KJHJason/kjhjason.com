mod blog;
mod constants;
mod database;
mod middleware;
mod model;
mod security;
mod utils;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::http::Method;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer};
use aws_config::BehaviorVersion;
use aws_sdk_s3 as s3;
use blog::api::{delete_blog, get_blog, publish_blog, update_blog, upload_blog_images};
use blog::auth::{admin_honeypot, login, login_honeypot};
use blog::csrf::get_csrf_token;
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

#[get("/favicon.ico")]
async fn favicon() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/favicon.ico")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    log::info!("Initialising API...");

    dotenv().ok();
    let db_future = async {
        db::init_db()
            .await
            .unwrap_or_else(|_| panic!("Failed to connect to database"))
    };
    let aws_future = async {
        let api_endpoint = std::env::var(constants::constants::AWS_ENDPOINT_URL).unwrap();
        let mut config = aws_config::defaults(BehaviorVersion::latest());
        config = config.endpoint_url(api_endpoint);
        config = config
            .app_name(aws_config::AppName::new("blog".to_string()).expect("Invalid app name"));
        config.load().await
    };
    let (db_client, aws_config) = tokio::join!(db_future, aws_future);

    let client = s3::Client::new(&aws_config);
    HttpServer::new(move || {
        let csrf_whitelist = vec![
            (Method::GET, "/".to_string()),
            (Method::GET, "/favicon.ico".to_string()),
            (Method::GET, "/csrf-token".to_string()),
        ];
        let csrf_middleware = middleware::csrf::CsrfMiddleware::new(None, csrf_whitelist);

        let auth_whitelist = vec![
            (Method::GET, "/".to_string()),
            (Method::GET, "/favicon.ico".to_string()),
            (Method::GET, "/csrf-token".to_string()),
            (Method::POST, "/admin".to_string()),
            (Method::POST, "/login".to_string()),
            (Method::POST, "/auth/login".to_string()),
            (Method::GET, "/auth/logout".to_string()),
        ];
        let auth_whitelist_regex = vec![(
            Method::GET,
            regex::Regex::new(r"^/blog/[a-fA-F\d]{24}$").unwrap(),
        )];
        let auth_middleware = middleware::auth::AuthMiddleware::new(
            None,
            constants::constants::AUTH_COOKIE_NAME,
            auth_whitelist,
            auth_whitelist_regex,
        );

        let cors = Cors::default()
            .supports_credentials()
            .allowed_origin_fn(|origin, _req_head| {
                let origin = origin.to_str().unwrap_or_else(|_| "").to_string();
                if constants::constants::DEBUG_MODE && origin.starts_with("http://localhost") {
                    return true;
                }
                if origin.starts_with(&constants::constants::get_client_full_url()) {
                    return true;
                }
                if origin.starts_with(&constants::constants::get_client_subdomain_url()) {
                    return true;
                }
                false
            })
            .allow_any_header() // needed due to the htmx custom headers in the frontend
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .max_age(60);

        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .app_data(web::Data::new(client.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(csrf_middleware)
            .wrap(cors)
            .wrap(auth_middleware)
            .service(favicon)
            .service(hello)
            .service(get_blog)
            .service(publish_blog)
            .service(update_blog)
            .service(delete_blog)
            .service(upload_blog_images)
            .service(admin_honeypot)
            .service(login_honeypot)
            .service(login)
            .service(get_csrf_token)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
