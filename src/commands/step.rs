use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use std::env;

#[command]
async fn step(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("StepFinance Wallet Link")
                .description("The Wallets-Solana-StepFinance-Link ")
                .color(Color::ORANGE)
                .field("StepFinance-Link", "https://app.step.finance/#/watch/".to_string() + &*env::var("WALLET_ADDRESS").unwrap_or("NOT-Found!".to_string()), false)

                .timestamp(Timestamp::now())
        })
    }).await?;
    Ok(())
}