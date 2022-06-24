use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    pub store: Store,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    pub latest_block_time_call: i64,
}