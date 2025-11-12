use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use local_ip_address::local_ip;
use tokio::net::TcpListener;
use tracing::info;

use ipchat::discovery::DiscoveryService;
use ipchat::graceful_shutdown::shutdown_signal;
use ipchat::node::Node;
use ipchat::server::router::make_router;
use ipchat::services::Services;
use ipchat::setup::Setup;
use ipchat::ws::WebSocket;

/// Default port for the HTTP server, stands for IPCH
const DEFAULT_PORT: u16 = 4724;

#[derive(Clone, Debug, Parser)]
pub struct StartCmd {
    /// Port to run the HTTP server on
    #[clap(short = 'p', long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

impl StartCmd {
    pub async fn exec(&self) -> Result<()> {
        let local_ip = local_ip()?;
        let local_ip = SocketAddr::new(local_ip, self.port);
        let setup = Setup::new(local_ip).await?;
        let discovery = DiscoveryService::new().await?;
        let node = Node::new()?;
        let node = node.shared();

        discovery.start_beacon(Arc::clone(&node)).await?;
        discovery.start_listener(Arc::clone(&node)).await?;

        let ws = WebSocket::new(Arc::clone(&node)).await?;
        ws.start().await?;

        info!(addr=%ws.addr(), "WebSocket listening");

        let ws = Arc::new(ws);
        let services = Services::new(discovery, setup.clone(), ws);
        let router = make_router(services).await?;
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        let listener = TcpListener::bind(addr).await?;
        let server_addr = listener.local_addr()?;

        info!(%server_addr, %local_ip, "HTTP server listening");

        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(shutdown_signal())
        .await?;

        info!("Shutting down…");

        Ok(())
    }
}
