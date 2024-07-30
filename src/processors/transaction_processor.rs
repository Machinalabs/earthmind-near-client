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

    async fn reveal(
        &self,
        event_data: EventData,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
