use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EventData {
    request_id: String,
    start_time: u64,
    reveal_miner_time: u64,
    commit_miner_time: u64,
    reveal_validator_time: u64,
    commit_validator_time: u64,
}

#[derive(Deserialize, Debug)]
pub struct EventJson {
    standard: String,
    version: String,
    event: String,
    data: Vec<EventData>,
}
