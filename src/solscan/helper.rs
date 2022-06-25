use std::env;
use std::time::Duration;

use log::{error, warn};

use crate::{solana, solscan, storage};
use crate::solscan::solscan::{load_market_token, SolscanError};
use crate::solscan::structs::market::MarketPrice;
use crate::solscan::structs::spl_transfer::{SplTransferContainer, Transfer};
use crate::solscan::structs::token::Token;

pub async fn get_wallet_balance_total() -> f64 {
    match solscan::solscan::load_account_tokens(env::var("WALLET_ADDRESS").unwrap().as_str()).await {
        Ok(tokens) => {
            let mut total_usdt: Vec<f64> = Vec::new();

            for token in tokens.iter() {
                warn!("API_limit_sleep_start");
                tokio::time::sleep(Duration::from_millis(300)).await;
                total_usdt.push(
                    match load_market_token(token.token_address.as_str()).await {
                        Ok(market_price) => {
                            market_price.price_usdt.unwrap_or(0.0) * token.token_amount.ui_amount
                        }
                        Err(_) => { 0.0 }
                    })
            }

            total_usdt.iter().sum::<f64>()
        }
        Err(_) => {
            0.0
        }
    }
}

pub async fn get_recent_spl_transfers() -> Option<Vec<Transfer>> {
    let timestamp_now = chrono::Utc::now().timestamp();

    match storage::storage_helper::storage_read() {
        Some(mut storage) => {
            match solscan::solscan::load_spl_transfers_timebased(env::var("WALLET_ADDRESS").unwrap().as_str(), 0, 100, storage.store.latest_block_time_call, timestamp_now).await {
                Ok(data) => {
                    if data.data.len() > 0 {
                        warn!("Found TXs gonna set new Timestamp");
                        storage.store.latest_block_time_call = timestamp_now;
                        storage::storage_helper::storage_write(storage);
                    }
                    Some(data.data)
                }
                Err(_) => { None }
            }
        }
        None => {
            error!("Unable to read Storage");
            None
        }
    }
}

pub async fn get_all_tokens() -> Option<Vec<Token>> {
    match solscan::solscan::load_account_tokens(env::var("WALLET_ADDRESS").unwrap().as_str()).await {
        Ok(tokens) => {
            Some(tokens)
        }
        Err(_) => {
            None
        }
    }
}