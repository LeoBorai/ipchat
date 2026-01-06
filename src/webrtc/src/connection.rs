use std::cell::RefCell;
use std::rc::Rc;

use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{
    RtcConfiguration, RtcDataChannel, RtcDataChannelEvent, RtcIceCandidateInit, RtcPeerConnection,
    RtcPeerConnectionIceEvent, RtcSdpType, RtcSessionDescriptionInit,
};

use crate::error::{IPChatWebRTCError, Result};
use crate::peer::PeerId;
use crate::types::{ConnectionState, IceServerConfig};

type DataCallback = Box<dyn Fn(Vec<u8>)>;
type StateCallback = Box<dyn Fn(ConnectionState)>;
type IceCandidateCallback = Box<dyn Fn(String, Option<String>, Option<u16>)>;

/// WebRTC peer connection
pub struct PeerConnection {
    peer_id: PeerId,
    rtc_connection: RtcPeerConnection,
    data_channel: Option<RtcDataChannel>,
    on_data: Rc<RefCell<Option<DataCallback>>>,
    on_state_change: Rc<RefCell<Option<StateCallback>>>,
    on_ice_candidate: Rc<RefCell<Option<IceCandidateCallback>>>,
    state: Rc<RefCell<ConnectionState>>,
}

impl PeerConnection {
    /// Create a new peer connection
    pub fn new(peer_id: String, ice_servers: JsValue) -> Result<PeerConnection> {
        let peer_id = PeerId::from(peer_id);
        let config = RtcConfiguration::new();

        if !ice_servers.is_null() && !ice_servers.is_undefined() {
            config.set_ice_servers(&ice_servers);
        }

        let rtc_connection = RtcPeerConnection::new_with_configuration(&config)
            .map_err(|e| IPChatWebRTCError::WebRtcError(format!("{:?}", e)))?;

        let connection = PeerConnection {
            peer_id,
            rtc_connection,
            data_channel: None,
            on_data: Rc::new(RefCell::new(None)),
            on_state_change: Rc::new(RefCell::new(None)),
            on_ice_candidate: Rc::new(RefCell::new(None)),
            state: Rc::new(RefCell::new(ConnectionState::Disconnected)),
        };

        connection.setup_handlers()?;

        Ok(connection)
    }

    /// Create a data channel (as initiator)
    pub fn create_data_channel(&mut self, label: &str) -> Result<()> {
        let channel = self.rtc_connection.create_data_channel(label);

        self.setup_data_channel_handlers(&channel)?;
        self.data_channel = Some(channel);

        Ok(())
    }

    /// Create and send an offer
    pub async fn create_offer(&self) -> Result<String> {
        let offer = wasm_bindgen_futures::JsFuture::from(self.rtc_connection.create_offer())
            .await
            .map_err(|e| IPChatWebRTCError::WebRtcError(format!("{:?}", e)))?;

        let offer_desc = offer.unchecked_into::<RtcSessionDescriptionInit>();

        wasm_bindgen_futures::JsFuture::from(
            self.rtc_connection.set_local_description(&offer_desc),
        )
        .await
        .map_err(|e| IPChatWebRTCError::WebRtcError(format!("{:?}", e)))?;

        let sdp = offer_desc.get_sdp();
        Ok(sdp.unwrap_throw())
    }

    /// Set remote offer and create answer
    pub async fn create_answer(&self, offer_sdp: String) -> Result<String> {
        let mut offer_desc = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
        offer_desc.sdp(&offer_sdp);

        wasm_bindgen_futures::JsFuture::from(
            self.rtc_connection.set_remote_description(&offer_desc),
        )
        .await
        .map_err(|e| IPChatWebRTCError::WebRtcError(format!("{:?}", e)))?;

        let answer = wasm_bindgen_futures::JsFuture::from(self.rtc_connection.create_answer())
            .await
            .map_err(|e| IPChatWebRTCError::WebRtcError(format!("{:?}", e)))?;

        let answer_desc = answer.unchecked_into::<RtcSessionDescriptionInit>();

        wasm_bindgen_futures::JsFuture::from(
            self.rtc_connection.set_local_description(&answer_desc),
        )
        .await
        .map_err(|e| IPChatWebRTCError::WebRtcError(format!("{:?}", e)))?;

        let sdp = answer_desc.get_sdp();
        Ok(sdp.unwrap_throw())
    }

    /// Set remote answer
    pub async fn set_remote_answer(&self, answer_sdp: String) -> Result<()> {
        let answer_desc = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
        answer_desc.set_sdp(&answer_sdp);

        wasm_bindgen_futures::JsFuture::from(
            self.rtc_connection.set_remote_description(&answer_desc),
        )
        .await
        .map_err(|e| IPChatWebRTCError::WebRtcError(format!("{:?}", e)))?;

        Ok(())
    }

    /// Add ICE candidate
    pub async fn add_ice_candidate(
        &self,
        candidate: String,
        sdp_mid: Option<String>,
        sdp_m_line_index: Option<u16>,
    ) -> Result<()> {
        let mut ice_candidate = RtcIceCandidateInit::new(&candidate);

        if let Some(mid) = sdp_mid {
            ice_candidate.sdp_mid(Some(&mid));
        }

        if let Some(line_index) = sdp_m_line_index {
            ice_candidate.sdp_m_line_index(Some(line_index));
        }

        wasm_bindgen_futures::JsFuture::from(
            self.rtc_connection
                .add_ice_candidate_with_opt_rtc_ice_candidate_init(Some(&ice_candidate)),
        )
        .await
        .map_err(|e| IPChatWebRTCError::WebRtcError(format!("{:?}", e)))?;

        Ok(())
    }

