use crate::domain::{Subscriber, SubscriberEmail};
use secrecy::{ExposeSecret, Secret};

use resend_rs::Resend;
use resend_rs::types::CreateEmailBaseOptions;
use tracing::info;

pub struct EmailClient {
    base_url: String,
    sender: SubscriberEmail,
    resend_api_key: Secret<String>,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail, resend_api_key: Secret<String>) -> Self {
        Self {
            base_url,
            sender,
            resend_api_key,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: Option<&str>,
    ) -> Result<(), resend_rs::Error> {
        let resend = Resend::new(self.resend_api_key.expose_secret());

        let from = self.sender.as_ref();
        let to = [recipient.as_ref()];

        let email = CreateEmailBaseOptions::new(from, to, subject)
            .with_html(html_content)
            .with_text(text_content.unwrap_or_default());

        let _email = resend.emails.send(email).await?;
        info!("{:?}", _email);

        Ok(())
    }

    pub async fn send_test_email(
        &self,
        recipient: SubscriberEmail,
    ) -> Result<(), resend_rs::Error> {
        let subject = "Test Email from EmailClient";
        let html_content = "<h1>This is a test email sent from EmailClient</h1>";

        self.send_email(recipient, subject, html_content, None)
            .await
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    #[tracing::instrument(
        name = "Sending a confirmation email to new subscriber",
        skip(subscriber, self, subscription_token)
    )]
    pub async fn send_confirmation_email(
        &self,
        subscriber: &Subscriber,
        subscription_token: &str,
    ) -> Result<(), resend_rs::Error> {
        let confirmation_link = format!(
            "http://{}/subscriptions/confirm?subscription_token={}",
            self.base_url(),
            subscription_token
        );

        let html_content = format!(
            "Welcome to our newsletter!<br />\
        Click <a href=\"{}\">here</a> to confirm your subscription. <br /> If you did not subscribe to our newsletter, please ignore this email.",
            confirmation_link
        );

        self.send_email(
            subscriber.email.to_owned(),
            "Please confirm your subscription",
            &html_content,
            None,
        )
        .await?;

        info!(
            "Confirmation email sent to subscriber: {:?}",
            subscriber.email()
        );
        Ok(())
    }
}
