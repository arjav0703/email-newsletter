use anyhow::Result;
use email_newsletter::domain::SubscriberEmail;
use email_newsletter::email_client::EmailClient;
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
    let email_config = EmailClient::new(
        config.email_settings.sender_email.clone(),
        config.email_settings.resend_api_key.clone(),
    );

    let test_recipient = SubscriberEmail::parse("arjavjain0703@gmail.com".to_string()).unwrap();
    email_config
        .send_test_email(test_recipient)
        .await
        .map_err(|e| {
            error!("Failed to send test email: {}", e);
            e
        })?;

    let connection = config.database_settings.try_connect().await.map_err(|e| {
        error!("Failed to connect to the database: {}", e);
        e
    })?;

    run(&address, connection, email_config)?.await?;
    Ok(())
}
