use leptos::prelude::*;

use crate::components::molecules::{Room, Sidebar};
use crate::components::organisms::ChatArea;

#[component]
pub fn Chat() -> impl IntoView {
    let username = RwSignal::new(String::from("myuser"));
    let messages = RwSignal::new(std::collections::HashMap::new());
    let is_sidebar_open = RwSignal::new(true);
    let is_connected = RwSignal::new(false);
    let server_url = RwSignal::new(String::from("ws://localhost:9000"));
    let connection_status = RwSignal::new(String::from("Disconnected"));
    let my_rooms = RwSignal::new(vec![]);
    let discovered_peers = RwSignal::new(vec![]);
    let active_room: RwSignal<Option<Room>> = RwSignal::new(None);
    let set_active_room = {
        let active_room = active_room.clone();
        move |room: Room| {
            let active_room = active_room.clone();
            active_room.set(Some(room.clone()));
        }
    };
    let open_sidebar = {
        let is_sidebar_open = is_sidebar_open.clone();
        move || {
            is_sidebar_open.set(true);
        }
    };
    let close_sidebar = {
        let is_sidebar_open = is_sidebar_open.clone();
        move || {
            is_sidebar_open.set(false);
        }
    };
    let request_peer_list = || {
        // Implementation to request peer list
    };
    let create_room = |room_name: String| {
        // Implementation to create a new room
    };

    view! {
        <div>
            <Sidebar
                username={username.read_only()}
                server_url={server_url.read_only()}
                is_sidebar_open={is_sidebar_open.read_only()}
                open_sidebar={open_sidebar}
                close_sidebar={close_sidebar}
                is_connected={is_connected.read_only()}
                set_active_room={set_active_room}
                connection_status={connection_status.read_only()}
                my_rooms={my_rooms.read_only()}
                discovered_peers={discovered_peers.read_only()}
                active_room={active_room.read_only()}
                request_peer_list={request_peer_list}
                // create_room={set_active_room}
            />
            <ChatArea
                username={username.read_only()}
                is_connected={is_connected.read_only()}
                active_room={active_room.read_only()}
                messages={messages.read_only()}
                send_message={|room_id, username, content| {
                    // Implementation to send message
                }}
            />
        </div>
    }
}
