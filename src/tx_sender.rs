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
