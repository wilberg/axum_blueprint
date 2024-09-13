use dotenvy::dotenv;
use std::env;
use tokio::sync::OnceCell;

struct ServerConfig {
    host: String,
    port: u16,
}

pub struct Config {
    server: ServerConfig,
}

impl Config {
    pub fn host(&self) -> &str {
        &self.server.host
    }
    pub fn port(&self) -> &u16 {
        &self.server.port
    }
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

async fn init_config() -> Config {
    dotenv().ok();

    Config {
        server: ServerConfig {
            host: env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
            port: env::var("PORT")
                .unwrap_or_else(|_| String::from("8080"))
                .parse::<u16>()
                .unwrap(),
        },
    }
}

pub async fn get() -> &'static Config {
    CONFIG.get_or_init(init_config).await
}
