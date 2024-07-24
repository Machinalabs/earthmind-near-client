use std::sync::{Arc, Mutex};

use near_crypto::SecretKey;
use near_jsonrpc_client::JsonRpcClient;
use near_sdk::AccountId;

use crate::models::EventData;
use crate::processors::TransactionProcessor;

pub struct Validator {
    client: &JsonRpcClient,
    db: &Arc<Mutex<rocksdb::DB>>,
    account_id: AccountId,
    secret_key: SecretKey,
}

impl Validator {
    pub fn new(
        client: &JsonRpcClient,
        db: &Arc<Mutex<rocksdb::DB>>,
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
impl TransactionProcessor for Validator {
    async fn process_transaction(
        &self,
        event_data: EventData,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Implementación específica para Miner
        // Usa self.answer cuando sea necesario
        println!("Validator Processor");
    }
}
