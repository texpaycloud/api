#![allow(warnings)]
use crate::common::queue::{QueueType, SQS};
use crate::config::email::EmailConfig;
use crate::email::EmailClient;
use crate::grpc::proto::email_server::Email;
use crate::grpc::proto::{EmailRequest, EmailResponse};
use anyhow::{Context, Error, Result};
use tonic::{
    transport::Server as TonicServer, Request as TonicRequest, Response as TonicResponse, Status,
};
use tracing::{error, info};

#[derive(Debug)]
pub struct EmailService {
    queue: SQS,
}

impl EmailService {
    pub async fn new() -> Result<Self, Error> {
        let queue = SQS::new().await;

        Ok(Self { queue })
    }
}

#[tonic::async_trait]
impl Email for EmailService {
    async fn send_email(
        &self,
        request: TonicRequest<EmailRequest>,
    ) -> Result<TonicResponse<EmailResponse>, Status> {
        info!("grpc send_email request: {:?}", request);

        let email = request.into_inner();

        // TODO: better error handling here
        if let Ok(()) = push_email_to_queue(&self.queue, email).await {
            info!("Email pushed to queue");
        } else {
            error!("Failed to push email to queue");
        }

        let response = EmailResponse {
            message: "Email sent".into(),
        };

        Ok(TonicResponse::new(response))
    }
}

async fn push_email_to_queue(queue: &SQS, email: EmailRequest) -> Result<(), Error> {
    let email = serde_json::to_string(&email).context("failed to serialize email")?;

    queue.push_one_message(QueueType::Email, email).await?;

    Ok(())
}
