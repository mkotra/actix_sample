use serde::Deserialize;
use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // Initialize dotenv to load environment variables from .env file, if it exists
        dotenv::dotenv().ok();

        // Load configuration from default.toml and environment variables
        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        config.try_deserialize()
    }
}
