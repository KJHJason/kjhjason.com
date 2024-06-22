use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub const EXPIRY_KEY: &str = "EXPIRY";

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created: chrono::DateTime<chrono::Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub expiry: chrono::DateTime<chrono::Utc>,
}

impl Session {
    pub fn new(user_id: ObjectId, exp: i64) -> Session {
        Session {
            _id: ObjectId::new(),
            user_id,
            created: chrono::Utc::now(),
            expiry: chrono::Utc::now() + chrono::Duration::seconds(exp),
        }
    }

    #[inline]
    pub fn is_expired(&self) -> bool {
        self.expiry < chrono::Utc::now()
    }
}
