use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use near_crypto::SecretKey;
use near_jsonrpc_client::JsonRpcClient;
use near_sdk::AccountId;

use crate::models::EventData;
use crate::processors::TransactionProcessor;

pub struct Miner {
    client: Arc<JsonRpcClient>,
    db: Arc<Mutex<rocksdb::DB>>,
    account_id: AccountId,
    secret_key: SecretKey,
}

impl Miner {
    pub fn new(
        client: Arc<JsonRpcClient>,
        db: Arc<Mutex<rocksdb::DB>>,
        account_id: AccountId,
        secret_key: SecretKey,
    ) -> Self {
        Self {
            client,
            db,
            account_id,
            secret_key,
        }
    }
}

#[async_trait]
impl TransactionProcessor for Miner {
    async fn process_transaction(
        &self,
        event_data: EventData,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Implementación específica para Miner
        println!("Miner Processor");
        println!("Event Data: {:?}", event_data);
        Ok(true) // o el valor que corresponda
    }
}
