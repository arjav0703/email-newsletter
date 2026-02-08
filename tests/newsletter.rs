mod helpers;
use helpers::{add_test_user, send_subscribe_req, spawn_app};

#[tokio::test]
async fn newsletter_returns_400_for_invalid_data() {
    let app = spawn_app().await;
    let address = &app.address;

    let test_data = serde_json::json!({
        "content" : "This is the content of the newsletter",
    });

    let response = reqwest::Client::new()
        .post(format!("http://{address}/newsletter"))
        .json(&test_data)
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn newsletter_rejects_unauthorized_users() {
    let app = spawn_app().await;
    let address = &app.address;

    let test_data = serde_json::json!({
        "title" : "Newsletter title",
        "content" : "This is the content of the newsletter",
        "html_content" : "<p>This is the content of the newsletter</p>",
    });

    let response = reqwest::Client::new()
        .post(format!("http://{address}/newsletter"))
        .basic_auth("wrong_user", Some("wrong_password"))
        .json(&test_data)
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn newsletter_sends_email_to_confirmed_subscribers() {
    let app = spawn_app().await;
    let address = &app.address;

    add_test_user(&app.db_pool, "test_user", "test_password")
        .await
        .expect("Failed to create test user");

    let body = "name=arjav&email=arjavjain0703%40gmail.com".to_string();
    let resp = send_subscribe_req(address, body)
        .await
        .expect("Failed to execute subscribe request");
    assert_eq!(resp.status().as_u16(), 200);

    let test_data = serde_json::json!({
        "title" : "Newsletter title",
        "content" : "This is the content of the newsletter",
        "html_content" : "<p>This is the content of the newsletter</p>",
    });

    let response = reqwest::Client::new()
        .post(format!("http://{address}/newsletter"))
        .basic_auth("test_user", Some("test_password"))
        .json(&test_data)
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(response.status().as_u16(), 200);
}
