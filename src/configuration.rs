//! # Configuration
//!
//! Handles reading and parsing application configuration from files.

use config::ConfigError;
use serde::Deserialize;

/// Application-level settings.
#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

/// Database connection settings.
#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    /// Generates a connection string for the Postgres database.
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    /// Generates a connection string to the Postgres instance (without specifying a DB).
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

/// Reads configuration from `config.yaml`.
pub fn get_configuration() -> Result<Settings, ConfigError> {
    // Initialize configuration reader
    let settings = config::Config::builder()
        .add_source(config::File::new("config.yaml", config::FileFormat::Yaml))
        .build()?;

    settings.try_deserialize::<Settings>()
}
