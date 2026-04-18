//! Left sidebar "ENTITY PALETTE". 35 Maltego-aligned entity rows, sorted
//! alphabetically, filtered by a pinned search box, inside a scrollable
//! inner container so the header/search stay fixed.

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::icons::{EntityIcon, IconSearch};
use crate::state::EntityType;

/// MIME type used in the drag-data transfer to identify an AltEgo entity
/// being dragged. The canvas drop handler parses this.
pub const DRAG_MIME: &str = "application/x-altego-entity";

#[component]
pub fn EntityPalette() -> impl IntoView {
    let query = RwSignal::new(String::new());

    // Alphabetically-sorted + filtered list. Recomputes on query change.
    let visible = Memo::new(move |_| {
        let q = query.get().to_lowercase();
        let mut list: Vec<EntityType> = EntityType::ALL.to_vec();
        list.sort_by_key(|e| e.label());
        if q.is_empty() {
            list
        } else {
            list.into_iter()
                .filter(|e| e.label().to_lowercase().contains(&q))
                .collect()
        }
    });

    let on_input = move |ev: web_sys::Event| {
        let v = event_target_value(&ev);
        query.set(v);
    };

    let on_clear = move |_| query.set(String::new());

    let has_query = move || !query.get().is_empty();
    let empty = move || visible.get().is_empty();

    view! {
        <aside class="w-60 shrink-0 flex flex-col overflow-hidden \
                      bg-zinc-100 dark:bg-zinc-950 \
                      border-r border-zinc-200 dark:border-zinc-800 \
                      text-zinc-900 dark:text-zinc-100">
            <div class="px-4 pt-4 pb-2 text-xs font-semibold tracking-[0.2em] \
                        text-zinc-500 dark:text-zinc-400">
                "ENTITY PALETTE"
            </div>
            <div class="px-3 pb-2">
                <div class="relative">
                    <span class="absolute inset-y-0 left-2 flex items-center \
                                 text-zinc-400 dark:text-zinc-500 pointer-events-none">
                        <IconSearch class="w-4 h-4".to_string() />
                    </span>
                    <input
                        type="text"
                        placeholder="Search entities…"
                        prop:value=move || query.get()
                        on:input=on_input
                        class="w-full text-sm pl-7 pr-7 py-1 rounded \
                               bg-white dark:bg-zinc-900 \
                               border border-zinc-300 dark:border-zinc-700 \
                               text-zinc-900 dark:text-zinc-100 \
                               placeholder:text-zinc-400 dark:placeholder:text-zinc-500 \
                               focus:outline-none focus:border-orange-500"
                    />
                    <Show when=has_query fallback=|| ()>
                        <button
                            class="absolute inset-y-0 right-1 px-1 flex items-center \
                                   text-zinc-400 hover:text-zinc-600 \
                                   dark:text-zinc-500 dark:hover:text-zinc-300"
                            title="Clear search"
                            on:click=on_clear
                        >
                            "×"
                        </button>
                    </Show>
                </div>
            </div>
            <div class="flex-1 overflow-y-auto">
                <Show when=empty fallback=|| ()>
                    <div class="px-4 py-6 text-xs italic \
                                text-zinc-400 dark:text-zinc-600 text-center">
                        "No matches"
                    </div>
                </Show>
                <For
                    each=move || visible.get()
                    key=|e| e.as_str()
                    children=move |e| view! { <PaletteRow entity=e /> }
                />
            </div>
        </aside>
    }
}

#[component]
fn PaletteRow(entity: EntityType) -> impl IntoView {
    let on_dragstart = move |ev: web_sys::DragEvent| {
        if let Some(dt) = ev.data_transfer() {
            let _ = dt.set_data(DRAG_MIME, entity.as_str());
            // Fallback MIME some browsers require.
            let _ = dt.set_data("text/plain", entity.as_str());
            dt.set_effect_allowed("copy");
        }
    };

    view! {
        <div
            draggable="true"
            data-entity=entity.as_str()
            on:dragstart=on_dragstart
            class="flex items-center gap-3 px-4 py-2 cursor-grab \
                   hover:bg-zinc-200/60 dark:hover:bg-zinc-800/60 \
                   active:cursor-grabbing"
        >
            <span class="text-zinc-600 dark:text-zinc-300 shrink-0">
                <EntityIcon entity=entity class="w-5 h-5".to_string() />
            </span>
            <span class="text-sm truncate">{entity.label()}</span>
        </div>
    }
}

fn event_target_value(ev: &web_sys::Event) -> String {
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|el| el.value())
        .unwrap_or_default()
}
