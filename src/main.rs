use clap::Parser;
use std::sync::{Arc, Mutex};

mod block_streamer;
mod cli;
mod constants;
mod database;

use block_streamer::run_mode;
use cli::{Cli, Modes, Networks};
use constants::{DB_PATH, NEAR_RPC_MAINNET, NEAR_RPC_TESTNET};
use database::init_db;
use near_jsonrpc_client::JsonRpcClient;
use serde::Deserialize;

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
            run_mode(&client, &db, cli.account_id, process_miner_transaction).await?;
        }
        Modes::Validator => {
            run_mode(&client, &db, cli.account_id, process_validator_transaction).await?;
        }
    }

    Ok(())
}
fn process_miner_transaction(
    client: &JsonRpcClient,
    logs: Vec<String>,
) -> Result<bool, Box<dyn std::error::Error>> {
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
                }
            }
            Err(e) => println!("Failed to process log: {}", e),
        }
    }
    Ok(false)
}

fn process_validator_transaction(
    client: &JsonRpcClient,
    logs: Vec<String>,
) -> Result<bool, Box<dyn std::error::Error>> {
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