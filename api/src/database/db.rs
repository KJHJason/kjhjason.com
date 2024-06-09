use crate::constants::constants;
use crate::model::blog::{Blog, BlogError};
use bson::oid::ObjectId;
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, Credential};
use mongodb::{Client, Collection};

#[derive(Clone)]
pub struct DbClient {
    client: Client,
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

    match Client::with_options(client_options) {
        Ok(client) => Ok(DbClient::new(client)),
        Err(e) => Err(e),
    }
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
