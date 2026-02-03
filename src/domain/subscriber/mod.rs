mod email;
mod name;
pub use email::SubscriberEmail;
pub use name::SubscriberName;

#[derive(serde::Deserialize, Debug)]
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
        self.email.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
