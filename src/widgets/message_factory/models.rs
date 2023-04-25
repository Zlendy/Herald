#[derive(Debug, Clone)]
pub struct MessageModel {
    pub id: u32,
    pub title: String,
    pub content: String,
}

impl MessageModel {
    pub fn new(id: u32, title: String, content: String) -> Self {
        Self { id, title, content }
    }
}

impl Default for MessageModel {
    fn default() -> Self {
        Self {
            id: 0,
            title: "Message title".to_string(),
            content: "Message content".to_string(),
        }
    }
}