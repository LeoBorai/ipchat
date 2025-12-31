use leptos::prelude::*;

use crate::components::atoms::sign_in_form::SignInForm;

#[component]
pub fn SignIn() -> impl IntoView {
    view! { <SignInForm /> }
}
