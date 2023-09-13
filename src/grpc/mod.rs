pub mod proto;

use tonic::{transport::Server as TonicServer, Request as TonicRequest, Response as TonicResponse, Status};
use proto::test_server::{Test, TestServer};
use proto::{TestRequest, TestResponse};
use anyhow::{Context, Result, Error};
use tracing::{info, error};

#[derive(Debug, Default)]
struct TestService;

#[tonic::async_trait]
impl Test for TestService {
    async fn test(&self, request: TonicRequest<TestRequest>) -> Result<TonicResponse<TestResponse>, Status> {
        let msg = request.into_inner().message;
        
        let reply = TestResponse {
            message: format!("Hello {}!", msg),
            server: "Test server".into(),
        };
        
        Ok(TonicResponse::new(reply))
    }
}

pub async fn run() -> Result<(), Error> {
    let addr = "[::1]:50051".parse().context("Failed to parse address")?;
    let test_service = TestService::default();
    
    let server = TonicServer::builder()
        .add_service(TestServer::new(test_service))
        .serve(addr);

    match server.await {
        Ok(_) => {
            info!("GRPC listening on http://{}", addr);
            return Ok(());
        },
        Err(err) => { 
            error!("Error starting GRPC server: {}", err);
            return Err(err.into()); 
        }
    }
}
    