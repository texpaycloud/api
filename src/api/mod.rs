use anyhow::{Context, Error, Result};
use hyper::{Body, Request, Response, Server, StatusCode};
use routerify::{ext::RequestExt, Middleware, RequestInfo, Router, RouterService};
use std::convert::Infallible;
use std::net::SocketAddr;
use tracing::{error, info};

async fn home_handler(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello world")))
}

async fn logger(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    // TODO: this middleware causes massive performance degredation
    // info!(
    //     "{} {} {}",
    //     req.remote_addr(),
    //     req.method(),
    //     req.uri().path()
    // );
    //

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

    Server::bind(&addr)
        .serve(service)
        .await
        .context("Failed to start server")?;
    info!("Listening on http://{}", addr);

    Ok(())
}
