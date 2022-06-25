use std::collections::HashSet;
use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use chrono::{DateTime, NaiveDateTime, Utc};
use log::{error, info, warn};
use serenity::async_trait;
use serenity::Client;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::futures::StreamExt;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::*;
use serenity::prelude::GatewayIntents;
use serenity::utils::Color;

use crate::commands::address::*;
use crate::commands::domain::*;
use crate::commands::help::*;
use crate::commands::ping::*;
use crate::commands::solanabeach::*;
use crate::commands::solscan::*;
use crate::commands::step::*;
use crate::commands::tokens::*;
use crate::solana::token_lookup::find_token_symbol;
use crate::solscan;
use crate::solscan::helper::{get_recent_spl_transfers, get_wallet_balance_total};
use crate::solscan::structs::spl_transfer::Transfer;

#[group]
#[commands(ping, help, address, domain, solscan, solanabeach, step, tokens)]
struct General;

/*pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}*/

struct Handler {
    is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        info!("Cache built successfully!");
        let ctx = Arc::new(ctx);

        if !self.is_loop_running.load(Ordering::Relaxed) {
            // Update Name (USDT Value)
            let ctx1 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    warn!("Running UpdateName LOOP");

                    let total_usdt = get_wallet_balance_total().await;
                    let name_text: String = format!("💰 {:.2} 💰 ", total_usdt);

                    //DISPLAY DATA
                    for _guild in _guilds.iter() {
                        match _guild.edit_nickname(&ctx1.http, Some(name_text.as_str())).await {
                            Ok(_) => { info!("Changed Bot nickname!") }
                            Err(_) => { error!("Unable to change bot nickname!") }
                        };
                    }
                    tokio::time::sleep(Duration::from_secs(env::var("LOOP_UPDATE_NAME_SLEEP").unwrap_or("10".to_string()).parse::<u64>().unwrap())).await;
                }
            });

            // Update TXs
            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    warn!("Running UpdateTransactions LOOP");

                    match get_recent_spl_transfers().await {
                        None => { warn!("No transactions found!"); }
                        Some(mut transfers) => {
                            warn!("found {} txs", transfers.len());

                            for mut transfer in transfers.into_iter().rev() {
                                if transfer.symbol.as_ref().unwrap_or(&find_token_symbol(transfer.token_address.as_str())).contains("USDC") {
                                    Self::post_tx_message(Arc::clone(&ctx2), &transfer, env::var("TRANSACTION_USDC_CHANNEL_ID").unwrap().parse::<u64>().unwrap_or(0)).await;
                                }
                                Self::post_tx_message(Arc::clone(&ctx2), &transfer, env::var("TRANSACTION_CHANNEL_ID").unwrap().parse::<u64>().unwrap_or(0)).await;
                            }
                        }
                    };
                    tokio::time::sleep(Duration::from_secs(env::var("LOOP_UPDATE_TX_SLEEP").unwrap_or("10".to_string()).parse::<u64>().unwrap())).await;
                }
            });

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }
    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

impl Handler {
    async fn post_tx_message(ctx: Arc<Context>, transfer: &Transfer, channelID: u64) {
        let mut changeAmount: f64 = 0.0;
        if transfer.decimals > 0 {
            changeAmount = (transfer.change_amount.to_string().parse::<f64>().unwrap()) / ((10_i32.pow(transfer.decimals as u32)) as f64);
        } else {
            changeAmount = transfer.change_amount.to_string().parse().unwrap();
        }


        let m = ChannelId(channelID).send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("ℹ SPL-Transfer ℹ")
                    .description(DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(transfer.block_time, 0), Utc))
                    .color(Color::ORANGE)
                    .field("TX-Signature: ", format!("{:?}", transfer.signature[0]), false)
                    .field("Change Amount: ", changeAmount, false)
                    .field("Token Symbol: ", transfer.symbol.as_ref().unwrap_or(&find_token_symbol(transfer.token_address.as_str())), false)
            })
        }).await;
    }
}


pub async fn bot_start() {
    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => { token }
        Err(_) => { panic!("ENV: DISCORD_TOKEN not set!") }
    };

    let http = Http::new(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework =
        StandardFramework::new().configure(|c| c.owners(owners).prefix("~")).group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
        })
        .await
        .expect("Err creating client");

    /*    {
            let mut data = client.data.write().await;
            data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        }*/

//    let shard_manager = client.shard_manager.clone();

    /*  tokio::spawn(async move {
          tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
          shard_manager.lock().await.shutdown_all().await;
      });
  */
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}