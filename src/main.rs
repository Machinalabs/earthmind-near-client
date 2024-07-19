use clap::Parser;
use std::sync::{Arc, Mutex};

mod block_streamer;
mod cli;
mod constants;
mod database;

use block_streamer::run_mode;
use cli::{Cli, Modes, Networks};
use constants::{DB_PATH, NEAR_RPC_MAINNET, NEAR_RPC_TESTNET};
use database::init_db;
use near_jsonrpc_client::JsonRpcClient;
//use serde::Deserialize;
//use near_jsonrpc_client::methods;
//use std::fs::File;
//use std::io::BufReader;
//use near_crypto::{InMemorySigner, KeyType, SecretKey};
//use std::str::FromStr;
//use near_primitives::views::QueryRequest;
//use near_primitives::types::{BlockReference, Finality};
//use near_jsonrpc_primitives::types::query::QueryResponseKind;
//use near_primitives::transaction::{Action, FunctionCallAction, Transaction};

// #[derive(Deserialize)]
// struct KeyFile {
//     account_id : String,
//     public_key : String,
//     private_key : String,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Open RocksDB connection
    let db = Arc::new(Mutex::new(init_db(DB_PATH)?));

    // Connect to the RPC client
    let client: JsonRpcClient;

    match cli.network {
        Networks::Mainnet => {
            client = JsonRpcClient::connect(NEAR_RPC_MAINNET);
        }
        Networks::Testnet => {
            client = JsonRpcClient::connect(NEAR_RPC_TESTNET);
        }
    }

    match cli.mode {
        Modes::Miner => {
            run_mode(&client, &db, cli.account_id, process_miner_transaction).await?;
        }
        Modes::Validator => {
            run_mode(&client, &db, cli.account_id, process_validator_transaction).await?;
        }
    }

    Ok(())
}

fn process_miner_transaction(client: &JsonRpcClient) -> Result<bool, Box<dyn std::error::Error>> {
    // Implement your miner-specific logic here

    // let file = File::open("/key-file.json")?;
    // let reader = BufReader::new(file);
    // let key_file: KeyFile = serde_json::from_reader(reader)?;

    // // Crea el signer usando la clave privada del archivo JSON
    // let private_key = SecretKey::from_str(&key_file.private_key)?;
    // let signer = InMemorySigner::from_secret_key(key_file.account_id.parse()?, private_key);

    // let account_id = signer.account_id.clone();
    // let request = methods::query::RpcQueryRequest {
    //     block_reference: BlockReference::Finality(Finality::Final),
    //     request: QueryRequest::ViewAccount { account_id: account_id.clone() },
    // };

    // let response = client.call(request).await?;

    // println!("Miner Account ID: {}", account_id);

    // if let QueryResponseKind::ViewAccount(result) = response.kind {
    //     println!("{:#?}", result);
    // }

    // // Access key
    // let access_key_query_response = client
    //     .call(methods::query::RpcQueryRequest {
    //         block_reference: BlockReference::latest(),
    //         request: near_primitives::views::QueryRequest::ViewAccessKey {
    //             account_id: signer.account_id.clone(),
    //             public_key: signer.public_key.clone(),
    //         },
    //     })
    //     .await?;

    // let current_nonce = match access_key_query_response.kind {
    //     QueryResponseKind::AccessKey(access_key) => access_key.nonce,
    //     _ => Err("failed to extract current nonce")?,
    // };

    // // transaction to sign
    // let transaction = Transaction {
    //     signer_id: signer.account_id.clone(),
    //     public_key: signer.public_key.clone(),
    //     nonce: current_nonce + 1,
    //     receiver_id: "earthmindprotocol.testnet".parse()?,
    //     block_hash: access_key_query_response.block_hash,
    //     actions: vec![Action::FunctionCall(Box::new(FunctionCallAction {
    //         method_name: "rate".to_string(),
    //         args: serde_json::json!({
    //             "account_id": other_account,
    //             "rating": rating,
    //         })
    //         .to_string()
    //         .into_bytes(),
    //         gas: 100_000_000_000_000, // 100 TeraGas
    //         deposit: 0,
    //     }))],
    // };
    println!("Processing as miner: {:?}", client);
    Ok(false)
}

fn process_validator_transaction(
    client: &JsonRpcClient,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Implement your validator-specific logic here
    println!("Processing as validator: {:?}", client);
    Ok(false)
}
