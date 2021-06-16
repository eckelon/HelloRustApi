use lazy_static::lazy_static;
use std::env;

pub struct Config {
    pub port: u16,
}

fn get_port() -> u16 {
    match env::var("HELLO_RUST_API_PORT") {
        Ok(variable) => variable.parse::<u16>().unwrap(),
        Err(_) => {
            log::warn!("HELLO_RUST_API_PORT env variable has not been customized!");
            3000
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config { port: get_port() };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config_customized_port() {
        env::set_var("HELLO_RUST_API_PORT", "8080");
        let port = get_port();
        assert_eq!(port, 8080);
    }
    #[test]
    fn test_config_default_port() {
        env::remove_var("HELLO_RUST_API_PORT");
        let port = get_port();
        assert_eq!(port, 3000);
    }
}
