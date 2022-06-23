use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SplTransfer {
    pub total: i64,
    pub data: Vec<Transfer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transfer {
    #[serde(rename = "_id")]
    pub id: String,
    pub address: String,
    pub signature: Vec<String>,
    #[serde(rename = "changeType")]
    pub change_type: ChangeType,
    #[serde(rename = "changeAmount")]
    pub change_amount: i64,
    pub decimals: i64,
    #[serde(rename = "postBalance")]
    pub post_balance: String,
    #[serde(rename = "preBalance")]
    pub pre_balance: String,
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    pub symbol: Option<String>,
    #[serde(rename = "blockTime")]
    pub block_time: i64,
    pub slot: i64,
    pub fee: i64,
    pub owner: String,
    pub balance: Balance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub amount: String,
    pub decimals: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChangeType {
    #[serde(rename = "dec")]
    Dec,
    #[serde(rename = "inc")]
    Inc,
}


