use crate::error::{IPChatWebRTCError, Result};
use crate::message::SignalingMessage;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CloseEvent, ErrorEvent, MessageEvent, WebSocket};

type MessageCallback = Box<dyn Fn(SignalingMessage)>;
type ErrorCallback = Box<dyn Fn(String)>;
type CloseCallback = Box<dyn Fn()>;

/// WebSocket-based signaling client
pub struct SignalingClient {
    ws: WebSocket,
    on_message: Rc<RefCell<Option<MessageCallback>>>,
    on_error: Rc<RefCell<Option<ErrorCallback>>>,
    on_close: Rc<RefCell<Option<CloseCallback>>>,
}

impl SignalingClient {
    /// Create a new signaling client
    pub fn new(url: &str) -> Result<SignalingClient> {
        let ws = WebSocket::new(url)
            .map_err(|e| IPChatWebRTCError::WebSocketError(format!("{:?}", e)))?;

        let on_message = Rc::new(RefCell::new(None));
        let on_error = Rc::new(RefCell::new(None));
        let on_close = Rc::new(RefCell::new(None));

        let client = SignalingClient {
            ws,
            on_message,
            on_error,
            on_close,
        };

        client.setup_handlers()?;

        Ok(client)
    }

    /// Send a signaling message
    pub fn send(&self, message: JsValue) -> Result<()> {
        let msg_str = message
            .as_string()
            .ok_or_else(|| IPChatWebRTCError::SerializationError("Invalid message".to_string()))?;

        self.ws
            .send_with_str(&msg_str)
            .map_err(|e| IPChatWebRTCError::WebSocketError(format!("{:?}", e)))?;

        Ok(())
    }

    /// Send a raw JSON string
    pub fn send_json(&self, json: &str) -> Result<()> {
        self.ws
            .send_with_str(json)
            .map_err(|e| IPChatWebRTCError::WebSocketError(format!("{:?}", e)))?;

        Ok(())
    }

    /// Register with the signaling server
    pub fn register(&self, device_name: String, room_id: Option<String>) -> Result<()> {
        let msg = SignalingMessage::Register {
            device_name,
            room_id,
        };

        let json = msg
            .to_json()
            .map_err(|e| IPChatWebRTCError::SerializationError(e.to_string()))?;

        self.send_json(&json)
    }

    /// Send WebRTC offer
    pub fn send_offer(&self, from: String, to: String, sdp: String) -> Result<()> {
        let msg = SignalingMessage::Offer { from, to, sdp };
        let json = msg
            .to_json()
            .map_err(|e| IPChatWebRTCError::SerializationError(e.to_string()))?;
        self.send_json(&json)
    }

    /// Send WebRTC answer
    pub fn send_answer(&self, from: String, to: String, sdp: String) -> Result<()> {
        let msg = SignalingMessage::Answer { from, to, sdp };
        let json = msg
            .to_json()
            .map_err(|e| IPChatWebRTCError::SerializationError(e.to_string()))?;
        self.send_json(&json)
    }

    /// Send ICE candidate
    pub fn send_ice_candidate(
        &self,
        from: String,
        to: String,
        candidate: String,
        sdp_mid: Option<String>,
        sdp_m_line_index: Option<u16>,
    ) -> Result<()> {
        let msg = SignalingMessage::IceCandidate {
            from,
            to,
            candidate,
            sdp_mid,
            sdp_m_line_index,
        };
        let json = msg
            .to_json()
            .map_err(|e| IPChatWebRTCError::SerializationError(e.to_string()))?;
        self.send_json(&json)
    }

    /// Close the connection
    pub fn close(&self) -> Result<()> {
        self.ws
            .close()
            .map_err(|e| IPChatWebRTCError::WebSocketError(format!("{:?}", e)))?;
        Ok(())
    }

    /// Check if connection is open
    pub fn is_open(&self) -> bool {
        self.ws.ready_state() == WebSocket::OPEN
    }
}

impl SignalingClient {
    fn setup_handlers(&self) -> Result<()> {
        // Setup onmessage handler
        let on_message = self.on_message.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Some(text) = e.data().as_string()
                && let Ok(msg) = SignalingMessage::from_json(&text)
                && let Some(callback) = on_message.borrow().as_ref()
            {
                callback(msg);
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        self.ws
            .set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        // Setup onerror handler
        let on_error = self.on_error.clone();
        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            if let Some(callback) = on_error.borrow().as_ref() {
                callback(e.message());
            }
        }) as Box<dyn FnMut(ErrorEvent)>);

        self.ws
            .set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        // Setup onclose handler
        let on_close = self.on_close.clone();
        let onclose_callback = Closure::wrap(Box::new(move |_e: CloseEvent| {
            if let Some(callback) = on_close.borrow().as_ref() {
                callback();
            }
        }) as Box<dyn FnMut(CloseEvent)>);

        self.ws
            .set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();

        Ok(())
    }

    /// Set message handler (internal use)
    pub fn set_on_message<F>(&self, callback: F)
    where
        F: Fn(SignalingMessage) + 'static,
    {
        *self.on_message.borrow_mut() = Some(Box::new(callback));
    }

    /// Set error handler (internal use)
    pub fn set_on_error<F>(&self, callback: F)
    where
        F: Fn(String) + 'static,
    {
        *self.on_error.borrow_mut() = Some(Box::new(callback));
    }

    /// Set close handler (internal use)
    pub fn set_on_close<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        *self.on_close.borrow_mut() = Some(Box::new(callback));
    }
}
