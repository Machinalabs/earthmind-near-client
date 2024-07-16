use near_jsonrpc_client::{methods, JsonRpcClient};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

mod block_streamer;
mod constants;
mod database;

use block_streamer::{fetch_block, find_transaction_in_block, specify_block_reference};
use constants::{ACCOUNT_TO_LISTEN, DB_PATH, FUNCTION_TO_LISTEN, NEAR_RPC_URL};
use database::{init_db, load_last_processed_block, save_last_processed_block};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open RocksDB connection
    let db = Arc::new(Mutex::new(init_db(DB_PATH)?));

    // Load the last processed block
    let last_processed_block = load_last_processed_block(&db)?;
    println!("Last processed block: {}", last_processed_block);

    // Connect to the RPC client
    let client = JsonRpcClient::connect(NEAR_RPC_URL);

    loop {
        // Load the last processed block
        let last_processed_block = load_last_processed_block(&db)?;
        println!("Last processed block: {}", last_processed_block);

        // Determine the block reference to fetch
        let block_reference = specify_block_reference(last_processed_block);

        // Fetch the block
        match fetch_block(&client, block_reference).await {
            Ok(block) => {
                println!("Processing block: {:#?}", block.header.height);

                // Check if the block contains the transaction of interest
                if find_transaction_in_block(&client, &block, ACCOUNT_TO_LISTEN, FUNCTION_TO_LISTEN)
                    .await?
                {
                    println!("Found transaction in block: {}", block.header.height);
                    // TODO: Implement your logic here to handle the found transaction
                }

                // Save the new block height as the last processed block
                let new_block_height = block.header.height;
                save_last_processed_block(&db, new_block_height)?;
                println!("Saved new block height: {}", new_block_height);
            }
            Err(err) => match err.handler_error() {
                // Handle unknown block error
                Some(methods::block::RpcBlockError::UnknownBlock { .. }) => {
                    println!("(i) Unknown block!");

                    // We skip the unknown block and save the block height of the unknown block as the last processed block
                    let new_block_height = last_processed_block + 1;
                    save_last_processed_block(&db, new_block_height)?;
                    println!("Saved new block height: {}", new_block_height);
                }
                // Handle other handled errors
                Some(err) => {
                    println!("(i) An error occurred `{:#?}`", err);
                    panic!("Other error!");
                }
                // Non handled errors
                _ => {
                    println!("(i) A non-handler error occurred `{:#?}`", err);
                    panic!("Non handled error!");
                }
            },
        }

        // Sleep for a short duration before checking for the next block
        sleep(Duration::from_secs(2)).await;
    }
}
