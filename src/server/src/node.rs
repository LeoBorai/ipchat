use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::chat::{Room, ServerMessage};
use crate::discovery::DiscoveryService;
use crate::peer::PeerInfo;
use crate::peer::PeerRoom;

pub type ArcNode = Arc<RwLock<Node>>;

pub struct Node {
    pub username: String,
    pub ip: Ipv4Addr,
    pub connections: HashMap<SocketAddr, UnboundedSender<ServerMessage>>,
    pub rooms: HashMap<Uuid, Room>,
    pub discovered_peers: HashMap<IpAddr, PeerInfo>,
}

impl Node {
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

    pub fn shared(self) -> ArcNode {
        Arc::new(RwLock::new(self))
    }

    pub fn add_peer(&mut self, peer_info: PeerInfo) {
        self.discovered_peers.insert(peer_info.ip, peer_info);
    }

    pub fn info(&self) -> PeerInfo {
        PeerInfo {
            ip: self.ip.into(),
            username: self.username.clone(),
            rooms: self
                .rooms
                .values()
                .map(|room| PeerRoom {
                    id: room.id,
                    ip: room.host,
                    name: room.name.clone(),
                })
                .collect(),
        }
    }
}
