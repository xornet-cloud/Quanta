/// API requests
pub mod api;
use crate::api::UserData;

/// Input and output in stdin/stdout
pub mod io;

/// Terminal UI
pub mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{} v{}\n", ui::BANNER, env!("CARGO_PKG_VERSION"));

    let username = io::get_input(false)?;
    let password = io::get_input(true)?;

    let user: UserData = match api::login(&username, &password).await {
        Ok(data) => {
            println!("Logged in as `{}`", data.user.username);
            data
        },
        Err(e) => {
            println!("Error: {}", e.to_string());
            std::process::exit(0);
        }
    };

    dbg!(user);

    Ok(())
}
