use serde::Deserialize;
use config::Config;

#[derive(Debug, Deserialize, Clone)]
pub enum EmailProvider {
    Sendgrid,
    Mailgun,
    SES,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmailConfig {
    pub provider: EmailProvider,
}

impl EmailConfig {
    pub fn new(config: &Config) -> Result<Self, config::ConfigError> {
        let provider_string = config.get::<String>("email.provider")?;

        Ok(Self {
            provider: match provider_string.as_str() {
                "sendgrid" => EmailProvider::Sendgrid,
                "mailgun" => EmailProvider::Mailgun,
                "SES" => EmailProvider::SES,
                _ => {
                    panic!("Invalid email provider: {}", provider_string);
                },
            }
        })
    }
}


