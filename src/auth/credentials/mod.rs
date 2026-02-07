use anyhow::Result;
use secrecy::Secret;
mod password;
mod try_from;
mod validate;

pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}

impl Credentials {
    pub fn from(username: &str, password: &str) -> Result<Self> {
        if username.is_empty() || password.is_empty() {
            anyhow::bail!("Username and password must not be empty");
        }
        Ok(Credentials {
            username: username.to_string(),
            password: Secret::new(password.to_string()),
        })
    }
}
