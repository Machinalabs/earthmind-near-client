use near_jsonrpc_client::{methods, JsonRpcClient};
use near_primitives::types::{BlockId, BlockReference, Finality};

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
) -> Result<near_primitives::views::BlockView, Box<dyn std::error::Error>> {
    let block_request = methods::block::RpcBlockRequest { block_reference };
    let block_response = client.call(block_request).await;

    match block_response {
        Ok(block_details) => {
            println!("{:#?}", block_details);
            Ok(block_details)
        }
        Err(err) => match err.handler_error() {
            // Handle unknown block error
            Some(methods::block::RpcBlockError::UnknownBlock { .. }) => {
                println!("(i) Unknown block!");
                panic!("Unknown block!");
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
