use anyhow::Result;
use email_newsletter::{
    auth::Credentials,
    config,
    email_client::EmailClient,
    run,
    telemetry::{get_subscriber, init_subscriber},
};
use secrecy::ExposeSecret;
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

    let super_user: Credentials = Credentials::from(
        &config.super_user.username,
        config.super_user.password.expose_secret(),
    )?;
    super_user
        .add_user_to_db(&connection)
        .await
        .unwrap_or_default();

    run(&address, connection, email_config, redis_uri)
        .await?
        .await?;
    Ok(())
}
