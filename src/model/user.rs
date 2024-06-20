use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub _id: ObjectId,
    username: String,
    password: String,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        User {
            _id: ObjectId::new(),
            username,
            password,
        }
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
}
