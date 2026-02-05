use actix_web::{ResponseError, http::StatusCode};

#[derive(Debug)]
pub struct PublishError(anyhow::Error);

impl std::fmt::Display for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for PublishError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl From<anyhow::Error> for PublishError {
    fn from(e: anyhow::Error) -> Self {
        PublishError(e)
    }
}
impl From<sqlx::Error> for PublishError {
    fn from(e: sqlx::Error) -> Self {
        PublishError(anyhow::anyhow!(e))
    }
}
impl From<resend_rs::Error> for PublishError {
    fn from(e: resend_rs::Error) -> Self {
        PublishError(anyhow::anyhow!(e))
    }
}
impl From<String> for PublishError {
    fn from(e: String) -> Self {
        PublishError(anyhow::anyhow!(e))
    }
}
