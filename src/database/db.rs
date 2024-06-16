use crate::constants::constants;
use crate::model::auth::{AuthError, User};
use crate::model::blog::{Blog, BlogError, BlogProjection};
use crate::security::pw_hasher;
use bson::oid::ObjectId;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use mongodb::options::{ClientOptions, Credential, IndexOptions};
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
    let opts = IndexOptions::builder().unique(true).build();
    let index = IndexModel::builder()
        .keys(doc! {"username": 1})
        .options(opts)
        .build();
    collection
        .create_index(index, None)
        .await
        .expect("Failed to create username index for user collection");

    let admin_username =
        std::env::var(constants::BLOG_ADMIN_USERNAME).expect("admin username not set");
    let admin_password =
        std::env::var(constants::BLOG_ADMIN_PASSWORD).expect("admin password not set");
    let hashed_admin_password = tokio::task::spawn_blocking(move || {
        pw_hasher::hash_password(&admin_password).expect("Failed to hash password")
    })
    .await
    .expect("Failed to hash password");

    let user = User::new(admin_username, hashed_admin_password);
    match collection.insert_one(user, None).await {
        Ok(_) => log::info!("Admin account created"),
        Err(e) => panic!("Failed to create admin account: {}", e),
    }
    log::info!("User collection initialised");
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

    let title_idx = IndexModel::builder().keys(doc! { "title": 1 }).build();
    let title_idx_future = collection.create_index(title_idx, None);

    let tag_idx = IndexModel::builder().keys(doc! { "tags": 1 }).build();
    let tag_idx_future = collection.create_index(tag_idx, None);

    let (title_result, tag_result) = tokio::join!(title_idx_future, tag_idx_future);
    let mut has_error = false;
    match title_result {
        Ok(_) => {}
        Err(e) => {
            has_error = true;
            log::error!("Failed to create title index: {}", e)
        }
    }
    match tag_result {
        Ok(_) => {}
        Err(e) => {
            has_error = true;
            log::error!("Failed to create tag index: {}", e)
        }
    }
    if has_error {
        panic!("Failed to create indexes for api collection");
    } else {
        log::info!("Blog collection initialised");
    }
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

    let client_ref = &client.client;
    let init_user_future = init_user_collection(client_ref);
    let init_blog_future = init_blog_collection(client_ref);
    tokio::join!(init_user_future, init_blog_future);

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

    pub fn get_custom_collection<T>(&self, collection_name: &str) -> Collection<T> {
        self.get_database(None).collection(collection_name)
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

    fn handle_result<T>(result: Result<Option<T>, mongodb::error::Error>) -> Result<T, BlogError> {
        match result {
            Ok(Some(blog)) => Ok(blog),
            Ok(None) => Err(BlogError::BlogNotFound),
            Err(err) => {
                log::error!("Failed to get blog from database: {}", err);
                Err(BlogError::InternalServerError)
            }
        }
    }

    pub async fn get_blog_post_projection(
        &self,
        id: &ObjectId,
        options: Option<FindOneOptions>,
    ) -> Result<BlogProjection, BlogError> {
        let blog_collection: Collection<BlogProjection> =
            self.get_custom_collection(constants::BLOG_COLLECTION);
        let result = blog_collection.find_one(doc! {"_id": id}, options).await;
        Self::handle_result(result)
    }

    pub async fn get_blog_post(
        &self,
        id: &ObjectId,
        options: Option<FindOneOptions>,
    ) -> Result<Blog, BlogError> {
        let result = self
            .get_blog_collection()
            .find_one(doc! {"_id": id}, options)
            .await;
        Self::handle_result(result)
    }
}
