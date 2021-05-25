use anyhow::{anyhow, Result};
use std::net::SocketAddr;
use std::time::Duration;
use log::info;
use std::fs::File;
use std::io::BufReader;

use solana_client::thin_client::{self, ThinClient};
use solana_client::rpc_client::RpcClient;
use solana_sdk::client::SyncClient;
use solana_sdk::account::Account;
use solana_sdk::signature::{read_keypair_file, Keypair};
use solana_sdk::commitment_config::CommitmentConfig;

pub struct Config {
    json_rpc_url: String,
    keypair: Keypair,
}

pub fn load_config() -> Result<Config> {
    let config_file = solana_cli_config::CONFIG_FILE.as_ref().ok_or_else(|| anyhow!("config file path"))?;
    let cli_config = solana_cli_config::Config::load(&config_file)?;
    let json_rpc_url = cli_config.json_rpc_url;
    let keypair = read_keypair_file(&cli_config.keypair_path).map_err(|e| anyhow!("{}", e))?;
    Ok(Config {
        json_rpc_url,
        keypair,
    })
}

pub fn connect(config: &Config) -> Result<RpcClient> {

    info!("connecting to solana node at {}", config.json_rpc_url);
    let client = RpcClient::new_with_commitment(config.json_rpc_url.clone(), CommitmentConfig::confirmed());

    let version = client.get_version()?;
    info!("RPC version: {:?}", version);

    Ok(client)
}

static KEYPAIR_PATH: &str = ".config/solana/id.json";

pub fn establish_payer(client: &RpcClient) -> Result<Account> {
    todo!()
}
