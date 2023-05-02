use serde_derive::{Serialize, Deserialize};
use serde_json::Value;

use super::error::ErrorModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientModel {
    pub id: Option<i32>,
    pub name: String,
    pub token: Option<String>,
}

impl ClientModel {
    pub fn new(name: &str) -> Self {
        Self {
            id: None,
            name: name.to_string(),
            token: None,
        }
    }
}

pub enum CreateClientEnum {
    Success(ClientModel),
    Error(ErrorModel),
    Unmatched(Value),
}