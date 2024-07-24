use clap::Parser;
use std::sync::{Arc, Mutex};

mod block_streamer;
mod cli;
mod constants;
mod database;
mod models;
mod processors;

use block_streamer::start_polling;
use cli::{Cli, Modes, Networks};
use constants::{DB_PATH, NEAR_RPC_MAINNET, NEAR_RPC_TESTNET};
use database::init_db;
use near_jsonrpc_client::JsonRpcClient;

use crate::processors::{Miner, TransactionProcessor, Validator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Open RocksDB connection
    let db = Arc::new(Mutex::new(init_db(DB_PATH)?));

    // Connect to the RPC client
    let client: Arc<JsonRpcClient>;

    match cli.network {
        Networks::Mainnet => {
            client = Arc::new(JsonRpcClient::connect(NEAR_RPC_MAINNET));
        }
        Networks::Testnet => {
            client = Arc::new(JsonRpcClient::connect(NEAR_RPC_TESTNET));
        }
    }

    // Create the processor based on the mode (passed as argument to the CLI)
    let processor: Arc<dyn TransactionProcessor> = match cli.mode {
        Modes::Miner => Arc::new(Miner::new(
            client.clone(),
            db.clone(),
            cli.account_id,
            cli.private_key,
        )),
        Modes::Validator => Arc::new(Validator::new(
            client.clone(),
            db.clone(),
            cli.account_id,
            cli.private_key,
        )),
    };

    start_polling(&client, &db, processor).await?;

    Ok(())
}

// callbacks -> function -> regresa una function que recibe el answer -> y asi evitar pasar por todos lados el answer

// async fn process_miner_transaction(
//     client: &JsonRpcClient,
//     logs: Vec<String>,
//     account_id: AccountId,
//     key: SecretKey,
// ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
//     for log in logs {
//         println!("Miner Log: {}", log);
//         match process_log(&log) {
//             Ok(event) => {
//                 for data in event.data {
//                     println!("Request ID: {}", data.request_id);
//                     println!("Start Time: {}", data.start_time);
//                     println!("Reveal Miner Time: {}", data.reveal_miner_time);
//                     println!("Commit Miner Time: {}", data.commit_miner_time);
//                     println!("Reveal Validator Time: {}", data.reveal_validator_time);
//                     println!("Commit Validator Time: {}", data.commit_validator_time);

//                     //let signer_account = AccountId::from_str(&account_id).map_err(|e| format!("Failed to parse sender_account_id: {}", e))?;
//                     //let signer_private_key = key;
//                     let signer = near_crypto::InMemorySigner::from_secret_key(
//                         account_id.clone(),
//                         key.clone(),
//                     );

//                     let commit_miner_result =
//                         commit_by_miner(client, &signer, data.request_id.clone(), answer.clone())
//                             .await;

//                     match commit_miner_result {
//                         Ok(_) => println!(
//                             "Commit by miner successful for request_id: {}",
//                             data.request_id
//                         ),
//                         Err(e) => println!("Failed to commit by miner: {}", e),
//                     }
//                 }
//             }
//             Err(e) => println!("Failed to process log: {}", e),
//         }
//     }
//     Ok(false)
// }

// async fn process_validator_transaction(
//     client: &JsonRpcClient,
//     logs: Vec<String>,
//     account: AccountId,
//     key: SecretKey,
// ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
//     for log in logs {
//         println!("Validator Log: {}", log);
//         match process_log(&log) {
//             Ok(event) => {
//                 for data in event.data {
//                     println!("Request ID: {}", data.request_id);
//                     println!("Start Time: {}", data.start_time);
//                     println!("Reveal Miner Time: {}", data.reveal_miner_time);
//                     println!("Commit Miner Time: {}", data.commit_miner_time);
//                     println!("Reveal Validator Time: {}", data.reveal_validator_time);
//                     println!("Commit Validator Time: {}", data.commit_validator_time);
//                 }
//             }
//             Err(e) => println!("Failed to process log: {}", e),
//         }
//     }
//     Ok(false)
// }

// TODO: Mover esto a block streamer para que deserilize el log y lo pase a los processors como EventData....

// fn process_log(log: &str) -> Result<EventJson, Box<dyn std::error::Error>> {
//     // Remove "EVENT_JSON:" prefix
//     let json_part = log.trim_start_matches("EVENT_JSON:");

//     // Deserialize from JSON to EventJson struct
//     let event: EventJson = serde_json::from_str(json_part)?;

//     Ok(event)
// }
