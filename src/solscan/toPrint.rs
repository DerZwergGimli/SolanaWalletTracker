use chrono::{DateTime, NaiveDateTime, Utc};

use crate::solscan::structs::token::Token;
use crate::solscan::structs::transaction::Transaction;

pub fn print_tokens(transactions: Vec<Token>) {
    println!("Token-Log: ");
    for (index, token) in transactions.iter().enumerate() {
        println!("{}_Token", index);
        println!("\t> Address: {:?}", token.token_address);
        println!("\t> Symbol: {:?}", token.token_symbol.as_ref().unwrap_or(&"?".to_string()));
        println!("\t> Amount: {:?}", token.token_amount.ui_amount);
    }
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
