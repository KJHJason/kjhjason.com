use crate::constants::constants::{
    BLOG_COLLECTION, DATABASE, DEBUG_MODE, LOCAL_URI, MONGO_CLIENT_APP_NAME,
};
use crate::model::blog::Blog;
use mongodb::options::{ClientOptions, Credential};
use mongodb::{Client, Collection};

#[derive(Clone)]
pub struct DbClient {
    client: Client,
}

pub async fn init_db() -> Result<DbClient, mongodb::error::Error> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| LOCAL_URI.into());

    let mut client_options = ClientOptions::parse(uri.clone()).await?;
    client_options.app_name = Some(MONGO_CLIENT_APP_NAME.to_string());
    if !DEBUG_MODE {
        if uri == LOCAL_URI {
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
            None => self.client.database(DATABASE),
        }
    }

    pub fn get_blog_collection(&self) -> Collection<Blog> {
        self.get_database(None).collection(BLOG_COLLECTION)
    }
}
