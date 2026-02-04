use crate::domain::SubscriberEmail;
use anyhow::Result;

use resend_rs::Resend;
use resend_rs::types::CreateEmailBaseOptions;
use tracing::info;

pub struct EmailClient {
    sender: SubscriberEmail,
    resend_api_key: String,
}

impl EmailClient {
    pub fn new(sender: SubscriberEmail, resend_api_key: String) -> Self {
        Self {
            sender,
            resend_api_key,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
    ) -> Result<()> {
        let resend = Resend::new(&self.resend_api_key);

        let from = self.sender.as_ref();
        let to = [recipient.as_ref()];

        let email = CreateEmailBaseOptions::new(from, to, subject).with_html(html_content);

        let _email = resend.emails.send(email).await?;
        info!("{:?}", _email);

        Ok(())
    }

    pub async fn send_test_email(&self, recipient: SubscriberEmail) -> Result<()> {
        let subject = "Test Email from EmailClient";
        let html_content = "<h1>This is a test email sent from EmailClient</h1>";

        self.send_email(recipient, subject, html_content).await
    }
}
