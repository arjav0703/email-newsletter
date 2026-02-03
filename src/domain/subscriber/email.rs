use validator::validate_email;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(email: String) -> Result<Self, String> {
        if validate_email(&email) {
            Ok(SubscriberEmail(email))
        } else {
            Err("Subscriber email must contain '@'".into())
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use claims::assert_err;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
}
