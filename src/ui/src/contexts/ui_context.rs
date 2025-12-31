use leptos::prelude::{Get, RwSignal, Set};

#[derive(Clone, Debug)]
pub struct UIContext {
    pub is_sidebar_open: RwSignal<bool>,
}

impl Default for UIContext {
    fn default() -> Self {
        Self {
            is_sidebar_open: RwSignal::new(true),
        }
    }
}

impl UIContext {
    pub fn toggle_sidebar(&self) {
        let current = self.is_sidebar_open.get();
        self.is_sidebar_open.set(!current);
    }
}
