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

pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    let _enter = request_span.enter();

    info!(
        "id -> {} Received subscription request: {} <{}>",
        request_id, form.name, form.email
    );

    if !form.validate() {
        error!("id -> {} Invalid form data: {:?}", request_id, form);
        return HttpResponse::BadRequest().finish();
    }

    let query_span = tracing::info_span!("Saving new subscriber in the database.");
    let res = sqlx::query!(
        r#"
        Insert into subscriptions (id, email, name, subscribed_at)
        values ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .instrument(query_span)
    .await;

    match res {
        Ok(_) => {
            info!("id -> {} Added user successfully", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            error!("id -> {} Failed to add user: {}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
