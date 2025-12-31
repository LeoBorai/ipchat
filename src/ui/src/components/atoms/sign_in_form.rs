use leptos::prelude::*;

#[component]
pub fn SignInForm<F>(on_submit: F) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let (username, set_username) = signal(String::new());

    let handle_submit = move |_| {
        let username_value = username.get();
        if !username_value.trim().is_empty() {
            on_submit(username_value);
        }
    };

    let _is_disabled = move || username.get().trim().is_empty();

    view! {
        <div class="flex items-center justify-center min-h-screen from-blue-50 to-indigo-100">
            <div class="bg-white p-8 rounded-2xl shadow-xl w-96">
                <div class="flex items-center justify-center mb-6">
                    <div class="bg-indigo-100 p-3 rounded-full">
                        <svg
                            class="w-8 h-8 text-indigo-600"
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
                            <path d="M12 20h.01"></path>
                            <path d="M8.5 16.429a5 5 0 0 1 7 0"></path>
                            <path d="M5 12.859a10 10 0 0 1 14 0"></path>
                            <path d="M2 8.82a15 15 0 0 1 20 0"></path>
                        </svg>
                    </div>
                </div>
                <h1 class="text-2xl font-bold text-center mb-2 text-gray-800">
                    "Local Network Chat"
                </h1>
                <p class="text-center text-gray-600 mb-6 text-sm">"Connect to Rust P2P Server"</p>

                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">
                            "Username"
                        </label>
                        <input
                            type="text"
                            placeholder="Enter your username"
                            prop:value=username
                            on:input=move |ev| {
                                set_username.set(event_target_value(&ev));
                            }

                            on:keypress=move |ev| {
                                if ev.key() == "Enter" && !username.get().trim().is_empty() {
                                    handle_submit(ev);
                                }
                            }

                            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>

                    // on:click=handle_submit
                    // disabled=is_disabled
                    <button class="w-full bg-indigo-600 text-white py-3 rounded-lg font-semibold hover:bg-indigo-700 transition disabled:bg-gray-400 disabled:cursor-not-allowed">
                        "Connect to Network"
                    </button>
                </div>
            </div>
        </div>
    }
}
