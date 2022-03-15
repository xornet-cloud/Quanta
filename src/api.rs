use serde::Deserialize;
use serde_json::{json, Value};
use reqwest::{StatusCode, Response};
use anyhow::anyhow;

const ENDPOINT: &str = "https://backend.xornet.cloud";

/// Requests to the API.
/// * `method` - The HTTP method to use. (GET, POST, etc.)
/// * `endpoint` - The endpoint to request to.
/// * `body` - The body of the request.
/// Example:
/// ```
/// let response = api::request("POST", "users/@login", json!({
///     "username": "username",
///     "password": "password"
/// }));
///
/// match response.status() {
///     StatusCode::OK => { println!("yay!"); },
///     _ => { eprintln!("epic fail"); }
/// }
/// ```
pub async fn request(method: &str, endpoint: &str, body: Value) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("{}/{}", ENDPOINT, endpoint);

    match method {
        "POST" => {
            let response = client
                .post(&url)
                .header("Content-Type", "application/json")
                .body(body.to_string())
                .send()
                .await?;
            Ok(response)
        }
        _ => todo!(),
    }
}

/// User data struct.
/// All the keys are matched to the API's response.
#[derive(Debug, Deserialize)]
pub struct UserData {
    pub user: User,

    /// The user's token.
    pub token: String,
}

/// User (information) struct.
/// All the keys are matched to the API's response.
#[derive(Debug, Deserialize)]
pub struct User {
    /// The name of the user.
    pub username: String,

    /// The user ID.
    pub uuid: String,

    /// User's avatar link.
    pub avatar: String,

    /// User's banner link.
    pub banner: String,

    pub created_at: i64,
    pub updated_at: i64,
}

/// Login to the API. Returns a token if successful.
/// * `username` - The username to login with.
/// * `password` - The password to login with.
pub async fn login(username: &str, password: &str) -> Result<UserData, anyhow::Error> {
    let response = request("POST", "users/@login", json!({
        "username": username,
        "password": password
    })).await?;

    match response.status() {
        StatusCode::OK => {
            let body = response.text().await?;
            let json: serde_json::Value = serde_json::from_str(&body)?;

            Ok(serde_json::from_value(json)?)
        }
        _ => {
            let body = response.text().await?;
            let json: serde_json::Value = serde_json::from_str(&body)?;
            let message = json.get("error").expect("No error???").to_string();

            Err(anyhow!(message))
        }
    }
}