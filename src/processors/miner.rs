use std::sync::{Arc, Mutex};

use near_crypto::SecretKey;
use near_jsonrpc_client::JsonRpcClient;
use near_sdk::AccountId;

use crate::models::EventData;
use crate::processors::TransactionProcessor;

pub struct Miner {
    client: &JsonRpcClient,
    db: &Arc<Mutex<rocksdb::DB>>,
    account_id: AccountId,
    secret_key: SecretKey,
}

impl Miner {
    pub fn new(
        client: &JsonRpcClient,
        db: &Arc<Mutex<rocksdb::DB>>,
        account_id: AccountId,
        secret_key: SecretKey,
    ) -> Self {
        Self {
            client,
            db,
            account_id,
            secret_key,
        }
    }
}

#[async_trait]
impl TransactionProcessor for Miner {
    async fn process_transaction(
        &self,
        event_data: EventData,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Implementación específica para Miner
        // Usa self.answer cuando sea necesario
        println!("Miner Processor");
    }
}

// TODO: Incluir la siguiente function en el trait TransactionProcessor, porque ambos hacen commit reveal...
// async fn commit_by_miner(
//     client: &JsonRpcClient,
//     signer: &InMemorySigner,
//     request_id: String,
//     answer: String,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let access_key_query_response = client
//         .call(methods::query::RpcQueryRequest {
//             block_reference: BlockReference::latest(),
//             request: near_primitives::views::QueryRequest::ViewAccessKey {
//                 account_id: signer.account_id.clone(),
//                 public_key: signer.public_key.clone(),
//             },
//         })
//         .await?;

//     let current_nonce = match access_key_query_response.kind {
//         QueryResponseKind::AccessKey(access_key) => access_key.nonce,
//         _ => Err("Failed to extract current nonce")?,
//     };

//     let transaction = Transaction {
//         signer_id: signer.account_id.clone(),
//         public_key: signer.public_key.clone(),
//         nonce: current_nonce + 1,
//         receiver_id: "earthmindprotocol.testnet".parse()?,
//         block_hash: access_key_query_response.block_hash,
//         actions: vec![Action::FunctionCall(Box::new(FunctionCallAction {
//             method_name: "commit_by_miner".to_string(),
//             args: serde_json::json!({
//                 "request_id" : request_id,
//                 "answer" : answer,
//             })
//             .to_string()
//             .into_bytes(),
//             gas: 100_000_000_000_000,
//             deposit: 0,
//         }))],
//     };

//     let tx_hash = transaction.get_hash_and_size().0;

//     let request = methods::send_tx::RpcSendTransactionRequest {
//         signed_transaction: transaction.sign(signer),
//         wait_until: TxExecutionStatus::Final,
//     };

//     let sent_at = time::Instant::now();
//     let response = match client.call(request).await {
//         Ok(response) => response,
//         Err(err) => {
//             match err.handler_error() {
//                 Some(RpcTransactionError::TimeoutError) => {}
//                 _ => Err(err)?,
//             }
//             loop {
//                 let response = client
//                     .call(methods::tx::RpcTransactionStatusRequest {
//                         transaction_info: TransactionInfo::TransactionId {
//                             tx_hash,
//                             sender_account_id: signer.account_id.clone(),
//                         },
//                         wait_until: TxExecutionStatus::Final,
//                     })
//                     .await;
//                 let received_at = time::Instant::now();
//                 let delta = (received_at - sent_at).as_secs();

//                 if delta > 60 {
//                     Err("time limit exceeded for the transaction to be recognized")?;
//                 }

//                 match response {
//                     Err(err) => match err.handler_error() {
//                         Some(RpcTransactionError::TimeoutError) => {}
//                         _ => Err(err)?,
//                     },
//                     Ok(response) => {
//                         break response;
//                     }
//                 }
//             }
//         }
//     };

//     let received_at = time::Instant::now();
//     let delta = (received_at - sent_at).as_secs();
//     println!("response gotten after: {}s", delta);
//     println!("response: {:#?}", response);

//     Ok(())
// }
