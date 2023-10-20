mod api;
mod common;
mod db;
mod email;
mod grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let mut db = db::connection::DbConnection::new();
    db.connect().await?;

    let grpc_task = tokio::spawn(grpc::run());
    let api_task = tokio::spawn(api::run());
    
    let _ = tokio::try_join!(grpc_task, api_task)?;

    Ok(())
}
