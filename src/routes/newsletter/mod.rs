use actix_web::{HttpResponse, web};
use anyhow::{Context, Result};
use sqlx::{PgPool, query};

use crate::{domain::SubscriberEmail, email_client, routes::newsletter::error::PublishError};
mod error;

#[derive(serde::Deserialize, Debug)]
pub struct NewsLetterData {
    pub title: String,
    pub content: String,
    pub html_content: String,
}

struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

#[tracing::instrument(
    name = "Publishing newsletter",
    skip(newsletter, connection, email_client)
)]
pub async fn publish_newsletter(
    newsletter: web::Json<NewsLetterData>,
    connection: web::Data<PgPool>,
    email_client: web::Data<email_client::EmailClient>,
) -> Result<HttpResponse, PublishError> {
    let subscribers = get_emails_from_database(connection.get_ref()).await?;

    for subscrber in subscribers {
        email_client
            .send_email(
                subscrber.email,
                &newsletter.title,
                &newsletter.html_content,
                Some(&newsletter.content),
            )
            .await
            .context("Failed to send newsletter email")?;
    }

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Fetch confirmed emails from the database", skip(connection))]
async fn get_emails_from_database(connection: &PgPool) -> Result<Vec<ConfirmedSubscriber>> {
    let res = query!(
        r#"
        SELECT email
        FROM subscriptions
        WHERE status = 'confirmed'
        "#
    )
    .fetch_all(connection)
    .await
    .context("Failed to fetch confirmed emails from the database")?
    .into_iter()
    .map(|record| match SubscriberEmail::parse(record.email) {
        Ok(email) => Ok(ConfirmedSubscriber { email }),
        Err(e) => Err(anyhow::anyhow!(e)).context("Failed to parse email from the database record"),
    })
    .collect::<Result<Vec<ConfirmedSubscriber>>>()?;
    Ok(res)
}
