use anyhow::{Context, Result, Error};
use chrono::{DateTime, Utc};
use hyper::{Body, Request, Response, Server, StatusCode};
use routerify::prelude::*;
use routerify::{Middleware, RequestInfo, Router, RouterService};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::Row;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use tracing::{error, info};

mod email;

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub rvs_id: Option<String>,
    pub stripe_client_id: Option<String>,
    pub clerk_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub client_id: i32,
    pub rvs_company_id: String,
    pub short_name: String,
    pub stripe_account_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_address: Option<String>,
    pub last_login_date: Option<DateTime<Utc>>,
    pub email: String,
    pub address: Option<String>,
    pub balance: i32,
    pub name: Option<String>,
    pub organization_id: Option<i32>,
    pub phone: Option<String>,
    pub rvs_customer_id: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub clerk_id: Option<String>,
    pub rvs_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub amount: i32,
    pub customer_balance: Option<i32>,
    pub customer_id: i32,
    pub failure_reason: Option<String>,
    pub fees: i32,
    pub stripe_id: String,
    pub stripe_payment_method: String,
    pub success: bool,
    pub organization_id: i32,
}

async fn home_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello world")))
}

async fn logger(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    info!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    
    Ok(req)
}

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    error!("{}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

fn router() -> Router<Body, Infallible> {
    Router::builder()
        .middleware(Middleware::pre(logger))
        .get("/", home_handler)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().context("Failed to load .env file")?;

    // Set up tracing
    tracing_subscriber::fmt::init();

    // Connect to database
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(32)
        .connect(&db_url)
        .await?;

    info!("Connected to database");

    // Serve API routes
    let router = router();
    let service = RouterService::new(router).unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = Server::bind(&addr).serve(service);

    info!("Listening on http://{}", addr);
    if let Err(err) = server.await {
        error!("Server error: {}", err);
    }

    Ok(())
}
