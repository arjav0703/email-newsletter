#[derive(Debug)]
pub struct SubscriberName(String);
#[derive(Debug)]
pub struct SubscriberEmail(String);

#[derive(Debug)]
pub struct Subscriber {
    pub name: SubscriberName,
    pub email: SubscriberEmail,
}

impl Subscriber {
    pub fn create(name: String, email: String) -> Result<Self, String> {
        let subscriber_name = SubscriberName::parse(name).ok_or("Invalid subscriber name")?;
        let subscriber_email = SubscriberEmail::parse(email)?;

        Ok(Subscriber {
            name: subscriber_name,
            email: subscriber_email,
        })
    }

    pub fn email(&self) -> &str {
        &self.email.0
    }

    pub fn name(&self) -> &str {
        &self.name.0
    }
}

impl SubscriberName {
    fn parse(name: String) -> Option<Self> {
        if Self::validate(&name) {
            Some(SubscriberName(name))
        } else {
            None
        }
    }

    fn validate(name: &str) -> bool {
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        for c in forbidden_characters {
            if name.contains(c) {
                return false;
            }
        }

        if name.len() > 256 {
            return false;
        }

        !name.trim().is_empty()
    }
}

impl SubscriberEmail {
    fn parse(email: String) -> Result<Self, String> {
        if email.contains('@') {
            Ok(SubscriberEmail(email))
        } else {
            Err("Subscriber email must contain '@'".into())
        }
    }
}
