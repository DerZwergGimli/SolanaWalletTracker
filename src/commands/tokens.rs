use std::borrow::Borrow;
use std::env;

use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

use crate::solana::token_lookup::find_token_symbol;
use crate::solscan::helper::get_all_tokens;
use crate::solscan::structs::token::Token;

#[command]
async fn tokens(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Loading...").await?;

    match get_all_tokens().await {
        None => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Wallet Tokens")
                        .description("List of all Tokens in the Wallet")
                        .color(Color::ORANGE)
                        .field("Error", "Error occurred fetching tokens", false)
                        .timestamp(Timestamp::now())
                })
            }).await?;
        }
        Some(mut tokens) => {
            let mut token_mapped: Vec<(String, String, bool)> = Vec::new();
            for token in tokens {
                token_mapped.push((
                    token.token_address.clone(),
                    token.token_symbol.unwrap_or(find_token_symbol(token.token_address.as_str())) + ": " + &*token.token_amount.ui_amount.to_string(),
                    false));
            }

            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Wallet Tokens")
                        .description("List of all Tokens in the Wallet")
                        .color(Color::ORANGE)
                        .fields(token_mapped)

                        .timestamp(Timestamp::now())
                })
            }).await?;
        }
    };


    Ok(())
}