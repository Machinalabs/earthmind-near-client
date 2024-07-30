use crate::database::Database;
use crate::models::EventData;
use crate::nonce_manager::NonceManager;
use crate::tx_builder::TxBuilder;
use crate::tx_sender::TxSender;

use async_trait::async_trait;
use near_sdk::AccountId;
use std::sync::Arc;

pub struct Validator {
    nonce_manager: Arc<NonceManager>,
    tx_builder: Arc<TxBuilder>,
    tx_sender: Arc<TxSender>,
    db: Arc<Database>,
    account_id: AccountId,
}

impl Validator {
    pub fn new(
        nonce_manager: Arc<NonceManager>,
        tx_builder: Arc<TxBuilder>,
        tx_sender: Arc<TxSender>,
        db: Arc<Database>,
        account_id: AccountId,
    ) -> Self {
        Self {
            nonce_manager,
            tx_builder,
            tx_sender,
            db,
            account_id,
        }
    }
}

#[async_trait]
impl TransactionProcessor for Validator {
    async fn process_transaction(
        &self,
        event_data: EventData,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Implementación específica para Validator
        println!("Validator Processor");
        println!("Event Data: {:?}", event_data);
        Ok(true) // o el valor que corresponda
    }

    // async fn commit(
    //     &self,
    //     event_data: EventData,
    // ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //     println!("Validator Commit");

    //     let answer = self.get_answer();

    //     let (current_nonce, block_hash) = get_nonce_and_tx_hash(&self.client, &self.signer).await?;

    //     let (transaction, tx_hash) = build_commit_transaction(
    //         &self.signer,
    //         event_data.request_id.clone(),
    //         answer.clone(),
    //         current_nonce,
    //         block_hash,
    //     );

    //     let request = methods::send_tx::RpcSendTransactionRequest {
    //         signed_transaction: transaction.sign(signer),
    //         wait_until: TxExecutionStatus::Final,
    //     };

    //     let commit_validator_result =
    //         send_transaction(&self.client, request, tx_hash, self.signer).await?;

    //     match commit_validator_result {
    //         Ok(_) => println!(
    //             "Commit by validator successful for request_id: {}",
    //             event_data.request_id.clone()
    //         ),
    //         Err(e) => println!("Failed to commit by validator: {}", e),
    //     }
    //     Ok(())
    // }

    // async fn reveal(
    //     &self,
    //     event_data: EventData,
    // ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //     let (current_nonce, block_hash) = get_nonce_and_tx_hash(client, signer).await?;

    //     let answer = generate_validator_answer();

    //     let message = "The best miners".to_string();

    //     let (transaction, tx_hash) = build_reveal_validator_transaction(
    //         signer,
    //         request_id,
    //         answer,
    //         message,
    //         current_nonce,
    //         block_hash,
    //     );

    //     let request = methods::send_tx::RpcSendTransactionRequest {
    //         signed_transaction: transaction.sign(signer),
    //         wait_until: TxExecutionStatus::Final,
    //     };

    //     let reveal_validator_result = send_transaction(client, request, tx_hash, signer).await?;

    //     match reveal_validator_result {
    //         Ok(_) => println!(
    //             "Reveal by validator successful for request_id: {}",
    //             event_data.request_id.clone()
    //         ),
    //         Err(e) => println!("Failed to reveal by validator: {}", e),
    //     }
    // }
}
