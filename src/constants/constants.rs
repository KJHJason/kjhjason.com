use once_cell::sync::Lazy;
use std::time;

pub const APP_NAME: &str = "kjhjasoncom"; // used for API SDKs like MongoDB
pub const SESSION_TIMEOUT: i64 = 60 * 60 * 24 * 1; // 1 day
pub const SESSION_TIMEOUT_REMEMBER: i64 = 60 * 60 * 24 * 30; // 1 month
pub const AUTH_COOKIE_NAME: &str = "_session";
pub const DOMAIN: &str = "kjhjason.com";
pub const CSRF_COOKIE_NAME: &str = "csrf-token";
pub const CSRF_HEADER_NAME: &str = "X-CSRF-Token";
pub const CSRF_TOKEN_LENGTH: usize = 32;
pub const CSRF_MAX_AGE: i64 = 60 * 60 * 24 * 1; // 1 day

pub const LOCAL_URI: &str = "mongodb://localhost:27017";
pub const DATABASE: &str = "kjhjason";
pub const BLOG_COLLECTION: &str = "blogs";
pub const USER_COLLECTION: &str = "users";
pub const SESSION_COLLECTION: &str = "sessions";

pub const TITLE_MAX_LENGTH: usize = 150;
pub const MAX_TAGS: usize = 8;

pub const MAX_FILE_SIZE: usize = 1024 * 1024 * 100;
pub const TEMP_DIR: &str = "uploads";

pub const BUCKET: &str = "kjhjason";
pub const BUCKET_FOR_TEMP: &str = "kjhjason-private";
pub const PUBLIC_S3_URL: &str = "https://storage.kjhjason.com";
pub const SIGNED_URL_MAX_AGE: time::Duration = time::Duration::from_secs(60 * 60 * 24 * 7);
pub const TEMP_OBJ_PREFIX: &str = "temp";

pub const CF_TURNSTILE_SITE_KEY: &str = "0x4AAAAAAAcnZh9gukmZdThg";

// env keys called once only on startup
pub const MONGODB_URI: &str = "MONGODB_URI";
pub const BLOG_ADMIN_USERNAME: &str = "BLOG_ADMIN_USERNAME";
pub const BLOG_ADMIN_EMAIL: &str = "BLOG_ADMIN_EMAIL";
pub const BLOG_ADMIN_PASSWORD: &str = "BLOG_ADMIN_PASSWORD";

// env keys
const __DEBUG_MODE: &str = "DEBUG_MODE";
const __R2_ACCOUNT_ID: &str = "R2_ACCOUNT_ID";
const __CF_TURNSTILE_SECRET_KEY: &str = "CF_TURNSTILE_SECRET_KEY";
const __SECRET_KEY: &str = "SECRET_KEY";
const __SECRET_KEY_SALT: &str = "SECRET_KEY_SALT";
const __CSRF_KEY_SALT: &str = "CSRF_KEY_SALT";
const __DB_ENCRYPTION_KEY: &str = "DB_ENCRYPTION_KEY";
const __DB_ENCRYPTION_KEY_AD: &str = "DB_ENCRYPTION_KEY_AD";

#[inline(always)]
fn get_env_var(var_name: &str) -> String {
    match std::env::var(var_name) {
        Ok(val) => val,
        Err(_) => panic!("{} is not set in .env file", var_name),
    }
}

pub fn get_debug_mode() -> bool {
    static DEBUG_MODE: Lazy<bool> = Lazy::new(|| get_env_var(__DEBUG_MODE) == "true");
    *DEBUG_MODE
}

macro_rules! generate_env_getter {
    ($fn_name:ident, $var_name:expr) => {
        pub fn $fn_name() -> String {
            static VALUE: Lazy<String> = Lazy::new(|| get_env_var($var_name));
            VALUE.clone()
        }
    };
}
macro_rules! generate_env_hex_to_bytes_getter {
    ($fn_name:ident, $var_name:expr) => {
        pub fn $fn_name() -> Vec<u8> {
            static VALUE: Lazy<Vec<u8>> =
                Lazy::new(|| hex::decode(get_env_var($var_name)).unwrap());
            VALUE.clone()
        }
    };
}

generate_env_hex_to_bytes_getter!(get_secret_key, __R2_ACCOUNT_ID);
generate_env_hex_to_bytes_getter!(get_secret_key_salt, __SECRET_KEY_SALT);
generate_env_hex_to_bytes_getter!(get_csrf_key_salt, __CSRF_KEY_SALT);
generate_env_hex_to_bytes_getter!(get_db_encryption_key, __DB_ENCRYPTION_KEY);
generate_env_hex_to_bytes_getter!(get_db_encryption_key_ad, __DB_ENCRYPTION_KEY_AD);
generate_env_getter!(get_r2_acc_id, __R2_ACCOUNT_ID);
generate_env_getter!(get_cf_turnstile_secret_key, __CF_TURNSTILE_SECRET_KEY);

macro_rules! generate_debug_dependent_val_getter {
    ($fn_name:ident, $debug_val:expr, $prod_val:expr) => {
        pub fn $fn_name() -> String {
            if get_debug_mode() {
                $debug_val.to_string()
            } else {
                $prod_val.to_string()
            }
        }
    };
}

generate_debug_dependent_val_getter!(get_domain, "localhost", DOMAIN);
generate_debug_dependent_val_getter!(get_blog_obj_prefix, "blog-dev", "blog");
