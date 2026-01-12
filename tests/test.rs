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
    let config = Settings::from_yaml()
        .await
        .expect("Failed to read configuration.");
    let pool = config
        .database_settings
        .connect()
        .await
        .expect("Failed to connect to DB.");

    let server = email_newsletter::run(&address, pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    let app = TestApp {
        address,
        db_pool: pool,
    };
    app
}

use email_newsletter::config::Settings;
use sqlx::PgPool;
#[tokio::test]
async fn test_subscribe() {
    let app = spawn_app().await;
    let address = &app.address;
    let test_data = vec![
        ("name=le%20guin&email=ursula_le_guin%40gmail.com", 200),
        ("name=le%20guin", 400),
        ("email=ursula_le_guin%40gmail.com", 400),
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

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}
