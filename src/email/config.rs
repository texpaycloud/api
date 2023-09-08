pub enum EmailProvider {
    Sendgrid,
    SES,
    Mailgun,
}

pub struct EmailConfig {
    pub provider: EmailProvider,
    pub api_key: String,
    pub domain: String,
    pub from: String,
    pub from_name: String,
}

