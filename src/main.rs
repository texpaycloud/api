mod api;
mod common;
mod config;
mod db;
mod email;
mod grpc;

use common::queue::SQS;
use config::email::EmailConfig;
use db::connection::DbConnection;
use email::EmailClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let settings = config::CONFIG.clone();

    let email_config = EmailConfig::new(&settings)?;
    let email_client = EmailClient::new(email_config).await?;

    tracing_subscriber::fmt::init();

    let mut db = DbConnection::new();
    db.connect().await?;

    let grpc_task = tokio::spawn(grpc::run());
    let api_task = tokio::spawn(api::run());

    let _ = tokio::try_join!(grpc_task, api_task)?;

    println!("Hello, world!");

    Ok(())
}
