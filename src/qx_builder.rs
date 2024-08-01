

use near_primitives::views::QueryRequest;
use serde_json::Value;
use near_sdk::AccountId;

pub struct QueryBuilder {
    account_id: AccountId,
    method_name: String,
    args: Value,
}

impl QueryBuilder {
    pub fn new(account_id: &str) -> Self {
        Self {
            account_id: account_id.parse().unwrap(),
            method_name: String::new(),
            args: Value::Null,
        }
    }

    pub fn with_method_name(mut self, method_name: &str) -> Self {
        self.method_name = method_name.to_string();
        self
    }

    pub fn with_args(mut self, args: Value) -> Self {
        self.args = args;
        self
    }

    pub fn build(self) -> QueryRequest {
        QueryRequest::CallFunction {
            account_id: self.account_id,
            method_name: self.method_name,
            args: near_primitives::types::FunctionArgs::from(
                serde_json::to_vec(&self.args).unwrap(),
            ),
        }
    }
}
