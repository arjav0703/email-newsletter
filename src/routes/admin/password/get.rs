use actix_session::Session;

use crate::routes::admin::SessionWrapper;

pub async fn password_get(session: Session) -> actix_web::HttpResponse {
    if let Some(response) = SessionWrapper::new(session).redirect_to_login_if_not_signed_in() {
        return response;
    }

    actix_web::HttpResponse::Ok().body(include_str!("password.html"))
}
