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
