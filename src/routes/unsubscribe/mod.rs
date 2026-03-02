use actix_web::HttpResponse;
use actix_web::web;
pub mod post;
use anyhow::Context;
use anyhow::Result;
pub use post::unsubscribe_post;
use sqlx::PgPool;

#[derive(serde::Deserialize, Debug)]
pub struct QueryData {
    unsubscribe_token: String,
}

pub async fn unsubscribe_form() -> HttpResponse {
    let html_content = include_str!("unsubscribe.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content)
}

pub async fn unsubscribe_get(
    query_parameters: web::Query<QueryData>,
    connection: web::Data<PgPool>,
) -> HttpResponse {
    match unsubscribe_user(&connection, &query_parameters.unsubscribe_token).await {
        Ok(_) => {
            let html_content = include_str!("unsubscribe_success.html");
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html_content)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Removing user from the database", skip(connection, token))]
async fn unsubscribe_user(connection: &PgPool, token: &str) -> Result<()> {
    let subscriber_id = sqlx::query!(
        r#"
        SELECT subscriber_id FROM unsubscribe_tokens
        WHERE unsubscribe_token = $1
        "#,
        token
    )
    .fetch_one(connection)
    .await
    .context("Failed to find a subscriber for the provided token")?
    .subscriber_id;

    sqlx::query!(
        r#"
            DELETE FROM unsubscribe_tokens
            WHERE subscriber_id = $1
            "#,
        subscriber_id
    )
    .execute(connection)
    .await
    .context("Failed to delete token from database")?;

    sqlx::query!(
        r#"
            DELETE FROM subscriptions
            WHERE id = $1
            "#,
        subscriber_id
    )
    .execute(connection)
    .await
    .context("Failed to delete user from subscription database. User might not exist")?;

    Ok(())
}
