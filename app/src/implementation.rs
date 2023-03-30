use serde_derive::{Serialize, Deserialize};
use serde_json::Value;

const GOTIFY: &str = "https://monitoring.beauvoir.local/gotify";

// TODO: Remove CXX-Qt dependency

pub struct GotifyRustop {
    username: String,
    password: String,
    token: String,
}

impl Default for GotifyRustop {
    fn default() -> Self {
        Self {
            username: String::from(""),
            password: String::from(""),
            token: String::from("N/A"),
        }
    }
}

impl GotifyRustop {
    pub async fn create_client(username: &str, password: &str) -> Result<Value, Box<dyn std::error::Error>> {   
        let body: ClientModel = ClientModel::new("Gotify Rustop");
    
        let client = reqwest::Client::new()
            .post(format!("{}/client", GOTIFY))
            .basic_auth(username, Some(password))
            .json::<ClientModel>(&body);
    
        let resp = client.send()
            .await?
            .json::<Value>()
            .await?;
    
        Ok(resp)
    }
}

#[derive(Serialize, Deserialize)]
struct ClientModel {
    id: Option<i32>,
    name: String,
    token: Option<String>,
}

impl ClientModel {
    fn new(name: &str) -> Self {
        Self {
            id: None,
            name: name.to_string(),
            token: None,
        }
    }
}
