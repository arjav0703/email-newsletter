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

impl FormData {
    pub fn validate(&self) -> bool {
        !self.name.is_empty() && self.validate_email()
    }

    fn validate_email(&self) -> bool {
        self.email.contains('@') && self.email.contains('.')
    }
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
    match insert_user(form, connection).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, connection)
)]
async fn insert_user(
    form: web::Form<FormData>,
    connection: web::Data<PgPool>,
) -> Result<(), sqlx::Error> {
    if !form.validate() {
        error!("Invalid form data: {:?}", form);
        return Err(sqlx::Error::Protocol("Invalid form data".into()));
    }

    sqlx::query!(
        r#"
        Insert into subscriptions (id, email, name, subscribed_at)
        values ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.as_ref())
    .instrument(tracing::info_span!("Inserting new subscriber"))
    .await?;

    info!("New subscriber details saved successfully: {:?}", form);
    Ok(())
}
