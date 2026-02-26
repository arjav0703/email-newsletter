use actix_web::{HttpResponse, error::InternalError, http::StatusCode, web};
use anyhow::Context;
use secrecy::{ExposeSecret, Secret};

use crate::{domain::SubscriberEmail, email_client::EmailClient};

#[derive(serde::Deserialize, Debug)]
pub struct Formdata {
    email: String,
}

pub async fn unsubscribe_post(
    formdata: web::Form<Formdata>,
    email_client: web::Data<EmailClient>,
    connection: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let email = SubscriberEmail::parse(formdata.email.clone())
        .map_err(|_| InternalError::new("Invalid email address", StatusCode::BAD_REQUEST))?;

    let token = Secret::new(uuid::Uuid::new_v4().to_string());
    store_token(&connection, &email, &token)
        .await
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    email_client
        .send_unsubscribe_email(email, token)
        .await
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(HttpResponse::Ok().finish())
}

async fn store_token(
    connection: &sqlx::PgPool,
    email: &SubscriberEmail,
    token: &Secret<String>,
) -> anyhow::Result<()> {
    let subscriber_id = sqlx::query!(
        r#"
        SELECT id FROM subscriptions WHERE email = $1
        "#,
        email.as_ref()
    )
    .fetch_one(connection)
    .await
    .context("Failed to fetch subscriber ID")?
    .id;

    sqlx::query!(
        r#"
        INSERT INTO unsubscribe_tokens (subscriber_id, unsubscribe_token)
        VALUES ($1, $2)
        "#,
        subscriber_id,
        token.expose_secret()
    )
    .execute(connection)
    .await
    .context("Failed to store subscription token")?;
    Ok(())
}

impl EmailClient {
    #[tracing::instrument(name = "Sending unsubscribe email", skip(self, token, recipient))]
    pub async fn send_unsubscribe_email(
        &self,
        recipient: SubscriberEmail,
        token: Secret<String>,
    ) -> Result<(), resend_rs::Error> {
        let unsubscribe_link = format!(
            "{}/unsubscribe?subscription_token={}",
            self.base_url(),
            token.expose_secret()
        );

        let subject = "Unsubscribe Confirmation";
        let html_content = format!(
            "<p>Click the link below to unsubscribe/p><p><a href=\"{}\">{}</a></p>",
            unsubscribe_link, unsubscribe_link
        );

        self.send_email(recipient, subject, &html_content, None)
            .await
    }
}
