use config::{Config, Environment};
use lazy_static::lazy_static;
use std::sync::RwLock;
use tracing::info;

pub mod email;

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Config = Config::builder()
        .add_source(Environment::with_prefix("CONFIG").separator("_"))
        .build()
        .unwrap();
}
