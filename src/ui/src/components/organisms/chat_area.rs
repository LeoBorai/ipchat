use leptos::prelude::*;
use std::collections::HashMap;
use web_sys::js_sys;

use crate::components::molecules::Room;

#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    pub sender: String,
    pub content: String,
    pub timestamp: String,
}

#[component]
pub fn ChatArea(
    #[prop(into)] username: Signal<String>,
    #[prop(into)] is_connected: Signal<bool>,
    #[prop(into)] active_room: Signal<Option<Room>>,
    #[prop(into)] messages: Signal<HashMap<String, Vec<Message>>>,
    send_message: impl Fn(String, String, String) + Clone + Send + Sync + 'static,
) -> impl IntoView {
    let (new_message, set_new_message) = signal(String::new());
    let username_clone = username.clone();
    let handle_send_message = move || {
        let message_text = new_message.get();
        if !message_text.trim().is_empty() {
            if let Some(room) = active_room.get() {
                send_message(room.id, message_text, username.get_untracked());
                set_new_message.set(String::new());
            }
        }
    };

    let format_time = |timestamp: &str| -> String {
        if let Ok(ts) = timestamp.parse::<i64>() {
            let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ts as f64));
            let hours = date.get_hours();
            let minutes = date.get_minutes();
            format!("{:02}:{:02}", hours, minutes)
        } else {
            "00:00".to_string()
        }
    };

    let get_room_messages = move || -> Vec<Message> {
        if let Some(room) = active_room.get() {
            messages.get().get(&room.id).cloned().unwrap_or_default()
        } else {
            Vec::new()
        }
    };

    if let Some(room) = active_room.get() {
        return view! {
            <div class="flex-1 flex flex-col">
                {/* Chat Header */}
                <div class="bg-white border-b border-gray-200 p-4">
                    <h2 class="font-bold text-lg text-gray-800">{room.name.clone()}</h2>
                    <p class="text-sm text-gray-500">"Your room"</p>
                </div>
                {/* Messages */}
                <div class="flex-1 overflow-y-auto p-4 space-y-3">
                    <For
                        each=move || get_room_messages()
                        key=|msg| msg.timestamp.clone()
                        children=move |msg| {
                            let is_own = msg.sender == username_clone.get_untracked();
                            let is_system = msg.sender == "System";
                            view! {
                                <div
                                    class=format!(
                                        "flex {}",
                                        if is_own { "justify-end" } else { "justify-start" },
                                    )
                                >

                                    <div class=format!(
                                        "max-w-xs px-4 py-2 rounded-lg {}",
                                        if is_system {
                                            "bg-gray-200 text-gray-700 text-sm italic"
                                        } else if is_own {
                                            "bg-indigo-600 text-white"
                                        } else {
                                            "bg-white border border-gray-200 text-gray-800"
                                        },
                                    )>
                                        {(!is_own && !is_system)
                                            .then(|| {
                                                view! {
                                                    <p class="text-xs font-semibold mb-1 opacity-75">
                                                        {msg.sender.clone()}
                                                    </p>
                                                }
                                            })}

                                        <p class="text-sm">{msg.content.clone()}</p>
                                        <p class=format!(
                                            "text-xs mt-1 {}",
                                            if is_own { "text-indigo-200" } else { "text-gray-500" },
                                        )>

                                            {format_time(&msg.timestamp)}
                                        </p>
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>
                {/* Message Input */}
                <div class="bg-white border-t border-gray-200 p-4">
                    <div class="flex gap-2">
                        <input
                            type="text"
                            placeholder=move || {
                                if is_connected.get() {
                                    "Type a message..."
                                } else {
                                    "Connecting..."
                                }
                            }

                            prop:value=new_message
                            on:input=move |ev| {
                                set_new_message.set(event_target_value(&ev));
                            }

                            on:keypress={
                                let handle_send_message = handle_send_message.clone();

                                move |ev| {
                                    if ev.key() == "Enter" {
                                        handle_send_message();
                                    }
                                }
                            }

                            disabled=move || !is_connected.get()
                            class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 disabled:bg-gray-100"
                        />
                        <button
                            on:click=move |_| handle_send_message()
                            disabled=move || {
                                !is_connected.get() || new_message.get().trim().is_empty()
                            }

                            class="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition disabled:bg-gray-400 disabled:cursor-not-allowed"
                        >
                            <svg
                                class="w-5 h-5"
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
                                <path d="m22 2-7 20-4-9-9-4Z"></path>
                                <path d="M22 2 11 13"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }.into_any();
    }

    view! {
        <div class="flex-1 flex flex-col">
            <div class="flex-1 flex items-center justify-center text-gray-500">
                <div class="text-center">
                    <svg
                        class="w-16 h-16 mx-auto mb-4 opacity-50"
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
                    <p class="text-lg font-semibold mb-2">"No room selected"</p>
                    <p class="text-sm">
                        "Create a room or join a peer's room to start chatting"
                    </p>
                    {move || {
                        (!is_connected.get())
                            .then(|| {
                                view! {
                                    <p class="text-sm text-red-500 mt-2">
                                        "Waiting for server connection..."
                                    </p>
                                }
                            })
                    }}

                </div>
            </div>
        </div>
    }.into_any()
}
