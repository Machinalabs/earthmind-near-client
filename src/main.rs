use std::sync::Arc;
use tokio::sync::Mutex;

use clap::Parser;
use near_crypto::InMemorySigner;
use near_jsonrpc_client::JsonRpcClient;

mod block_streamer;
mod cli;
mod clients;
mod constants;
mod database;
mod models;
mod nonce_manager;
mod processors;
mod qx_builder;
mod qx_sender;
mod tx_builder;
mod tx_sender;

use block_streamer::start_polling;
use cli::{Cli, Modes, Networks};
use constants::{DB_PATH, DEFAULT_TIMEOUT, NEAR_RPC_MAINNET, NEAR_RPC_TESTNET};
use database::init_db;
use nonce_manager::NonceManager;
use tx_builder::TxBuilder;
use tx_sender::TxSender;

use crate::processors::{Aggregator, Miner, TransactionProcessor, Validator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Open RocksDB connection
    let db = Arc::new(Mutex::new(init_db(DB_PATH)?));

    // Connect to the RPC client
    let client = Arc::new(match cli.network {
        Networks::Mainnet => JsonRpcClient::connect(NEAR_RPC_MAINNET),
        Networks::Testnet => JsonRpcClient::connect(NEAR_RPC_TESTNET),
    });

    // Create signer
    let signer = InMemorySigner::from_secret_key(cli.account_id.clone(), cli.private_key.clone());

    // Initialize components
    let nonce_manager = Arc::new(NonceManager::new(client.clone(), Arc::new(signer.clone())));
    let tx_builder = Arc::new(Mutex::new(TxBuilder::new(signer, cli.network)));
    let tx_sender = Arc::new(TxSender::new(client.clone(), DEFAULT_TIMEOUT));

    // Create the processor based on the mode
    let processor: Arc<dyn TransactionProcessor> = match cli.mode {
        Modes::Miner => Arc::new(Miner::new(
            nonce_manager.clone(),
            tx_builder.clone(),
            tx_sender.clone(),
            db.clone(),
            cli.account_id.clone(),
        )),
        Modes::Validator => Arc::new(Validator::new(
            nonce_manager.clone(),
            tx_builder.clone(),
            tx_sender.clone(),
            db.clone(),
            cli.account_id.clone(),
        )),
        Modes::Aggregator => Arc::new(Aggregator::new(
            nonce_manager.clone(),
            tx_builder.clone(),
            tx_sender.clone(),
            db.clone(),
            cli.account_id,
        )),
    };

    start_polling(&client, &db, processor).await?;
    Ok(())
}
