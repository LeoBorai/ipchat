use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::console;

use ipchat_webrtc::{DeviceManager, IPChatWebRTCConfig, Peer};

thread_local! {
    static MANAGER: RefCell<Option<Rc<RefCell<DeviceManager>>>> = const { RefCell::new(None) };
    static PEERS: RefCell<Vec<Peer>> = const { RefCell::new(Vec::new()) };
}

pub async fn connect(server_url: String, device_name: String) -> Result<String, JsValue> {
    console::log_1(&format!("Connecting to {} as {}", server_url, device_name).into());

    let config = IPChatWebRTCConfig::new(server_url);
    let manager = DeviceManager::new(config, device_name);
    let manager_rc = Rc::new(RefCell::new(manager));

    manager_rc
        .borrow_mut()
        .connect()
        .await
        .map_err(|e| JsValue::from_str(&format!("Connection failed: {:?}", e)))?;

    let peer_id = manager_rc
        .borrow()
        .peer_id()
        .ok_or_else(|| JsValue::from_str("No peer ID assigned"))?;

    console::log_1(&format!("Connected with peer ID: {}", peer_id).into());

    // Store manager globally
    MANAGER.with(|m| {
        *m.borrow_mut() = Some(manager_rc);
    });

    Ok(peer_id)
}

pub fn disconnect() -> Result<(), JsValue> {
    MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            manager
                .borrow_mut()
                .disconnect()
                .map_err(|e| JsValue::from_str(&format!("Disconnect failed: {:?}", e)))?;

            console::log_1(&"Disconnected".into());
        }
        *m.borrow_mut() = None;
        Ok(())
    })
}

pub fn get_peers() -> Result<JsValue, JsValue> {
    MANAGER.with(|m| {
        let manager = m.borrow();
        let manager = manager
            .as_ref()
            .ok_or_else(|| JsValue::from_str("Not connected"))?;

        let peers = manager.borrow().get_peers();

        // Update local cache
        PEERS.with(|p| {
            *p.borrow_mut() = peers.clone();
        });

        serde_wasm_bindgen::to_value(&peers).map_err(|e| JsValue::from_str(&e.to_string()))
    })
}

pub async fn connect_to_peer(peer_id: String) -> Result<(), JsValue> {
    console::log_1(&format!("Connecting to peer: {}", peer_id).into());

    let manager = MANAGER.with(|m| {
        m.borrow()
            .as_ref()
            .ok_or_else(|| JsValue::from_str("Not connected"))
            .cloned()
    })?;

    manager
        .borrow_mut()
        .connect_to_peer(peer_id.clone())
        .await
        .map_err(|e| JsValue::from_str(&format!("Peer connection failed: {:?}", e)))?;

    console::log_1(&format!("Connected to peer: {}", peer_id).into());

    Ok(())
}

pub fn send_message(peer_id: String, message: String) -> Result<(), JsValue> {
    MANAGER.with(|m| {
        let manager = m.borrow();
        let manager = manager
            .as_ref()
            .ok_or_else(|| JsValue::from_str("Not connected"))?;

        manager
            .borrow()
            .send_message(peer_id.clone(), message.clone())
            .map_err(|e| JsValue::from_str(&format!("Send failed: {:?}", e)))?;

        console::log_1(&format!("Sent message to {}: {}", peer_id, message).into());

        Ok(())
    })
}

pub fn set_message_callback(callback: js_sys::Function) -> Result<(), JsValue> {
    MANAGER.with(|m| {
        let manager = m.borrow();
        let manager = manager
            .as_ref()
            .ok_or_else(|| JsValue::from_str("Not connected"))?;

        manager.borrow().set_on_message(move |peer_id, message| {
            console::log_1(&format!("Received from {}: {}", peer_id, message).into());

            let this = JsValue::null();
            let peer_id_js = JsValue::from_str(&peer_id);
            let message_js = JsValue::from_str(&message);
            let _ = callback.call2(&this, &peer_id_js, &message_js);
        });

        Ok(())
    })
}

pub fn set_peer_update_callback(callback: js_sys::Function) -> Result<(), JsValue> {
    MANAGER.with(|m| {
        let manager = m.borrow();
        let manager = manager
            .as_ref()
            .ok_or_else(|| JsValue::from_str("Not connected"))?;

        manager.borrow().set_on_peer_update(move |peers| {
            console::log_1(&format!("Peers updated: {} peer(s)", peers.len()).into());

            let this = JsValue::null();

            if let Ok(peers_js) = serde_wasm_bindgen::to_value(&peers) {
                let _ = callback.call1(&this, &peers_js);
            }
        });

        Ok(())
    })
}

/// Check if connected to signaling server
pub fn is_connected() -> bool {
    MANAGER.with(|m| {
        m.borrow()
            .as_ref()
            .map(|manager| manager.borrow().is_connected())
            .unwrap_or(false)
    })
}

pub fn get_device_name() -> Option<String> {
    MANAGER.with(|m| {
        m.borrow()
            .as_ref()
            .map(|manager| manager.borrow().device_name())
    })
}

pub fn get_peer_id() -> Option<String> {
    MANAGER.with(|m| {
        m.borrow()
            .as_ref()
            .and_then(|manager| manager.borrow().peer_id())
    })
}

pub async fn quick_connect(server_url: String) -> Result<String, JsValue> {
    let random_suffix: u32 = (js_sys::Math::random() * 10000.0) as u32;
    let device_name = format!("Device-{}", random_suffix);

    connect(server_url, device_name).await
}

pub async fn send_to_first_peer(message: String) -> Result<(), JsValue> {
    let peers_value = get_peers()?;
    let peers: Vec<Peer> = serde_wasm_bindgen::from_value(peers_value)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    if peers.is_empty() {
        return Err(JsValue::from_str("No peers available"));
    }

    let first_peer = &peers[0];

    connect_to_peer(first_peer.id.to_string()).await?;
    send_message(first_peer.id.to_string(), message)?;

    Ok(())
}
