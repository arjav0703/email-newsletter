mod helpers;

use helpers::spawn_app;

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;

    let url = format!("http://{}/subscriptions/confirm", app.address);

    let response = reqwest::get(url).await.unwrap();

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn confirmation_link_returns_a_200_if_token_is_valid() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let body = "name=arjav&email=arjavjain0703%40gmail.com";

    client
        .post(format!("http://{}/subscribe", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    let saved = sqlx::query!("SELECT subscription_token FROM subscription_tokens",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    let url = format!(
        "http://{}/subscriptions/confirm?subscription_token={}",
        app.address, saved.subscription_token
    );

    let response = reqwest::get(url).await.unwrap();

    assert_eq!(response.status().as_u16(), 200);

    let confirmed = sqlx::query!("SELECT status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(confirmed.status, "confirmed");
}
