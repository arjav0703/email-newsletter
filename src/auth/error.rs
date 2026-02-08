use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header},
};

#[derive(Debug)]
pub struct AuthError(anyhow::Error);

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AuthError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        if self
            .0
            .to_string()
            .contains("Failed to validate credentials")
        {
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

impl From<anyhow::Error> for AuthError {
    fn from(e: anyhow::Error) -> Self {
        AuthError(e)
    }
}
impl From<sqlx::Error> for AuthError {
    fn from(e: sqlx::Error) -> Self {
        AuthError(anyhow::anyhow!(e))
    }
}
impl From<resend_rs::Error> for AuthError {
    fn from(e: resend_rs::Error) -> Self {
        AuthError(anyhow::anyhow!(e))
    }
}
impl From<String> for AuthError {
    fn from(e: String) -> Self {
        AuthError(anyhow::anyhow!(e))
    }
}
