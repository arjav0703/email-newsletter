// use crate::{domain::Subscriber, email_client::EmailClient};
use actix_web::{HttpResponse, web};
// use anyhow::Result;
// use chrono::Utc;
// use sqlx::PgPool;
// use tracing::{error, info, instrument::Instrument};
// use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct QueryData {
    subscription_token: String,
}

#[tracing::instrument(
    name = "Confirming pending subscription (/confirm)",
    skip(formdata),
    // fields(
    //     subscriber_email = %form.email,
    //     subscriber_name = %form.name
    // )
)]
pub async fn confirm_subsciption(formdata: web::Query<QueryData>) -> HttpResponse {
    if formdata.subscription_token.is_empty() {
        return HttpResponse::BadRequest().finish();
    }
    HttpResponse::Ok().finish()
}
