use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: i64,
}
