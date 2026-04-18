//! Custom frameless title bar.
//!
//! Background is the drag-region; buttons are marked `.no-drag` so they stay
//! clickable. Double-clicking the drag region toggles maximize.

use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::components::icons::{IconSunburst, IconWinClose, IconWinMaximize, IconWinMinimize};
use crate::state::AppState;
use crate::tauri_bridge;

#[component]
pub fn TitleBar() -> impl IntoView {
    let state = AppState::expect();

    let on_minimize = move |_| spawn_local(async { tauri_bridge::minimize().await });
    let on_maximize = move |_| spawn_local(async { tauri_bridge::toggle_maximize().await });
    let on_close = move |_| spawn_local(async { tauri_bridge::close().await });

    let on_dbl = move |_| spawn_local(async { tauri_bridge::toggle_maximize().await });

    // Reactive "— filename" suffix when a file is open. Uses the basename
    // only so long paths don't blow out the title bar.
    let file_suffix = move || {
        state
            .current_file_path
            .get()
            .as_deref()
            .map(basename)
            .map(|name| format!(" — {name}"))
            .unwrap_or_default()
    };

    view! {
        <div
            data-tauri-drag-region="true"
            on:dblclick=on_dbl
            class="h-10 shrink-0 flex items-center justify-between \
                   bg-zinc-100 dark:bg-zinc-950 \
                   border-b border-zinc-200 dark:border-zinc-800 \
                   text-zinc-700 dark:text-zinc-200 \
                   select-none"
        >
            <div
                data-tauri-drag-region="true"
                class="flex items-center gap-2 pl-3 pointer-events-none min-w-0"
            >
                <span class="text-orange-500 shrink-0">
                    <IconSunburst class="w-5 h-5".to_string() />
                </span>
                <span class="text-xs font-semibold tracking-[0.2em] truncate">
                    "ALTEGO 2"{file_suffix}
                </span>
            </div>
            <div class="flex items-center h-full no-drag">
                <TitleBarButton on_click=Box::new(on_minimize) kind=ButtonKind::Normal title="Minimize">
                    <IconWinMinimize class="w-4 h-4".to_string() />
                </TitleBarButton>
                <TitleBarButton on_click=Box::new(on_maximize) kind=ButtonKind::Normal title="Maximize">
                    <IconWinMaximize class="w-3.5 h-3.5".to_string() />
                </TitleBarButton>
                <TitleBarButton on_click=Box::new(on_close) kind=ButtonKind::Close title="Close">
                    <IconWinClose class="w-4 h-4".to_string() />
                </TitleBarButton>
            </div>
        </div>
    }
}

/// Extract the final path component from either a unix or windows path.
fn basename(path: &str) -> &str {
    let after_fwd = path.rsplit('/').next().unwrap_or(path);
    after_fwd.rsplit('\\').next().unwrap_or(after_fwd)
}

#[derive(Clone, Copy)]
enum ButtonKind {
    Normal,
    Close,
}

#[component]
fn TitleBarButton(
    on_click: Box<dyn Fn(web_sys::MouseEvent) + 'static>,
    kind: ButtonKind,
    title: &'static str,
    children: Children,
) -> impl IntoView {
    let hover = match kind {
        ButtonKind::Normal => "hover:bg-zinc-200 dark:hover:bg-zinc-800",
        ButtonKind::Close => "hover:bg-red-500 hover:text-white",
    };
    let class = format!(
        "no-drag h-full w-12 flex items-center justify-center \
         text-zinc-600 dark:text-zinc-300 transition-colors \
         {hover}"
    );
    view! {
        <button class=class title=title on:click=on_click>
            {children()}
        </button>
    }
}
