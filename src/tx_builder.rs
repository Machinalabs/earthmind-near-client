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

pub fn build_reveal_validator_transaction(
    signer: &InMemorySigner,
    request_id: String,
    answer: Vec<AccountId>,
    message: String,
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
            method_name: "reveal_by_miner".to_string(),
            args: serde_json::json!({
                "request_id": request_id,
                "answer": answer,
                "message":message,
            })
            .to_string()
            .into_bytes(),
            gas: 100_000_000_000_000,
            deposit: 0,
        }))],
    };

    (transaction.clone(), transaction.get_hash_and_size().0)
}

pub fn build_reveal_miner_transaction(
    signer: &InMemorySigner,
    request_id: String,
    answer: bool,
    message: String,
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
            method_name: "reveal_by_miner".to_string(),
            args: serde_json::json!({
                "request_id": request_id,
                "answer": answer,
                "message":message,
            })
            .to_string()
            .into_bytes(),
            gas: 100_000_000_000_000,
            deposit: 0,
        }))],
    };

    (transaction.clone(), transaction.get_hash_and_size().0)
}
