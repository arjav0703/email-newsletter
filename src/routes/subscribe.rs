use crate::{domain::Subscriber, email_client::EmailClient};
use actix_web::{HttpResponse, web};
use anyhow::Result;
use chrono::Utc;
use sqlx::PgPool;
use tracing::{error, info, instrument::Instrument};
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

    if send_confirmation_email(&subscriber, &email_client, &subscription_token)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    if store_token(connection.clone(), *subscriber.id(), &subscription_token)
        .await
        .is_err()
    {
        error!(
            "Failed to store subscription token for subscriber: {:?}",
            subscriber
        );
        return HttpResponse::InternalServerError().finish();
    }

    match insert_subscriber(&subscriber, connection).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(subscriber, connection)
)]
async fn insert_subscriber(
    subscriber: &Subscriber,
    connection: web::Data<PgPool>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        Insert into subscriptions (id, email, name, subscribed_at, status)
        values ($1, $2, $3, $4, 'pending_confirmation')
        "#,
        subscriber.id(),
        subscriber.email(),
        subscriber.name(),
        Utc::now()
    )
    .execute(connection.as_ref())
    .instrument(tracing::info_span!("Inserting new subscriber"))
    .await?;

    info!(
        "New subscriber details saved successfully: {:?}",
        subscriber
    );
    Ok(())
}

#[tracing::instrument(
    name = "Sending a confirmation email to new subscriber",
    skip(subscriber, email_client, subscription_token)
)]
async fn send_confirmation_email(
    subscriber: &Subscriber,
    email_client: &EmailClient,
    subscription_token: &str,
) -> Result<()> {
    let confirmation_link = format!(
        "http://{}/subscriptions/confirm?subscription_token={}",
        email_client.base_url(),
        subscription_token
    );

    let html_content = format!(
        "Welcome to our newsletter!<br />\
        Click <a href=\"{}\">here</a> to confirm your subscription.",
        confirmation_link
    );

    email_client
        .send_email(
            subscriber.email.to_owned(),
            "Please confirm your subscription",
            &html_content,
        )
        .await?;

    info!(
        "Confirmation email sent to subscriber: {:?}",
        subscriber.email()
    );
    Ok(())
}

#[tracing::instrument(
    name = "Store subscription token",
    skip(subscription_token, connection)
)]
pub async fn store_token(
    connection: web::Data<PgPool>,
    subscriber_id: Uuid,
    subscription_token: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscription_token, subscriber_id)
        VALUES ($1, $2)"#,
        subscription_token,
        subscriber_id
    )
    .execute(connection.as_ref())
    .await
    .map_err(|e| {
        tracing::error!("Query failed: {:?}", e);
        e
    })?;
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
