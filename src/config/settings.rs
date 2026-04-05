use serde::Deserialize;
use config::{Config, Environment};
use dotenvy::dotenv;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database_url: String,
    pub port: u16,
    #[serde(default = "default_max_connections")]
    pub db_max_connections: u32,
}

fn default_max_connections() -> u32 {
    5
}

impl Settings {
    pub fn new() -> Self {
        dotenv().ok();

        let config = Config::builder()
            .add_source(Environment::default())
            .build()
            .expect("Failed to load environment variables");

        config
            .try_deserialize::<Settings>()
            .expect("Failed to deserialize config")
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}