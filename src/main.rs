
/// API requests
pub mod api;

/// Input and output in stdin/stdout
pub mod io;

/// Terminal UI
pub mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{} v{}\n", ui::BANNER, env!("CARGO_PKG_VERSION"));

    let username = io::get_input(false)?;
    let password = io::get_input(true)?;

    let token = match api::login(&username, &password).await {
        Ok(token) => {
            println!("Logged in as `{}`", username);
            println!("Token: {}", token);
        },
        Err(e) => {
            println!("Error: {}", e.to_string());
        }
    };

    Ok(())
}
