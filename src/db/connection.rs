use anyhow::{Context, Error, Result};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::PgPool;
use std::env;
use tracing::{error, info};

pub struct DbConnection {
    pool: Option<PgPool>,
}

impl DbConnection {
    pub fn new() -> Self {
        Self { pool: None }
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        self.pool = Some(
            PgPoolOptions::new()
                .max_connections(32)
                .connect(&db_url)
                .await
                .unwrap(),
        );

        info!("Connected to database");

        Ok(())
    }
}
