use std::env;
use std::env::VarError;

use pretty_env_logger::env_logger;
use serenity::async_trait;
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::framework::standard::macros::{command, group};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::bot_helper::bot_start;

mod bot_helper;
mod commands;
mod solana;
mod solscan;
mod storage;

#[tokio::main]
async fn main() {
    println!("Starting Application...");
    env_logger::init();

    match env::var("WALLET_ADDRESS") {
        Ok(_) => { ; }
        Err(_) => { panic!("ENV: WALLET_ADDRESS is not set!"); }
    }


    bot_start().await;
}

