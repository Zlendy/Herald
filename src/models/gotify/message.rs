use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize)]
pub struct MessageModel { // MessageExternal Model
    pub appid: i64,
    pub date: String,
    // pub extras: Option<T>, // Extras are unsupported
    pub id: i64,
    pub message: String,
    pub priority: Option<i64>,
    pub title: Option<String>,
}

impl MessageModel {
    #[allow(dead_code)]
    pub fn new(id: i64, title: String, message: String) -> Self {
        Self {
            appid: 0,
            title: Some(title),
            message,
            date: String::new(),
            id,
            priority: None,
        }
    }
}

impl Default for MessageModel {
    fn default() -> Self {
        Self {
            appid: 0,
            title: Some("Message title".to_string()),
            message: "Message content".to_string(),
            date: String::new(),
            id: 0,
            priority: None,
        }
    }
}