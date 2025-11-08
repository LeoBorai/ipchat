use std::net::Ipv4Addr;

use anyhow::Result;

use crate::discovery::DiscoveryService;

pub struct PeerInfo {
    pub username: String,
    pub ip: Ipv4Addr,
}

impl PeerInfo {
    pub fn new(username: String) -> Result<Self> {
        let ip = DiscoveryService::find_local_ip()?;
        Ok(Self { username, ip })
    }
}
