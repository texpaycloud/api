use crate::email::Email;
use crate::email::EmailService;
use anyhow::{Error, Result};

pub struct MailgunProvider;

impl MailgunProvider {
    pub fn new() -> Self {
        todo!("implement Mailgun provider");
    }
}

#[async_trait::async_trait]
impl EmailService for MailgunProvider {
    async fn send_one(&self, email: Email) -> Result<(), Error> {
        todo!("implement Mailgun send_one");
    }
}
