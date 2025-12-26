#[tokio::test]
async fn test_health_status() {
    let address = spawn_app().await;

    let response = reqwest::get(format!("http://{address}/status"))
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

async fn spawn_app() -> String {
    let port = rand::random_range(2000..9000);
    let address = format!("127.0.0.1:{}", port);

    let server = email_newsletter::run(&address).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    address
}

#[tokio::test]
async fn test_subscribe() {
    let address = spawn_app().await;
    let test_data = vec![
        ("name=le%20guin&email=ursula_le_guin%40gmail.com", 200),
        ("name=le%20guin", 400),
        ("email=ursula_le_guin%40gmail.com", 400),
        ("", 400),
    ];

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
}
