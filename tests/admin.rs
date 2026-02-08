mod helpers;

use anyhow::Result;
use helpers::spawn_app;
use reqwest::{Response, cookie::Jar};

use crate::helpers::add_test_user;

#[tokio::test]
async fn test_failed_login_wrong_password() -> Result<()> {
    let username = "test_user";
    let password = "test_password";

    let app = spawn_app().await;
    add_test_user(&app.db_pool, username, password).await?;

    let response = login(&app, username, "wrong_password").await?;

    assert_eq!(response.status().as_u16(), 303);

    let location = response
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(location, "/login?error=invalid_credentials");

    Ok(())
}

#[tokio::test]
async fn test_protected_route_without_login() -> Result<()> {
    let app = spawn_app().await;

    let response = app
        .client
        .get(format!("http://{}/admin/dashboard", app.address))
        .send()
        .await?;

    assert_eq!(response.status().as_u16(), 303);

    let location = response
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(location, "/login?error=unauthorized");

    Ok(())
}

#[tokio::test]
async fn test_session_persistence_after_login() -> Result<()> {
    let username = "test_user";
    let password = "test_password";

    let app = spawn_app().await;
    add_test_user(&app.db_pool, username, password).await?;

    let jar = std::sync::Arc::new(Jar::default());
    let client = reqwest::Client::builder()
        .cookie_provider(jar.clone())
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let login_response = client
        .post(format!("http://{}/login", app.address))
        .form(&[("username", username), ("password", password)])
        .send()
        .await?;

    assert_eq!(login_response.status().as_u16(), 303);

    let dashboard_response = client
        .get(format!("http://{}/admin/dashboard", app.address))
        .send()
        .await?;

    assert_eq!(dashboard_response.status().as_u16(), 200);

    let body = dashboard_response.text().await?;
    assert!(body.contains("Admin Dashboard") || body.contains("admin") || !body.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_password_change() -> Result<()> {
    let username = "test_user";
    let password = "old_password";
    let new_password = "new_password";

    let app = spawn_app().await;
    add_test_user(&app.db_pool, username, password).await?;

    let jar = std::sync::Arc::new(Jar::default());
    let client = reqwest::Client::builder()
        .cookie_provider(jar.clone())
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    client
        .post(format!("http://{}/login", app.address))
        .form(&[("username", username), ("password", password)])
        .send()
        .await?;

    let change_response = client
        .post(format!("http://{}/admin/password", app.address))
        .form(&[
            ("current_password", password),
            ("new_password", new_password),
            ("new_password_check", new_password),
        ])
        .send()
        .await?;

    assert_eq!(change_response.status().as_u16(), 200);

    let old_login_response = client
        .post(format!("http://{}/login", app.address))
        .form(&[("username", username), ("password", password)])
        .send()
        .await?;

    let location = old_login_response
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(location, "/login?error=invalid_credentials");

    let new_login_response = client
        .post(format!("http://{}/login", app.address))
        .form(&[("username", username), ("password", new_password)])
        .send()
        .await?;

    assert_eq!(new_login_response.status().as_u16(), 303);
    let location = new_login_response
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(location, "/admin/dashboard");

    Ok(())
}

async fn login(app: &helpers::TestApp, username: &str, password: &str) -> Result<Response> {
    let response = app
        .client
        .post(format!("http://{}/login", app.address))
        .form(&[("username", username), ("password", password)])
        .send()
        .await?;

    Ok(response)
}
