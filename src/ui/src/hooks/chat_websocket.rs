use leptos::prelude::{Get, Signal, expect_context};

use crate::contexts::chat_websocket_context::{ChatWebSocketContext, Message, Peer, Room};
use crate::services::chat_websocket_service::ConnectionStatus;

pub fn use_chat_ws() -> ChatWebSocketContext {
    expect_context::<ChatWebSocketContext>()
}

pub fn use_is_connected() -> Signal<bool> {
    Signal::derive(move || use_chat_ws().is_connected.get())
}

pub fn use_connection_status() -> Signal<ConnectionStatus> {
    Signal::derive(move || use_chat_ws().connection_status.get())
}

pub fn use_my_rooms() -> Signal<Vec<Room>> {
    Signal::derive(move || use_chat_ws().my_rooms.get())
}

pub fn use_discovered_peers() -> Signal<Vec<Peer>> {
    Signal::derive(move || use_chat_ws().discovered_peers.get())
}

pub fn use_active_room() -> Signal<Option<Room>> {
    Signal::derive(move || use_chat_ws().active_room.get())
}

pub fn use_messages() -> Signal<std::collections::HashMap<String, Vec<Message>>> {
    Signal::derive(move || use_chat_ws().messages.get())
}
