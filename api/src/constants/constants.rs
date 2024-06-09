pub const DEBUG_MODE: bool = true;
pub const APP_NAME: &str = "KJHJason's Blog Backend API"; // used for API SDKs like MongoDB
pub const SESSION_TIMEOUT: i64 = 60 * 60 * 24 * 7; // 1 week

pub const LOCAL_URI: &str = "mongodb://localhost:27017";
pub const DATABASE: &str = "kjhjason";
pub const BLOG_COLLECTION: &str = "blog";
pub const USER_COLLECTION: &str = "user";

pub const TITLE_MAX_LENGTH: usize = 150;
pub const MAX_TAGS: usize = 8;

pub const MAX_THUMBNAIL_FILE_SIZE: usize = 1024 * 1024 * 10;
pub const TEMP_DIR: &str = "/uploads/";

pub const BUCKET: &str = "kjhjason";
pub const TEMP_OBJ_PREFIX: &str = "temp";
pub const BLOG_OBJ_PREFIX: &str = "blog";

// env keys
pub const AWS_ENDPOINT_URL: &str = "AWS_ENDPOINT_URL";
pub const AUTH_SECRET_KEY: &str = "AUTH_SECRET_KEY";
pub const BLOG_ADMIN_USERNAME: &str = "BLOG_ADMIN_USERNAME";
pub const BLOG_ADMIN_PASSWORD: &str = "BLOG_ADMIN_PASSWORD";
