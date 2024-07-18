use near_jsonrpc_client::errors::JsonRpcError;
use near_jsonrpc_client::methods::block::RpcBlockError;
use near_jsonrpc_client::methods::chunk::ChunkReference;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_primitives::hash::CryptoHash;
use near_primitives::types::{BlockId, BlockReference, Finality};
use near_primitives::views::{ActionView, BlockView, ChunkView};

use crate::constants::*;
use crate::database::{load_last_processed_block, save_last_processed_block};
use std::sync::{Arc, Mutex};

pub fn specify_block_reference(last_processed_block: u64) -> BlockReference {
    if last_processed_block == 0 {
        BlockReference::Finality(Finality::Final)
    } else {
        BlockReference::BlockId(BlockId::Height(last_processed_block + 1))
    }
}

pub async fn fetch_block(
    client: &JsonRpcClient,
    block_reference: BlockReference,
) -> Result<BlockView, JsonRpcError<RpcBlockError>> {
    let block_request = methods::block::RpcBlockRequest { block_reference };
    let block_response = client.call(block_request).await;
    block_response
}

pub async fn fetch_chunk(
    client: &JsonRpcClient,
    chunk_hash: CryptoHash,
) -> Result<ChunkView, Box<dyn std::error::Error>> {
    let chunk_reference = ChunkReference::ChunkHash {
        chunk_id: chunk_hash,
    };

    let chunk_request = methods::chunk::RpcChunkRequest {
        chunk_reference: chunk_reference,
    };

    let chunk_response = client.call(chunk_request).await;

    match chunk_response {
        Ok(chunk_details) => {
            // println!("{:#?}", chunk_details);
            Ok(chunk_details)
        }
        Err(err) => match err.handler_error() {
            // Handle unknown chunk error
            Some(methods::chunk::RpcChunkError::UnknownChunk { .. }) => {
                println!("(i) Unknown chunk!");
                panic!("Unknown chunk!");
            }
            // Handle other handler errors
            Some(err) => {
                println!("(i) An error occurred `{:#?}`", err);
                panic!("Other error!");
            }
            // Handle non-handler errors
            _ => {
                println!("(i) A non-handler error occurred `{:#?}`", err);
                panic!("Non handled error!");
            }
        },
    }
}

pub async fn find_transaction_in_block(
    client: &JsonRpcClient,
    block: &BlockView,
    account_id: &str,
    method_name: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    for chunk_header in &block.chunks {
        let chunk_hash = chunk_header.chunk_hash;
        let chunk = fetch_chunk(client, chunk_hash).await?;

        for transaction in &chunk.transactions {
            if transaction.receiver_id == account_id {
                for action in &transaction.actions {
                    if let ActionView::FunctionCall {
                        method_name: action_method_name,
                        ..
                    } = action
                    {
                        if action_method_name == method_name {
                            return Ok(true);
                        }
                    }
                }
            }
        }
    }
    Ok(false)
}

pub async fn run_mode<F>(
    client: &JsonRpcClient,
    db: &Arc<Mutex<rocksdb::DB>>,
    process_transaction: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&JsonRpcClient) -> Result<bool, Box<dyn std::error::Error>>
        + Send
        + Sync,
{
    loop {
        let last_processed_block = load_last_processed_block(db)?;
        println!("Last processed block: {}", last_processed_block);

        let block_reference = specify_block_reference(last_processed_block);
        match fetch_block(client, block_reference).await {
            Ok(block) => {
                println!("Processing block: {:#?}", block.header.height);

                // Check if the block contains the transaction of interest
                if find_transaction_in_block(&client, &block, ACCOUNT_TO_LISTEN, FUNCTION_TO_LISTEN)
                    .await?
                {
                    println!("Found transaction in block: {}", block.header.height);
                    // TODO: Implement your logic here to handle the found transaction
                    let _ = process_transaction(client);
                }

                // Save the new block height as the last processed block
                let new_block_height = block.header.height;
                save_last_processed_block(db, new_block_height)?;
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
                } // Handle other handled errors
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

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}
