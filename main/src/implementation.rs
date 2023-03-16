use serde_derive::{Serialize, Deserialize};
use serde_json::Value;

const GOTIFY: &str = "https://monitoring.beauvoir.local/gotify";

#[cxx_qt::bridge]
mod gotify_rustop {
    use super::*;

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject(qml_uri = "Rust.Test", qml_version = "0.1")]
    pub struct GotifyRustop {
        #[qproperty]
        username: QString,
        #[qproperty]
        password: QString,
    }

    impl Default for GotifyRustop {
        fn default() -> Self {
            Self {
                username: QString::from(""),
                password: QString::from(""),
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

    impl qobject::GotifyRustop {
        #[qinvokable]
        pub fn login(&self) {
            // Synchronous operation
            let username = self.rust().username.to_string();
            let password = self.rust().password.to_string();

            println!("Username: \"{username}\"");
            println!("Password: \"{password}\"");

            // Asynchronous operation
            tokio::spawn(async move {
                let response = GotifyRustop::create_client(username.as_str(), password.as_str()).await.unwrap();
                println!("{:#?}", &response);
            });
        }
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
