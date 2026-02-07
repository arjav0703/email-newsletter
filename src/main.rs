use anyhow::Result;
use email_newsletter::email_client::EmailClient;
use email_newsletter::{auth, run};
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
    let email_config = EmailClient::new(
        config.email_settings.base_url.clone(),
        config.email_settings.sender_email.clone(),
        config.email_settings.resend_api_key.clone(),
    );
    let redis_uri = config.redis_settings.uri.clone();
    let connection = config.database_settings.try_connect().await.map_err(|e| {
        error!("Failed to connect to the database: {}", e);
        e
    })?;

    if config.test_mode {
        auth::create_test_user(&connection).await?;
    }

    run(&address, connection, email_config, redis_uri)
        .await?
        .await?;
    Ok(())
}
