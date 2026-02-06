use actix_web::{
    App, HttpRequest, HttpServer, Responder,
    dev::Server,
    web::{self, Data},
};
use anyhow::Result;
use tracing::info;
use tracing_actix_web::TracingLogger;
pub mod config;
pub mod domain;
pub mod email_client;
pub mod telemetry;
use email_client::EmailClient;

mod routes {
    pub mod confirm_subscription;
    pub mod home;
    pub mod newsletter;
    pub mod status;
    pub mod subscribe;
}
use routes::{
    confirm_subscription::confirm_subsciption, home::home, newsletter::publish_newsletter,
    status::status, subscribe::subscribe,
};
use sqlx::PgPool;

pub fn run(address: &str, connection: PgPool, email_client: EmailClient) -> Result<Server> {
    info!("Starting server at http://{}", address);

    let connection = Data::new(connection);
    let email_client = Data::new(email_client);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(connection.clone())
            .app_data(email_client.clone())
            .wrap(TracingLogger::default())
            .route("/", web::get().to(home))
            .route("/status", web::get().to(status))
            .route("/subscribe", web::post().to(subscribe))
            .route("/subscriptions/confirm", web::get().to(confirm_subsciption))
            .route("/newsletter", web::post().to(publish_newsletter))
    })
    .bind(address)?
    .run();

    Ok(server)
}
