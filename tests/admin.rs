mod helpers;
use actix_web::HttpResponse;
use anyhow::Result;
use helpers::spawn_app;
use reqwest::Client;

use crate::helpers::add_test_user;

#[tokio::test]
async fn check_auth() -> Result<()> {
    let username = "test_user";
    let password = "test_password";

    let app = spawn_app().await;
    add_test_user(&app.db_pool, username, password).await?;

    let status = login(username, password, &app.address).await.unwrap();
    assert_eq!(status, 200);

    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize)]
struct FormData {
    pub username: String,
    pub password: String,
}

async fn login(username: &str, password: &str, address: &str) -> Result<u16> {
    let client = Client::new();
    let formdata = FormData {
        username: username.to_string(),
        password: password.to_string(),
    };
    let req = client
        .post(format!("http://{address}/login"))
        // .basic_auth(username, Some(password))
        .form(&formdata)
        .send()
        .await
        .unwrap();
    let status = req.status();
    let header = req.headers();
    dbg!(header, status);
    todo!();
    // Ok(1)
}
