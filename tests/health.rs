mod helpers;
use helpers::spawn_app;

#[tokio::test]
async fn test_health_status() {
    let address = spawn_app().await.address;

    let response = reqwest::get(format!("http://{address}/status"))
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
