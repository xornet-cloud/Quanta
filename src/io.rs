use std::io::{stdin, stdout, Error, Write};

/// Get input from stdin. Returns a String if successful, otherwise an Error.
/// * `is_password` - Whether the input should be a password.
pub fn get_input(is_password: bool) -> Result<String, Error> {
    let mut buffer = String::new();

    match is_password {
        false => {
            print!("Username: ");
            stdout().flush()?;
            stdin().read_line(&mut buffer)?;
        },
        true => {
            let password = rpassword::prompt_password("Password: ")?;
            buffer = password.to_string();
        }
    }

    Ok(buffer.trim_end_matches('\n').to_string())
}