use std::env;

use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

#[command]
async fn address(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Solana Wallet Address")
                .description("The Wallets-Solana-Address ")
                .color(Color::ORANGE)
                .field("SOL-Address", env::var("WALLET_ADDRESS").unwrap_or("NOT-Found!".to_string()), false)

                .timestamp(Timestamp::now())
        })
    }).await?;
    Ok(())
}