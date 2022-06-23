use reqwest::Error;

use crate::solscan::r#const::SOLSCANBASEURL;
use crate::solscan::r#struct::Token;

pub async fn load_account_tokens(account_address: &str) {
    let api_result = get_account_tokens(account_address).await;
    match api_result {
        Ok(json_data) => {
            info!("Successfully fetched account tokens!");
            match serde_json::from_str::<Vec<Token>>(json_data.as_str()) {
                Ok(data) => {
                    info!("Successfully serialized json data!");
                    //println!("{:?}", data)
                }
                Err(_) => { error!("Unable to serialize json data!") }
            }
        }
        Err(_) => { error!("Unable to fetch account tokens!") }
    }
}

pub async fn get_account_tokens(account_address: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();

    let result = client
        .get(SOLSCANBASEURL.to_string() + "/account/tokens?account=" + account_address)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?
        .text()
        .await?;

    Ok(result)
}

fn convert_json(json: String) -> serde_json::Result<()> {
    Ok(serde_json::from_str(json.as_str())?)
}