//! Themed HTML menu bar (AltEgo / File / Tools / Help) with dropdown menus.
//!
//! State: `AppState.menu_open` holds `Option<&'static str>`. Click toggles,
//! hover-switches while another menu is already open, `Escape` or click
//! outside closes.

use leptos::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;

use crate::components::icons::IconCheck;
use crate::state::{AppState, GraphFile, Theme};
use crate::tauri_bridge;

#[component]
pub fn MenuBar() -> impl IntoView {
    let state = AppState::expect();

    // Close the open menu on clicks outside any `.menu-root` or on Escape.
    Effect::new(move |_| {
        let Some(window) = web_sys::window() else { return };
        // mousedown anywhere
        let mousedown = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
            if state.menu_open.get_untracked().is_none() {
                return;
            }
            let Some(target) = ev.target() else { return };
            let Ok(el) = target.dyn_into::<Element>() else { return };
            if el.closest(".menu-root").ok().flatten().is_none() {
                state.menu_open.set(None);
            }
        });
        let _ = window
            .add_event_listener_with_callback("mousedown", mousedown.as_ref().unchecked_ref());
        mousedown.forget();

        // Escape key
        let keydown = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
            if ev.key() == "Escape" {
                state.menu_open.set(None);
            }
        });
        let _ = window
            .add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref());
        keydown.forget();
    });

    view! {
        <div class="h-8 shrink-0 flex items-stretch \
                    bg-zinc-100 dark:bg-zinc-950 \
                    border-b border-zinc-200 dark:border-zinc-800 \
                    text-sm text-zinc-700 dark:text-zinc-300">
            <AltEgoMenu />
            <FileMenu />
            <ToolsMenu />
            <HelpMenu />
        </div>
    }
}

// --------------------------------------------------------------------------
// Individual menus
// --------------------------------------------------------------------------

#[component]
fn AltEgoMenu() -> impl IntoView {
    let on_about = move |_| {
        if let Some(w) = web_sys::window() {
            let _ = w.alert_with_message("AltEgo 2 — Libre OSINT Platform\n\nPhase 1 UI prototype.");
        }
    };
    let on_quit = move |_| spawn_local(async { tauri_bridge::close().await });
    view! {
        <Menu id="altego" label="AltEgo">
            <MenuItem label="About AltEgo 2" on_click=Box::new(on_about) />
            <MenuSeparator />
            <MenuItem label="Quit" on_click=Box::new(on_quit) />
        </Menu>
    }
}

#[component]
fn FileMenu() -> impl IntoView {
    let state = AppState::expect();

    let on_new = move |_| state.clear();

    // "Save": write directly to `current_file_path` if we have one, otherwise
    // fall back to Save As (dialog).
    let on_save = move |_| {
        let maybe_path = state.current_file_path.get_untracked();
        let graph = state.snapshot_graph_file();
        spawn_local(async move {
            let json = match serde_json::to_string_pretty(&graph) {
                Ok(s) => s,
                Err(e) => {
                    alert(&format!("Failed to serialise graph:\n{e}"));
                    return;
                }
            };
            match maybe_path {
                Some(path) => match tauri_bridge::save_graph_to(path, json).await {
                    Ok(()) => { /* saved silently */ }
                    Err(e) => alert(&format!("Save failed:\n{e}")),
                },
                None => match tauri_bridge::save_graph(json).await {
                    Ok(Some(path)) => state.current_file_path.set(Some(path)),
                    Ok(None) => { /* user cancelled */ }
                    Err(e) => alert(&format!("Save failed:\n{e}")),
                },
            }
        });
    };

    // "Save As…": always show the dialog, then remember the returned path.
    let on_save_as = move |_| {
        let graph = state.snapshot_graph_file();
        spawn_local(async move {
            let json = match serde_json::to_string_pretty(&graph) {
                Ok(s) => s,
                Err(e) => {
                    alert(&format!("Failed to serialise graph:\n{e}"));
                    return;
                }
            };
            match tauri_bridge::save_graph(json).await {
                Ok(Some(path)) => state.current_file_path.set(Some(path)),
                Ok(None) => { /* user cancelled */ }
                Err(e) => alert(&format!("Save failed:\n{e}")),
            }
        });
    };

    let on_open = move |_| {
        spawn_local(async move {
            match tauri_bridge::load_graph().await {
                Ok(Some(loaded)) => match serde_json::from_str::<GraphFile>(&loaded.contents) {
                    Ok(graph) => {
                        state.load_graph_file(graph);
                        state.current_file_path.set(Some(loaded.path));
                    }
                    Err(e) => alert(&format!("Failed to parse {}:\n{}", loaded.path, e)),
                },
                Ok(None) => { /* cancelled */ }
                Err(e) => alert(&format!("Open failed:\n{e}")),
            }
        });
    };

    view! {
        <Menu id="file" label="File">
            <MenuItem label="New Graph" on_click=Box::new(on_new) />
            <MenuItem label="Open…" on_click=Box::new(on_open) />
            <MenuSeparator />
            <MenuItem label="Save" on_click=Box::new(on_save) />
            <MenuItem label="Save As…" on_click=Box::new(on_save_as) />
        </Menu>
    }
}

