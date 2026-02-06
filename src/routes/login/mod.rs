use actix_web::HttpResponse;

mod post;
pub use post::login_post;

pub async fn login_get() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("login.html"))
}
