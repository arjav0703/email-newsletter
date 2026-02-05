mod helpers;
use helpers::spawn_app;

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
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "arjavjain0703@gmail.com");
    assert_eq!(saved.name, "arjav");
    assert_eq!(saved.status, "pending_confirmation");
}
