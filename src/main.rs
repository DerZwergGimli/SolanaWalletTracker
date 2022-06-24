mod bot_helper;
mod commands;

use std::env;
use pretty_env_logger::env_logger;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use crate::bot_helper::bot_start;




#[tokio::main]
async fn main() {
    env_logger::init();

    bot_start().await;
}

