use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::connection::{PeerConnection, ice_servers_to_js};
use crate::error::{IPChatWebRTCError, Result};
use crate::message::{Message, MessageType, PeerInfo, SignalingMessage};
use crate::peer::{Peer, PeerId};
use crate::signaling::SignalingClient;
use crate::types::IPChatWebRTCConfig;

type PeerUpdateCallback = Box<dyn Fn(Vec<PeerInfo>)>;
type MessageCallback = Box<dyn Fn(String, String)>;
type FileOfferCallback = Box<dyn Fn(String, String)>;

/// Manages device discovery and peer connections
pub struct DeviceManager {
    config: IPChatWebRTCConfig,
    own_peer_id: Option<PeerId>,
    own_device_name: String,
    signaling: Option<SignalingClient>,
    peers: Rc<RefCell<HashMap<String, Peer>>>,
    connections: Rc<RefCell<HashMap<String, PeerConnection>>>,
    on_peer_update: Rc<RefCell<Option<PeerUpdateCallback>>>,
    on_message: Rc<RefCell<Option<MessageCallback>>>,
    on_file_offer: Rc<RefCell<Option<FileOfferCallback>>>,
}

impl DeviceManager {
    /// Create a new device manager
    pub fn new(config: IPChatWebRTCConfig, device_name: String) -> Self {
        Self {
            config,
            own_peer_id: None,
            own_device_name: device_name,
            signaling: None,
            peers: Rc::new(RefCell::new(HashMap::new())),
            connections: Rc::new(RefCell::new(HashMap::new())),
            on_peer_update: Rc::new(RefCell::new(None)),
            on_message: Rc::new(RefCell::new(None)),
            on_file_offer: Rc::new(RefCell::new(None)),
        }
    }

    /// Connect to signaling server
    pub async fn connect(&mut self) -> Result<()> {
        let signaling = SignalingClient::new(&self.config.signaling_server)?;

        // Setup signaling message handler
        self.setup_signaling_handlers(&signaling)?;

        // Wait for connection to open
        let start = js_sys::Date::now();
        while !signaling.is_open() {
            if js_sys::Date::now() - start > 5000.0 {
                return Err(IPChatWebRTCError::ConnectionError(
                    "Connection timeout".to_string(),
                ));
            }
            // Small delay
            wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
                web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 10)
                    .unwrap();
            }))
            .await
            .unwrap();
        }

        // Register with server
        signaling.register(self.own_device_name.clone(), self.config.room_id.clone())?;

        self.signaling = Some(signaling);

        Ok(())
    }

    /// Disconnect from signaling server
    pub fn disconnect(&mut self) -> Result<()> {
        // Close all peer connections
        for (_, connection) in self.connections.borrow().iter() {
            connection.close();
        }
        self.connections.borrow_mut().clear();

        // Close signaling connection
        if let Some(signaling) = &self.signaling {
            signaling.close()?;
        }
        self.signaling = None;

        Ok(())
    }

    /// Get list of discovered peers
    pub fn get_peers(&self) -> Vec<Peer> {
        self.peers
            .borrow()
            .values()
            .map(|p| Peer {
                id: p.id.clone(),
                name: p.name.clone(),
                device_type: p.device_type.clone(),
            })
            .collect()
    }

    /// Connect to a specific peer
    pub async fn connect_to_peer(&mut self, peer_id: String) -> Result<()> {
        let ice_servers = ice_servers_to_js(&self.config.ice_servers)?;
        let mut connection = PeerConnection::new(peer_id.clone(), ice_servers)?;

        // Create data channel
        connection.create_data_channel("data")?;

        // Setup connection callbacks
        self.setup_connection_handlers(&connection, &peer_id)?;

        // Create offer
        let offer_sdp = connection.create_offer().await?;

        // Send offer through signaling
        if let (Some(signaling), Some(own_id)) = (&self.signaling, &self.own_peer_id) {
            signaling.send_offer(own_id.to_string(), peer_id.clone(), offer_sdp)?;
        }

        self.connections.borrow_mut().insert(peer_id, connection);

        Ok(())
    }

    /// Send a text message to a peer
    pub fn send_message(&self, peer_id: String, content: String) -> Result<()> {
        let connections = self.connections.borrow();
        let connection = connections
            .get(&peer_id)
            .ok_or_else(|| IPChatWebRTCError::PeerNotFound(peer_id.clone()))?;

        if let Some(own_id) = &self.own_peer_id {
            let message = Message::text(own_id.to_string(), peer_id, content);

            let json = message
                .to_json()
                .map_err(|e| IPChatWebRTCError::SerializationError(format!("{:?}", e)))?;

            connection.send_text(&json)?;
        }

        Ok(())
    }

    /// Send file offer to peer
    pub fn send_file_offer(&self, peer_id: String, files_json: String) -> Result<()> {
        let connections = self.connections.borrow();
        let connection = connections
            .get(&peer_id)
            .ok_or_else(|| IPChatWebRTCError::PeerNotFound(peer_id.clone()))?;

        connection.send_text(&files_json)?;

        Ok(())
    }

    /// Get own peer ID
    pub fn peer_id(&self) -> Option<String> {
        self.own_peer_id.as_ref().map(|id| id.to_string())
    }

    /// Get device name
    pub fn device_name(&self) -> String {
        self.own_device_name.clone()
    }

    /// Check if connected to signaling server
    pub fn is_connected(&self) -> bool {
        self.signaling
            .as_ref()
            .map(|s| s.is_open())
            .unwrap_or(false)
    }
}