fn alert(msg: &str) {
    if let Some(w) = web_sys::window() {
        let _ = w.alert_with_message(msg);
    }
}

#[component]
fn ToolsMenu() -> impl IntoView {
    let state = AppState::expect();
    let is_dark: Signal<bool> = Signal::derive(move || state.theme.get() == Theme::Dark);
    let is_locked: Signal<bool> = Signal::derive(move || state.viewport.get().locked);
    let toggle_theme = move |_| state.toggle_theme();
    let toggle_lock = move |_| state.toggle_lock();
    view! {
        <Menu id="tools" label="Tools">
            <MenuItem
                label="Toggle Theme"
                checked=is_dark
                on_click=Box::new(toggle_theme)
            />
            <MenuItem
                label="Toggle Lock Canvas"
                checked=is_locked
                on_click=Box::new(toggle_lock)
            />
        </Menu>
    }
}

#[component]
fn HelpMenu() -> impl IntoView {
    view! {
        <Menu id="help" label="Help">
            <MenuItem label="Documentation" disabled=true on_click=Box::new(|_| {}) />
            <MenuItem label="Keyboard Shortcuts" disabled=true on_click=Box::new(|_| {}) />
        </Menu>
    }
}

// --------------------------------------------------------------------------
// Generic menu primitives
// --------------------------------------------------------------------------

#[component]
fn Menu(id: &'static str, label: &'static str, children: ChildrenFn) -> impl IntoView {
    let state = AppState::expect();
    let open = move || state.menu_open.get() == Some(id);

    let on_click = move |_| {
        state.menu_open.update(|cur| {
            *cur = if *cur == Some(id) { None } else { Some(id) };
        });
    };
    let on_enter = move |_| {
        // Only switch if something is already open (hover-switch behaviour).
        if state.menu_open.get_untracked().is_some() {
            state.menu_open.set(Some(id));
        }
    };

    let btn_class = move || {
        if open() {
            "menu-trigger h-full px-3 flex items-center cursor-default \
             bg-zinc-200 dark:bg-zinc-800 text-zinc-900 dark:text-zinc-50"
                .to_string()
        } else {
            "menu-trigger h-full px-3 flex items-center cursor-default \
             hover:bg-zinc-200 dark:hover:bg-zinc-800"
                .to_string()
        }
    };

    view! {
        <div class="menu-root relative h-full">
            <button
                class=btn_class
                on:click=on_click
                on:mouseenter=on_enter
            >
                {label}
            </button>
            <Show when=open>
                <div class="absolute left-0 top-full z-50 min-w-[220px] py-1 \
                            bg-white dark:bg-zinc-900 \
                            border border-zinc-200 dark:border-zinc-800 \
                            shadow-lg rounded-sm">
                    {children()}
                </div>
            </Show>
        </div>
    }
}

#[component]
fn MenuItem(
    label: &'static str,
    on_click: Box<dyn Fn(web_sys::MouseEvent) + 'static>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] checked: Option<Signal<bool>>,
) -> impl IntoView {
    let state = AppState::expect();
    let handle = move |ev: web_sys::MouseEvent| {
        if disabled {
            return;
        }
        on_click(ev);
        state.menu_open.set(None);
    };
    let class = if disabled {
        "flex items-center gap-2 px-3 py-1.5 text-sm text-zinc-400 dark:text-zinc-600 cursor-not-allowed"
    } else {
        "flex items-center gap-2 px-3 py-1.5 text-sm \
         text-zinc-700 dark:text-zinc-200 \
         hover:bg-orange-500 hover:text-white cursor-default"
    };
    let is_checked = move || checked.map(|s| s.get()).unwrap_or(false);
    view! {
        <div class=class on:mousedown=handle>
            <span class="w-4 h-4 flex items-center justify-center">
                <Show when=is_checked>
                    <IconCheck class="w-4 h-4".to_string() />
                </Show>
            </span>
            <span>{label}</span>
        </div>
    }
}

#[component]
fn MenuSeparator() -> impl IntoView {
    view! {
        <div class="h-px my-1 bg-zinc-200 dark:bg-zinc-800"></div>
    }
}
