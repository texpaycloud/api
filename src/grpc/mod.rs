mod email;
pub mod proto;

use anyhow::{Context, Error, Result};
use email::EmailService;
use proto::email_server::{Email, EmailServer};
use proto::test_server::{Test, TestServer};
use proto::{TestRequest, TestResponse};
use tonic::{
    transport::Server as TonicServer, Request as TonicRequest, Response as TonicResponse, Status,
};
use tracing::{error, info};

#[derive(Debug, Default)]
struct TestService;

#[tonic::async_trait]
impl Test for TestService {
    async fn test(
        &self,
        request: TonicRequest<TestRequest>,
    ) -> Result<TonicResponse<TestResponse>, Status> {
        // let msg = request.into_inner().message;

        // let reply = TestResponse {
        //     message: format!("Hello {}!", msg),
        //     server: "Test server".into(),
        // };

        // info!("grpc test: {:?}", &reply);

        Ok(TonicResponse::new(TestResponse {
            message: "Hello World!".into(),
            server: "Test server".into(),
        }))
    }
}

pub async fn run() -> Result<(), Error> {
    let addr = "127.0.0.1:50051"
        .parse()
        .context("Failed to parse address")?;

    let test_service = TestService::default();
    let email_service = EmailService::new().await?;

    let server = TonicServer::builder()
        .add_service(TestServer::new(test_service))
        .add_service(EmailServer::new(email_service))
        .serve(addr);

    match server.await {
        Ok(_) => {
            info!("GRPC listening on http://{}", addr);
            return Ok(());
        }
        Err(err) => {
            error!("Error starting GRPC server: {}", err.to_string());
            return Err(err.into());
        }
    }
}
