use email_newsletter::config::Settings;
use email_newsletter::email_client::EmailClient;
use sqlx::{Connection, Executor, PgConnection, PgPool};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let port = rand::random_range(2000..9000);
    let address = format!("127.0.0.1:{}", port);
    let mut config = Settings::from_yaml()
        .await
        .expect("Failed to read configuration.");
    config.database_settings.database_name = format!("test_db_{}", uuid::Uuid::new_v4());
    let pool = configure_database(&config).await;

    let email_config = EmailClient::new(
        config.email_settings.base_url.clone(),
        config.email_settings.sender_email.clone(),
        config.email_settings.resend_api_key.clone(),
    );
    let redis_uri = config.redis_settings.uri.clone();

    let server = email_newsletter::run(&address, pool.clone(), email_config, redis_uri)
        .await
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: pool,
    }
}

pub async fn configure_database(config: &Settings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(
        &config
            .database_settings
            .connection_string_without_db()
            .await,
    )
    .await
    .expect("Failed to connect to postgress");

    connection
        .execute(
            format!(
                r#"CREATE DATABASE "{}";"#,
                config.database_settings.database_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.database_settings.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

use anyhow::Result;
pub async fn send_subscribe_req(address: &str, body: String) -> Result<reqwest::Response> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://{address}/subscribe"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    Ok(response)
}
