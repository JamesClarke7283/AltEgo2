//! Theme initialisation + persistence.
//!
//! On startup we read `localStorage["altego.theme"]`, falling back to the
//! system `prefers-color-scheme` media query. An `Effect` watches `theme`
//! and (a) toggles the `dark` class on `<html>` so Tailwind's
//! `@custom-variant dark` kicks in, (b) writes the new value back to
//! localStorage so the choice survives reloads.

use leptos::prelude::*;

use crate::state::{AppState, Theme};

const STORAGE_KEY: &str = "altego.theme";

/// Call once from `<App/>` right after `provide_context(AppState::new())`.
pub fn init(state: AppState) {
    // Seed theme from stored preference OR system.
    let initial = load_stored_theme().unwrap_or_else(system_prefers_dark_or_light);
    state.theme.set(initial);

    // Keep <html> class + localStorage in sync.
    Effect::new(move |_| {
        let theme = state.theme.get();
        apply_html_class(theme);
        save_stored_theme(theme);
    });
}

fn load_stored_theme() -> Option<Theme> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let raw = storage.get_item(STORAGE_KEY).ok()??;
    Theme::from_str(&raw)
}

fn save_stored_theme(theme: Theme) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(STORAGE_KEY, theme.as_str());
        }
    }
}

fn system_prefers_dark_or_light() -> Theme {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(mql)) = window.match_media("(prefers-color-scheme: dark)") {
            if mql.matches() {
                return Theme::Dark;
            }
            return Theme::Light;
        }
    }
    // Screenshot shows dark mode as the primary style, so default to it.
    Theme::Dark
}

fn apply_html_class(theme: Theme) {
    if let Some(window) = web_sys::window() {
        if let Some(doc) = window.document() {
            if let Some(root) = doc.document_element() {
                let list = root.class_list();
                match theme {
                    Theme::Dark => {
                        let _ = list.add_1("dark");
                    }
                    Theme::Light => {
                        let _ = list.remove_1("dark");
                    }
                }
            }
        }
    }
}
