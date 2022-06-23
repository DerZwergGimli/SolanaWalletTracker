use reqwest::Error;

use crate::solscan::r#const::SOLSCANBASEURL;
use crate::solscan::solscan::SolscanError::{UNABLE_TO_FETCH, UNABLE_TO_SERIALIZE};
use crate::solscan::structs::token::Token;
use crate::solscan::structs::transaction::Transaction;

#[derive(Debug)]
pub enum SolscanError {
    UNABLE_TO_FETCH,
    UNABLE_TO_SERIALIZE,
}


pub async fn load_account_tokens(account_address: &str) -> Result<Vec<Token>, SolscanError> {
    let api_result = get_account_tokens(account_address).await;
    match api_result {
        Ok(json_data) => {
            info!("Successfully fetched account tokens!");
            match serde_json::from_str::<Vec<Token>>(json_data.as_str()) {
                Ok(data) => {
                    info!("Successfully serialized json data!");
                    //println!("{:?}", data)
                    Ok(data)
                }
                Err(_) => {
                    error!("Unable to serialize json data!");
                    Err(UNABLE_TO_SERIALIZE)
                }
            }
        }
        Err(_) => {
            error!("Unable to fetch account tokens!");
            Err(UNABLE_TO_FETCH)
        }
    }
}

pub async fn load_account_transactions(account_address: &str, limit: i32) -> Result<Vec<Transaction>, SolscanError> {
    let api_result = get_account_transactions(account_address, limit).await;
    match api_result {
        Ok(json_data) => {
            info!("Successfully fetched account transactions!");
            match serde_json::from_str::<Vec<Transaction>>(json_data.as_str()) {
                Ok(data) => {
                    info!("Successfully serialized json data!");
                    //println!("{:?}", data)
                    Ok(data)
                }
                Err(_) => {
                    error!("Unable to serialize json data!");
                    Err(UNABLE_TO_SERIALIZE)
                }
            }
        }
        Err(_) => {
            error!("Unable to fetch account transactions!");
            Err(UNABLE_TO_FETCH)
        }
    }
}

async fn get_account_tokens(account_address: &str) -> Result<String, Error> {
    let result = reqwest::Client::new()
        .get(SOLSCANBASEURL.to_string() + "/account/tokens?account=" + account_address)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?
        .text()
        .await?;

    Ok(result)
}

async fn get_account_transactions(account_address: &str, limit: i32) -> Result<String, Error> {
    let result = reqwest::Client::new()
        .get(SOLSCANBASEURL.to_string() + "/account/transactions?account=" + account_address + "&limit=" + limit.to_string().as_str())
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?
        .text()
        .await?;

    Ok(result)
}
