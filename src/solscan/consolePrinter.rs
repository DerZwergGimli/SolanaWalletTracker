use chrono::{DateTime, NaiveDateTime, Utc};

use crate::solana::token_lookup::find_token_symbol;
use crate::solscan::solscan::load_market_token;
use crate::solscan::structs::spl_transfer::SplTransferContainer;
use crate::solscan::structs::token::Token;
use crate::solscan::structs::transaction::Transaction;

pub async fn print_tokens(tokens: Vec<Token>) {
    let mut total_usdt: Vec<f64> = Vec::new();

    println!("Token-Log: ");
    for (index, token) in tokens.iter().enumerate() {
        println!("{}_Token", index);
        println!("\t> Address: {:?}", token.token_address);
        println!("\t> Symbol: {:?}", token.token_symbol.as_ref().unwrap_or(&"?".to_string()));
        println!("\t> Amount: {:?}", token.token_amount.ui_amount);
        total_usdt.push(load_market_token(token.token_address.as_str()).await.unwrap().price_usdt.unwrap_or(0.0) * token.token_amount.ui_amount);
        println!("\t> MarketPrice: {:?} USDT", total_usdt.last().unwrap());
    }
    println!("> Total USDT: {:?}", total_usdt.iter().sum::<f64>());
}


pub fn print_transactions(transactions: Vec<Transaction>) {
    println!("Transactions-Log: ");
    for (index, transaction) in transactions.iter().enumerate() {
        println!("{}_Transaction", index);
        println!("\t> ⏲ BlockTime: {:?} [{:?}]", transaction.block_time, DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(transaction.block_time, 0), Utc));
        println!("\t> TX-Hash: {:?}", transaction.tx_hash);
        println!("\t> Signer: {:?}", transaction.signer);
        println!("\t> Status: {:?}", transaction.status);
    }
}


pub fn print_spl_transfers(spl_transfer: SplTransferContainer) {
    println!("SPL-Transfer-Log: ");
    for (index, transfer) in spl_transfer.data.iter().enumerate() {
        println!("{}_Transfer", index);
        println!("\t> ⏲ BlockTime: {:?} [{:?}]", transfer.block_time, DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(transfer.block_time, 0), Utc));
        println!("\t> Signature: {:?}", transfer.signature[0]);
        println!("\t> ChangeType: {:?}", transfer.change_type);
        println!("\t> ChangeAmount: {:?}", transfer.change_amount);
        println!("\t> TokenAddress: {:?}", transfer.token_address);

        println!("\t> Symbol: {:?}", transfer.symbol.as_ref().unwrap_or(&find_token_symbol(transfer.token_address.as_str())));
    }
}