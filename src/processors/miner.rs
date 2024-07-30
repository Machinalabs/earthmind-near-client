use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use near_crypto::{InMemorySigner, SecretKey};
use near_jsonrpc_client::{methods, JsonRpcClient};

use near_primitives::{views::TxExecutionStatus, hash::CryptoHash, transaction::Transaction, action::FunctionCallAction, action::Action};
use near_sdk::{env, AccountId};

use crate::models::EventData;
use crate::processors::transaction_processor::{
    build_commit_transaction, get_nonce_and_tx_hash, send_transaction,
};
use crate::processors::TransactionProcessor;

pub struct Miner {
    client: Arc<JsonRpcClient>,
    db: Arc<Mutex<rocksdb::DB>>,
    account_id: AccountId,
    secret_key: SecretKey,
    signer: near_crypto::InMemorySigner,
}

impl Miner {
    pub fn new(
        client: Arc<JsonRpcClient>,
        db: Arc<Mutex<rocksdb::DB>>,
        account_id: AccountId,
        secret_key: SecretKey,
    ) -> Self {
        let signer =
            near_crypto::InMemorySigner::from_secret_key(account_id.clone(), secret_key.clone());

        Self {
            client,
            db,
            account_id,
            secret_key,
            signer,
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
        println!("Miner Commit");

        // let answer = hash_miner_answer(
        //     self.signer.account_id.clone(),
        //     event_data.request_id.clone(),
        //     true,
        //     "The best option".to_string(),
        // );
        let answer = "422fa60e22dc75c98d21bb975323c5c0b854d6b0b7a63d6446b3bbb628b65a5b".to_string();
        let commit_miner_result = commit_by_miner(
            &self.client,
            &self.signer,
            event_data.request_id.clone(),
            answer.clone(),
        )
        .await;

        match commit_miner_result {
            Ok(_) => println!(
                "Commit by miner successful for request_id: {}",
                event_data.request_id.clone()
            ),
            Err(e) => println!("Failed to commit by miner: {}", e),
        }
        Ok(())
    }

    async fn reveal(&self, event_data: EventData) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Miner Reveal");

        let answer = true;
        let message = "It's cool".to_string();

        let reveal_miner_result = reveal_by_miner(
            &self.client,
            &self.signer,
            event_data.request_id.clone(),
            answer,
            message
        )
        .await;

        match reveal_miner_result {
            Ok(_) => println!(
                "Reveal by miner successful for request_id: {}",
                event_data.request_id.clone()
            ),
            Err(e) => println!("Failed to reveal by miner: {}", e),
        }
        Ok(())
    }
}

// fn hash_miner_answer(
//     miner: AccountId,
//     request_id: String,
//     answer: bool,
//     message: String,
// ) -> String {
//     let concatenated_answer = format!("{}{}{}{}", request_id, miner, answer, message);
//     let value = env::keccak256(concatenated_answer.as_bytes());

//     //@dev Return the hash of the answer
//     hex::encode(value)
// }

async fn commit_by_miner(
    client: &JsonRpcClient,
    signer: &InMemorySigner,
    request_id: String,
    answer: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let (current_nonce, block_hash) = get_nonce_and_tx_hash(client, signer).await?;

    let (transaction, tx_hash) =
        build_commit_transaction(signer, request_id, answer, current_nonce, block_hash);

    let request = methods::send_tx::RpcSendTransactionRequest {
        signed_transaction: transaction.sign(signer),
        wait_until: TxExecutionStatus::Final,
    };

    send_transaction(client, request, tx_hash, signer).await?;

    Ok(())
}

async fn reveal_by_miner(client: &JsonRpcClient,
    signer: &InMemorySigner,
    request_id: String,
    answer: bool,
    message: String)->Result<(), Box<dyn std::error::Error>> {

        let (current_nonce, block_hash) = get_nonce_and_tx_hash(client, signer).await?;

        let (transaction, tx_hash) =
            build_reveal_miner_transaction(signer, request_id, answer, message, current_nonce, block_hash);
    
        let request = methods::send_tx::RpcSendTransactionRequest {
            signed_transaction: transaction.sign(signer),
            wait_until: TxExecutionStatus::Final,
        };
    
        send_transaction(client, request, tx_hash, signer).await?;


    Ok(())
}

pub fn build_reveal_miner_transaction(
    signer: &InMemorySigner,
    request_id: String,
    answer: bool,
    message: String, 
    current_nonce: u64,
    block_hash: CryptoHash,
) -> (Transaction, CryptoHash) {
    let transaction = Transaction {
        signer_id: signer.account_id.clone(),
        public_key: signer.public_key.clone(),
        nonce: current_nonce + 1,
        receiver_id: "earthmindprotocol.testnet".parse().unwrap(),
        block_hash,
        actions: vec![Action::FunctionCall(Box::new(FunctionCallAction {
            method_name: "reveal_by_miner".to_string(),
            args: serde_json::json!({
                "request_id": request_id,
                "answer": answer,
                "message":message,
            })
            .to_string()
            .into_bytes(),
            gas: 100_000_000_000_000,
            deposit: 0,
        }))],
    };

    (transaction.clone(), transaction.get_hash_and_size().0)
}