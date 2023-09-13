mod email;
mod common;
mod grpc;
mod api;
mod db;

use anyhow::{Context, Result};
use tokio::task;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().context("Failed to load .env file")?;

    // Set up tracing
    tracing_subscriber::fmt::init();
    
    // Connect to database
    let mut db = db::connection::DbConnection::new(); 
    db.connect().await.context("Failed to connect to database").unwrap();

    // Spawn a new task for the gRPC server
    let grpc_task = task::spawn(async {
        grpc::run().await.context("Failed to start grpc server").unwrap();
    });

    let api_task = task::spawn(async {
        api::run().await.context("Failed to start API server").unwrap();
    });
    
    tokio::try_join!(grpc_task, api_task)?;

    Ok(())
}
