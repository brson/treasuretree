use anyhow::{anyhow, Result, Context};
use log::info;
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::time::Duration;

use solana_client::rpc_client::RpcClient;
use solana_client::thin_client::{self, ThinClient};
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

static DEPLOY_PATH: &str = "target/deploy";
static PROGRAM_SO_PATH: &str = "geonft_solana.so";
static PROGRAM_KEYPAIR_PATH: &str = "geonft_solana-keypair.json";

pub fn check_program(config: &RpcClient) -> Result<()> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let deploy_path = format!("{}/../../{}", manifest_dir, DEPLOY_PATH);
    let program_so_path = format!("{}/{}", deploy_path, PROGRAM_SO_PATH);
    let program_keypair_path = format!("{}/{}", deploy_path, PROGRAM_KEYPAIR_PATH);

    info!("loading program keypair from {}", program_keypair_path);

    let program_keypair = read_keypair_file(&program_keypair_path)
        .map_err(|e| anyhow!("{}", e))
        .context("unable to load program keypair")?;

    todo!()
}
