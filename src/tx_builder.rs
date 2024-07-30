use near_crypto::InMemorySigner;
use near_jsonrpc_client::JsonRpcClient;
use near_primitives::{
    action::{Action, FunctionCallAction},
    types::{CryptoHash, Transaction},
};
use near_sdk::AccountId;

use crate::{cli::Networks, constants::*};

pub struct TxBuilder {
    signer: Arc<InMemorySigner>,
    network: Networks,
}

impl TxBuilder {
    pub fn new(signer: Arc<InMemorySigner>, network: Networks) -> Self {
        Self { signer, network }
    }

    pub fn build_transaction(
        &self,
        actions: Vec<Action>,
        nonce: u64,
        block_hash: CryptoHash,
    ) -> (Transaction, CryptoHash) {
        let transaction = Transaction {
            signer_id: self.signer.account_id.clone(),
            public_key: self.signer.public_key.clone(),
            nonce: nonce,
            receiver_id: self.get_receiver_id(),
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

    fn get_receiver_id(&self) -> AccountId {
        match self.network {
            Networks::Mainnet => EARTHMIND_PROTOCOL_CONTRACT_MAINNET.parse().unwrap(),
            Networks::Testnet => EARTHMIND_PROTOCOL_CONTRACT_TESTNET.parse().unwrap(),
        }
    }
}

// builder => withMethodName
// builder => withArgs

// methodNames: commit_by_miner, reveal_by_miner, reveal_by_validator, commit_by_validator
// solo cambia:
// method_name: "commit_by_miner".to_string(),
//             args: serde_json::json!({
//                 "request_id": request_id,
//                 "answer": answer,
//             })

// luego
// method_name: "reveal_by_miner".to_string(),
//             args: serde_json::json!({
//                 "request_id": request_id,
//                 "answer": answer,
//                 "message":message,
//             })

// luego
// method_name: "reveal_by_miner".to_string(),
// args: serde_json::json!({
//     "request_id": request_id,
//     "answer": answer,
//     "message":message,
// })
