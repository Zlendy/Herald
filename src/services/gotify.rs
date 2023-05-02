use serde_json::Value;

use crate::models::gotify::client::ClientModel;

pub(crate) struct GotifyService {
    pub token: Option<String>,
}

impl GotifyService {
    async fn login(username: String, password: String) {
        
    }
}

async fn create_client(
    username: &str,
    password: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let body = ClientModel::new("Gotify Rustop");

    let client = reqwest::Client::new()
        .post(format!("{}/client", ""))
        .basic_auth(username, Some(password))
        .json::<ClientModel>(&body);

    let resp = client.send().await?.json::<Value>().await?;

    Ok(resp)
}