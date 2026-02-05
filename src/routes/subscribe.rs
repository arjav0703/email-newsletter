use crate::{domain::Subscriber, email_client::EmailClient};
use actix_web::{HttpResponse, web};
use anyhow::Result;
use chrono::Utc;
use sqlx::{Executor, PgPool, Transaction};
use tracing::{error, info};
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, connection, email_client),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> HttpResponse {
    let subscription_token = generate_subscription_token();

    let subscriber = match Subscriber::create(form.name.clone(), form.email.clone()) {
        Ok(subscriber) => subscriber,
        Err(e) => {
            error!("Invalid Subscriber Details: {}", e);
            return HttpResponse::BadRequest().finish();
        }
    };

    let mut transaction = match connection.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            error!("Failed to begin transaction: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if insert_subscriber(&subscriber, &mut transaction)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    };

    if store_token(&mut transaction, *subscriber.id(), &subscription_token)
        .await
        .is_err()
    {
        error!(
            "Failed to store subscription token for subscriber: {:?}",
            subscriber
        );
        return HttpResponse::InternalServerError().finish();
    }

    if email_client
        .send_confirmation_email(&subscriber, &subscription_token)
        .await
        .is_err()
    {
        if transaction.rollback().await.is_err() {
            error!(
                "Failed to rollback transaction for subscriber: {:?}",
                subscriber
            );
        }
        return HttpResponse::InternalServerError().finish();
    }

    if transaction.commit().await.is_err() {
        error!(
            "Failed to commit transaction for subscriber: {:?}",
            subscriber
        );
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(subscriber, transaction)
)]
async fn insert_subscriber(
    subscriber: &Subscriber,
    transaction: &mut Transaction<'_, sqlx::Postgres>,
) -> Result<(), sqlx::Error> {
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

    transaction.execute(query).await?;

    info!(
        "New subscriber details saved successfully: {:?}",
        subscriber
    );
    Ok(())
}

#[tracing::instrument(
    name = "Store subscription token",
    skip(subscription_token, transaction)
)]
pub async fn store_token(
    transaction: &mut Transaction<'_, sqlx::Postgres>,
    subscriber_id: Uuid,
    subscription_token: &str,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscription_token, subscriber_id)
        VALUES ($1, $2)"#,
        subscription_token,
        subscriber_id
    );
    transaction.execute(query).await?;
    Ok(())
}

use rand::{Rng, distr::Alphanumeric, rng};
pub fn generate_subscription_token() -> String {
    let mut rng = rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}
