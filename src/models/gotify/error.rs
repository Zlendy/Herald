use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorModel {
    pub error: String,

    #[serde(rename="errorCode")]
    pub error_code: i64,

    #[serde(rename="errorDescription")]
    pub error_descripton: String,
}

impl ErrorModel {
    pub fn new(description: String) -> Self {
        Self {
            error: "".to_string(),
            error_code: 0,
            error_descripton: description,
        }
    }
}