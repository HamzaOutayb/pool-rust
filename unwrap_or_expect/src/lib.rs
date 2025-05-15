pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    let res = match security_level {
        Security::Message => {
            server.expect("ERROR: program stops")
        }
        
        Security::Unknown => {
            server.unwrap()
        }
        Security::Warning => {
            server.unwrap_or("WARNING: check the server")
        }
        Security::NotFound => {
            match server {
                Ok(a) => a,
                Err(err) => &format!("Not found: {}", err),
            }
        }

        Security::UnexpectedUrl => server.unwrap_err()
    };
    res.to_string()
}