use crate::admin::SessionWrapper;
use actix_session::Session;
use actix_web::HttpResponse;

pub async fn newsletter_get(session: Session) -> HttpResponse {
    let username: String = session.get("username").unwrap_or(None).unwrap_or_default();

    if let Some(response) = SessionWrapper::new(session).redirect_to_login_if_not_signed_in() {
        return response;
    }

    let html = include_str!("./newsletter.html").replace("{{USERNAME}}", &username);

    HttpResponse::Ok().body(html)
}
