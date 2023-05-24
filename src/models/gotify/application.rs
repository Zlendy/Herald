use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationModel {
    pub description: String,
    pub id: i32,
    pub image: String,
    pub internal: bool,
    pub name: String,
    pub token: String,
}

// impl ApplicationModel {
//     pub fn new(name: &str) -> Self {
//         Self {
//             id: None,
//             name: name.to_string(),
//             token: None,
//         }
//     }
// }
