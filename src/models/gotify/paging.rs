use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagingModel {
    pub limit: i64,
    pub next: Option<String>,
    pub since: i64,
    pub size: i64,
}
