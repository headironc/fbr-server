use once_cell::sync::OnceCell;
use std::env::var;
use std::net::SocketAddr;
use tracing::info;

static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Debug)]
pub struct Config {
    addr: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        let addr = Self::env_addr();

        Self { addr }
    }
}

impl Config {
    pub fn instance() -> &'static Config {
        CONFIG.get_or_init(Self::default)
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    fn env_addr() -> SocketAddr {
        let host = if cfg!(debug_assertions) {
            [127, 0, 0, 1]
        } else {
            [0, 0, 0, 0]
        };

        let port = match var("PORT") {
            Ok(port) => match port.parse::<u16>() {
                Ok(port) => port,
                Err(_) => {
                    info!("Invalid PORT number, using default 5008");
                    5008
                }
            },
            Err(_) => {
                info!("Env variable PORT not found, using default 5008");
                5008
            }
        };

        SocketAddr::from((host, port))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_addr() {
        std::env::set_var("PORT", "5008");

        let addr = Config::env_addr();

        assert_eq!(addr.port(), 5008);
    }
}
