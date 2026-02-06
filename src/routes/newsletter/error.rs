use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header},
};

#[derive(Debug)]
pub struct PublishError(anyhow::Error);

impl std::fmt::Display for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for PublishError {
    fn status_code(&self) -> StatusCode {
        if self.0.to_string().contains("Failed to extract credentials") {
            return StatusCode::BAD_REQUEST;
        }

        if self.0.to_string().contains("Unauthorized") {
            let mut h = HttpResponse::new(StatusCode::UNAUTHORIZED);
            h.headers_mut().insert(
                header::WWW_AUTHENTICATE,
                header::HeaderValue::from_static("Basic realm=\"Restricted Area\""),
            );

            return StatusCode::UNAUTHORIZED;
        }

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
