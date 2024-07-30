use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct EventData {
    pub request_id: String,
    start_time: u64,
    reveal_miner_time: u64,
    commit_miner_time: u64,
    reveal_validator_time: u64,
    commit_validator_time: u64,
}

#[derive(Deserialize, Debug)]
pub struct EventLog {
    standard: String,
    version: String,
    event: String,
    pub data: Vec<EventData>,
}