impl DeviceManager {
    fn setup_signaling_handlers(&self, signaling: &SignalingClient) -> Result<()> {
        let peers = self.peers.clone();
        let connections = self.connections.clone();
        let _config = self.config.clone();
        let on_peer_update = self.on_peer_update.clone();
        let own_peer_id = Rc::new(RefCell::new(self.own_peer_id.clone()));

        signaling.set_on_message(move |msg| {
            match msg {
                SignalingMessage::Registered { peer_id } => {
                    *own_peer_id.borrow_mut() = Some(PeerId::from(peer_id));
                }
                SignalingMessage::PeersUpdate { peers: peer_list } => {
                    let mut peers_map = peers.borrow_mut();
                    peers_map.clear();

                    for peer_info in &peer_list {
                        let peer = Peer::new(
                            PeerId::from(peer_info.id.clone()),
                            peer_info.name.clone(),
                            peer_info.device_type.clone(),
                        );
                        peers_map.insert(peer_info.id.clone(), peer);
                    }

                    if let Some(callback) = on_peer_update.borrow().as_ref() {
                        callback(peer_list);
                    }
                }
                SignalingMessage::Offer {
                    from: _,
                    to: _,
                    sdp: _,
                } => {
                    // Handle incoming offer
                    // Would need to create answer and send back
                }
                SignalingMessage::Answer {
                    from: _,
                    to: _,
                    sdp: _,
                } => {
                    // Handle incoming answer
                    // Would need to set remote description
                }
                SignalingMessage::IceCandidate {
                    from: _,
                    to: _,
                    candidate: _,
                    sdp_mid: _,
                    sdp_m_line_index: _,
                } => {
                    // Handle ICE candidate
                    // Would need to add to appropriate connection
                }
                SignalingMessage::PeerDisconnected { peer_id } => {
                    peers.borrow_mut().remove(&peer_id);
                    connections.borrow_mut().remove(&peer_id);
                }
                _ => {}
            }
        });

        Ok(())
    }

    fn setup_connection_handlers(&self, connection: &PeerConnection, peer_id: &str) -> Result<()> {
        let on_message = self.on_message.clone();
        let peer_id_clone = peer_id.to_string();

        connection.set_on_data(move |data| {
            if let Ok(text) = String::from_utf8(data)
                && let Ok(message) = Message::from_json(&text)
                && let MessageType::Text { content } = message.payload
                && let Some(callback) = on_message.borrow().as_ref()
            {
                callback(peer_id_clone.clone(), content);
            }
        });

        Ok(())
    }

    /// Set peer update callback (for JS)
    pub fn set_on_peer_update<F>(&self, callback: F)
    where
        F: Fn(Vec<PeerInfo>) + 'static,
    {
        *self.on_peer_update.borrow_mut() = Some(Box::new(callback));
    }

    /// Set message received callback (for JS)
    pub fn set_on_message<F>(&self, callback: F)
    where
        F: Fn(String, String) + 'static,
    {
        *self.on_message.borrow_mut() = Some(Box::new(callback));
    }
}
