use near_jsonrpc_client::{methods, JsonRpcClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // @dev Open RocksDB connection
    let db = Arc::new(Mutex::new(init_db(DB_PATH)?));

    // @dev Load the last processed block from RocksDB
    let mut last_processed_block = load_last_processed_block(&db)?;

    // @dev Connect to the RPC client
    let client = JsonRpcClient::connect("https://rpc.testnet.near.org");

    // // @dev Tolerate only 3 retries
    // for _ in 1..=3 {
    //     let block_reference = specify_block_reference()?;

    //     // @dev Make an RPC call to get block details
    //     match client
    //         .call(methods::block::RpcBlockRequest { block_reference })
    //         .await
    //     {
    //         // @dev Print block details if the call is successful
    //         Ok(block_details) => {
    //             println!("{:#?}", block_details);
    //             break;
    //         }
    //         // @dev Handle errors appropriately
    //         Err(err) => match err.handler_error() {
    //             // @dev Handle unknown block error
    //             Some(methods::block::RpcBlockError::UnknownBlock { .. }) => {
    //                 println!("(i) Unknown block!");
    //                 continue;
    //             }
    //             // @dev Handle other handler errors
    //             Some(err) => {
    //                 println!("(i) An error occurred `{:#?}`", err);
    //                 continue;
    //             }
    //             // @dev Handle non-handler errors
    //             _ => println!("(i) A non-handler error occurred `{:#?}`", err),
    //         },
    //     };
    // }

    Ok(())
}
