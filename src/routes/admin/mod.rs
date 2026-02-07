use actix_session::Session;
use actix_web::{HttpResponse, http::header::LOCATION};

#[tracing::instrument(name = "Admin Dashboard", skip(session))]
pub async fn dashboard(session: Session) -> HttpResponse {
    let username: Option<String> = session.get("username").unwrap_or(None);

    if username.is_none() {
        return HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login?error=unauthorized"))
            .finish();
    }

    HttpResponse::Ok().body("Welcome to the admin dashboard!")
}
