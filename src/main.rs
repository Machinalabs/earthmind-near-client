use clap::Parser;
use near_crypto::SecretKey;
use std::sync::{Arc, Mutex};

mod block_streamer;
mod cli;
mod constants;
mod database;

use block_streamer::run_mode;
use cli::{Cli, Modes, Networks};
use constants::{DB_PATH, NEAR_RPC_MAINNET, NEAR_RPC_TESTNET};
use database::init_db;
use near_jsonrpc_client::{methods, JsonRpcClient};
use serde::Deserialize;
use near_sdk::AccountId;
use near_crypto::InMemorySigner;
use near_primitives::types::BlockReference;
use near_primitives::transaction::Transaction;
use near_jsonrpc_primitives::types::transactions::{RpcTransactionError,TransactionInfo};
use near_primitives::action::{Action, FunctionCallAction};
use near_primitives::views::TxExecutionStatus;
use std::time;
use near_jsonrpc_primitives::types::query::QueryResponseKind;

//use std::str::FromStr;

#[derive(Deserialize, Debug)]
struct EventData {
    request_id: String,
    start_time: u64,
    reveal_miner_time: u64,
    commit_miner_time: u64,
    reveal_validator_time: u64,
    commit_validator_time: u64,
}

#[derive(Deserialize, Debug)]
struct EventJson {
    standard: String,
    version: String,
    event: String,
    data: Vec<EventData>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Open RocksDB connection
    let db = Arc::new(Mutex::new(init_db(DB_PATH)?));

    // Connect to the RPC client
    let client: JsonRpcClient;

    match cli.network {
        Networks::Mainnet => {
            client = JsonRpcClient::connect(NEAR_RPC_MAINNET);
        }
        Networks::Testnet => {
            client = JsonRpcClient::connect(NEAR_RPC_TESTNET);
        }
    }

    match cli.mode {
        Modes::Miner => {
            run_mode(&client, &db, cli.account_id, cli.private_key, cli.answer, process_miner_transaction).await?;
        }
        Modes::Validator => {
            run_mode(&client, &db, cli.account_id, cli.private_key, cli.answer, process_validator_transaction).await?;
        }
    }

    Ok(())
}

async fn process_miner_transaction(
    client: &JsonRpcClient,
    logs: Vec<String>,
    account_id: AccountId,
    key: SecretKey, 
    answer : String
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    for log in logs {
        println!("Miner Log: {}", log);
        match process_log(&log) {
            Ok(event) => {
                for data in event.data {
                    println!("Request ID: {}", data.request_id);
                    println!("Start Time: {}", data.start_time);
                    println!("Reveal Miner Time: {}", data.reveal_miner_time);
                    println!("Commit Miner Time: {}", data.commit_miner_time);
                    println!("Reveal Validator Time: {}", data.reveal_validator_time);
                    println!("Commit Validator Time: {}", data.commit_validator_time);

                    //let signer_account = AccountId::from_str(&account_id).map_err(|e| format!("Failed to parse sender_account_id: {}", e))?;
                    //let signer_private_key = key;
                    let signer = near_crypto::InMemorySigner::from_secret_key(account_id.clone(), key.clone());
                    
                    let commit_miner_result = commit_by_miner(client, &signer, data.request_id.clone(), answer.clone()).await;

                    match commit_miner_result {
                        Ok(_) => println!("Commit by miner successful for request_id: {}", data.request_id),
                        Err(e) => println!("Failed to commit by miner: {}", e),
                    }
                }
            }
            Err(e) => println!("Failed to process log: {}", e),
        }
    }
    Ok(false)
}

async fn process_validator_transaction(
    client: &JsonRpcClient,
    logs: Vec<String>,
    account: AccountId,
    key: SecretKey,
    answer :  String
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    for log in logs {
        println!("Validator Log: {}", log);
        match process_log(&log) {
            Ok(event) => {
                for data in event.data {
                    println!("Request ID: {}", data.request_id);
                    println!("Start Time: {}", data.start_time);
                    println!("Reveal Miner Time: {}", data.reveal_miner_time);
                    println!("Commit Miner Time: {}", data.commit_miner_time);
                    println!("Reveal Validator Time: {}", data.reveal_validator_time);
                    println!("Commit Validator Time: {}", data.commit_validator_time);

                    
                }
            }
            Err(e) => println!("Failed to process log: {}", e),
        }
    }
    Ok(false)
}

fn process_log(log: &str) -> Result<EventJson, Box<dyn std::error::Error>> {
    // Remove "EVENT_JSON:" prefix
    let json_part = log.trim_start_matches("EVENT_JSON:");

    // Deserialize from JSON to EventJson struct
    let event: EventJson = serde_json::from_str(json_part)?;
    
    Ok(event)
}

async fn commit_by_miner(
    client: &JsonRpcClient,
    signer: &InMemorySigner,
    request_id: String,
    answer :  String
) -> Result<(), Box<dyn std::error::Error>> {

    let access_key_query_response = client.call(methods::query::RpcQueryRequest {
        block_reference: BlockReference::latest(),
        request: near_primitives::views::QueryRequest::ViewAccessKey {
            account_id: signer.account_id.clone(),
            public_key: signer.public_key.clone(),
        },
    }).await?;

    let current_nonce = match access_key_query_response.kind {
        QueryResponseKind::AccessKey(access_key) => access_key.nonce,
        _ => Err("Failed to extract current nonce")?,
    };

    let transaction = Transaction {
        signer_id : signer.account_id.clone(),
        public_key : signer.public_key.clone(),
        nonce : current_nonce + 1,
        receiver_id : "earthmindprotocol.testnet".parse()?,
        block_hash : access_key_query_response.block_hash,
        actions: vec![Action::FunctionCall(Box::new(FunctionCallAction{
            method_name: "commit_by_miner".to_string(), 
            args: serde_json::json!({
                "request_id" : request_id,
                "answer" : answer,
            }).to_string().into_bytes(),
            gas: 100_000_000_000_000,
            deposit: 0,
        }))]
    };

    let tx_hash = transaction.get_hash_and_size().0;

    let request = methods::send_tx::RpcSendTransactionRequest {
        signed_transaction: transaction.sign(signer),
        wait_until: TxExecutionStatus::Final,
    };

    let sent_at = time::Instant::now();
    let response = match client.call(request).await {
        Ok(response) => response,
        Err(err) => {
            match err.handler_error() {
                Some(RpcTransactionError::TimeoutError) => {}
                _ => Err(err)?,
            }
            loop {
                let response = client
                    .call(methods::tx::RpcTransactionStatusRequest {
                        transaction_info: TransactionInfo::TransactionId {
                            tx_hash,
                            sender_account_id: signer.account_id.clone(),
                        },
                        wait_until: TxExecutionStatus::Final,
                    })
                    .await;
                let received_at = time::Instant::now();
                let delta = (received_at - sent_at).as_secs();

                if delta > 60 {
                    Err("time limit exceeded for the transaction to be recognized")?;
                }

                match response {
                    Err(err) => match err.handler_error() {
                        Some(RpcTransactionError::TimeoutError) => {}
                        _ => Err(err)?,
                    },
                    Ok(response) => {
                        break response;
                    }
                }
            }
        }
    };

    let received_at = time::Instant::now();
    let delta = (received_at - sent_at).as_secs();
    println!("response gotten after: {}s", delta);
    println!("response: {:#?}", response);

    Ok(())
}
