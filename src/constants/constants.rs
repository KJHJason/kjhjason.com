pub const DEBUG_MODE: bool = true;
pub const APP_NAME: &str = "KJHJason's Blog API"; // used for API SDKs like MongoDB
pub const SESSION_TIMEOUT: i64 = 60 * 60 * 24 * 1; // 1 day
pub const SESSION_TIMEOUT_REMEMBER: i64 = 60 * 60 * 24 * 30; // 1 month
pub const AUTH_COOKIE_NAME: &str = "_session";
pub const DOMAIN: &str = "kjhjason.com";
pub const CSRF_COOKIE_NAME: &str = "csrf-token";
pub const CSRF_HEADER_NAME: &str = "X-CSRF-Token";
pub const CSRF_TOKEN_LENGTH: usize = 32;
pub const CSRF_MAX_AGE: i64 = 60 * 60 * 24 * 1; // 1 day

pub const HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";

pub const LOCAL_URI: &str = "mongodb://localhost:27017";
pub const DATABASE: &str = "kjhjason";
pub const BLOG_COLLECTION: &str = "api";
pub const USER_COLLECTION: &str = "user";

pub const TITLE_MAX_LENGTH: usize = 150;
pub const MAX_TAGS: usize = 8;

pub const MAX_FILE_SIZE: usize = 1024 * 1024 * 100;
pub const TEMP_DIR: &str = "uploads";

pub const BUCKET: &str = "kjhjason.com";
pub const TEMP_OBJ_PREFIX: &str = "temp";
pub const BLOG_OBJ_PREFIX: &str = "api";

// env keys
pub const SECRET_KEY: &str = "SECRET_KEY";
pub const SECRET_KEY_SALT: &str = "SECRET_KEY_SALT";
pub const CSRF_KEY_SALT: &str = "CSRF_KEY_SALT";
pub const BLOG_ADMIN_USERNAME: &str = "BLOG_ADMIN_USERNAME";
pub const BLOG_ADMIN_PASSWORD: &str = "BLOG_ADMIN_PASSWORD";

pub fn get_domain() -> String {
    if DEBUG_MODE {
        "localhost".to_string()
    } else {
        DOMAIN.to_string()
    }
}
