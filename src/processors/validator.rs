use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use near_crypto::{InMemorySigner, SecretKey};
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_primitives::{views::TxExecutionStatus, hash::CryptoHash, action::Action, action::FunctionCallAction, transaction::Transaction};
//use near_sdk::{env, require, AccountId};
use near_sdk::AccountId;

use crate::models::EventData;
use crate::processors::TransactionProcessor;

use crate::processors::transaction_processor::{
    build_commit_transaction, get_nonce_and_tx_hash, send_transaction,
};

pub struct Validator {
    client: Arc<JsonRpcClient>,
    db: Arc<Mutex<rocksdb::DB>>,
    account_id: AccountId,
    secret_key: SecretKey,
    signer: InMemorySigner,
}

impl Validator {
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
impl TransactionProcessor for Validator {
    async fn process_transaction(
        &self,
        event_data: EventData,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Implementación específica para Validator
        println!("Validator Processor");
        println!("Event Data: {:?}", event_data);
        Ok(true) // o el valor que corresponda
    }

    async fn commit(
        &self,
        event_data: EventData,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Validator Commit");

        // let answer = hash_validator_answer(
        //     self.signer.account_id.clone(),
        //     event_data.request_id.clone(),
        //     generate_validator_answer(),
        //     "The best miners".to_string(),
        // );
        let answer = "422fa60e22dc75c98d21bb975323c5c0b854d6b0b7a63d6446b3bbb628b65a5b".to_string();

        let commit_validator_result = commit_by_validator(
            &self.client,
            &self.signer,
            event_data.request_id.clone(),
            answer.clone(),
        )
        .await;

        match commit_validator_result {
            Ok(_) => println!(
                "Commit by validator successful for request_id: {}",
                event_data.request_id.clone()
            ),
            Err(e) => println!("Failed to commit by validator: {}", e),
        }
        Ok(())
    }

    async fn reveal(&self, event_data: EventData) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Implementación específica para Validator
        println!("Validator Reveal");

        // To reveal we need request_id : String, Answer : Vec<AccountId>, message: String
        let answer = generate_validator_answer();
        let message = "The best miners".to_string();

        let reveal_validator_result = reveal_by_validator(&self.client,
            &self.signer,
            event_data.request_id.clone(),
            answer,
            message).await;
        Ok(())
    }
}

// pub fn hash_validator_answer(
//     validator: AccountId,
//     request_id: String,
//     answer: Vec<AccountId>,
//     message: String,
// ) -> String {
//     require!(answer.len() == 10, "Invalid answer");

//     let mut concatenated_answer: Vec<u8> = Vec::new();

//     concatenated_answer.extend_from_slice(request_id.as_bytes());
//     concatenated_answer.extend_from_slice(validator.as_bytes());

//     let value: Vec<u8> = answer
//         .iter()
//         .flat_map(|id| id.as_bytes())
//         .copied()
//         .collect();
//     concatenated_answer.extend_from_slice(&value);
//     concatenated_answer.extend_from_slice(message.as_bytes());

//     let value = env::keccak256(&concatenated_answer);

//     //@dev Return the hash of the answer
//     hex::encode(value)
// }

fn generate_validator_answer() -> Vec<AccountId> {
    let value = vec![
        "miner1.near".parse().unwrap(),
        "miner2.near".parse().unwrap(),
        "miner3.near".parse().unwrap(),
        "miner4.near".parse().unwrap(),
        "miner5.near".parse().unwrap(),
        "miner6.near".parse().unwrap(),
        "miner7.near".parse().unwrap(),
        "miner8.near".parse().unwrap(),
        "miner9.near".parse().unwrap(),
        "miner10.near".parse().unwrap(),
    ];
    value
}

async fn commit_by_validator(
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

async fn reveal_by_validator(client: &JsonRpcClient,
    signer: &InMemorySigner,
    request_id: String,
    answer: Vec<AccountId>,
    message: String)->Result<(), Box<dyn std::error::Error>> {

        let (current_nonce, block_hash) = get_nonce_and_tx_hash(client, signer).await?;

        let (transaction, tx_hash) =
            build_reveal_validator_transaction(signer, request_id, answer, message, current_nonce, block_hash);
    
        let request = methods::send_tx::RpcSendTransactionRequest {
            signed_transaction: transaction.sign(signer),
            wait_until: TxExecutionStatus::Final,
        };
    
        send_transaction(client, request, tx_hash, signer).await?;

    Ok(())
}

pub fn build_reveal_validator_transaction(
    signer: &InMemorySigner,
    request_id: String,
    answer: Vec<AccountId>,
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