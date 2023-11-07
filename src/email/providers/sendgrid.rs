use crate::email::EmailService;
use crate::email::Email;
use anyhow::{Error, Result};

pub struct SendgridProvider;

impl SendgridProvider {
    pub fn new() -> Self{
        todo!("implement Sendgrid provider");
    }
}

#[async_trait::async_trait]
impl EmailService for SendgridProvider {
    async fn send_one(&self, email: Email) -> Result<(), Error> {
        todo!("implement Sendgrid send_one");
    }
}
