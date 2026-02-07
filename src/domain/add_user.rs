use crate::auth::Credentials;
use anyhow::{Context, Result};
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use secrecy::ExposeSecret;
use sqlx::PgPool;

impl Credentials {
    #[tracing::instrument(
        name = "Adding new user to database [users table]",
        skip(self, connection)
    )]
    pub async fn add_user_to_db(&self, connection: &PgPool) -> Result<()> {
        let password = self.password.clone();

        let password_hash = tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
            let argon2 = Argon2::default();

            argon2
                .hash_password(password.expose_secret().as_bytes(), &salt)
                .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))
                .map(|hash| hash.to_string())
        })
        .await
        .context("Password hashing task panicked")?
        .context("Failed to hash password")?;

        sqlx::query!(
            r#"
            INSERT INTO users (user_id, username, password_hash)
            VALUES (gen_random_uuid(), $1, $2)
            "#,
            self.username,
            password_hash
        )
        .execute(connection)
        .await
        .context("Failed to insert new user into the database")?;

        Ok(())
    }
}

pub async fn create_test_user(connection: &PgPool) -> Result<()> {
    tracing::warn!("Adding test user to the database");

    let test_credentials = Credentials::from("testuser", "password123")?;
    test_credentials
        .add_user_to_db(connection)
        .await
        .unwrap_or_default();

    Ok(())
}
