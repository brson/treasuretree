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

pub fn establish_connection() -> Result<RpcClient> {
    let rpc_addr = "127.0.0.1:8899";
    let timeout = 1000;

    info!("connecting to solana node, RPC: {}, timeout: {}ms",
          rpc_addr, timeout);

    let rpc_addr: SocketAddr = rpc_addr.parse().expect("");

    let client = RpcClient::new_socket_with_timeout(rpc_addr, Duration::from_millis(timeout));

    let version = client.get_version()?;
    info!("RPC version: {:?}", version);

    Ok(client)
}

static KEYPAIR_PATH: &str = ".config/solana/id.json";

pub fn establish_payer(client: &RpcClient) -> Result<Account> {
    let mut home_dir = dirs::home_dir().ok_or_else(|| anyhow!("no home dir"))?;
    let keypair_path = home_dir.join(KEYPAIR_PATH);
    let file = File::open(keypair_path)?;
    let mut reader = BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(reader)?;
    let array = json.as_array().ok_or_else(|| anyhow!("keypair json not array"))?;
    let array: Vec<u8> = serde_json::from_value(json)?;
    todo!()
}
