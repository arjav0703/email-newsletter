use actix_web::HttpResponse;
pub mod post;
pub use post::unsubscribe_post;

pub async fn unsubscribe_get() -> HttpResponse {
    let html_content = include_str!("unsubscribe.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content)
}
