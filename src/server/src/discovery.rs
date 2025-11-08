use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::{Context, Result, bail};
use if_addrs::{IfAddr, Interface, get_if_addrs};
use serde::{Deserialize, Serialize};
use socket2::{Domain, Protocol, Socket, Type};
use tokio::net::UdpSocket;
use tokio::time::{self, Duration};
use tracing::{error, info};

use crate::chat::PeerInfo;
use crate::peer::SharedPeer;

const BROADCAST_INTERVAL_SECS: u64 = 5;
const DISCOVERY_PORT: u16 = 9001;
const LISTENER_BUFFER_SIZE: usize = 1024;

/// An IPv4 address representing the local machine's IP in the local network.
pub type LocalIp = Ipv4Addr;

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscoveryMessage {
    pub username: String,
    pub ip: Ipv4Addr,
}

/// Service in charge of discovering other chat clients in the local network.
pub struct DiscoveryService {
    local_ip: Ipv4Addr,
    broadcast_ip: Ipv4Addr,
}

impl DiscoveryService {
    pub async fn new() -> Result<Self> {
        let local_ip = Self::find_local_ip()?;
        let broadcast_ip = Self::find_broadcast_ip(local_ip)?;

        info!(%local_ip, %broadcast_ip, "Configured Discovery Service Successfully");

        Ok(Self {
            local_ip,
            broadcast_ip,
        })
    }

    /// Creates and starts the beacon task that periodically broadcasts
    /// the client's presence to the local network.
    pub async fn start_beacon(&self, peer: SharedPeer) -> Result<()> {
        let broadcast_ip = self.broadcast_ip;
        let socket = Self::create_broadcast_socket()?;
        let peer = peer.read().await;
        let message = DiscoveryMessage {
            username: peer.username.clone(),
            ip: self.local_ip,
        };

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(BROADCAST_INTERVAL_SECS));
            let broadcast_addr = SocketAddr::new(IpAddr::V4(broadcast_ip), 9001);

            loop {
                interval.tick().await;

                match serde_json::to_string(&message) {
                    Ok(json) => match socket.send_to(json.as_bytes(), broadcast_addr).await {
                        Ok(len) => {
                            info!(len=%len, broadcast_addr=%broadcast_addr, "Broadcast message sent");
                        }
                        Err(e) => {
                            error!(?e, %broadcast_addr, "Failed to send broadcast");
                        }
                    },
                    Err(e) => {
                        error!(?e, "Failed to serialize message");
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn start_listener(&self, peer: SharedPeer) -> Result<()> {
        let socket = Self::create_broadcast_socket()?;
        let broadcast_ip = self.broadcast_ip;
        let local_ip = self.local_ip;

        tokio::spawn(async move {
            let mut buf = [0u8; LISTENER_BUFFER_SIZE];

            info!(%broadcast_ip, "Listening for Broadcast");

            loop {
                match socket.recv_from(&mut buf).await {
                    Ok((len, addr)) => {
                        // Skip messages from self
                        if addr.ip() == IpAddr::V4(local_ip) {
                            continue;
                        }

                        if let Ok(msg_str) = std::str::from_utf8(&buf[..len])
                            && let Ok(disc_msg) = serde_json::from_str::<DiscoveryMessage>(msg_str)
                        {
                            peer.write().await.add_peer(PeerInfo {
                                ip: disc_msg.ip.into(),
                                username: disc_msg.username.clone(),
                                rooms: vec![],
                            });
                            info!(%addr, username=disc_msg.username, "Discovered peer");
                        }
                    }
                    Err(e) => {
                        error!(?e, "Failed to receive discovery message");
                    }
                }
            }
        });

        Ok(())
    }

    fn create_broadcast_socket() -> Result<UdpSocket> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

        socket.set_broadcast(true)?;
        socket.set_reuse_address(true)?;

        #[cfg(unix)]
        socket.set_reuse_port(true)?;

        let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), DISCOVERY_PORT);

        socket.bind(&bind_addr.into())?;
        socket.set_nonblocking(true)?;

        let std_socket: std::net::UdpSocket = socket.into();

        Ok(UdpSocket::from_std(std_socket)?)
    }

    /// Finds the Network Interface for the LocalIp provided, then retrieves its
    /// netmask to compute the broadcast address.
    ///
    /// If no matching interface is found, it falls back to assuming a /24 network.
    fn find_broadcast_ip(local_ip: LocalIp) -> Result<Ipv4Addr> {
        let ifas = get_if_addrs().context("Failed to get network interfaces")?;

        for iface in ifas {
            if let IfAddr::V4(addr) = iface.addr
                && addr.ip == local_ip
            {
                return Ok(Self::compute_broadcast_from_netmask(local_ip, addr.netmask));
            }
        }

        // Fallback: assume /24 network
        let mut octets = local_ip.octets();
        octets[3] = 255;
        let broadcast = Ipv4Addr::from(octets);

        Ok(broadcast)
    }

    /// Computes the broadcast address given an IP and its netmask.
    fn compute_broadcast_from_netmask(ip: Ipv4Addr, netmask: Ipv4Addr) -> Ipv4Addr {
        let ip_u32 = u32::from_be_bytes(ip.octets());
        let mask_u32 = u32::from_be_bytes(netmask.octets());
        let broadcast_u32 = ip_u32 | !mask_u32;

        Ipv4Addr::from(broadcast_u32.to_be_bytes())
    }

    /// Retrieves a Local IP Address suitable for local network communication.
    pub fn find_local_ip() -> Result<LocalIp> {
        let if_addrs = get_if_addrs()?;

        if let Some(ip) = Self::find_common_network_interfaces(&if_addrs) {
            return Ok(ip);
        }

        if let Some(ip) = Self::find_first_non_loopback_ip(&if_addrs) {
            return Ok(ip);
        }

        bail!("Failed to find suitable network interfaces")
    }

    /// Common Network Interfaces are those that usually represent active network connections,
    /// such as Ethernet (eth0, enp3s0) and Wi-Fi (wlan0, wlp2s0) interfaces.
    fn find_common_network_interfaces(ifas: &Vec<Interface>) -> Option<LocalIp> {
        for iface in ifas {
            if let IfAddr::V4(addr) = &iface.addr {
                let ip = addr.ip;

                if !ip.is_loopback() && !ip.is_link_local() {
                    // Prefer common network interfaces
                    if iface.name.starts_with("eth")
                        || iface.name.starts_with("en")
                        || iface.name.starts_with("wlan")
                        || iface.name.starts_with("wlp")
                    {
                        println!("Selected interface: {} ({})", iface.name, ip);
                        return Some(ip);
                    }
                }
            }
        }

        None
    }

    /// Fallback method to find the first non-loopback, non-link-local IPv4 address.
    fn find_first_non_loopback_ip(ifas: &Vec<Interface>) -> Option<LocalIp> {
        for iface in ifas {
            if let IfAddr::V4(addr) = &iface.addr {
                let ip = addr.ip;

                if !ip.is_loopback() && !ip.is_link_local() {
                    println!("Selected interface: {} ({})", iface.name, ip);
                    return Some(ip);
                }
            }
        }

        None
    }
}
