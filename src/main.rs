mod api;
mod client;
mod constants;
mod database;
mod middleware;
mod model;
mod security;
mod utils;

use actix_files::NamedFile;
use actix_web::http::Method;
use actix_web::{get, middleware::Logger, web, App, HttpServer};
use api::admin::{delete_blog, preview_blog, publish_blog, update_blog, upload_blog_images};
use api::auth::{admin_honeypot, login, logout};
use api::blog::{blog_exists, get_blog};
use api::csrf::get_csrf_token;
use api::general::api_index;
use client::admin::new_blog;
use client::auth::{login_admin, login_auth, login_redirect};
use client::general::{blog, blog_id, experiences, index, projects, skills};
use database::db;
use dotenv::dotenv;
use google_cloud_auth::credentials::CredentialsFile;
use google_cloud_storage::client::{Client as GcsClient, ClientConfig};

#[get("/favicon.ico")]
async fn favicon() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./static/images/favicon.ico")?)
}

fn configure_auth_middleware() -> middleware::auth::AuthMiddleware {
    let auth_whitelist = vec![
        (Method::GET, "/".to_string()),
        (Method::GET, "/favicon.ico".to_string()),
        (Method::GET, "/api/csrf-token".to_string()),
        (Method::POST, "/api/admin".to_string()),
        (Method::POST, "/api/login".to_string()),
        (Method::POST, "/api/auth/login".to_string()),
        (Method::POST, "/api/auth/logout".to_string()),
    ];
    let auth_whitelist_regex = vec![(
        Method::GET,
        regex::Regex::new(r"^/api/[a-fA-F\d]{24}$").unwrap(),
    )];
    let auth_middleware = middleware::auth::AuthMiddleware::new(
        None,
        constants::constants::AUTH_COOKIE_NAME,
        auth_whitelist,
        auth_whitelist_regex,
    );
    auth_middleware
}

fn configure_csrf_middleware() -> middleware::csrf::CsrfMiddleware {
    let csrf_whitelist = vec![
        (Method::GET, "/".to_string()),
        (Method::GET, "/favicon.ico".to_string()),
        (Method::GET, "/api/csrf-token".to_string()),
    ];
    let csrf_whitelist_regex = vec![];
    let csrf_middleware =
        middleware::csrf::CsrfMiddleware::new(None, csrf_whitelist, csrf_whitelist_regex);
    csrf_middleware
}

fn configure_csp_middleware() -> middleware::csp::CspMiddleware {
    let csp_whitelist = vec![
        (Method::GET, "/favicon.ico".to_string()),
        (Method::GET, "/csrf-token".to_string()),
    ];

    let api_regex = regex::Regex::new(r"^/api/.*$").unwrap();
    let csp_whitelist_regex = vec![
        (Method::GET, api_regex.clone()),
        (Method::POST, api_regex.clone()),
        (Method::PUT, api_regex.clone()),
        (Method::DELETE, api_regex.clone()),
        (Method::OPTIONS, api_regex.clone()),
        (Method::GET, regex::Regex::new(r"^/static/.*$").unwrap()),
    ];
    let csp_options = middleware::csp::ContentSecurityPolicies {
        script_src: vec![
            "'self'".to_string(),
            "'unsafe-eval'".to_string(), // needed for htmx to work
            "https://unpkg.com/htmx.org@1.9.12".to_string(),
            "https://unpkg.com/htmx.org@1.9.12/dist/ext/client-side-templates.js".to_string(),
            "https://unpkg.com/htmx.org@1.9.12/dist/ext/response-targets.js".to_string(),
            "https://unpkg.com/htmx.org@1.9.12/dist/ext/json-enc.js".to_string(),
        ],
        style_src: vec!["'self'".to_string()],
        default_src: vec![],
        base_uri: vec![],
        img_src: vec![],
        font_src: vec![],
        object_src: vec![],
        form_action: vec![],
        frame_ancestors: vec![],
    };
    let csp_middleware =
        middleware::csp::CspMiddleware::new(32, csp_whitelist, csp_whitelist_regex, csp_options);
    csp_middleware
}

fn configure_hsts_middleware() -> middleware::hsts::HstsMiddleware {
    let hsts_options = middleware::hsts::HstsOptions {
        max_age: if constants::constants::DEBUG_MODE {
            0
        } else {
            60 * 60 * 24 * 365
        },
        include_subdomains: !constants::constants::DEBUG_MODE,
        preload: false,
    };
    let hsts_middleware = middleware::hsts::HstsMiddleware::new(hsts_options);
    hsts_middleware
}

fn configure_cache_control_middleware() -> middleware::cache_control::CacheControlMiddleware {
    let cache_paths = if constants::constants::DEBUG_MODE {
        middleware::cache_control::CachePaths {
            strict_paths: vec![],
            regex_paths: vec![],
        }
    } else {
        middleware::cache_control::CachePaths {
            strict_paths: vec![
                middleware::cache_control::CacheStrictPathValue {
                    path: "/".to_string(),
                    value: "public, max-age=86400, must-revalidate".to_string(), // 1 day
                },
                middleware::cache_control::CacheStrictPathValue {
                    path: "/favicon.ico".to_string(),
                    value: "public, max-age=31536000".to_string(), // 1 year
                },
            ],
            regex_paths: vec![middleware::cache_control::CachePathValue {
                path: regex::Regex::new(r"^/static/.*$").unwrap(),
                value: "public, max-age=31536000, must-revalidate".to_string(), // 1 year
            }],
        }
    };
    let cache_control_middleware =
        middleware::cache_control::CacheControlMiddleware::new(cache_paths);
    cache_control_middleware
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
            .wrap(configure_csrf_middleware())
            .wrap(configure_auth_middleware())
            .wrap(configure_csp_middleware())
            .wrap(configure_hsts_middleware())
            .wrap(configure_cache_control_middleware())
            .wrap(middleware::content_type::ContentTypeMiddleware)
            .service(
                // Note: index file is added here as an error will be thrown if the file in the static path is not found
                // e.g. /static/test.png will return some error text which is not ideal.
                actix_files::Files::new("/static", "./static").index_file("index.html"),
            )
            // below are the client routes
            .service(favicon)
            .service(index)
            .service(experiences)
            .service(projects)
            .service(skills)
            .service(blog)
            .service(blog_id)
            // below are the auth routes
            .service(login_redirect)
            .service(login_admin)
            .service(login_auth)
            // below are the admin routes + API routes
            .service(new_blog)
            .service(preview_blog)
            .service(publish_blog)
            .service(update_blog)
            .service(delete_blog)
            .service(upload_blog_images)
            // below are the API routes
            .service(api_index)
            .service(get_blog)
            .service(blog_exists)
            .service(admin_honeypot)
            .service(login)
            .service(logout)
            .service(get_csrf_token)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
