use crate::constants::constants;
use crate::errors::{auth::AuthError, blog::BlogError, session::SessionError};
use crate::models::projected_user::ProjectedUser;
use crate::models::{
    blog::Blog, projected_blog::ProjectedBlog, session::Session, user, user::User,
};
use bson::oid::ObjectId;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use mongodb::{Client, Collection};

#[derive(Clone)]
pub struct DbClient {
    client: Client,
}

impl DbClient {
    pub fn new(client: Client) -> Self {
        DbClient { client }
    }

    #[inline]
    pub fn get_client(&self) -> &Client {
        &self.client
    }

    #[inline]
    pub fn get_database(&self, db: Option<&str>) -> mongodb::Database {
        match db {
            Some(db_name) => self.client.database(db_name),
            None => self.client.database(constants::DATABASE),
        }
    }

    #[inline]
    pub fn get_custom_collection<T>(&self, collection_name: &str) -> Collection<T> {
        self.get_database(None).collection(collection_name)
    }

    #[inline]
    pub fn get_blog_collection(&self) -> Collection<Blog> {
        self.get_database(None)
            .collection(constants::BLOG_COLLECTION)
    }

    #[inline]
    pub fn get_user_collection(&self) -> Collection<User> {
        self.get_database(None)
            .collection(constants::USER_COLLECTION)
    }

    #[inline]
    pub fn get_session_collection(&self) -> Collection<Session> {
        self.get_database(None)
            .collection(constants::SESSION_COLLECTION)
    }

    pub async fn get_session_by_id(&self, id: &ObjectId) -> Result<Session, SessionError> {
        match self
            .get_session_collection()
            .find_one(doc! {"_id": id}, None)
            .await
        {
            Ok(Some(session)) => Ok(session),
            Ok(None) => Err(SessionError::NotFound),
            Err(err) => {
                log::error!("Failed to get session from database: {:?}", err);
                Err(SessionError::InternalServerError)
            }
        }
    }

    #[inline]
    fn handle_user_result<T>(
        result: Result<Option<T>, mongodb::error::Error>,
    ) -> Result<T, AuthError> {
        match result {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(AuthError::UserNotFound),
            Err(err) => {
                log::error!("Failed to get user from database: {:?}", err);
                Err(AuthError::InternalServerError)
            }
        }
    }

    pub async fn get_projected_user_by_id(
        &self,
        id: &ObjectId,
        options: Option<FindOneOptions>,
    ) -> Result<ProjectedUser, AuthError> {
        let col: Collection<ProjectedUser> = self.get_custom_collection(constants::USER_COLLECTION);
        let result = col.find_one(doc! {"_id": id}, options).await;
        Self::handle_user_result(result)
    }

    pub async fn get_user_by_username_or_email(
        &self,
        username_or_email: &str,
    ) -> Result<User, AuthError> {
        let result = self
            .get_user_collection()
            .find_one(
                doc! {"$or": [{user::USERNAME_KEY: username_or_email}, {user::EMAIL_KEY: username_or_email}]},
                None,
            )
            .await;
        Self::handle_user_result(result)
    }

    #[inline]
    fn handle_blog_result<T>(
        result: Result<Option<T>, mongodb::error::Error>,
    ) -> Result<T, BlogError> {
        match result {
            Ok(Some(blog)) => Ok(blog),
            Ok(None) => Err(BlogError::BlogNotFound),
            Err(err) => {
                log::error!("Failed to get blog from database: {:?}", err);
                Err(BlogError::InternalServerError)
            }
        }
    }

    pub async fn get_projected_blog_post(
        &self,
        id: &ObjectId,
        options: Option<FindOneOptions>,
    ) -> Result<ProjectedBlog, BlogError> {
        let blog_collection: Collection<ProjectedBlog> =
            self.get_custom_collection(constants::BLOG_COLLECTION);
        let result = blog_collection.find_one(doc! {"_id": id}, options).await;
        Self::handle_blog_result(result)
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
        Self::handle_blog_result(result)
    }
}
