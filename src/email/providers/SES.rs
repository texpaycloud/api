use crate::email::Email;
use crate::email::EmailService;
use anyhow::{Error, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::Client;
use tracing::{error, info};

pub struct SESProvider {
    client: Client,
}

impl SESProvider {
    pub async fn new() -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);

        Self { client }
    }
}

#[async_trait::async_trait]
impl EmailService for SESProvider {
    /// Sends an email using Amazon Simple Email Service (SES).
    ///
    /// # Arguments
    ///
    /// * `email` - An `Email` object
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the email is sent successfully.
    /// * `Err(err)` if there is an error sending the email.
    ///
    /// # Errors
    ///
    /// This function will return an error if the SES client fails to send the email.
    async fn send_one(&self, email: Email) -> Result<(), Error> {
        info!("Sending email via SES: {:?}", email);

        let destination = Destination::builder().to_addresses(email.to).build();
        let subject = Content::builder()
            .data(email.subject)
            .charset("UTF-8")
            .build();
        let body_content = Content::builder().data(email.body).charset("UTF-8").build().expect("Failed to build body content");
        let body = Body::builder().text(body_content).build();

        let msg = Message::builder().subject(subject.expect("Failed to build subject")).body(body).build();

        let email_content = EmailContent::builder().simple(msg).build();

        let res = self
            .client
            .send_email()
            .from_email_address(email.from)
            .destination(destination)
            .content(email_content)
            .send()
            .await;

        if let Err(err) = res {
            error!("Failed to send email via SES: {:?}", err);
            return Err(err.into());
        }

        Ok(())
    }
}
