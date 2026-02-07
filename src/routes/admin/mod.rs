use actix_web::HttpResponse;

pub async fn dashboard() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to the admin dashboard!")
}
