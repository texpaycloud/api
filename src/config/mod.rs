use config::{Config, Environment};
use lazy_static::lazy_static;

pub mod email;

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Config = Config::builder()
        .add_source(Environment::with_prefix("CONFIG").separator("_"))
        .build()
        .unwrap();
}
