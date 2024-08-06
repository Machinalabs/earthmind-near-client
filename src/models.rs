use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct EventData {
    pub request_id: String,
    pub start_time: u64,
    pub reveal_miner_time: u64,
    pub commit_miner_time: u64,
    pub reveal_validator_time: u64,
    pub commit_validator_time: u64,
}

#[derive(Deserialize, Debug)]
pub struct EventLog {
    standard: String,
    version: String,
    event: String,
    pub data: Vec<EventData>,
}
