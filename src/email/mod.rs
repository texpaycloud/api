pub mod providers;

use super::config::email::EmailConfig;
use super::config::email::EmailProvider;
use crate::grpc::proto::EmailRequest;
use anyhow::{Error, Result};

#[derive(Debug)]
pub struct Email {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub body: String,
}

impl From<EmailRequest> for Email {
    fn from(request: EmailRequest) -> Self {
        Self {
            to: request.to,
            from: request.from,
            subject: request.subject,
            body: request.body,
        }
    }
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
            EmailProvider::Sendgrid => Box::new(providers::sendgrid::SendgridProvider::new()),
            EmailProvider::Mailgun => Box::new(providers::mailgun::MailgunProvider::new()),
            EmailProvider::SES => Box::new(providers::ses::SESProvider::new().await),
        };

        Ok(Self { provider, config })
    }
}
