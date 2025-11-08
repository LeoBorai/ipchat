use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;

use ipchat::discovery::DiscoveryService;
use ipchat::peer::Peer;
use ipchat::server::router::make_router;
use ipchat::services::Services;
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
        let discovery = DiscoveryService::new().await?;
        let peer = Peer::new(self.username.clone())?;
        let peer = peer.shared();

        discovery.start_beacon(Arc::clone(&peer)).await?;
        discovery.start_listener(Arc::clone(&peer)).await?;

        info!("Discovery service running. Press Ctrl+C to stop.");

        let ws = WebSocket::new(Arc::clone(&peer)).await?;
        ws.start().await?;

        info!(addr=%ws.addr(), "WebSocket listening");

        let ws = Arc::new(ws);
        let services = Services::new(setup.clone(), ws);
        let router = make_router(services).await?;
        let listener = TcpListener::bind("0.0.0.0:7878").await?;
        let server_addr = listener.local_addr()?;

        info!(%server_addr, "HTTP server listening");

        axum::serve(listener, router)
            .with_graceful_shutdown(shutdown_signal())
            .await?;

        info!("Shutting down...");

        Ok(())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
