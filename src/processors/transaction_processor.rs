use crate::models::EventData;
use async_trait::async_trait;

use near_crypto::InMemorySigner;
use near_jsonrpc_client::{methods, JsonRpcClient};

use near_jsonrpc_primitives::types::{
    query::QueryResponseKind, transactions::RpcTransactionError, transactions::TransactionInfo,
};

use near_primitives::{
    action::Action, action::FunctionCallAction, hash::CryptoHash, transaction::Transaction,
    types::BlockReference, views::TxExecutionStatus,
};

use std::time;

#[async_trait]
pub trait TransactionProcessor: Send + Sync {
    async fn process_transaction(
        &self,
        event_data: EventData,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;

    async fn commit(
        &self,
        event_data: EventData,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn reveal(&self, event_data: EventData) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

pub async fn get_nonce_and_tx_hash(
    client: &JsonRpcClient,
    signer: &InMemorySigner,
) -> Result<(u64, CryptoHash), Box<dyn std::error::Error>> {
    let access_key_query_response = client
        .call(methods::query::RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
            },
        })
        .await?;

    match access_key_query_response.kind {
        QueryResponseKind::AccessKey(access_key) => {
            Ok((access_key.nonce, access_key_query_response.block_hash))
        }
        _ => Err("Failed to extract current nonce".into()),
    }
}

pub fn build_commit_transaction(
    signer: &InMemorySigner,
    request_id: String,
    answer: String,
    current_nonce: u64,
    block_hash: CryptoHash,
) -> (Transaction, CryptoHash) {
    let transaction = Transaction {
        signer_id: signer.account_id.clone(),
        public_key: signer.public_key.clone(),
        nonce: current_nonce + 1,
        receiver_id: "earthmindprotocol.testnet".parse().unwrap(),
        block_hash,
        actions: vec![Action::FunctionCall(Box::new(FunctionCallAction {
            method_name: "commit_by_miner".to_string(),
            args: serde_json::json!({
                "request_id": request_id,
                "answer": answer,
            })
            .to_string()
            .into_bytes(),
            gas: 100_000_000_000_000,
            deposit: 0,
        }))],
    };

    (transaction.clone(), transaction.get_hash_and_size().0)
}

pub async fn send_transaction(
    client: &JsonRpcClient,
    request: methods::send_tx::RpcSendTransactionRequest,
    tx_hash: CryptoHash,
    signer: &InMemorySigner,
) -> Result<
    near_jsonrpc_primitives::types::transactions::RpcTransactionResponse,
    Box<dyn std::error::Error>,
> {
    let sent_at = time::Instant::now();
    let response = match client.call(request.clone()).await {
        Ok(response) => response,
        Err(err) => {
            match err.handler_error() {
                Some(RpcTransactionError::TimeoutError) => {}
                _ => return Err(err.into()),
            }
            loop {
                let response = client
                    .call(methods::tx::RpcTransactionStatusRequest {
                        transaction_info: TransactionInfo::TransactionId {
                            tx_hash,
                            sender_account_id: signer.account_id.clone(),
                        },
                        wait_until: TxExecutionStatus::Final,
                    })
                    .await;
                let received_at = time::Instant::now();
                let delta = (received_at - sent_at).as_secs();

                if delta > 60 {
                    return Err("Time limit exceeded for the transaction to be recognized".into());
                }

                match response {
                    Err(err) => match err.handler_error() {
                        Some(RpcTransactionError::TimeoutError) => {}
                        _ => return Err(err.into()),
                    },
                    Ok(response) => break response,
                }
            }
        }
    };

    let received_at = time::Instant::now();
    let delta = (received_at - sent_at).as_secs();
    println!("Response received after: {}s", delta);
    println!("Response: {:#?}", response);

    Ok(response)
}
