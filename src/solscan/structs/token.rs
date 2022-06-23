use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    #[serde(rename = "tokenAmount")]
    pub token_amount: TokenAmount,
    #[serde(rename = "tokenAccount")]
    pub token_account: String,
    #[serde(rename = "tokenName")]
    pub token_name: String,
    #[serde(rename = "tokenIcon")]
    pub token_icon: String,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: i64,
    pub lamports: i64,
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAmount {
    pub amount: String,
    pub decimals: i64,
    #[serde(rename = "uiAmount")]
    pub ui_amount: f64,
    #[serde(rename = "uiAmountString")]
    pub ui_amount_string: String,
}