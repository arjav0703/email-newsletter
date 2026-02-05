use crate::email_client::EmailClient;
use actix_web::{HttpResponse, web};
use anyhow::Result;
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct QueryData {
    subscription_token: String,
}

#[tracing::instrument(
    name = "Confirming pending subscription (/confirm)",
    skip(query_parameters)
)]
pub async fn confirm_subsciption(
    query_parameters: web::Query<QueryData>,
    connection: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> HttpResponse {
    let token = query_parameters.subscription_token.clone();

    if token.is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    let id = match get_subscriber_id_from_token(&connection, &token).await {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::InternalServerError().finish();
        }
    };

    match id {
        Some(id) => {
            if set_id_confirmed(&connection, id).await.is_err() {
                return HttpResponse::InternalServerError().finish();
            } else {
                return HttpResponse::Ok().finish();
            }
        }
        None => {
            error!("No subscriber found for the provided token");
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[tracing::instrument(
    name = "Fetching subscription token details from database",
    skip(connection, subscription_token)
)]
async fn get_subscriber_id_from_token(
    connection: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>> {
    let result = sqlx::query!(
        "SELECT subscriber_id FROM subscription_tokens \
      WHERE subscription_token = $1",
        subscription_token,
    )
    .fetch_optional(connection)
    .await
    .map_err(|e| {
        tracing::error!("Query Failed: {:?}", e);
        e
    })?;

    Ok(result.map(|r| r.subscriber_id))
}

#[tracing::instrument(name = "Marking subscriber as confirmed", skip(connection, id))]
async fn set_id_confirmed(connection: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query!(
        "UPDATE subscriptions \
      SET status = 'confirmed' \
      WHERE id = $1",
        id,
    )
    .execute(connection)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update subscription status: {:?}", e);
        e
    })?;

    Ok(())
}
