#[tokio::test]
async fn test_health_status() {
    spawn_app().await;

    let response = reqwest::get("http://127.0.0.1:8080/status")
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

async fn spawn_app() {
    let server = email_newsletter::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
