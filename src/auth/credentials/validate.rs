use super::Credentials;
use crate::auth::AuthError;
use anyhow::{Context, Result};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use secrecy::{ExposeSecret, Secret};
use sqlx::{PgPool, query};

impl Credentials {
    #[tracing::instrument(name = "Validating credentials", skip(self, connection))]
    pub async fn validate(&self, connection: &PgPool) -> Result<bool, AuthError> {
        let password_hash_str = match self.fetch_password_hash_from_database(connection).await {
            Ok(hash) => hash,
            Err(_) => {
                return Ok(false);
            }
        };

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
    ) -> Result<Secret<String>, AuthError> {
        let res = query!(
            r#"
            SELECT password_hash
            FROM users
            WHERE username = $1
            "#,
            self.username
        )
        .fetch_optional(connection)
        .await
        .context("Failed to fetch user credentials from the database")?;

        match res {
            Some(row) => Ok(row.password_hash.into()),
            None => Err(AuthError::from(anyhow::anyhow!("User not found"))),
        }
    }
}
