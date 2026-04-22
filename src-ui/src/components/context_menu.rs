//! Right-click context menu for graph nodes.
//!
//! The menu is a `position: fixed` div rendered at app-root level (not
//! inside the SVG viewport) so it's unaffected by pan/zoom. Visibility is
//! driven by `AppState.context_menu`. Dismissal mirrors the menu-bar
//! pattern: outside click or Escape clears the signal.

use leptos::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::Element;

use crate::gadgets::{gadgets_for, GadgetDef};
use crate::state::{AppState, ContextMenuState, Node};

/// Approximate menu dimensions (Tailwind `w-64` = 256 px; height grows
/// with items). Used to clamp the on-screen position so the menu never
/// renders off-screen or under the right sidebar.
const MENU_WIDTH: f64 = 256.0;
const MENU_HEIGHT_ESTIMATE: f64 = 180.0;
/// Right sidebar width from `right_sidebar.rs` — duplicated here rather
/// than imported so this file stays self-contained.
const RIGHT_SIDEBAR_WIDTH: f64 = 288.0;

#[component]
pub fn NodeContextMenu() -> impl IntoView {
    let state = AppState::expect();

    // Dismiss on outside-click / Escape. Mirrors `menu_bar.rs:21–47`.
    Effect::new(move |_| {
        let Some(window) = web_sys::window() else { return };
        let mousedown =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
                if state.context_menu.get_untracked().is_none() {
                    return;
                }
                let Some(target) = ev.target() else { return };
                let Ok(el) = target.dyn_into::<Element>() else { return };
                if el.closest(".gadget-context-menu").ok().flatten().is_none() {
                    state.context_menu.set(None);
                }
            });
        let _ = window
            .add_event_listener_with_callback("mousedown", mousedown.as_ref().unchecked_ref());
        mousedown.forget();

        let keydown = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(
            move |ev: web_sys::KeyboardEvent| {
                if ev.key() == "Escape" {
                    state.context_menu.set(None);
                }
            },
        );
        let _ =
            window.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref());
        keydown.forget();
    });

    let visible = move || state.context_menu.get().is_some();

    view! {
        <Show when=visible fallback=|| ()>
            {move || state.context_menu.get().map(|ctx| view! {
                <MenuBody ctx=ctx />
            })}
        </Show>
    }
}

#[component]
fn MenuBody(ctx: ContextMenuState) -> impl IntoView {
    let state = AppState::expect();
    let gadgets = gadgets_for(ctx.entity_type);

    // Clamp position so the menu stays fully visible.
    let (vw, vh) = viewport_size();
    let x_max = vw - MENU_WIDTH - RIGHT_SIDEBAR_WIDTH - 8.0;
    let y_max = vh - MENU_HEIGHT_ESTIMATE - 8.0;
    let x = ctx.screen_pos.0.clamp(4.0, x_max.max(4.0));
    let y = ctx.screen_pos.1.clamp(4.0, y_max.max(4.0));

    let style = format!(
        "position: fixed; left: {x}px; top: {y}px; width: {MENU_WIDTH}px; z-index: 1000;"
    );

    // Entity label for the header — read from AppState rather than
    // storing it in ContextMenuState, since it may change (properties are
    // editable).
    let node_id = ctx.node_id;
    let header = move || -> String {
        state
            .nodes
            .with(|m| m.get(&node_id).cloned())
            .map(|n: Node| {
                let name = n
                    .properties
                    .values()
                    .next()
                    .cloned()
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| n.entity_type.label().to_string());
                format!("{} · {}", n.entity_type.label(), name)
            })
            .unwrap_or_else(|| "Node".to_string())
    };

    view! {
        <div
            class="gadget-context-menu \
                   rounded-md shadow-xl \
                   bg-white dark:bg-zinc-900 \
                   border border-zinc-200 dark:border-zinc-700 \
                   text-sm text-zinc-700 dark:text-zinc-200 \
                   overflow-hidden select-none"
            style=style
            on:contextmenu=|ev: web_sys::MouseEvent| ev.prevent_default()
        >
            <div class="px-3 py-2 border-b border-zinc-200 dark:border-zinc-700 \
                        text-[11px] uppercase tracking-wider \
                        text-zinc-500 dark:text-zinc-400">
                {header}
            </div>
            {if gadgets.is_empty() {
                view! {
                    <div class="px-3 py-3 text-zinc-400 dark:text-zinc-500 italic">
                        "No gadgets for this entity type"
                    </div>
                }.into_any()
            } else {
                view! {
                    <ul>
                        {gadgets.iter().map(|g| view! {
                            <GadgetMenuItem gadget=*g />
                        }).collect_view()}
                    </ul>
                }.into_any()
            }}
        </div>
    }
}

#[component]
fn GadgetMenuItem(gadget: &'static GadgetDef) -> impl IntoView {
    let state = AppState::expect();
    let on_click = move |_ev: web_sys::MouseEvent| {
        // Capture the target node before closing the menu (which zeros
        // ctx).
        let ctx = state.context_menu.get_untracked();
        state.context_menu.set(None);
        let Some(ctx) = ctx else { return };
        let node = state.nodes.with_untracked(|m| m.get(&ctx.node_id).cloned());
        let Some(node) = node else { return };
        (gadget.run)(state, &node);
    };

    view! {
        <li
            class="px-3 py-2 cursor-pointer \
                   hover:bg-zinc-100 dark:hover:bg-zinc-800 \
                   flex flex-col gap-0.5"
            on:mousedown=on_click
        >
            <span class="font-medium">{gadget.label}</span>
            <span class="text-[11px] text-zinc-500 dark:text-zinc-400">
                {gadget.description}
            </span>
        </li>
    }
}

/// Read `(innerWidth, innerHeight)` as f64, with safe fallbacks.
fn viewport_size() -> (f64, f64) {
    let Some(w) = web_sys::window() else {
        return (1920.0, 1080.0);
    };
    let vw = w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1920.0);
    let vh = w
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(1080.0);
    (vw, vh)
}
