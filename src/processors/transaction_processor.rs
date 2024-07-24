use crate::models::EventData;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionProcessor: Send + Sync {
    async fn process_transaction(
        &self,
        event_data: EventData,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;

    async fn commit(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn reveal(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
