use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use near_crypto::SecretKey;
use near_jsonrpc_client::JsonRpcClient;
use near_sdk::AccountId;

use crate::models::EventData;
use crate::processors::TransactionProcessor;

pub struct Miner {
    client: Arc<JsonRpcClient>,
    db: Arc<Mutex<rocksdb::DB>>,
    account_id: AccountId,
    secret_key: SecretKey,
    signer: near_crypto::InMemorySigner,
}

impl Miner {
    pub fn new(
        client: Arc<JsonRpcClient>,
        db: Arc<Mutex<rocksdb::DB>>,
        account_id: AccountId,
        secret_key: SecretKey,
    ) -> Self {
        let signer = near_crypto::InMemorySigner::from_secret_key(account_id.clone(), key.clone());

        Self {
            client,
            db,
            account_id,
            secret_key,
            signer,
        }
    }
}

#[async_trait]
impl TransactionProcessor for Miner {
    async fn process_transaction(
        &self,
        event_data: EventData,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        println!("Miner Processor");
        println!("Event Data: {:?}", event_data);
        Ok(true)
    }

    async fn commit(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Miner Commit");

        let commit_miner_result =
            commit_by_miner(client, &signer, data.request_id.clone(), answer.clone()).await;
        
        // TODO: ya aqui puedes acceder al signer via self.signer 
        match commit_miner_result {
            Ok(_) => println!(
                "Commit by miner successful for request_id: {}",
                data.request_id
            ),
            Err(e) => println!("Failed to commit by miner: {}", e),
        }
    }

    async fn reveal(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Miner Reveal");
        Ok(())
    }
}

async fn commit_by_miner(
    client: &JsonRpcClient,
    signer: &InMemorySigner,
    request_id: String,
    answer: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO abstraer en get_nonce
    let access_key_query_response = client
        .call(methods::query::RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
            },
        })
        .await?;

    let current_nonce = match access_key_query_response.kind {
        QueryResponseKind::AccessKey(access_key) => access_key.nonce,
        _ => Err("Failed to extract current nonce")?,
    };

    // TODO Abstraer transaction builder
    const (txhash, transaction) = transactionBuilder.build(asd, asd, asd);

    let request = methods::send_tx::RpcSendTransactionRequest {
        signed_transaction: transaction.sign(signer),
        wait_until: TxExecutionStatus::Final,
    };

    // TODO abstraer send tx
    send_tx(client, request, signer).await?;

    let sent_at = time::Instant::now();
    let response = match client.call(request).await {
        Ok(response) => response,
        Err(err) => {
            match err.handler_error() {
                Some(RpcTransactionError::TimeoutError) => {}
                _ => Err(err)?,
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
                    Err("time limit exceeded for the transaction to be recognized")?;
                }

                match response {
                    Err(err) => match err.handler_error() {
                        Some(RpcTransactionError::TimeoutError) => {}
                        _ => Err(err)?,
                    },
                    Ok(response) => {
                        break response;
                    }
                }
            }
        }
    };

    let received_at = time::Instant::now();
    let delta = (received_at - sent_at).as_secs();
    println!("response gotten after: {}s", delta);
    println!("response: {:#?}", response);

    Ok(())
}
