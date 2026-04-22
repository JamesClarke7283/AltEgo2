//! Right sidebar: OVERVIEW / DETAIL VIEW / PROPERTY VIEW stacked vertically.

use leptos::prelude::*;

use crate::components::icons::EntityIcon;
use crate::state::{AppState, Node, Selection};

#[component]
pub fn RightSidebar() -> impl IntoView {
    view! {
        <aside class="w-72 shrink-0 flex flex-col overflow-y-auto \
                      bg-zinc-100 dark:bg-zinc-950 \
                      border-l border-zinc-200 dark:border-zinc-800 \
                      text-zinc-900 dark:text-zinc-100">
            <Overview />
            <DetailView />
            <PropertyView />
        </aside>
    }
}

#[component]
fn SectionHeader(label: &'static str) -> impl IntoView {
    view! {
        <div class="px-4 pt-5 pb-2 text-xs font-semibold tracking-[0.2em] \
                    text-zinc-500 dark:text-zinc-400">
            {label}
        </div>
    }
}

#[component]
fn Overview() -> impl IntoView {
    let state = AppState::expect();
    let node_count = move || state.nodes.with(|m| m.len());
    let edge_count = move || state.edges.with(|m| m.len());
    view! {
        <div>
            <SectionHeader label="OVERVIEW" />
            <div class="px-4 pb-4 space-y-1 text-sm">
                <StatRow label="Nodes:" value=Signal::derive(node_count) />
                <StatRow label="Edges:" value=Signal::derive(edge_count) />
            </div>
            <Divider />
        </div>
    }
}

#[component]
fn StatRow(label: &'static str, value: Signal<usize>) -> impl IntoView {
    view! {
        <div class="flex justify-between">
            <span class="text-zinc-600 dark:text-zinc-400">{label}</span>
            <span class="text-blue-500 dark:text-blue-400 font-mono tabular-nums">
                {move || value.get()}
            </span>
        </div>
    }
}

#[component]
fn Divider() -> impl IntoView {
    view! { <div class="mx-4 border-t border-zinc-200 dark:border-zinc-800"></div> }
}

// --------------------------------------------------------------------------

#[component]
fn DetailView() -> impl IntoView {
    let state = AppState::expect();
    view! {
        <div>
            <SectionHeader label="DETAIL VIEW" />
            <div class="px-4 pb-4 min-h-[120px]">
                {move || match state.selection.get() {
                    Selection::Node(id) => state.nodes.with(|m| {
                        m.get(&id).map(|n| view! { <DetailNode node=n.clone() /> }.into_any())
                    }).unwrap_or_else(|| empty_detail().into_any()),
                    _ => empty_detail().into_any(),
                }}
            </div>
            <Divider />
        </div>
    }
}

fn empty_detail() -> impl IntoView {
    view! {
        <div class="h-24 flex items-center justify-center \
                    text-sm italic text-zinc-400 dark:text-zinc-600">
            "No selection"
        </div>
    }
}

#[component]
fn DetailNode(node: Node) -> impl IntoView {
    let label_name = node
        .properties
        .iter()
        .next()
        .and_then(|(_, v)| if v.is_empty() { None } else { Some(v.clone()) })
        .unwrap_or_else(|| format!("Unnamed {}", node.entity_type.label()));
    let (px, py) = node.position;
    view! {
        <div class="mt-2 flex items-center gap-3">
            <span class="text-orange-500">
                <EntityIcon entity=node.entity_type class="w-6 h-6".to_string() />
            </span>
            <div class="flex flex-col">
                <span class="text-sm font-medium">{label_name}</span>
                <span class="text-xs text-zinc-500 dark:text-zinc-400">
                    {node.entity_type.label()}
                </span>
            </div>
        </div>
        <div class="mt-3 space-y-1 text-xs text-zinc-500 dark:text-zinc-400 font-mono">
            <div>"ID: " {node.id.0}</div>
            <div>{format!("Position: ({:.0}, {:.0})", px, py)}</div>
        </div>
    }
}

// --------------------------------------------------------------------------

#[component]
fn PropertyView() -> impl IntoView {
    let state = AppState::expect();
    view! {
        <div>
            <SectionHeader label="PROPERTY VIEW" />
            <div class="px-4 pb-6 min-h-[160px]">
                {move || match state.selection.get() {
                    Selection::Node(id) => {
                        let has = state.nodes.with(|m| m.contains_key(&id));
                        if has {
                            view! { <PropertyEditor node_id=id /> }.into_any()
                        } else {
                            empty_property().into_any()
                        }
                    }
                    _ => empty_property().into_any(),
                }}
            </div>
        </div>
    }
}

fn empty_property() -> impl IntoView {
    view! {
        <div class="h-24 flex items-center justify-center \
                    text-sm italic text-zinc-400 dark:text-zinc-600 text-center">
            "Select a node to edit properties"
        </div>
    }
}

#[component]
fn PropertyEditor(node_id: crate::state::NodeId) -> impl IntoView {
    let state = AppState::expect();
    // Grab current keys (stable order via IndexMap).
    let keys = state
        .nodes
        .with_untracked(|m| m.get(&node_id).map(|n| n.properties.keys().cloned().collect::<Vec<_>>()))
        .unwrap_or_default();

    view! {
        <div class="mt-2 space-y-3">
            {keys.into_iter().map(|key| {
                let key_for_label = key.clone();
                let key_for_get = key.clone();
                let key_for_set = key.clone();
                let current = move || {
                    state.nodes.with(|m| {
                        m.get(&node_id).and_then(|n| n.properties.get(&key_for_get).cloned())
                            .unwrap_or_default()
                    })
                };
                let on_input = move |ev: web_sys::Event| {
                    let v = event_target_value(&ev);
                    // Goes through the AppState helper so the dirty
                    // flag flips on the first keystroke (→ title-bar
                    // dot turns white + pulsing).
                    state.update_node_property(node_id, &key_for_set, v);
                };
                view! {
                    <label class="block">
                        <span class="block text-[11px] uppercase tracking-wider \
                                     text-zinc-500 dark:text-zinc-400 mb-1">
                            {key_for_label}
                        </span>
                        <input
                            type="text"
                            prop:value=current
                            on:input=on_input
                            class="w-full px-2 py-1 rounded-sm text-sm \
                                   bg-white dark:bg-zinc-900 \
                                   border border-zinc-300 dark:border-zinc-700 \
                                   text-zinc-900 dark:text-zinc-100 \
                                   focus:outline-none focus:border-orange-500"
                        />
                    </label>
                }
            }).collect_view()}
        </div>
    }
}

fn event_target_value(ev: &web_sys::Event) -> String {
    use wasm_bindgen::JsCast;
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|el| el.value())
        .unwrap_or_default()
}
