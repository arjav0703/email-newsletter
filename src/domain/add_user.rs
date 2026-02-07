use crate::auth::Credentials;
use anyhow::{Context, Result};
use sqlx::PgPool;

impl Credentials {
    #[tracing::instrument(
        name = "Adding new user to database [users table]",
        skip(self, connection)
    )]
    pub async fn add_user_to_db(&self, connection: &PgPool) -> Result<()> {
        let password = self.password.clone();

        let password_hash = Credentials::generate_password_hash(password).await?;
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
