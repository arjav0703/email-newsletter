use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, web};
use anyhow::Result;

pub fn run(address: &str) -> Result<Server> {
    println!("Starting server at http://{}", address);

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
            .route("/status", web::get().to(status_report))
    })
    .bind(address)?
    .run();

    Ok(server)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn status_report() -> HttpResponse {
    HttpResponse::Ok().finish()
}
