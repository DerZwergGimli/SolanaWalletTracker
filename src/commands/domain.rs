use std::env;

use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

#[command]
async fn domain(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Solana Wallet Domain Address")
                .description("The Wallets-Solana-Domain-Address ")
                .color(Color::ORANGE)
                .field("SOL-Domain", env::var("WALLET_ADDRESS_DOMAIN").unwrap_or("Not-Found".to_string()), false)

                .timestamp(Timestamp::now())
        })
    }).await?;
    Ok(())
}