    /// Send data through the data channel
    pub fn send(&self, data: &[u8]) -> Result<()> {
        if let Some(channel) = &self.data_channel {
            channel
                .send_with_u8_array(data)
                .map_err(|e| IPChatWebRTCError::ConnectionError(format!("{:?}", e)))?;
            Ok(())
        } else {
            Err(IPChatWebRTCError::ConnectionError(
                "Data channel not available".to_string(),
            ))
        }
    }

    /// Send a text message
    pub fn send_text(&self, text: &str) -> Result<()> {
        self.send(text.as_bytes())
    }

    /// Close the connection
    pub fn close(&self) {
        if let Some(channel) = &self.data_channel {
            channel.close();
        }
        self.rtc_connection.close();
    }

    /// Get current connection state
    pub fn state(&self) -> ConnectionState {
        *self.state.borrow()
    }

    /// Get peer ID
    pub fn peer_id(&self) -> String {
        self.peer_id.to_string()
    }
}

impl PeerConnection {
    fn setup_handlers(&self) -> Result<()> {
        let on_ice = self.on_ice_candidate.clone();
        let onicecandidate = Closure::wrap(Box::new(move |e: RtcPeerConnectionIceEvent| {
            if let Some(candidate) = e.candidate()
                && let Some(callback) = on_ice.borrow().as_ref()
            {
                callback(
                    candidate.candidate(),
                    candidate.sdp_mid(),
                    candidate.sdp_m_line_index(),
                );
            }
        }) as Box<dyn FnMut(RtcPeerConnectionIceEvent)>);

        self.rtc_connection
            .set_onicecandidate(Some(onicecandidate.as_ref().unchecked_ref()));
        onicecandidate.forget();

        // Connection state change handler
        let state = self.state.clone();
        let on_state = self.on_state_change.clone();
        let onconnectionstatechange = Closure::wrap(Box::new(move || {
            // Note: Would need to get actual state from rtc_connection
            // This is simplified for demonstration
            if let Some(callback) = on_state.borrow().as_ref() {
                callback(*state.borrow());
            }
        }) as Box<dyn FnMut()>);

        self.rtc_connection
            .set_onconnectionstatechange(Some(onconnectionstatechange.as_ref().unchecked_ref()));
        onconnectionstatechange.forget();

        // Data channel handler (for receiving end)
        let _on_data = self.on_data.clone();
        let ondatachannel = Closure::wrap(Box::new(move |e: RtcDataChannelEvent| {
            let _channel = e.channel();
            // Setup handlers for received channel
            // (simplified, would need full implementation)
        }) as Box<dyn FnMut(RtcDataChannelEvent)>);

        self.rtc_connection
            .set_ondatachannel(Some(ondatachannel.as_ref().unchecked_ref()));
        ondatachannel.forget();

        Ok(())
    }

    fn setup_data_channel_handlers(&self, channel: &RtcDataChannel) -> Result<()> {
        let on_data = self.on_data.clone();

        let onmessage = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
            if let Ok(array_buffer) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                let data = uint8_array.to_vec();

                if let Some(callback) = on_data.borrow().as_ref() {
                    callback(data);
                }
            }
        }) as Box<dyn FnMut(web_sys::MessageEvent)>);

        channel.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();

        Ok(())
    }

    /// Set data received callback
    pub fn set_on_data<F>(&self, callback: F)
    where
        F: Fn(Vec<u8>) + 'static,
    {
        *self.on_data.borrow_mut() = Some(Box::new(callback));
    }

    /// Set state change callback
    pub fn set_on_state_change<F>(&self, callback: F)
    where
        F: Fn(ConnectionState) + 'static,
    {
        *self.on_state_change.borrow_mut() = Some(Box::new(callback));
    }

    /// Set ICE candidate callback
    pub fn set_on_ice_candidate<F>(&self, callback: F)
    where
        F: Fn(String, Option<String>, Option<u16>) + 'static,
    {
        *self.on_ice_candidate.borrow_mut() = Some(Box::new(callback));
    }
}

/// Helper to convert IceServerConfig to JS value
pub fn ice_servers_to_js(servers: &[IceServerConfig]) -> Result<JsValue> {
    let array = Array::new();

    for server in servers {
        let obj = Object::new();

        let urls = Array::new();
        for url in &server.urls {
            urls.push(&JsValue::from_str(url));
        }

        Reflect::set(&obj, &JsValue::from_str("urls"), &urls)
            .map_err(|e| IPChatWebRTCError::JsError(format!("{:?}", e)))?;

        if let Some(username) = &server.username {
            Reflect::set(
                &obj,
                &JsValue::from_str("username"),
                &JsValue::from_str(username),
            )
            .map_err(|e| IPChatWebRTCError::JsError(format!("{:?}", e)))?;
        }

        if let Some(credential) = &server.credential {
            Reflect::set(
                &obj,
                &JsValue::from_str("credential"),
                &JsValue::from_str(credential),
            )
            .map_err(|e| IPChatWebRTCError::JsError(format!("{:?}", e)))?;
        }

        array.push(&obj);
    }

    Ok(array.into())
}
