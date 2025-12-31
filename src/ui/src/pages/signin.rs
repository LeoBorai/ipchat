use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::atoms::sign_in_form::SignInForm;
use crate::hooks::session::use_username;

#[component]
pub fn SignIn() -> impl IntoView {
    let username = use_username();
    let navigate = use_navigate();

    Effect::new(move || {
        if username.get().is_some() {
            navigate("/", Default::default());
        }
    });

    view! { <SignInForm /> }
}
