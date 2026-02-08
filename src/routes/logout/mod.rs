use actix_session::Session;
use actix_web::HttpResponse;

pub async fn logout(session: Session) -> HttpResponse {
    session.purge();
    HttpResponse::Ok().body(include_str!("logout.html"))
}
