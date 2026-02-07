pub async fn subscribe_get() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().body(include_str!("./subscribe.html"))
}
