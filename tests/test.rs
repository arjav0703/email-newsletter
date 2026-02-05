use email_newsletter::email_client::EmailClient;
use sqlx::{Connection, Executor, PgConnection, PgPool};

#[tokio::test]
async fn test_health_status() {
    let address = spawn_app().await.address;

    let response = reqwest::get(format!("http://{address}/status"))
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

struct TestApp {
    address: String,
    db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
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

    let server = email_newsletter::run(&address, pool.clone(), email_config)
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: pool,
    }
}

use email_newsletter::config::Settings;
#[tokio::test]
async fn test_subscribe() {
    let app = spawn_app().await;
    let address = &app.address;
    let test_data = vec![
        ("name=arjav&email=arjavjain0703%40gmail.com", 200),
        ("name=arjav", 400),
        ("email=arjavjain0703%40gmail.com", 400),
        ("", 400),
    ];

    let config = Settings::from_yaml()
        .await
        .expect("Failed to read configuration.");

    let client = reqwest::Client::new();

    for (body, expected_status) in test_data {
        let response = client
            .post(format!("http://{address}/subscribe"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            response.status().as_u16(),
            expected_status,
            "Failed for body: {}",
            body
        );
        println!("Test passed for body: {}", body);
    }
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "arjavjain0703@gmail.com");
    assert_eq!(saved.name, "arjav");
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

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;

    let url = format!("http://{}/subscriptions/confirm", app.address);
    dbg!(&url);

    let response = reqwest::get(&format!("http://{}/subscriptions/confirm", app.address))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);
}
