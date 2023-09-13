mod email;
mod common;
mod grpc;
mod api;
mod db;

use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().context("Failed to load .env file")?;

    // Set up tracing
    tracing_subscriber::fmt::init();
    
    // Connect to database
    let mut db = db::connection::DbConnection::new(); 
    db.connect().await.context("Failed to connect to database")?;

    // Serve API routes
    api::run().await.context("Failed to start API server")?;
    
    // Serve grpc
    grpc::run().await.context("Failed to start grpc server")?;

    Ok(())
}
