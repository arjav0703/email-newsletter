use actix_session::Session;
use actix_web::{HttpResponse, http::header::LOCATION};

mod password;
pub use password::{password_get, password_post};

mod newsletter;
pub use newsletter::newsletter_get;

#[tracing::instrument(name = "Admin Dashboard", skip(session))]
pub async fn dashboard(session: Session) -> HttpResponse {
    if let Some(response) = SessionWrapper::new(session).redirect_to_login_if_not_signed_in() {
        return response;
    }

    HttpResponse::Ok().body(include_str!("./admin.html"))
}

struct SessionWrapper(Session);

impl SessionWrapper {
    pub fn new(session: Session) -> Self {
        Self(session)
    }

    pub fn redirect_to_login_if_not_signed_in(&self) -> Option<HttpResponse> {
        let username: Option<String> = self.0.get("username").unwrap_or(None);

        if username.is_none() {
            Some(
                HttpResponse::SeeOther()
                    .insert_header((LOCATION, "/login?error=unauthorized"))
                    .finish(),
            )
        } else {
            None
        }
    }
}
