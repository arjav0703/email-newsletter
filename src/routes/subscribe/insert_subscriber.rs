use crate::domain::Subscriber;
use anyhow::{Context, Result};
use chrono::Utc;
use sqlx::{Executor, Transaction};
use tracing::info;

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(subscriber, transaction)
)]
pub async fn insert_subscriber(
    subscriber: &Subscriber,
    transaction: &mut Transaction<'_, sqlx::Postgres>,
) -> Result<()> {
    let query = sqlx::query!(
        r#"
        Insert into subscriptions (id, email, name, subscribed_at, status)
        values ($1, $2, $3, $4, 'pending_confirmation')
        "#,
        subscriber.id(),
        subscriber.email(),
        subscriber.name(),
        Utc::now()
    );

    transaction
        .execute(query)
        .await
        .context("Failed to insert subscriber details")?;

    info!(
        "New subscriber details saved successfully: {:?}",
        subscriber
    );
    Ok(())
}
