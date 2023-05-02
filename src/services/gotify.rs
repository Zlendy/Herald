use serde_json::Value;

use crate::models::gotify::{client::{ClientModel, CreateClientEnum}, error::ErrorModel};

const GOTIFY: &str = "http://monitoring.beauvoir.local/gotify";

pub struct GotifyService {
    pub url: Option<String>,
    pub token: Option<String>,
}

impl GotifyService {
    pub fn new() -> Self {
        Self {
            url: None,
            token: None,
        }
    }

    pub async fn set_url(url: String) {
        
    }

    pub async fn create_client(
        username: &str,
        password: &str,
    ) -> CreateClientEnum {
        let body = ClientModel::new("Herald");
    
        let client = reqwest::Client::new()
            .post(format!("{}/client", GOTIFY))
            .basic_auth(username, Some(password))
            .json::<ClientModel>(&body);
    
        let result = client.send().await;

        let Ok(response) = result else {
            let description = "Could not create a new client.".to_string();
            log::error!("{}", description);

            return CreateClientEnum::Error(ErrorModel::new(description));
        };

        let status = response.status();
        let value = &response.json::<Value>().await.unwrap();

        log::debug!("{}", value);

        if status.is_client_error() {
            let model: ErrorModel = serde_json::from_value(value.clone()).unwrap();
            log::error!("{}: {}", model.error, model.error_descripton);

            return CreateClientEnum::Error(model);
        }

        if status.is_success() {
            let model: ClientModel = serde_json::from_value(value.clone()).unwrap();
            log::info!("Created client \"{}\"", model.name);

            return CreateClientEnum::Success(model);
        }

        CreateClientEnum::Unmatched(value.clone())
    }
}