use std::sync::Arc;

use near_crypto::InMemorySigner;
use near_jsonrpc_client::methods;
use near_jsonrpc_client::JsonRpcClient;
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::hash::CryptoHash;
use near_primitives::types::BlockReference;
use near_primitives::views::QueryRequest;

pub struct NonceManager {
    client: Arc<JsonRpcClient>,
    signer: Arc<InMemorySigner>,
}

impl NonceManager {
    pub fn new(client: Arc<JsonRpcClient>, signer: Arc<InMemorySigner>) -> Self {
        Self { client, signer }
    }

    pub async fn get_nonce_and_tx_hash(
        &self,
    ) -> Result<(u64, CryptoHash), Box<dyn std::error::Error + Send + Sync>> {
        let access_key_query_response = self
            .client
            .call(methods::query::RpcQueryRequest {
                block_reference: BlockReference::latest(),
                request: QueryRequest::ViewAccessKey {
                    account_id: self.signer.account_id.clone(),
                    public_key: self.signer.public_key.clone(),
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
}
