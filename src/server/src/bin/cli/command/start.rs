use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use tracing::info;

use ipchat::chat::ChatService;
use ipchat::discovery::DiscoveryService;
use ipchat::peer::Peer;

#[derive(Clone, Debug, Parser)]
pub struct StartCmd {
    /// Username to use in the chat
    #[clap(short = 'u', long)]
    username: String,
}

impl StartCmd {
    pub async fn exec(&self) -> Result<()> {
        let discovery = DiscoveryService::new().await?;
        let peer = Peer::new(self.username.clone())?;
        let peer = peer.shared();

        discovery.start_beacon(Arc::clone(&peer)).await?;
        discovery.start_listener(Arc::clone(&peer)).await?;

        info!("Discovery service running. Press Ctrl+C to stop.\n");

        let chat = ChatService::new(Arc::clone(&peer));
        chat.start("0.0.0.0:8080").await?;

        tokio::signal::ctrl_c().await?;
        info!("Shutting down...");

        Ok(())
    }
}
