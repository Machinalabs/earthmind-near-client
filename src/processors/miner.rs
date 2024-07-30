use crate::database::Database;
use crate::models::EventData;
use crate::nonce_manager::NonceManager;
use crate::tx_builder::TxBuilder;
use crate::tx_sender::TxSender;

use async_trait::async_trait;
use near_sdk::AccountId;
use std::sync::Arc;

pub struct Miner {
    nonce_manager: Arc<NonceManager>,
    tx_builder: Arc<TxBuilder>,
    tx_sender: Arc<TxSender>,
    db: Arc<Database>,
    account_id: AccountId,
}

impl Miner {
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
impl TransactionProcessor for Miner {
    async fn process_transaction(
        &self,
        event_data: EventData,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        println!("Miner Processor");
        println!("Event Data: {:?}", event_data);

        match self.commit(event_data.clone()).await {
            Ok(_) => {
                println!("Commit successful");
                Ok(true)
            }
            Err(e) => {
                println!("Failed to commit: {}", e);
                Err(e)
            }
        }
    }

    // async fn commit(
    //     &self,
    //     event_data: EventData,
    // ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //     println!("Miner Commit");
    //     // TODO: Integrar esto aqui
    //     // let (current_nonce, block_hash) = get_nonce_and_tx_hash(client, signer).await?;

    //     // let (transaction, tx_hash) =
    //     //     build_commit_transaction(signer, request_id, answer, current_nonce, block_hash);

    //     // let request = methods::send_tx::RpcSendTransactionRequest {
    //     //     signed_transaction: transaction.sign(signer),
    //     //     wait_until: TxExecutionStatus::Final,
    //     // };

    //     // send_transaction(client, request, tx_hash, signer).await?;

    //     // let answer = hash_miner_answer(
    //     //     self.signer.account_id.clone(),
    //     //     event_data.request_id.clone(),
    //     //     true,
    //     //     "The best option".to_string(),
    //     // );
    //     let answer = "422fa60e22dc75c98d21bb975323c5c0b854d6b0b7a63d6446b3bbb628b65a5b".to_string();
    //     let commit_miner_result = commit_by_miner(
    //         &self.client,
    //         &self.signer,
    //         event_data.request_id.clone(),
    //         answer.clone(),
    //     )
    //     .await;

    //     match commit_miner_result {
    //         Ok(_) => println!(
    //             "Commit by miner successful for request_id: {}",
    //             event_data.request_id.clone()
    //         ),
    //         Err(e) => println!("Failed to commit by miner: {}", e),
    //     }
    //     Ok(())
    // }

    // async fn reveal(
    //     &self,
    //     event_data: EventData,
    // ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //     println!("Miner Reveal");

    //     // TODO: Integrar esto aqui
    //     // let (current_nonce, block_hash) = get_nonce_and_tx_hash(client, signer).await?;

    //     // let (transaction, tx_hash) = build_reveal_miner_transaction(
    //     //     signer,
    //     //     request_id,
    //     //     answer,
    //     //     message,
    //     //     current_nonce,
    //     //     block_hash,
    //     // );

    //     // let request = methods::send_tx::RpcSendTransactionRequest {
    //     //     signed_transaction: transaction.sign(signer),
    //     //     wait_until: TxExecutionStatus::Final,
    //     // };

    //     // send_transaction(client, request, tx_hash, signer).await?;
    //     let answer = true;
    //     let message = "It's cool".to_string();

    //     let reveal_miner_result = reveal_by_miner(
    //         &self.client,
    //         &self.signer,
    //         event_data.request_id.clone(),
    //         answer,
    //         message,
    //     )
    //     .await;

    //     match reveal_miner_result {
    //         Ok(_) => println!(
    //             "Reveal by miner successful for request_id: {}",
    //             event_data.request_id.clone()
    //         ),
    //         Err(e) => println!("Failed to reveal by miner: {}", e),
    //     }
    //     Ok(())
    // }
}
