#[tokio::test]
async fn test_health_status() {
    let port = rand::random_range(2000..9000);
    let address = format!("127.0.0.1:{}", port);

    spawn_app(&address).await;

    let response = reqwest::get(format!("http://{address}/status"))
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

async fn spawn_app(address: &str) {
    let server = email_newsletter::run(address).expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
