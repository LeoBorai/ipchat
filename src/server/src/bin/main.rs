use anyhow::Result;

use ipchat::{discovery::DiscoveryService, peer::PeerInfo};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter_layer)
        .init();

    let discovery = DiscoveryService::new().await?;
    let peer_info = PeerInfo::new(String::from("Leo"))?;

    discovery.start_beacon(&peer_info).await?;
    discovery.start_listener().await?;

    println!("Discovery service running. Press Ctrl+C to stop.\n");

    tokio::signal::ctrl_c().await?;
    println!("\nShutting down...");

    Ok(())
}
