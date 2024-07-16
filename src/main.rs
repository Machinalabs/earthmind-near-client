use near_jsonrpc_client::JsonRpcClient;
use std::sync::{Arc, Mutex};

mod block_streamer;
mod constants;
mod database;

use block_streamer::{fetch_block, specify_block_reference};
use constants::DB_PATH;
use database::{init_db, load_last_processed_block, save_last_processed_block};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open RocksDB connection
    let db = Arc::new(Mutex::new(init_db(DB_PATH)?));

    // Load the last processed block
    let last_processed_block = load_last_processed_block(&db)?;
    println!("Last processed block: {}", last_processed_block);

    // Connect to the RPC client
    let client = JsonRpcClient::connect("https://rpc.testnet.near.org");

    // Determine the block reference to fetch
    let block_reference = specify_block_reference(last_processed_block);

    // Fetch and process the block
    let block = fetch_block(&client, block_reference).await?;
    println!("Processing block: {:#?}", block.header.height);

    // TODO: Implement your processing logic here

    let new_block_height = block.header.height;

    // Save the new block height as the last processed block
    save_last_processed_block(&db, new_block_height)?;
    println!("Saved new block height: {}", new_block_height);

    Ok(())
}
