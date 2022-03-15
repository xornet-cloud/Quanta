use serde_json::json;
use reqwest::StatusCode;

const ENDPOINT: &str = "https://backend.xornet.cloud";

/// Login to the API. Returns a token if successful.
pub async fn login(username: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}/users/@login", ENDPOINT);
    let body = json!({
        "username": username,
        "password": password
    });

    let res = client
        .post(&url)
        .json(&body)
        .send().await?;

    match res.status() {
        StatusCode::OK => {
            let body = res.text().await?;
            let json: serde_json::Value = serde_json::from_str(&body)?;
            let token = json.get("token").expect("No token in login").to_string();

            Ok(token)
        }
        _ => {
            let body = res.text().await?;
            let json: serde_json::Value = serde_json::from_str(&body)?;
            let message = json.get("error").expect("No error???").to_string();

            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, message)))
        }
    }
}