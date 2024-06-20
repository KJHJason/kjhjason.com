use crate::{constants::constants, middleware};
use actix_web::http::Method;

pub fn configure_auth_middleware() -> middleware::auth::AuthMiddleware {
    let auth_whitelist = vec![
        (Method::GET, "/".to_string()),
        (Method::GET, "/favicon.ico".to_string()),
        (Method::GET, "/experiences".to_string()),
        (Method::GET, "/projects".to_string()),
        (Method::GET, "/skills".to_string()),
        (Method::GET, "/certificates".to_string()),
        (Method::GET, "/awards".to_string()),
        (Method::GET, "/resume".to_string()),
        (Method::GET, "/blogs".to_string()),
        (Method::GET, "/admin".to_string()),
        (Method::GET, "/login".to_string()),
        (Method::GET, "/auth/login".to_string()),
        (Method::GET, "/api".to_string()),
        (Method::GET, "/api/csrf-token".to_string()),
        (Method::POST, "/api/admin".to_string()),
        (Method::POST, "/api/login".to_string()),
        (Method::POST, "/api/auth/login".to_string()),
        (Method::POST, "/api/logout".to_string()),
    ];
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

pub fn configure_csp_middleware() -> middleware::csp::CspMiddleware {
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
            "'unsafe-eval'".to_string(), // needed for htmx to work for events like hx-on:click
            "https://challenges.cloudflare.com/turnstile/v0/api.js".to_string(),
            "https://unpkg.com/htmx.org@2.0.0".to_string(),
            "https://unpkg.com/htmx-ext-response-targets@latest/response-targets.js".to_string(),
        ],
        style_src: vec![
            "'self'".to_string(),
            "https://cdn.jsdelivr.net/npm/@sweetalert2/theme-dark@latest/dark.css".to_string(),
        ],
        frame_src: vec![
            "'self'".to_string(),
            "https://challenges.cloudflare.com/".to_string(),
        ],
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
