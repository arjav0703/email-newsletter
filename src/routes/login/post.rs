use crate::auth::{AuthError, Credentials};
use actix_web::{HttpResponse, http::header::LOCATION, web};
use anyhow::{Context, Result};
use secrecy::Secret;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct LoginFormData {
    pub username: String,
    pub password: Secret<String>,
}

pub async fn login_post(
    request_data: web::Form<LoginFormData>,
    connection: web::Data<PgPool>,
) -> Result<HttpResponse, AuthError> {
    let credentials = Credentials {
        username: request_data.username.clone(),
        password: request_data.password.clone(),
    };

    let is_valid = credentials
        .validate(connection.as_ref())
        .await
        .context("Failed to validate credentials")?;

    if !is_valid {
        return Ok(HttpResponse::Unauthorized()
            .append_header((LOCATION, "/login"))
            .finish());
    }

    Ok(HttpResponse::SeeOther()
        .append_header((LOCATION, "/"))
        .finish())
}
