mod api;
mod common;
mod config;
mod db;
mod email;
mod grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let settings = config::CONFIG.clone();
    println!("{:?}", settings);


    let email_config = config::email::EmailConfig::new(&settings)?;
    let email_client = email::EmailClient::new(email_config).await?;
    
    tracing_subscriber::fmt::init();



    let mut db = db::connection::DbConnection::new();
    db.connect().await?;

    let grpc_task = tokio::spawn(grpc::run());
    let api_task = tokio::spawn(api::run());

    let _ = tokio::try_join!(grpc_task, api_task)?;

    println!("Hello, world!");

    Ok(())
}
