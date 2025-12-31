mod components;
mod contexts;
mod hooks;
mod pages;
mod services;
mod utils;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::contexts::chat_websocket_context::ChatWebSocketContext;
use crate::contexts::node_context::NodeContext;
use crate::contexts::session_context::SessionContext;
use crate::contexts::ui_context::UIContext;
use crate::pages::home::Home;
use crate::pages::signin::SignIn;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(NodeContext::default());
    provide_context(ChatWebSocketContext::default());
    provide_context(SessionContext::default());
    provide_context(UIContext::default());

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
        <Title text="IPChat" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>
                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>
            <Router>
                <Routes fallback=|| view! { NotFound }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/signin") view=SignIn />
                </Routes>
            </Router>
        </ErrorBoundary>
    }
}
