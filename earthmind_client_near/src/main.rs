use anyhow::Result;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_primitives::types::{BlockReference, Finality, FunctionArgs, StateChanges, StateChangesExt};
use near_primitives::views::{QueryRequest, StateChangesRequestView};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
// @dev use it to transactions to mainnet
use near_jsonrpc_primitives::types::transactions::TransactionInfo;
use near_primitives::hash::CryptoHash;
use near_primitives::views::TxExecutionStatus;
use near_workspaces::AccountId;
use serde_json::json;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    // @dev request server status from testnet RPC
     let client = JsonRpcClient::connect("https://rpc.testnet.near.org");

    // let request = methods::status::RpcStatusRequest; // no params

    // // call a method on the server via the connected client
    // let response = client.call(request).await?;

    // println!("RESPONSE: {:?}", response);

    // assert!(matches!(
    //     response,
    //     methods::status::RpcStatusResponse { .. }
    // ));

    // let block_reference = near_primitives::types::BlockReference::BlockId(near_primitives::types::BlockId::Height(168739614));

    // // @dev request information about hasserualcala.testnet
    // let request_account_info = methods::query::RpcQueryRequest {
    //     block_reference,
    //     request: QueryRequest::ViewAccount {
    //         account_id: "hasselalcalag.testnet".parse()?,
    //     },
    // };

    // let response_account_info = client.call(request_account_info).await?;

    //  println!("\n\nACCOUNT INFO RESPONSE: {:?}\n\n", response_account_info);

    //@dev Deploy the contract greeting
    //Tx_Hash_hello_world_contract:  Dik7apKS65UCthCGUuYgRxKnFyUtdx1EWHF6eRD7pMM4
    // calling the method get_greeting()

    // let request_call_method = methods::query::RpcQueryRequest {
    //     block_reference: BlockReference::latest(),
    //     request: QueryRequest::CallFunction {
    //         account_id: "halcalag.testnet".parse()?,
    //         method_name: "get_greeting".parse()?,
    //         args: FunctionArgs::from(json!({}).to_string().into_bytes())
    //     },
    // };

    // let response_call_method = client.call(request_call_method).await?;

    // // Verificar y extraer el valor del result
    // if let QueryResponseKind::CallResult(call_result) = response_call_method.kind {
    //     let result_bytes = call_result.result;
    //     if let Ok(result_string) = String::from_utf8(result_bytes) {
    //         println!("CALL METHOD GET GREETING: {:?}\n\n", result_string);
    //     } else {
    //         println!("Failed to convert result to string");
    //     }
    // } else {
    //     println!("Unexpected response kind");
    // }
    
    // @dev query transaction status from mainnet RPC
    let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");

    //let request = methods::status::RpcStatusRequest;
    //let response = client.call(request).await?;

    //println!("\n\nConnection to TESTNET: {:?}", response);

    let tx_hash = CryptoHash::from_str("3jnfi85GhTcX3ZdPxF2EtS7vQddAwzAAooGwmffK5fgD")
        .map_err(|e| anyhow::anyhow!("Failed to parse transaction hash: {}", e))?;

    let sender_account_id: AccountId = "halcalag.testnet"
        .parse()
        .map_err(|e| anyhow::anyhow!("Failed to parse sender account ID: {}", e))?;

    let tx_status_request = methods::tx::RpcTransactionStatusRequest {
        transaction_info: TransactionInfo::TransactionId {
            tx_hash,
            sender_account_id,
        },
        wait_until: TxExecutionStatus::Executed,
    };

    let tx_status = client
        .call(tx_status_request)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch transaction status: {}", e))?;

    println!("TRANSACTION STATUS: {:?}", tx_status);

    // let block_reference = near_primitives::types::BlockReference::BlockId(
    //     near_primitives::types::BlockId::Height(168739614));

    //     match client
    //     .call(methods::block::RpcBlockRequest { block_reference })
    //     .await
    // {
    //     Ok(block_details) => println!("\nBLOCK DETAILS: {:#?}", block_details),
    //     Err(err) => match err.handler_error() {
    //         Some(methods::block::RpcBlockError::UnknownBlock { .. }) => {
    //             println!("(i) Unknown block!");
    //         }
    //         Some(err) => {
    //             println!("(i) An error occurred `{:#?}`", err);
    //         }
    //         _ => println!("(i) A non-handler error ocurred `{:#?}`", err),
    //     },
    // };


    let tx_hash = CryptoHash::from_str("3xn4mDr5v5H2AG8EBRvZHFc6VqKRKcWufUaRwK2pZCqs")
        .map_err(|e| anyhow::anyhow!("Failed to parse transaction hash: {}", e))?;

    let sender_account_id: AccountId = "hasselalcalag.testnet"
        .parse()
        .map_err(|e| anyhow::anyhow!("Failed to parse sender account ID: {}", e))?;

    match client
            .call(methods::tx::RpcTransactionStatusRequest {
                transaction_info: methods::tx::TransactionInfo::TransactionId {
                    tx_hash,
                    sender_account_id : sender_account_id.clone(),
                },
                wait_until: TxExecutionStatus::Final,
            })
            .await
        {
            Ok(tx_details) => println!("TRANSACTION {:#?}", tx_details),
            Err(err) => match err.handler_error() {
                Some(err) => {
                    println!("(i) An error occurred `{:#?}`", err);
                    
                }
                _ => println!("(i) A non-handler error occurred `{:#?}`", err),
            },
        };
        
        let block_reference = near_primitives::types::BlockReference::BlockId(near_primitives::types::BlockId::Height(168739614));
        let result_changes = client.call(methods::EXPERIMENTAL_changes::RpcStateChangesInBlockByTypeRequest {
            block_reference,
            state_changes_request: StateChangesRequestView::AccountChanges { account_ids: vec![sender_account_id] }
        }).await; 

        println!("RESULT CHANGES {:?}",result_changes);

    Ok(())
}
