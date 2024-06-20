use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Display, Debug)]
pub struct BlogIdentifier {
    pub id: String,
}
