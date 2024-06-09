use crate::constants::constants;
use crate::model::auth::{AuthError, User};
use crate::model::blog::{Blog, BlogError};
use crate::security::pw_hasher;
use bson::oid::ObjectId;
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, Credential};
use mongodb::{Client, Collection, IndexModel};

#[derive(Clone)]
pub struct DbClient {
    client: Client,
}

async fn init_user_collection(client: &Client) {
    let db = client.database(constants::DATABASE);
    let collection: Collection<User> = db.collection(constants::USER_COLLECTION);

    // check if the collection already exists
    let result = collection.find_one(None, None).await;
    match result {
        Ok(Some(_)) => return,
        _ => {}
    }

    // although there will only be one account, just do this for future-proofing
    let index = IndexModel::builder().keys(doc! {"username": 1}).build();
    collection
        .create_index(index, None)
        .await
        .expect("Failed to create username index for user collection");

    let admin_username =
        std::env::var(constants::BLOG_ADMIN_USERNAME).expect("admin username not set");
    let admin_password =
        std::env::var(constants::BLOG_ADMIN_PASSWORD).expect("admin password not set");
    let hashed_admin_password =
        pw_hasher::hash_password(&admin_password).expect("Failed to hash password");

    let user = User::new(admin_username, hashed_admin_password);
    match collection.insert_one(user, None).await {
        Ok(_) => log::info!("Admin account created"),
        Err(e) => panic!("Failed to create admin account: {}", e),
    }
}

async fn init_blog_collection(client: &Client) {
    let db = client.database(constants::DATABASE);
    let collection: Collection<Blog> = db.collection(constants::BLOG_COLLECTION);

    // check if the collection already exists
    let result = collection.find_one(None, None).await;
    match result {
        Ok(Some(_)) => return,
        _ => {}
    }

    let index = IndexModel::builder().keys(doc! { "title": 1 }).build();
    collection
        .create_index(index, None)
        .await
        .expect("Failed to create title index for blog collection");

    let index = IndexModel::builder().keys(doc! { "tags": 1 }).build();
    collection
        .create_index(index, None)
        .await
        .expect("Failed to create tags index for blog collection");
}

pub async fn init_db() -> Result<DbClient, mongodb::error::Error> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| constants::LOCAL_URI.into());

    let mut client_options = ClientOptions::parse(uri.clone()).await?;
    client_options.app_name = Some(constants::APP_NAME.to_string());
    if !constants::DEBUG_MODE {
        if uri == constants::LOCAL_URI {
            panic!("Cannot use local URI in production mode");
        }
        let username = std::env::var("MONGODB_USERNAME").unwrap();
        let password = std::env::var("MONGODB_PASSWORD").unwrap();
        client_options.credential = Some(
            Credential::builder()
                .username(username.to_string())
                .password(password.to_string())
                .build(),
        );
    }

    let client = match Client::with_options(client_options) {
        Ok(client) => DbClient::new(client),
        Err(e) => return Err(e),
    };

    init_user_collection(&client.client).await;
    init_blog_collection(&client.client).await;
    Ok(client)
}

impl DbClient {
    fn new(client: Client) -> Self {
        DbClient { client }
    }

    pub fn get_database(&self, db: Option<&str>) -> mongodb::Database {
        match db {
            Some(db_name) => self.client.database(db_name),
            None => self.client.database(constants::DATABASE),
        }
    }

    pub fn get_blog_collection(&self) -> Collection<Blog> {
        self.get_database(None)
            .collection(constants::BLOG_COLLECTION)
    }

    pub fn get_user_collection(&self) -> Collection<User> {
        self.get_database(None)
            .collection(constants::USER_COLLECTION)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<User, AuthError> {
        match self
            .get_user_collection()
            .find_one(doc! {"username": username}, None)
            .await
        {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(AuthError::UserNotFound),
            Err(err) => {
                log::error!("Failed to get user from database: {}", err);
                Err(AuthError::InternalServerError)
            }
        }
    }

    pub async fn get_blog_post(&self, id: &ObjectId) -> Result<Blog, BlogError> {
        match self
            .get_blog_collection()
            .find_one(doc! {"_id": id}, None)
            .await
        {
            Ok(Some(blog)) => Ok(blog),
            Ok(None) => Err(BlogError::BlogNotFound),
            Err(err) => {
                log::error!("Failed to get blog from database: {}", err);
                Err(BlogError::InternalServerError)
            }
        }
    }
}
