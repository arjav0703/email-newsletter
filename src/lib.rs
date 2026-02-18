use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::{
    App, HttpServer,
    cookie::Key,
    dev::Server,
    web::{self, Data},
};
use anyhow::Result;
use secrecy::{ExposeSecret, Secret};
use tracing::info;
use tracing_actix_web::TracingLogger;
pub mod auth;
pub mod config;
pub mod domain;
pub mod email_client;
pub mod telemetry;
use email_client::EmailClient;

mod routes {
    pub mod admin;
    pub mod confirm_subscription;
    pub mod home;
    pub mod login;
    pub mod logout;
    pub mod newsletter;
    pub mod status;
    pub mod subscribe;
    pub mod unsubscribe;
}
use routes::{
    admin, confirm_subscription::confirm_subsciption, home::home, login, logout::logout,
    newsletter::publish_newsletter, status::status, subscribe::subscribe_get,
    subscribe::subscribe_post, unsubscribe::unsubscribe_get,
};
use sqlx::PgPool;

pub async fn run(
    address: &str,
    connection: PgPool,
    email_client: EmailClient,
    redis_uri: Secret<String>,
) -> Result<Server> {
    info!("Starting server at http://{}", address);

    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;
    let secret_key = Key::generate();

    let connection = Data::new(connection);
    let email_client = Data::new(email_client);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(connection.clone())
            .app_data(email_client.clone())
            .wrap(TracingLogger::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .route("/", web::get().to(home))
            .route("/status", web::get().to(status))
            .route("/subscribe", web::get().to(subscribe_get))
            .route("/subscribe", web::post().to(subscribe_post))
            .route("/subscriptions/confirm", web::get().to(confirm_subsciption))
            .route("/newsletter", web::post().to(publish_newsletter))
            .route("/login", web::post().to(login::login_post))
            .route("/login", web::get().to(login::login_get))
            .route("/logout", web::get().to(logout))
            .route("/unsubscribe", web::get().to(unsubscribe_get))
            .route("/admin/dashboard", web::get().to(admin::dashboard))
            .route("/admin/password", web::get().to(admin::password_get))
            .route("/admin/newsletter", web::get().to(admin::newsletter_get))
            .route("/admin/password", web::post().to(admin::password_post))
    })
    .bind(address)?
    .run();

    Ok(server)
}
