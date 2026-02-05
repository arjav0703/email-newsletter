use anyhow::{Context, Result};
use sqlx::{Executor, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "Store subscription token",
    skip(subscription_token, transaction)
)]
pub async fn store_token(
    transaction: &mut Transaction<'_, sqlx::Postgres>,
    subscriber_id: Uuid,
    subscription_token: &str,
) -> Result<()> {
    let query = sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscription_token, subscriber_id)
        VALUES ($1, $2)"#,
        subscription_token,
        subscriber_id
    );
    transaction
        .execute(query)
        .await
        .context("Failed to store subscription token")?;
    Ok(())
}
