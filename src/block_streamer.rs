use near_jsonrpc_client::{methods, JsonRpcClient};
use near_primitives::types::{BlockReference, Finality};

// BLOCK STREAMER

// @dev This function specifies the block reference to always be the final block
pub fn specify_block_reference() -> std::io::Result<BlockReference> {
    Ok(BlockReference::Finality(Finality::Final))
}
