use crate::models::EventData;
use crate::nonce_manager::NonceManager;
use crate::qx_sender::QuerySender;
use crate::tx_builder::TxBuilder;
use crate::tx_sender::TxSender;

use crate::qx_builder::QueryBuilder;
use async_trait::async_trait;
use near_jsonrpc_client::methods;
use near_primitives::views::TxExecutionStatus;
use near_sdk::AccountId;

use std::sync::Arc;
use tokio::sync::Mutex;

use super::utils;
use super::TransactionProcessor;

pub struct Validator {
    nonce_manager: Arc<NonceManager>,
    tx_builder: Arc<Mutex<TxBuilder>>,
    tx_sender: Arc<TxSender>,
    db: Arc<Mutex<rocksdb::DB>>,
    account_id: AccountId,
}

impl Validator {
    pub fn new(
        nonce_manager: Arc<NonceManager>,
        tx_builder: Arc<Mutex<TxBuilder>>,
        tx_sender: Arc<TxSender>,
        db: Arc<Mutex<rocksdb::DB>>,
        account_id: AccountId,
    ) -> Self {
        Self {
            nonce_manager,
            tx_builder,
            tx_sender,
            db,
            account_id,
        }
    }
}

#[async_trait]
impl TransactionProcessor for Validator {
    async fn process_transaction(
        &self,
        event_data: EventData,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Implementación específica para Validator
        println!("Validator Processor");
        println!("Event Data: {:?}", event_data);

        match self.commit(event_data.clone()).await {
            Ok(_) => {
                println!("Commit successful");
                Ok(true)
            }
            Err(e) => {
                println!("Failed to commit: {}", e);
                Err(e)
            }
        }
    }

    async fn commit(
        &self,
        event_data: EventData,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Validator Commit");

        let answer = utils::generate_validator_answer();

        let query = QueryBuilder::new("earthmindprotocol.testnet".to_string())
            .with_method_name("hash_validator_answer")
            .with_args(serde_json::json!({
                "validator": self.account_id.to_string(),
                "request_id": event_data.request_id,
                "answer": answer,
                "message": "This are the best miners",
            }))
            .build();

        let query_sender = QuerySender::new(self.tx_sender.client.clone());
        let query_result = query_sender.send_query(query).await?;

        println!("QUERY RESULT: {}", query_result);

        let (nonce, block_hash) = self.nonce_manager.get_nonce_and_tx_hash().await?;

        let mut tx_builder = self.tx_builder.lock().await;

        let (tx, _) = tx_builder
            .with_method_name("commit_by_validator")
            .with_args(serde_json::json!({
                "request_id": event_data.request_id,
                "answer": query_result,
            }))
            .build(nonce, block_hash);

        let signer = &tx_builder.signer;

        let request = methods::send_tx::RpcSendTransactionRequest {
            signed_transaction: tx.sign(signer),
            wait_until: TxExecutionStatus::Final,
        };

        self.tx_sender.send_transaction(request).await?;

        println!(
            "Commit by validator successful for request_id: {}",
            event_data.request_id
        );
        Ok(())
    }

    async fn reveal(
        &self,
        event_data: EventData,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Reveal by validator");

        let (nonce, block_hash) = self.nonce_manager.get_nonce_and_tx_hash().await?;

        let mut tx_builder = self.tx_builder.lock().await;

        let (tx, _) = tx_builder
            .with_method_name("reveal_by_validator")
            .with_args(serde_json::json!({
                "request_id": event_data.request_id,
                "answer": utils::generate_validator_answer(),
                "message": "This are the best miners",
            }))
            .build(nonce, block_hash);

        let signer = &tx_builder.signer;

        let request = methods::send_tx::RpcSendTransactionRequest {
            signed_transaction: tx.sign(signer),
            wait_until: TxExecutionStatus::Final,
        };

        self.tx_sender.send_transaction(request).await?;

        Ok(())
    }
}
