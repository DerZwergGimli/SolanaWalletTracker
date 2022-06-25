use std::env;

use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

#[command]
async fn solanabeach(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("SolanaBeach Wallet Link")
                .description("The Wallets-Solana-SolanaBeach-Link ")
                .color(Color::ORANGE)
                .field("SolanaBeach-Link", "https://solanabeach.io/address/".to_string() + &*env::var("WALLET_ADDRESS").unwrap_or("NOT-Found!".to_string()), false)

                .timestamp(Timestamp::now())
        })
    }).await?;
    Ok(())
}