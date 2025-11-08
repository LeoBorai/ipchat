use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::chat::{PeerInfo, Room, ServerMessage};
use crate::discovery::DiscoveryService;

pub type SharedPeer = Arc<RwLock<Peer>>;

pub struct Peer {
    pub username: String,
    pub ip: Ipv4Addr,
    pub connections: HashMap<SocketAddr, UnboundedSender<ServerMessage>>,
    pub rooms: HashMap<Uuid, Room>,
    pub discovered_peers: HashMap<IpAddr, PeerInfo>,
}

impl Peer {
    pub fn new(username: String) -> Result<Self> {
        let ip = DiscoveryService::find_local_ip()?;

        Ok(Self {
            username,
            ip,
            connections: HashMap::new(),
            rooms: HashMap::new(),
            discovered_peers: HashMap::new(),
        })
    }

    pub fn shared(self) -> SharedPeer {
        Arc::new(RwLock::new(self))
    }

    pub fn add_peer(&mut self, peer_info: PeerInfo) {
        self.discovered_peers.insert(peer_info.ip, peer_info);
    }
}
