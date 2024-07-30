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
