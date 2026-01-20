use actix_web::{HttpResponse, web};
use chrono::Utc;
use log::{error, info, warn};

use sqlx::PgPool;
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

    info!(
        "id -> {} Received subscription request: {} <{}>",
        request_id, form.name, form.email
    );

    if !form.validate() {
        warn!("id -> {} Invalid form data: {:?}", request_id, form);
        return HttpResponse::BadRequest().finish();
    }

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
