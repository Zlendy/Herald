use std::env;
use std::io::stdin;
use dotenv::dotenv;

use reqwest::Response;
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;
use serde_json::value::from_value;

use lazy_static::lazy_static;

lazy_static! {
    static ref GOTIFY: String = format!("https://{}", env::var("BASE_URL").unwrap());
}
//
#[tokio::main]
async fn main() {
    dotenv().ok(); // Load env variables in .env
    
    println!("-- Create client --");
    let mut username = String::new();
    println!("Username: ");
    stdin().read_line(&mut username).unwrap();

    let mut password = String::new();
    println!("Password: ");
    stdin().read_line(&mut password).unwrap();

    let client = create_client(
            username.as_str().trim(),
            password.as_str().trim(),
        ).await.unwrap();
    println!("{:#?}", &client);

    let token = &client["token"];
    println!("{}", &token);


    println!("\n-- Get current user --");
    let current_user = get_current_user(&token.to_string()).await.unwrap();
    println!("{:#?}", &current_user);

    
    println!("\n-- Delete client --");
    let response = delete_client(
            &token.to_string(),
            from_value::<i32>(client["id"].clone()).unwrap(),
        ).await.unwrap();
    println!("{:#?}", &response);
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

async fn create_client(username: &str, password: &str) -> Result<Value, Box<dyn std::error::Error>> {   
    let body = ClientModel::new("Gotify Rustop");

    let client = reqwest::Client::new()
        .post(format!("{}/client", *GOTIFY))
        .basic_auth(username, Some(password))
        .json::<ClientModel>(&body);

    let resp = client.send()
        .await?
        .json::<Value>()
        .await?;

    Ok(resp)
}

async fn get_current_user(token: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new()
        .get(format!("{}/current/user", *GOTIFY))
        .header("X-Gotify-Key", token);

    let resp = client.send()
        .await?
        .json::<Value>()
        .await?;

    Ok(resp)
}

async fn delete_client(token: &str, id: i32) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new()
        .delete(format!("{}/client/{id}", *GOTIFY))
        .header("X-Gotify-Key", token);

    let resp = client.send().await?;

    Ok(resp)
}