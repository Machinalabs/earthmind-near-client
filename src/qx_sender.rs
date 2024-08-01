// use near_jsonrpc_client::{methods, JsonRpcClient};
// use near_jsonrpc_primitives::types::query::RpcQueryResponse;
// use near_primitives::hash::CryptoHash;
// use near_primitives::types::AccountId;
// use near_primitives::views::QueryRequest;
// use std::sync::Arc;
// use std::time::{Duration, Instant};

// pub struct QuerySender {
//     client: Arc<JsonRpcClient>,
//     timeout: Duration,
// }

// impl QuerySender {
//     pub fn new(client: Arc<JsonRpcClient>, timeout: Duration) -> Self {
//         Self { client, timeout }
//     }

//     pub async fn send_query(
//         &self,
//         request: methods::query::RpcQueryRequest,
//     ) -> Result<RpcQueryResponse, Box<dyn std::error::Error + Send + Sync>> {
//         let sent_at = Instant::now();

//         match self.client.call(request.clone()).await {
//             Ok(response) => {
//                 self.log_response_time(sent_at);
//                 Ok(response)
//             }
//             Err(err) => {
//                 if let Some(_) = err.handler_error() {
//                     self.wait_for_query(request, sent_at).await
//                 } else {
//                     Err(err.into())
//                 }
//             }
//         }
//     }

//     async fn wait_for_query(
//         &self,
//         request: methods::query::RpcQueryRequest,
//         sent_at: Instant,
//     ) -> Result<RpcQueryResponse, Box<dyn std::error::Error + Send + Sync>> {
//         loop {
//             let response = self.client.call(request.clone()).await;

//             if sent_at.elapsed() > self.timeout {
//                 return Err("Time limit exceeded for the query to be recognized".into());
//             }

//             match response {
//                 Ok(response) => {
//                     self.log_response_time(sent_at);
//                     return Ok(response);
//                 }
//                 Err(err) => {
//                     if let Some(_) = err.handler_error() {
//                         continue;
//                     }
//                     return Err(err.into());
//                 }
//             }
//         }
//     }

//     fn log_response_time(&self, sent_at: Instant) {
//         let delta = sent_at.elapsed().as_secs();
//         println!("Response received after: {}s", delta);
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = Arc::new(JsonRpcClient::connect("https://rpc.testnet.near.org"));
//     let query_sender = QuerySender::new(client.clone(), Duration::from_secs(30));

//     let query = QueryBuilder::new("earthmindprotocol.testnet")
//         .with_method_name("hash_miner_answer")
//         .with_args(json!({
//             "miner": "hasserualcala.testnet",
//             "request_id": "02116dd955aa5412e0020f879a67c36ffb8132b10c3aa5478cc6ac4954ede6a1",
//             "answer": true,
//             "message": "It's the best option",
//         }))
//         .build();

//     let response = query_sender.send_query(query).await?;

//     if let QueryResponseKind::CallResult(result) = response.kind {
//         let result_str = String::from_utf8(result.result)?;
//         println!("QUERY RESULT: {}", result_str);
//     }

//     Ok(())
// }

use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::RpcQueryResponse;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct QuerySender {
    client: Arc<JsonRpcClient>,
    timeout: Duration,
}

impl QuerySender {
    pub fn new(client: Arc<JsonRpcClient>, timeout: Duration) -> Self {
        Self { client, timeout }
    }

    pub async fn send_query(
        &self,
        request: methods::query::RpcQueryRequest,
    ) -> Result<RpcQueryResponse, Box<dyn std::error::Error + Send + Sync>> {
        let sent_at = Instant::now();

        match self.client.call(request.clone()).await {
            Ok(response) => {
                self.log_response_time(sent_at);
                Ok(response)
            }
            Err(err) => {
                if let Some(_) = err.handler_error() {
                    self.wait_for_query(request, sent_at).await
                } else {
                    Err(err.into())
                }
            }
        }
    }

    async fn wait_for_query(
        &self,
        request: methods::query::RpcQueryRequest,
        sent_at: Instant,
    ) -> Result<RpcQueryResponse, Box<dyn std::error::Error + Send + Sync>> {
        loop {
            let response = self.client.call(request.clone()).await;

            if sent_at.elapsed() > self.timeout {
                return Err("Time limit exceeded for the query to be recognized".into());
            }

            match response {
                Ok(response) => {
                    self.log_response_time(sent_at);
                    return Ok(response);
                }
                Err(err) => {
                    if let Some(_) = err.handler_error() {
                        continue;
                    }
                    return Err(err.into());
                }
            }
        }
    }

    fn log_response_time(&self, sent_at: Instant) {
        let delta = sent_at.elapsed().as_secs();
        println!("Response received after: {}s", delta);
    }
}
