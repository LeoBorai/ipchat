use leptos::prelude::{RwSignal, Set};

#[derive(Clone, Debug)]
pub struct SessionContext {
    pub username: RwSignal<Option<String>>,
}

impl Default for SessionContext {
    fn default() -> Self {
        Self {
            username: RwSignal::new(None),
        }
    }
}

impl SessionContext {
    pub fn set_username(&self, username: Option<String>) {
        self.username.set(username);
    }
}
