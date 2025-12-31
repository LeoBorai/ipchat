use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::{components::templates::Chat, hooks::session::use_username};

#[component]
pub fn Home() -> impl IntoView {
    let username = use_username();
    let navigate = use_navigate();

    Effect::new(move || {
        if username.get().is_none() {
            navigate("/signin", Default::default());
        }
    });

    view! { <Chat /> }
}
