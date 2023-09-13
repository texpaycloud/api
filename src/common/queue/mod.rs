use anyhow::{Result, Error};
use tracing::{info, error};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::{Client, types::Message};

pub struct SQS {
    client: Client,
}

pub enum QueueType {
    Email,
    SMS,
}

impl SQS {
    pub async fn new() -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);
        
        Self { client }
    }
    
    async fn get_available_queues(&self) -> Result<Vec<String>, Error> {
        match self.client.list_queues().send().await {
            Ok(res) => {
                if let Some(urls) = res.queue_urls {
                    return Ok(urls)
                } else {
                    return Err(Error::msg("No queues found"))
                }
            },
            Err(err) => {
                error!("Failed to get available queues: {:?}", err);
                return Err(err.into());
            }
        }
    }
    
    async fn get_queue_url(&self, queue_type: QueueType) -> Result<String, Error> {
        let queue_name = match queue_type {
            QueueType::Email => "email",
            QueueType::SMS => "sms",
        };
        
        let queue_urls = self.get_available_queues().await?;
        
        let url = match queue_urls.iter().find(|&url| url.contains(queue_name)) {
            Some(url) => Ok(url.to_string()),
            None => Err(Error::msg(format!("No SQS queue found containing: {}", queue_name))),
        };
        
       return match url {
            Ok(url) => Ok(url),
            Err(err) => Err(err)
        } 
    }        
    

    /// Pulls a single message from the specified queue.
    ///
    /// This function retrieves a single message from the queue specified by `queue_type`.
    /// After retrieving the message, it attempts to delete the message from the queue.
    ///
    /// # Arguments
    ///
    /// * `queue_type` - A QueueType enum indicating the type of the queue from which to pull the message.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Message))` - If a message is successfully pulled and deleted from the queue.
    /// * `Ok(None)` - If there are no messages in the queue.
    /// * `Err(Error)` - If there is an error pulling or deleting the message from the queue.
    pub async fn pull_one_message(&self, queue_type: QueueType) -> Result<Option<Message>, Error> {
        let queue_url = self.get_queue_url(queue_type).await?;
        let receive_response = self.client.receive_message().queue_url(&queue_url).send().await;
        
        if let Err(err) = receive_response {
            error!("Failed to pull message from queue: {:?}", err);
            return Err(err.into())
        }

       let messages = receive_response.unwrap().messages.unwrap_or_default();
       let cloned = messages[0].clone();
       let delete_response = self.client.delete_message().queue_url(&queue_url).send().await;
       
       if let Err(err) = delete_response {
           error!("Failed to delete message from queue: {:?}", err);
           return Err(err.into())
       }
       
       if messages.is_empty() {
           return Ok(None);
       }
       
       Ok(Some(cloned))
    }
}
