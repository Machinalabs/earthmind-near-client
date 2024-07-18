use clap::Parser;
use std::sync::{Arc, Mutex};

mod block_streamer;
mod cli;
mod constants;
mod database;

use block_streamer::run_mode;
use cli::{Cli, Modes, Networks};
use constants::{DB_PATH, NEAR_RPC_TESTNET, NEAR_RPC_MAINNET};
use database::init_db;
use near_jsonrpc_client::JsonRpcClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Open RocksDB connection
    let db = Arc::new(Mutex::new(init_db(DB_PATH)?));

    // Connect to the RPC client
    let client : JsonRpcClient;

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
            run_mode(&client, &db, process_miner_transaction).await?;
        }
        Modes::Validator => {
            run_mode(&client, &db, process_validator_transaction).await?;
        }
    }

    //cli.private_key --> obtener near account de esta private key y debe ser obligatorio

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
