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

pub trait EmailService {
    fn send(&self, email: Email) -> Result<(), Error>;
}
