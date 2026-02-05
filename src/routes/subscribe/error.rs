use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub struct SubscribeError(anyhow::Error);

impl std::fmt::Display for SubscribeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for SubscribeError {
    fn status_code(&self) -> StatusCode {
        if self.0.chain().any(|e| {
            let s = e.to_string();
            s.contains("validation") || s.contains("invalid") || s.contains("Validation")
        }) {
            return StatusCode::BAD_REQUEST;
        }

        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
}

impl From<anyhow::Error> for SubscribeError {
    fn from(err: anyhow::Error) -> Self {
        SubscribeError(err)
    }
}

impl From<sqlx::Error> for SubscribeError {
    fn from(err: sqlx::Error) -> Self {
        SubscribeError(anyhow::Error::from(err).context("Database error"))
    }
}

impl From<resend_rs::Error> for SubscribeError {
    fn from(err: resend_rs::Error) -> Self {
        SubscribeError(anyhow::Error::from(err).context("Email sending error"))
    }
}

impl From<String> for SubscribeError {
    fn from(err: String) -> Self {
        SubscribeError(anyhow::anyhow!("Validation error: {}", err))
    }
}
