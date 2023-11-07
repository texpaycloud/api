pub mod providers;

use anyhow::{Error, Result};
use super::config::email::EmailConfig;
use super::config::email::EmailProvider;

#[derive(Debug)]
pub struct Email {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub body: String,
}

#[async_trait::async_trait]
pub trait EmailService {
    async fn send_one(&self, email: Email) -> Result<(), Error>;
}

pub struct EmailClient {
    pub provider: Box<dyn EmailService>,
    pub config: EmailConfig,
}

impl EmailClient {
    pub async fn new(config: EmailConfig) -> Result<Self> {
        let provider: Box<dyn EmailService> = match config.provider {
            EmailProvider::Sendgrid => {
                Box::new(providers::sendgrid::SendgridProvider::new())
            },
            EmailProvider::Mailgun => {
                Box::new(providers::mailgun::MailgunProvider::new())
            },
            EmailProvider::SES => {
                Box::new(providers::SES::SESProvider::new().await)
            },
        };
        
        Ok(Self {
            provider,
            config,
        })
    }
}
