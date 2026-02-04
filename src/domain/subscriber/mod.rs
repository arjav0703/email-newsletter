mod email;
mod name;
pub use email::SubscriberEmail;
pub use name::SubscriberName;
use uuid::Uuid;

#[derive(Debug)]
pub struct Subscriber {
    pub name: SubscriberName,
    pub email: SubscriberEmail,
    pub id: Uuid,
}

impl Subscriber {
    pub fn create(name: String, email: String) -> Result<Self, String> {
        let subscriber_name = SubscriberName::parse(name).ok_or("Invalid subscriber name")?;
        let subscriber_email = SubscriberEmail::parse(email)?;

        Ok(Subscriber {
            name: subscriber_name,
            email: subscriber_email,
            id: Uuid::new_v4(),
        })
    }

    pub fn email(&self) -> &str {
        self.email.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn id(&self) -> &Uuid {
        self.id.as_ref()
    }
}
