use anyhow::Result;
use std::net::SocketAddr;
use std::time::Duration;
use log::info;

use solana_client::thin_client::{self, ThinClient};
use solana_client::rpc_client::RpcClient;
use solana_sdk::client::SyncClient;

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
