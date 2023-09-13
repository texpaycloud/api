use anyhow::{Context, Result, Error};
use hyper::{Body, Request, Response, Server, StatusCode};
use routerify::prelude::*;
use routerify::{Middleware, RequestInfo, Router, RouterService};
use std::net::SocketAddr;
use std::convert::Infallible;
use tracing::{info, error};


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

pub async fn run() -> Result<(), Error> {
    let router = router();
    let service = RouterService::new(router).unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = Server::bind(&addr).serve(service);

    match server.await {
        Ok(_) => {
            info!("Listening on http://{}", addr);
            return Ok(());
        },
        Err(err) => { 
        error!("Server error: {}", err);
            return Err(err.into()); 
        }
    }
    }