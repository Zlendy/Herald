use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ClientModel {
    id: Option<i32>,
    name: String,
    token: Option<String>,
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