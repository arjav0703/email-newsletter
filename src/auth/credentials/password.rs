use anyhow::{Context, Result};
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use secrecy::{ExposeSecret, Secret};
use sqlx::{PgPool, query};

use super::Credentials;

impl Credentials {
    #[tracing::instrument(name = "Changing user password", skip(self, connection, new_password))]
    pub async fn change_password(
        &self,
        new_password: Secret<String>,
        connection: &PgPool,
    ) -> Result<()> {
        let new_password_hash = Self::generate_password_hash(new_password).await?;
        query!(
            "UPDATE users SET password_hash = $1 WHERE username = $2",
            new_password_hash,
            self.username
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    pub async fn generate_password_hash(password: Secret<String>) -> Result<String> {
        let password_hash = tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
            let argon2 = Argon2::default();

            argon2
                .hash_password(password.expose_secret().as_bytes(), &salt)
                .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))
                .map(|hash| hash.to_string())
        })
        .await
        .context("Password hashing task panicked")?;
        password_hash.context("Failed to hash password")
    }
}
