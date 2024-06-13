use crate::model::blog::BlogError;
use bson::oid::ObjectId;
use std::str::FromStr;

pub fn validate_id(id: &str) -> Result<ObjectId, BlogError> {
    match ObjectId::from_str(id) {
        Ok(id) => Ok(id),
        Err(_) => Err(BlogError::InvalidObjectId),
    }
}
