use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use tracing::info;

use ipchat::discovery::DiscoveryService;
use ipchat::domain::Services;
use ipchat::peer::Peer;
use ipchat::setup::Setup;
use ipchat::ws::WebSocket;

#[derive(Clone, Debug, Parser)]
pub struct StartCmd {
    /// Username to use in the chat
    #[clap(short = 'u', long)]
    username: String,
}

impl StartCmd {
    pub async fn exec(&self) -> Result<()> {
        let setup = Setup::new().await?;
        let services = Services::new(setup.clone());
        let discovery = DiscoveryService::new().await?;
        let peer = Peer::new(self.username.clone())?;
        let peer = peer.shared();

        discovery.start_beacon(Arc::clone(&peer)).await?;
        discovery.start_listener(Arc::clone(&peer)).await?;

        info!("Discovery service running. Press Ctrl+C to stop.");

        let ws = WebSocket::new(Arc::clone(&peer));
        ws.start().await?;

        tokio::signal::ctrl_c().await?;
        info!("Shutting down...");

        Ok(())
    }
}
