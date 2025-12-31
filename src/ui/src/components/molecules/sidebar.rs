use leptos::prelude::*;

use ipchat::proto::Room;

use crate::hooks::chat_websocket::{
    use_active_room, use_chat_ws, use_connection_status, use_is_connected, use_nodes, use_rooms,
};
use crate::hooks::node::use_server_url;
use crate::hooks::ui::{use_is_sidebar_open, use_toggle_sidebar};

#[component]
pub fn Sidebar(#[prop(into)] username: Signal<String>) -> impl IntoView {
    let server_url = use_server_url();
    let connection_status = use_connection_status();
    let rooms = use_rooms();
    let nodes = use_nodes();
    let active_room = use_active_room();
    let is_sidebar_open = use_is_sidebar_open();
    let toggle_sidebar = use_toggle_sidebar();
    let is_connected = use_is_connected();
    let (new_room_name, set_new_room_name) = signal(String::new());
    let (show_create_room, set_show_create_room) = signal(false);

    let handle_create_room = move || {
        let room_name = new_room_name.get();
        if !room_name.trim().is_empty() {
            let chat_ws_ctx = use_chat_ws();
            leptos::task::spawn_local(async move {
                if let Err(err) = chat_ws_ctx.create_room(room_name).await {
                    leptos::logging::error!("Failed to create room: {:?}", err);
                }
            });
            set_new_room_name.set(String::new());
            set_show_create_room.set(false);
        }
    };

    view! {
        <>
            <button
                on:click=move |_| toggle_sidebar()
                class="fixed bottom-4 left-4 z-50 p-2 bg-indigo-600 text-white rounded-lg shadow-lg hover:bg-indigo-700 transition md:hidden"
                title=move || {
                    if is_sidebar_open.get() { "Close sidebar" } else { "Open sidebar" }
                }
            >

                <svg
                    class=move || {
                        format!(
                            "w-5 h-5 transition-transform {}",
                            if is_sidebar_open.get() { "rotate-0" } else { "rotate-180" },
                        )
                    }

                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <rect width="18" height="18" x="3" y="3" rx="2"></rect>
                    <path d="M9 3v18"></path>
                </svg>
            </button>
            <div
                id="sidebar"
                class=move || {
                    format!(
                        "w-80 bg-white border-r border-gray-200 flex flex-col transition-transform duration-300 {} fixed md:relative h-full z-40",
                        if is_sidebar_open.get() { "translate-x-0" } else { "-translate-x-full" },
                    )
                }
            >
                <div id="user-info" class="p-4 border-b border-gray-200 bg-indigo-50">
                    <div class="flex items-center justify-between mb-2">
                        <div class="flex items-center gap-3">
                            <div class="bg-indigo-600 p-2 rounded-full">
                                <svg
                                    class="w-5 h-5 text-white"
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                >
                                    <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
                                    <circle cx="12" cy="7" r="4"></circle>
                                </svg>
                            </div>
                            <div>
                                <p class="font-semibold text-gray-800">{username}</p>
                                <p class="text-xs text-gray-600">{server_url}</p>
                            </div>
                        </div>
                        <button
                            // on:click=move |_| request_peer_list()
                            class="p-2 hover:bg-indigo-100 rounded-full transition"
                            title="Refresh peers"
                        >
                            <svg
                                class="w-4 h-4 text-indigo-600"
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
                                <path d="M3 3v5h5"></path>
                                <path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"></path>
                                <path d="M16 16h5v5"></path>
                            </svg>
                        </button>
                    </div>
                    <div class="flex items-center gap-2">
                        <div class=move || {
                            format!(
                                "w-2 h-2 rounded-full {}",
                                if is_connected.get() { "bg-green-500" } else { "bg-red-500" },
                            )
                        }></div>

                        <p class="text-xs text-gray-600">
                            {move || format!("{}", connection_status.get())}
                        </p>
                    </div>
                </div>
                <div id="rooms" class="flex-1 overflow-y-auto">
                    <div class="p-4">
                        <div class="flex items-center justify-between mb-3">
                            <h3 class="font-semibold text-gray-700 flex items-center gap-2">
                                <svg
                                    class="w-4 h-4"
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                >
                                    <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
                                </svg>
                                "Rooms"
                            </h3>
                            <button
                                id="create-room-btn"
                                on:click=move |_| set_show_create_room.set(!show_create_room.get())
                                class="p-1 hover:bg-gray-100 rounded"
                            >
                                <svg
                                    class="w-4 h-4 text-indigo-600"
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                >
                                    <path d="M5 12h14"></path>
                                    <path d="M12 5v14"></path>
                                </svg>
                            </button>
                        </div>
                        <For
                            each=move || rooms.get()
                            key=|room| room.id
                            children={
                                let set_active_room = {
                                    let chat_ws_ctx = use_chat_ws();
                                    move |room: Room| {
                                        chat_ws_ctx.set_active_room(Some(room));
                                    }
                                };
                                move |room: Room| {
                                    let room_clone = room.clone();
                                    let active = active_room.get();
                                    let is_active = active
                                        .as_ref()
                                        .map(|r| r.id == room.id)
                                        .unwrap_or(false);

                                    view! {
                                        <button
                                            on:click={
                                                let room = room_clone.clone();
                                                let set_active_room = set_active_room.clone();
                                                move |_| { set_active_room(room.clone()) }
                                            }
                                            class=format!(
                                                "cursor-pointer w-full text-left p-3 rounded-lg mb-2 transition {}",
                                                if is_active {
                                                    "bg-indigo-100 border border-indigo-300"
                                                } else {
                                                    "hover:bg-gray-50 border border-transparent"
                                                },
                                            )
                                        >
                                            <p class="font-medium text-gray-800">{room.name.clone()}</p>
                                            <p class="text-xs text-gray-500">
                                                {format!("{} participant(s)", room.participants.len())}
                                            </p>
                                        </button>
                                    }
                                }
                            }
                        />

                        <Show when=move || show_create_room.get()>
                            <div class="mb-3 flex gap-2">
                                <input
                                    type="text"
                                    class="flex-1 px-3 py-2 border border-gray-300 rounded text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
                                    placeholder="Room name"
                                    prop:value=new_room_name
                                    on:input={
                                        let set_new_room_name = set_new_room_name;
                                        move |ev| {
                                            set_new_room_name.set(event_target_value(&ev));
                                        }
                                    }
                                    on:keypress={
                                        let handle_create_room = handle_create_room;
                                        move |ev| {
                                            if ev.key() == "Enter" {
                                                handle_create_room();
                                            }
                                        }
                                    }
                                />
                                <button
                                    on:click={
                                        let handle_create_room = handle_create_room;
                                        move |_| handle_create_room()
                                    }
                                    class="px-3 py-2 bg-indigo-600 text-white rounded text-sm hover:bg-indigo-700"
                                >
                                    "Add"
                                </button>
                            </div>
                        </Show>
                    </div>
                    <div class="p-4 border-t border-gray-200">
                        <h3 class="font-semibold text-gray-700 mb-3 flex items-center gap-2">
                            <svg
                                class="w-4 h-4"
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path>
                                <circle cx="9" cy="7" r="4"></circle>
                                <path d="M22 21v-2a4 4 0 0 0-3-3.87"></path>
                                <path d="M16 3.13a4 4 0 0 1 0 7.75"></path>
                            </svg>
                            {move || { format!("Nodes ({})", nodes.get().len()) }}
                        </h3>
                        {move || {
                            let nodes = nodes.get();
                            if nodes.is_empty() {
                                view! {
                                    <p class="text-sm text-gray-500">
                                        {if is_connected.get() {
                                            "No nodes discovered yet..."
                                        } else {
                                            "Connect to see nodes"
                                        }}

                                    </p>
                                }
                                    .into_any()
                            } else {
                                nodes
                                    .into_iter()
                                    .map(|node| {
                                        view! {
                                            <div class="mb-4 p-3 bg-gray-50 rounded-lg">
                                                <div class="flex items-center gap-2 mb-2">
                                                    <div class="w-2 h-2 bg-green-500 rounded-full"></div>
                                                    <p class="font-medium text-sm text-gray-800">
                                                        {node.ip.to_string()}
                                                    </p>
                                                </div>
                                            </div>
                                        }
                                    })
                                    .collect_view()
                                    .into_any()
                            }
                        }}
                    </div>
                </div>
            </div>
        </>
    }
}
