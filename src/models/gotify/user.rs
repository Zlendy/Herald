use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserModel {
    pub admin: bool,
    pub id: i64,
    pub name: String,
}
