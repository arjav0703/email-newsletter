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
    #[tracing::instrument(name = "Extracting credentials from headers", skip(headers))]
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
    #[tracing::instrument(name = "Validating credentials", skip(self, connection))]
    pub async fn validate(&self, connection: &PgPool) -> Result<bool> {
        let password_hash_str = self
            .fetch_password_hash_from_database(connection)
            .await
            .context("Failed to fetch password hash from database")?;

        let password = self.password.clone();

        let is_valid = tokio::task::spawn_blocking(move || {
            let password_hash =
                PasswordHash::new(password_hash_str.expose_secret()).map_err(|e| {
                    anyhow::anyhow!("Failed to parse password hash from database: {:?}", e)
                })?;

            let is_valid = Argon2::default()
                .verify_password(password.expose_secret().as_bytes(), &password_hash)
                .is_ok();

            Ok::<bool, anyhow::Error>(is_valid)
        })
        .await
        .context("Password verification task panicked")?
        .context("Password verification failed")?;

        Ok(is_valid)
    }

    #[tracing::instrument(name = "Fetching password hash from database", skip(self, connection))]
    async fn fetch_password_hash_from_database(
        &self,
        connection: &PgPool,
    ) -> Result<Secret<String>> {
        let res = query!(
            r#"
            SELECT password_hash
            FROM users
            WHERE username = $1
            "#,
            self.username
        )
        .fetch_one(connection)
        .await
        .context("Failed to fetch user credentials from the database")?;

        Ok(res.password_hash.into())
    }
}
