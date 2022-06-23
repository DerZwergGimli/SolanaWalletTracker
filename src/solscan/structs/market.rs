use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketPrice {
    #[serde(rename = "priceUsdt")]
    pub price_usdt: Option<f64>,
    #[serde(rename = "volumeUsdt")]
    pub volume_usdt: Option<i64>,
    #[serde(rename = "marketCapFD")]
    pub market_cap_fd: Option<i64>,
    #[serde(rename = "marketCapRank")]
    pub market_cap_rank: Option<i64>,
    #[serde(rename = "priceChange24h")]
    pub price_change24_h: Option<f64>,
}
