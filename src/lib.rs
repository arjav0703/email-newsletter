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
    pub mod status;
    pub mod subscribe;
}
use routes::{confirm_subscription::confirm_subsciption, status::status, subscribe::subscribe};
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
            .route("/", web::get().to(greet))
            .route("/status", web::get().to(status))
            .route("/subscribe", web::post().to(subscribe))
            .route("/subscriptions/confirm", web::get().to(confirm_subsciption))
    })
    .bind(address)?
    .run();

    Ok(server)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
