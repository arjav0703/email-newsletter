use crate::domain::subscriber::Subscriber;
use actix_web::{HttpResponse, web};
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
    skip(form, connection),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> HttpResponse {
    let subscriber = match Subscriber::create(form.name.clone(), form.email.clone()) {
        Ok(subscriber) => subscriber,
        Err(e) => {
            error!("Invalid Subscriber Details: {}", e);
            return HttpResponse::BadRequest().finish();
        }
    };

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
        Insert into subscriptions (id, email, name, subscribed_at)
        values ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
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
