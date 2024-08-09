use crate::constants::ACCOUNT_TO_LISTEN;
use crate::models::EventData;
use crate::qx_builder::QueryBuilder;
use crate::qx_sender::QuerySender;
use near_jsonrpc_client::JsonRpcClient;

use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait TransactionProcessor: Send + Sync {
    async fn process_transaction(
        &self,
        event_data: EventData,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;

    async fn commit(
        &self,
        event_data: EventData,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn reveal(
        &self,
        event_data: EventData,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn obtain_top_ten(
        &self,
        event_data: EventData,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    // Default method to all the implementations to synchronize
    async fn get_stage(
        &self,
        tx_sender: Arc<JsonRpcClient>,
        event_data: EventData,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        println!("Get Stage");

        let query = QueryBuilder::new(ACCOUNT_TO_LISTEN.to_string())
            .with_method_name("get_stage")
            .with_args(serde_json::json!({
                "start_time": event_data.start_time,
            }))
            .build();

        let query_sender = QuerySender::new(tx_sender);
        let stage = query_sender.send_query(query).await?;

        Ok(stage)
    }
}
