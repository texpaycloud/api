pub mod config;
pub mod providers;

use anyhow::{Result, Error};

#[derive(Debug)]
pub struct Email {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub body: String,
}

pub enum EmailProvider {
    Sendgrid,
    SES,
    Mailgun,
}

#[async_trait::async_trait]
pub trait EmailService {
    async fn send_one(&self, email: Email) -> Result<(), Error>;
}
