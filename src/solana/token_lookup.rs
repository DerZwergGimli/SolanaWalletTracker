use std::fs;
use std::io::Read;

use crate::solana::token::Tokens;
use crate::solana::token_lookup::TokenLookupError::UnableToSerializeTokenFile;

#[derive(Debug)]
enum TokenLookupError {
    UnableToSerializeTokenFile,
}

pub fn find_token_symbol(address: &str) -> String {
    let token_list = load_token_list().unwrap();
    match token_list.tokens.into_iter().find(|t| t.address.contains(address)) {
        None => { "Unknown token".to_string() }
        Some(token) => { token.symbol.to_string() }
    }
}

fn load_token_list() -> Result<Tokens, TokenLookupError> {
    let mut file = fs::File::open("./files/token-list.json")
        .expect("file should open read only");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    match serde_json::from_str::<Tokens>(&data) {
        Ok(tokens) => { Ok(tokens) }
        Err(_) => {
            error!("Unable to parse Token list");
            Err(UnableToSerializeTokenFile)
        }
    }
}