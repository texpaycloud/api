use crate::email::EmailService;
use crate::email::Email;
use anyhow::{Result, Error};
use tracing::{info, error};

pub struct SESProvider {}

impl EmailService for SESProvider {
    fn send(&self, email: Email) -> Result<(), Error> {
        info!("Sending email via SES: {:?}", email);
        Ok(())
    }
}
