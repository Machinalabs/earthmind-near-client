use crate::nonce_manager::NonceManager;
use crate::qx_builder::QueryBuilder;
use crate::qx_sender::QuerySender;
use crate::tx_builder::TxBuilder;
use crate::tx_sender::TxSender;

use near_sdk::AccountId;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Aggregator {
    nonce_manager: Arc<NonceManager>,
    tx_builder: Arc<Mutex<TxBuilder>>,
    tx_sender: Arc<TxSender>,
    db: Arc<Mutex<rocksdb::DB>>,
    account_id: AccountId,
}