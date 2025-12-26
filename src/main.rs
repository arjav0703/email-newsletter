use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/status", web::get().to(status_report))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn status_report() -> impl Responder {
    HttpResponse::Ok()
}
