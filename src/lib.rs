use actix_web::{App, HttpRequest, HttpServer, Responder, dev::Server, middleware::Logger, web};
use anyhow::Result;
use log::info;
pub mod config;
pub mod telemetry;

mod routes {
    pub mod status;
    pub mod subscribe;
}
use routes::{status::status, subscribe::subscribe};
use sqlx::PgPool;

pub fn run(address: &str, connection: PgPool) -> Result<Server> {
    info!("Starting server at http://{}", address);

    let connection = web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(connection.clone())
            .wrap(Logger::default())
            .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
            .route("/status", web::get().to(status))
            .route("/subscribe", web::post().to(subscribe))
    })
    .bind(address)?
    .run();

    Ok(server)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
