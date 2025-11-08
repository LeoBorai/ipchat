use std::sync::Arc;

use anyhow::Result;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use ipchat::peer::Peer;
use ipchat::{chat::ChatService, discovery::DiscoveryService};

#[tokio::main]
async fn main() -> Result<()> {
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter_layer)
        .init();

    let discovery = DiscoveryService::new().await?;
    let peer = Peer::new(String::from("Leo"))?;
    let peer = peer.shared();

    discovery.start_beacon(Arc::clone(&peer)).await?;
    discovery.start_listener(Arc::clone(&peer)).await?;

    println!("Discovery service running. Press Ctrl+C to stop.\n");

    let chat = ChatService::new(Arc::clone(&peer));
    chat.start("0.0.0.0:8080").await?;

    tokio::signal::ctrl_c().await?;
    println!("\nShutting down...");

    Ok(())
}
