# Discord Wallet Tracker

[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.svg?v=103)](https://github.com/ellerbrock/open-source-badges/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![badge_discord](https://badgen.net/badge/icon/discord?icon=discord&label)
[![buid-test](https://github.com/DerZwergGimli/SolanaWalletTracker/actions/workflows/rust-test.yml/badge.svg)](https://github.com/DerZwergGimli/SolanaWalletTracker/actions/workflows/rust-test.yml)
[![deploy-to-dockerhub](https://github.com/DerZwergGimli/SolanaWalletTracker/actions/workflows/docker-build.yml/badge.svg)](https://github.com/DerZwergGimli/SolanaWalletTracker/actions/workflows/docker-build.yml)
![](https://badgen.net/docker/pulls/derzwerggimli//solana_wallet_tracker)
![](https://badgen.net/docker/size/derzwerggimli//solana_wallet_tracker)

![bot_icon](icon.drawio.png)

This repo contains a Discord Wallet Tracker.

- Old version
  of this bot can be found
  here: [discord_solana_wallet_tracker (python)](https://github.com/DerZwergGimli/discord_solana_wallet_tracker)

## Features

- Easy to SetUp with Docker and Environment Variables.
- Sets his "Nickname" according to the provided Wallet-Balance,
- Posts all SPL-Transactions into a Channel.
- Posts all USDC SPL-Transactions into a seperate Channel.
- Can answer to some basic commands.
- Bot Command "prefix"/(~) can be set via EnviromentVariables

## Commands

| command      | description                     |
|--------------|---------------------------------|
| ~help        | Shows a help-message            | 
| ~address     | Prints the wallet-address       | 
| ~domain      | Prints the wallet-domain        | 
| ~solscan     | Prints a link to solcan.io      | 
| ~solanaBeach | Prints a link to solanabeach.io | 
| ~step        | Prints a link to step.finance   |
| ~tokens      | Prints a list of tokens         |

## Environment Variables

```
BOT_PREFIX=~
DISCORD_TOKEN=<PLACE-YOUR-DISCORD-TOKEN>
LOOP_UPDATE_NAME_SLEEP=60
LOOP_UPDATE_TX_SLEEP=5 
RUST_LOG=error 
TRANSACTION_CHANNEL_ID=<PLACE-YOUR-DISCORD-CHANNEL-ID>
TRANSACTION_USDC_CHANNEL_ID=<PLACE-YOUR-OTHER-DISCORD-CHANNEL-ID>
WALLET_ADDRESS=<PLACE_YOUR-SOLANA-WALLET-ADDRESS-HERE>
WALLET_ADDRESS_DOMAIN=<PLACE-YOUR-SOLANA-WALLET-DOMAIN-HERE>
```

## Deployment

1) Make sure `doocker` and `docker-compose` is installed on your system.
2) Copy `docker-compose.yaml.sample` to `docker-compose.yaml`
3) Change the `enviroment` entries to your according your discord-settings.

## Development Links

- [QuickType](https://app.quicktype.io/) a generator for Types from JSON

## Development Docker prepare

- `cargo chef prepare --recipe-path recipe.json`
- `docker image build .`

## Donations

If you want to get me a ??? so I won't ????

- Solana-Wallet-Address: `BSW9zp3iJUcemTVWN4EThcaF6FxBQqP2wgnapSt1Z5mt`
- Solana-Wallet-Domain: `coffeeplease.sol    `