use std::env;
use dotenv::dotenv;

use serde_json::Value;
use serde_json::value::from_value;

use lazy_static::lazy_static;

lazy_static! {
    static ref GOTIFY: String = format!("https://{}", env::var("BASE_URL").unwrap());
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load env variables in .env

    println!("-- Application list --");
    let apps = get_applications().await.unwrap();
    println!("{:#?}", &apps);

    println!("\n-- Message list --");
    let messages = get_messages().await.unwrap();
    println!("{:#?}", &messages);
    
    println!("\n-- Message list from third application --");
    let message = get_message(
            from_value::<i32>(apps[2]["id"].clone()
        ).unwrap()).await.unwrap();
    println!("{:#?}", &message);
}

// This code should be abstracted into one generic function

async fn get_applications() -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new()
        .get(format!("{}/application", *GOTIFY))
        .header("X-Gotify-Key", env::var("TOKEN").unwrap());

    let resp = client.send()
        .await?
        .json::<Value>()
        .await?;

    Ok(resp)
}

async fn get_messages() -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new()
        .get(format!("{}/message", *GOTIFY))
        .header("X-Gotify-Key", env::var("TOKEN").unwrap());

    let resp = client.send()
        .await?
        .json::<Value>()
        .await?;

    Ok(resp)
}

async fn get_message(id: i32) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new()
        .get(format!("{}/application/{id}/message", *GOTIFY))
        .header("X-Gotify-Key", env::var("TOKEN").unwrap());

    let resp = client.send()
        .await?
        .json::<Value>()
        .await?;

    Ok(resp)
}