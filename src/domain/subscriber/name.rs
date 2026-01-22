#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(name: String) -> Option<Self> {
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

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
