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
pub async fn request(
        method: &str,
        endpoint: &str,
        body: Value,
        user: &Option<UserData>
    ) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("{}/{}", ENDPOINT, endpoint);

    match method {
        "GET" => {
            let response = client
                .get(&url)
                .header("Authorization", match user {
                    Some(user) => user.token.clone(),
                    None => "unset".to_string()
                })
                .send()
                .await?;
            Ok(response)
        }
        "POST" => {
            let response = client
                .post(&url)
                .header("Content-Type", "application/json")
                .header("Authorization", match user {
                    Some(user) => user.token.clone(),
                    None => "unset".to_string()
                })
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
#[derive(Debug, Clone, Deserialize)]
pub struct UserData {
    pub user: User,

    /// The user's token.
    pub token: String,
}

/// User (information) struct.
/// All the keys are matched to the API's response.
#[derive(Debug, Clone, Deserialize)]
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
    let response = request(
        "POST",
        "users/@login",
        json!({
            "username": username,
            "password": password
        }),
        &None
    ).await?;

    match response.status() {
        StatusCode::OK => {
            let body = response.text().await?;
            let json: serde_json::Value = serde_json::from_str(&body)?;

            Ok(serde_json::from_value(json)?)
        }
        _ => {
            let body = response.text().await?;
            let json: serde_json::Value = serde_json::from_str(&body)?;
            let message = json.get("error").expect("Got an error but there is no error message").to_string();

            Err(anyhow!(message))
        }
    }
}

/// Single machine data struct.
/// All the keys are matched to the API's response.
#[derive(Debug, Deserialize)]
pub struct Machine {
    /// The static data of the machine.
    pub static_data: Option<StaticData>,

    /// The user ID of the owner of the machine.
    pub owner_uuid: String,

    /// The hardware ID.
    pub hardware_uuid: String,

    /// The (host) name of the machine.
    pub name: String,

    pub status: String,
    // HACK: never seen a machine without the empty array
    // assuming it's a string
    pub access: Option<Vec<String>>,

    pub created_at: i64,
    pub updated_at: i64,

    /// ID of the machine.
    pub uuid: String,
}

/// The static data of the machine.
#[derive(Debug, Deserialize)]
pub struct StaticData {
    /// The machine's host name.
    pub hostname: String,

    /// The machine's version.
    pub os_version: Option<String>,

    /// The machine's operating system.
    pub os_name: String,

    /// Total cores this machine has.
    pub cpu_cores: u16,

    /// The CPU model name of the machine.
    pub cpu_model: String,

    /// The thread count for the single core.
    pub cpu_threads: u16,

    /// Total memory this machine has.
    pub total_mem: Option<u64>,

    /// The reporter version of this machine.
    pub reporter_version: Option<String>,
}

pub async fn get_machines(user: &UserData) -> Result<Vec<Machine>, anyhow::Error> {
    let response = request(
        "GET",
        "users/@me/machines",
        json!({}),
        &Some(user.clone())
    ).await?;

    match response.status() {
        StatusCode::OK => {
            let body = response.text().await?;
            let json: serde_json::Value = serde_json::from_str(&body)?;

            Ok(serde_json::from_value(json)?)
        }
        _ => {
            let body = response.text().await?;
            let json: serde_json::Value = serde_json::from_str(&body)?;
            let message = json.get("error").expect("Got an error but there is no error message").to_string();

            Err(anyhow!(message))
        }
    }
}