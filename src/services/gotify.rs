use std::sync::{Arc, Mutex, MutexGuard};

use lazy_static::lazy_static;
use reqwest::{Method, RequestBuilder};
use serde_json::Value;

use crate::models::gotify::{
    client::{ClientModel, CreateClientEnum},
    error::ErrorModel,
    paged_messages::PagedMessagesModel,
    version_info::VersionInfoModel,
};

lazy_static! {
    pub static ref GOTIFY_SERVICE: Arc<Mutex<GotifyService>> =
        Arc::new(Mutex::new(GotifyService::new()));
}

pub struct GotifyService {
    pub base_url: Option<String>,
    pub token: Option<String>,
}

impl GotifyService {
    fn new() -> Self {
        Self {
            base_url: None,
            token: None,
        }
    }

    #[allow(dead_code)]
    fn basic_request_builder(
        &self,
        method: Method,
        base_url: String,
        path: String,
    ) -> RequestBuilder {
        reqwest::Client::new().request(method, format!("{}{}", base_url, path))
    }

    #[allow(dead_code)]
    fn request_builder(&self, method: Method, path: String) -> RequestBuilder {
        let base_url = self
            .base_url
            .clone()
            .unwrap_or("http://localhost".to_string());

        self.basic_request_builder(method, base_url, path)
    }

    #[allow(dead_code)]
    async fn request_auth(&self, method: Method, path: String) -> Value {
        let token = self.token.clone().unwrap_or("".to_string());

        let client = self
            .request_builder(method, path)
            .header("X-Gotify-Key", token);

        client.send().await.unwrap().json::<Value>().await.unwrap()
    }

    pub fn instance() -> MutexGuard<'static, GotifyService> {
        return GOTIFY_SERVICE.lock().unwrap();
    }

    #[allow(dead_code)]
    pub async fn set_base_url(
        &mut self,
        base_url: String,
    ) -> Result<VersionInfoModel, Box<dyn std::error::Error>> {
        let request =
            self.basic_request_builder(Method::GET, base_url.clone(), "/version".to_string());
        let response = request.send().await?;
        let json = response.json::<Value>().await?;

        match serde_json::from_value::<VersionInfoModel>(json.clone()) {
            Ok(model) => {
                log::info!("Found Gotify server version {}.", model.version);
                self.base_url = Some(base_url);
                return Ok(model);
            }
            Err(error) => {
                log::error!("No Gotify instance was found.");
                return Err(Box::new(error));
            }
        }
    }

    pub async fn create_client(&mut self, username: &str, password: &str) -> CreateClientEnum {
        let body = ClientModel::new("Herald");

        let client = self
            .request_builder(Method::POST, "/client".to_string())
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
            self.token = model.token.clone();

            return CreateClientEnum::Success(model);
        }

        CreateClientEnum::Unmatched(value.clone())
    }

    pub async fn get_messages(&self) -> Result<PagedMessagesModel, Box<dyn std::error::Error>> {
        let value = self.request_auth(Method::GET, "/message".to_string()).await;

        match serde_json::from_value::<PagedMessagesModel>(value) {
            Ok(messages) => {
                log::info!("Retrieved messages");
                return Ok(messages);
            }
            Err(err) => {
                log::error!("{}", err);
                return Err(Box::new(err));
            }
        }
    }
}
