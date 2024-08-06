use crate::block_streamer::extract_logs;
use crate::constants::ACCOUNT_TO_LISTEN;
use crate::models::EventData;
use crate::nonce_manager::NonceManager;
use crate::qx_builder::QueryBuilder;
use crate::qx_sender::QuerySender;
use crate::tx_builder::TxBuilder;
use crate::tx_sender::TxSender;

use async_trait::async_trait;
use near_jsonrpc_client::methods;
use near_primitives::views::TxExecutionStatus;
use near_sdk::AccountId;
use tokio::time::{sleep, Duration};

use std::sync::Arc;
use tokio::sync::Mutex;

use super::TransactionProcessor;

pub struct Miner {
    nonce_manager: Arc<NonceManager>,
    tx_builder: Arc<Mutex<TxBuilder>>,
    tx_sender: Arc<TxSender>,
    db: Arc<Mutex<rocksdb::DB>>,
    account_id: AccountId,
}

impl Miner {
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
impl TransactionProcessor for Miner {
    async fn process_transaction(
        &self,
        event_data: EventData,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        println!("Miner Processor");
        println!("Event Data: {:?}", event_data);

        match self.commit(event_data.clone()).await {
            Ok(_) => {
                println!("Commit miner successful");
                //Ok(true)
            }
            Err(e) => {
                println!("Failed to commit by miner: {}", e);
                //Err(e)
            }
        }

        // Wait 30 seconds
        sleep(Duration::from_secs(15)).await;

        // Miner reveal

        match self.reveal(event_data.clone()).await {
            Ok(_) => {
                println!("Reveal miner successful");
                Ok(true)
            }
            Err(e) => {
                println!("Failed to reveal by miner: {}", e);
                Err(e)
            }
        }
    }

    async fn commit(
        &self,
        event_data: EventData,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Miner Commit");

        let miner = self.account_id.clone();
        println!("ACCOUNT ID_ use to commit: {}", miner);

        println!("REQUEST_ID USE TO COMMIT: {}", event_data.request_id);

        let answer = true;
        println!("ANSWER: {}", true);

        let message = "It's the best option";
        println!("MESSGE: {}", message);

        let query = QueryBuilder::new(ACCOUNT_TO_LISTEN.to_string())
            .with_method_name("hash_miner_answer")
            .with_args(serde_json::json!({
                "miner": miner,
                "request_id": event_data.request_id,
                "answer": answer,
                "message": message,
            }))
            .build();

        let query_sender = QuerySender::new(self.tx_sender.client.clone());
        let hash_miner_answer = query_sender.send_query(query).await?;

        println!("HASH MINER ANSWER: {}", hash_miner_answer);

        let (nonce, block_hash) = self.nonce_manager.get_nonce_and_tx_hash().await?;

        let mut tx_builder = self.tx_builder.lock().await;

        println!("REQUEST_ID ENVIADO: {}", event_data.request_id);
        println!("ANSWER ENVIADA: {}", hash_miner_answer);

        let (tx, _) = tx_builder
            .with_method_name("commit_by_miner")
            .with_args(serde_json::json!({
                "request_id": event_data.request_id,
                "answer": hash_miner_answer,
            }))
            .build(nonce, block_hash);

        let signer = &tx_builder.signer;

        let request = methods::send_tx::RpcSendTransactionRequest {
            signed_transaction: tx.sign(signer),
            wait_until: TxExecutionStatus::Final,
        };

        let tx_response = self.tx_sender.send_transaction(request).await?;
        println!("TRANSACTION RESPONSE MINER COMMIT: {:?}", tx_response);

        let log_tx = extract_logs(&tx_response);

        println!("COMMIT_MINER_LOG: {:?}", log_tx);

        println!(
            "Commit by miner successful for request_id: {}",
            event_data.request_id
        );
        Ok(())
    }

    async fn reveal(
        &self,
        event_data: EventData,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Reveal by miner");

        let (nonce, block_hash) = self.nonce_manager.get_nonce_and_tx_hash().await?;

        let mut tx_builder = self.tx_builder.lock().await;

        println!("REQUEST_ID USE TO REVEAL: {}", event_data.request_id);

        let answer = true;
        println!("ANSWER: {}", true);

        let message = "It's the best option";
        println!("MESSGE: {}", message);

        let (tx, _) = tx_builder
            .with_method_name("reveal_by_miner")
            .with_args(serde_json::json!({
                "request_id": event_data.request_id,
                "answer": answer,
                "message": message,
            }))
            .build(nonce, block_hash);

        let signer = &tx_builder.signer;

        let request = methods::send_tx::RpcSendTransactionRequest {
            signed_transaction: tx.sign(signer),
            wait_until: TxExecutionStatus::Final,
        };

        let tx_response = self.tx_sender.send_transaction(request).await?;
        println!("TRANSACTION RESPONSE MINER REVEAL: {:?}", tx_response);
        let log_tx = extract_logs(&tx_response);
        println!("REVEAL_MINER_LOG: {:?}", log_tx);

        Ok(())
    }
}
