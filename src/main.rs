use clap::Parser;
//use near_jsonrpc_client::{methods, JsonRpcClient};
use std::sync::{Arc, Mutex};
//use tokio::time::{sleep, Duration};

mod block_streamer;
mod cli;
mod constants;
mod database;

//use block_streamer::{fetch_block, find_transaction_in_block, specify_block_reference};
//use constants::{ACCOUNT_TO_LISTEN, DB_PATH, FUNCTION_TO_LISTEN, NEAR_RPC_URL};
//use database::{init_db, load_last_processed_block, save_last_processed_block};
use block_streamer::run_mode;
use cli::{Cli, Modes};
use constants::{DB_PATH, NEAR_RPC_URL};
use database::init_db;
use near_jsonrpc_client::JsonRpcClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Open RocksDB connection
    let db = Arc::new(Mutex::new(init_db(DB_PATH)?));

    // Connect to the RPC client
    let client = JsonRpcClient::connect(NEAR_RPC_URL);

    match cli.mode {
        Modes::Miner => {
            run_mode(&client, &db, process_miner_transaction).await?;
        }
        Modes::Validator => {
            run_mode(&client, &db, process_validator_transaction).await?;
        }
    }

    Ok(())
}

fn process_miner_transaction(client: &JsonRpcClient) -> Result<bool, Box<dyn std::error::Error>> {
    // Implement your miner-specific logic here
    println!("Process as miner");
    Ok(false)
}

fn process_validator_transaction(
    client: &JsonRpcClient,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Implement your validator-specific logic here
    println!("Process as validator");
    Ok(false)
}
