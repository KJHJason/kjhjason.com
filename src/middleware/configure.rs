use crate::{constants::constants, middleware};
use actix_web::http::Method;

macro_rules! get_client_routes {
    () => {
        vec![
            (Method::GET, "/"),
            (Method::GET, "/favicon.ico"),
            (Method::GET, "/robots.txt"),
            (Method::GET, "/sitemap.xml"),
            (Method::GET, "/experiences"),
            (Method::GET, "/testimonials"),
            (Method::GET, "/projects"),
            (Method::GET, "/skills"),
            (Method::GET, "/certificates"),
            (Method::GET, "/awards"),
            (Method::GET, "/resume"),
            (Method::GET, "/blogs"),
            (Method::GET, "/admin"),
            (Method::GET, "/login"),
            (Method::GET, "/api"),
            (Method::GET, "/api/health"),
            (Method::GET, "/api/csrf-token"),
        ]
    };
}

macro_rules! add_login_uri_path {
    ($whitelist:ident) => {
        let login_uri = constants::get_login_uri_path();
        $whitelist.push((Method::GET, &login_uri));
    };
}

macro_rules! add_login_api_uri_path {
    ($whitelist:ident) => {
        let login_api_uri = format!("/api{}", constants::get_login_uri_path());
        $whitelist.push((Method::POST, &login_api_uri));
    };
}

pub fn configure_auth_middleware() -> middleware::auth::AuthMiddleware {
    let mut auth_whitelist = get_client_routes!();
    auth_whitelist.extend(vec![
        (Method::POST, "/api/admin"),
        (Method::POST, "/api/login"),
        (Method::POST, "/api/auth/login"),
        (Method::POST, "/api/logout"),
    ]);
    add_login_uri_path!(auth_whitelist);
    add_login_api_uri_path!(auth_whitelist);
    let auth_whitelist_regex = vec![
        (Method::GET, regex::Regex::new(r"^/blogs/[\w-]+$").unwrap()),
        (Method::GET, regex::Regex::new(r"^/static/.*$").unwrap()),
    ];
    let auth_middleware = middleware::auth::AuthMiddleware::new(
        None,
        constants::AUTH_COOKIE_NAME,
        auth_whitelist,
        auth_whitelist_regex,
    );
    auth_middleware
}

pub fn configure_csrf_middleware() -> middleware::csrf::CsrfMiddleware {
    let mut csrf_whitelist = get_client_routes!();
    add_login_uri_path!(csrf_whitelist);
    let csrf_whitelist_regex = vec![];
    let csrf_middleware =
        middleware::csrf::CsrfMiddleware::new(None, csrf_whitelist, csrf_whitelist_regex);
    csrf_middleware
}

pub fn configure_csp_middleware() -> middleware::csp::CspMiddleware {
    let csp_whitelist = vec![
        (Method::GET, "/favicon.ico"),
        (Method::GET, "/robots.txt"),
        (Method::GET, "/sitemap.xml"),
        (Method::GET, "/api"),
    ];
    let api_regex = regex::Regex::new(r"^/api/.*$").unwrap();
    let csp_whitelist_regex = vec![
        (Method::GET, api_regex.clone()),
        (Method::POST, api_regex.clone()),
        (Method::PUT, api_regex.clone()),
        (Method::PATCH, api_regex.clone()),
        (Method::DELETE, api_regex.clone()),
        (Method::OPTIONS, api_regex.clone()),
        (Method::GET, regex::Regex::new(r"^/static/.*$").unwrap()),
    ];
    let csp_options = middleware::csp::ContentSecurityPolicies {
        script_src: vec![
            "'self'",
            "'unsafe-eval'", // needed for htmx to work for responses like parsing the html content for the blog
            "https://challenges.cloudflare.com/turnstile/v0/api.js",
        ],
        style_src: vec![
            "'self'",
            "https://cdn.jsdelivr.net/npm/@sweetalert2/theme-dark@latest/dark.css",
        ],
        frame_src: vec!["'self'", "https://challenges.cloudflare.com/"],
        default_src: vec![],
        base_uri: vec!["'self'"],
        img_src: vec![],
        font_src: vec![],
        object_src: vec!["'none'"],
        form_action: vec![],
        frame_ancestors: vec![],
    };
    let csp_middleware =
        middleware::csp::CspMiddleware::new(32, csp_whitelist, csp_whitelist_regex, csp_options);
    csp_middleware
}

pub fn configure_hsts_middleware() -> middleware::hsts::HstsMiddleware {
    let hsts_options = middleware::hsts::HstsOptions {
        max_age: if constants::get_debug_mode() {
            0
        } else {
            60 * 60 * 24 * 365
        },
        include_subdomains: false,
        preload: false,
    };
    let hsts_middleware = middleware::hsts::HstsMiddleware::new(hsts_options);
    hsts_middleware
}

pub fn configure_cache_control_middleware() -> middleware::cache_control::CacheControlMiddleware {
    let cache_paths = if constants::get_debug_mode() {
        middleware::cache_control::CachePaths {
            strict_paths: vec![],
            regex_paths: vec![],
        }
    } else {
        middleware::cache_control::CachePaths {
            strict_paths: vec![middleware::cache_control::CacheStrictPathValue {
                path: "/favicon.ico",
                value: "public, max-age=31536000", // 1 year
            }],
            regex_paths: vec![
                middleware::cache_control::CachePathValue {
                    path: regex::Regex::new(r"^/static/pdfjs/.*$").unwrap(),
                    value: "public, max-age=86400, must-revalidate", // 1 day for files in pdfjs directory
                },
                middleware::cache_control::CachePathValue {
                    path: regex::Regex::new(r"^/static/.*(\.js|\.css)$").unwrap(),
                    value: "public, max-age=86400, must-revalidate", // 1 day for css/js files
                },
                middleware::cache_control::CachePathValue {
                    path: regex::Regex::new(r"^/static/.*$").unwrap(),
                    value: "public, max-age=15768000, must-revalidate", // 6 months
                },
            ],
        }
    };
    let cache_control_middleware =
        middleware::cache_control::CacheControlMiddleware::new(cache_paths);
    cache_control_middleware
}
