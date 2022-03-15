use std::io::{stdin, stdout, Error, Write};

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