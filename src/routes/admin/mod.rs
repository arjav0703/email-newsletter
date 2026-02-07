use actix_session::Session;
use actix_web::{HttpResponse, http::header::LOCATION};

#[tracing::instrument(name = "Admin Dashboard", skip(session))]
pub async fn dashboard(session: Session) -> HttpResponse {
    let username: Option<String> = session.get("username").unwrap_or(None);

    if username.is_none() {
        return HttpResponse::Unauthorized()
            .insert_header((LOCATION, "/login"))
            .body("Unauthorized: Please log in to access the admin dashboard.");
    }

    HttpResponse::Ok().body("Welcome to the admin dashboard!")
}
