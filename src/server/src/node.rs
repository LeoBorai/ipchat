use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::discovery::DiscoveryService;
use crate::proto::{NodeInfo, PeerRoom, Room, ServerMessage};

pub type ArcNode = Arc<RwLock<Node>>;

pub struct Node {
    pub ip: Ipv4Addr,
    pub connections: HashMap<SocketAddr, UnboundedSender<ServerMessage>>,
    pub rooms: HashMap<Uuid, Room>,
    pub discovered_nodes: HashMap<IpAddr, NodeInfo>,
}

impl Node {
    pub fn new() -> Result<Self> {
        let ip = DiscoveryService::find_local_ip()?;

        Ok(Self {
            ip,
            connections: HashMap::new(),
            rooms: HashMap::new(),
            discovered_nodes: HashMap::new(),
        })
    }

    pub fn shared(self) -> ArcNode {
        Arc::new(RwLock::new(self))
    }

    pub fn add_node(&mut self, node_info: NodeInfo) {
        self.discovered_nodes.insert(node_info.ip, node_info);
    }

    pub fn info(&self) -> NodeInfo {
        NodeInfo {
            ip: self.ip.into(),
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
