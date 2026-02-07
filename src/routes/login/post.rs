use crate::auth::{AuthError, Credentials};
use actix_session::Session;
use actix_web::{HttpResponse, http::header::LOCATION, web};
use anyhow::{Context, Result};
use secrecy::Secret;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct LoginFormData {
    pub username: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "Login Form [POST]", skip(request_data, connection, session))]
pub async fn login_post(
    request_data: web::Form<LoginFormData>,
    connection: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse, AuthError> {
    let credentials = Credentials {
        username: request_data.username.clone(),
        password: request_data.password.clone(),
    };

    let is_valid = credentials
        .validate(connection.as_ref())
        .await
        .context("Failed to validate credentials")?;

    match is_valid {
        false => {
            return Ok(HttpResponse::SeeOther()
                .append_header((LOCATION, "/login?error=invalid_credentials"))
                .finish());
        }
        true => {
            session
                .insert("username", request_data.username.clone())
                .context("Failed to store username in session")?;
            return Ok(HttpResponse::SeeOther()
                .append_header((LOCATION, "/admin/dashboard"))
                .finish());
        }
    }
}
