use near_crypto::InMemorySigner;
use near_primitives::{
    action::{Action, FunctionCallAction},
    hash::CryptoHash,
    transaction::Transaction,
};
use near_sdk::AccountId;
use serde_json::Value;

use crate::{cli::Networks, constants::*};

pub struct TxBuilder {
    pub signer: InMemorySigner,
    network: Networks,
    method_name: String,
    args: Value,
    gas: u64,
    deposit: u128,
}

impl TxBuilder {
    pub fn new(signer: InMemorySigner, network: Networks) -> Self {
        Self {
            signer,
            network,
            method_name: String::new(),
            args: Value::Null,
            gas: 100_000_000_000_000,
            deposit: 0,
        }
    }

    pub fn with_method_name(&mut self, method_name: &str) -> &mut Self {
        self.method_name = method_name.to_string();
        self
    }

    pub fn with_args(&mut self, args: Value) -> &mut Self {
        self.args = args;
        self
    }

    pub fn with_gas(&mut self, gas: u64) -> &mut Self {
        self.gas = gas;
        self
    }

    pub fn with_deposit(&mut self, deposit: u128) -> &mut Self {
        self.deposit = deposit;
        self
    }

    pub fn build(&self, nonce: u64, block_hash: CryptoHash) -> (Transaction, CryptoHash) {
        let transaction = Transaction {
            signer_id: self.signer.account_id.clone(),
            public_key: self.signer.public_key.clone(),
            nonce,
            receiver_id: self.get_receiver_id(),
            block_hash,
            actions: vec![Action::FunctionCall(Box::new(FunctionCallAction {
                method_name: self.method_name.clone(),
                args: serde_json::to_vec(&self.args).unwrap(),
                gas: self.gas,
                deposit: self.deposit,
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
