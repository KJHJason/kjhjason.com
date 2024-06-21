use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectedUser {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub totp_secret: Option<Vec<u8>>,
}
