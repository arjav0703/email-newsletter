use actix_web::http::header::{AUTHORIZATION, HeaderMap};
use anyhow::{Context, Result};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use base64::{Engine, engine::general_purpose::STANDARD};
use secrecy::{ExposeSecret, Secret};
use sqlx::{PgPool, query};

pub struct Credentials {
    username: String,
    password: Secret<String>,
}

impl Credentials {
    pub fn from(username: &str, password: &str) -> Result<Self> {
        if username.is_empty() || password.is_empty() {
            anyhow::bail!("Username and password must not be empty");
        }
        Ok(Credentials {
            username: username.to_string(),
            password: Secret::new(password.to_string()),
        })
    }
}

impl TryFrom<HeaderMap> for Credentials {
    fn try_from(headers: HeaderMap) -> Result<Self> {
        let auth_header = headers
            .get(AUTHORIZATION)
            .context("Authorization header is missing")?
            .to_str()
            .context("Authorization header is not valid UTF-8")?;

        let base64_credentials = auth_header
            .strip_prefix("Basic ")
            .context("Authorization header must use Basic authentication scheme")?;

        let decoded_bytes = STANDARD
            .decode(base64_credentials)
            .context("Failed to decode base64 credentials")?;

        let decoded_credentials =
            String::from_utf8(decoded_bytes).context("Credentials are not valid UTF-8")?;

        let (username, password) = decoded_credentials
            .split_once(':')
            .context("Invalid credentials format - expected 'username:password'")?;

        Credentials::from(username, password)
    }

    type Error = anyhow::Error;
}

impl Credentials {
    pub async fn validate(&self, connection: &PgPool) -> Result<bool> {
        let res = query!(
            r#"
            SELECT username, password_hash
            FROM users
            WHERE username = $1
            "#,
            self.username
        )
        .fetch_one(connection)
        .await
        .context("Failed to fetch user from database")?;

        let password_hash = PasswordHash::new(&res.password_hash)
            .map_err(|e| anyhow::anyhow!("Failed to parse password hash from database: {:?}", e))?;

        let argon2 = Argon2::default();
        let is_valid = argon2
            .verify_password(self.password.expose_secret().as_bytes(), &password_hash)
            .is_ok();

        Ok(is_valid)
    }
}
