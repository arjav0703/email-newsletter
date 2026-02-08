mod helpers;
use std::collections::HashMap;

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

    let status = login(username, password, &app.address, app.client)
        .await
        .unwrap();
    assert_eq!(status, 200);

    Ok(())
}

async fn login(username: &str, password: &str, address: &str, client: Client) -> Result<u16> {
    let mut formdata = HashMap::new();
    formdata.insert("username", username);
    formdata.insert("password", password);

    let req = client
        .post(format!("http://{address}/login"))
        .form(&formdata)
        .send()
        .await
        .unwrap();
    let status = req.status();
    let header = req.text().await.unwrap();
    dbg!(header, status);
    Ok(1)
}
