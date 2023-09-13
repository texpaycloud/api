pub struct EmailConfig {
    pub provider: super::EmailProvider,
    pub api_key: String,
    pub domain: String,
    pub from: String,
    pub from_name: String,
}

