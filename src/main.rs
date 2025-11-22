//! # Velo Binary
//!
//! The entry point for the Velo application. It initializes the environment,
//! connects to the database, and starts the HTTP server.

use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use velo::configuration::get_configuration;
use velo::startup::run;
use velo::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("velo".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");

    let connection =
        PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
