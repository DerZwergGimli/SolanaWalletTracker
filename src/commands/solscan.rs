use std::env;

use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

#[command]
async fn solscan(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Solscan Wallet Link")
                .description("The Wallets-Solana-Solscan-Link ")
                .color(Color::ORANGE)
                .field("Solcan-Link", "https://solscan.io/account/".to_string() + &*env::var("WALLET_ADDRESS").unwrap_or("NOT-Found!".to_string()), false)

                .timestamp(Timestamp::now())
        })
    }).await?;
    Ok(())
}