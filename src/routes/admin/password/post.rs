use crate::auth::{AuthError, Credentials};
use actix_session::Session;
use actix_web::{HttpResponse, web};
use anyhow::{Context, Result};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

#[derive(serde::Deserialize, Debug)]
pub struct PasswordChangeRequest {
    current_password: String,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

impl PasswordChangeRequest {
    fn validate_password(&self) -> bool {
        self.new_password.expose_secret() == self.new_password_check.expose_secret()
    }
}

#[tracing::instrument(name = "Changing user password", skip(formdata, session, connection))]
pub async fn password_post(
    formdata: web::Form<PasswordChangeRequest>,
    session: Session,
    connection: web::Data<PgPool>,
) -> Result<HttpResponse, AuthError> {
    let username: Option<String> = session
        .get("username")
        .unwrap_or(None)
        .context("Unauthorized user")?;
    if username.is_none() {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    }

    if !formdata.validate_password() {
        return Ok(HttpResponse::BadRequest().body("New password and confirmation do not match"));
    }

    let credentials = Credentials::from(&username.unwrap_or_default(), &formdata.current_password)
        .context("Failed to parse credentials")?;

    if !credentials.validate(connection.get_ref()).await? {
        return Ok(HttpResponse::Unauthorized().body("Current password is incorrect"));
    }

    credentials
        .change_password(formdata.new_password.clone(), connection.get_ref())
        .await?;

    Ok(HttpResponse::Ok().finish())
}
