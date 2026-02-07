use crate::{domain::Subscriber, email_client::EmailClient};
use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use super::{
    error::SubscribeError, insert_subscriber::insert_subscriber, store_token::store_token,
};
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
pub async fn subscribe_post(
    form: web::Form<FormData>,
    connection: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> Result<HttpResponse, SubscribeError> {
    let subscription_token = generate_subscription_token();

    let subscriber = Subscriber::create(form.name.clone(), form.email.clone())?;

    let mut transaction = connection.begin().await?;

    insert_subscriber(&subscriber, &mut transaction).await?;

    store_token(&mut transaction, *subscriber.id(), &subscription_token).await?;

    email_client
        .send_confirmation_email(&subscriber, &subscription_token)
        .await?;

    transaction.commit().await?;

    Ok(HttpResponse::Ok().finish())
}

use rand::{Rng, distr::Alphanumeric, rng};
pub fn generate_subscription_token() -> String {
    let mut rng = rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}
