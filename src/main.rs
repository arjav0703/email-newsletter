use anyhow::Result;
use email_newsletter::run;
use email_newsletter::{
    config,
    telemetry::{get_subscriber, init_subscriber},
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Tracing subscriber
    let subscriber = get_subscriber("email_newsletter".into(), "info".into());
    init_subscriber(subscriber);
    info!("Starting up...");

    // (from config.yaml)
    let config = config::Settings::from_yaml().await?;
    let address = config.get_address();

    let connection = config.database_settings.try_connect().await.map_err(|e| {
        error!("Failed to connect to the database: {}", e);
        e
    })?;

    run(&address, connection)?.await?;
    Ok(())
}
