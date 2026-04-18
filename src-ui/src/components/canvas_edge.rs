//! Edge + edge-preview rendering.

use leptos::prelude::*;

use crate::state::{AppState, DragKind, Edge, EdgeId, Selection};

#[component]
pub fn EdgeView(edge: Edge) -> impl IntoView {
    let state = AppState::expect();
    let edge_id: EdgeId = edge.id;
    let from_id = edge.from;
    let to_id = edge.to;

    let coords = move || -> Option<(f64, f64, f64, f64)> {
        state.nodes.with(|m| {
            let a = m.get(&from_id)?.position;
            let b = m.get(&to_id)?.position;
            Some((a.0, a.1, b.0, b.1))
        })
    };

    let selected = move || state.selection.get() == Selection::Edge(edge_id);

    let stroke_class = move || {
        if selected() {
            "stroke-orange-500"
        } else {
            "stroke-zinc-400 dark:stroke-zinc-500"
        }
    };
    let stroke_width = move || if selected() { 2.0 } else { 1.5 };

    let on_pointer_down = move |ev: web_sys::PointerEvent| {
        ev.stop_propagation();
        state.selection.set(Selection::Edge(edge_id));
    };

    view! {
        <Show when=move || coords().is_some() fallback=|| ()>
            {move || {
                let (x1, y1, x2, y2) = coords().unwrap_or((0.0, 0.0, 0.0, 0.0));
                view! {
                    <line
                        x1=x1 y1=y1 x2=x2 y2=y2
                        class=move || format!("{}  cursor-pointer", stroke_class())
                        stroke-width=move || stroke_width().to_string()
                        on:pointerdown=on_pointer_down
                    />
                }
            }}
        </Show>
    }
}

/// Rubber-band preview line while the user is dragging a new edge from a
/// node's handle.
#[component]
pub fn EdgePreview() -> impl IntoView {
    let state = AppState::expect();

    let line = move || -> Option<(f64, f64, f64, f64)> {
        match state.drag.get() {
            DragKind::NewEdge { from, cursor_world } => state.nodes.with(|m| {
                let p = m.get(&from)?.position;
                Some((p.0, p.1, cursor_world.0, cursor_world.1))
            }),
            _ => None,
        }
    };

    view! {
        <Show when=move || line().is_some() fallback=|| ()>
            {move || {
                let (x1, y1, x2, y2) = line().unwrap_or((0.0, 0.0, 0.0, 0.0));
                view! {
                    <line
                        x1=x1 y1=y1 x2=x2 y2=y2
                        class="stroke-orange-500"
                        stroke-width="2"
                        stroke-dasharray="4 4"
                        pointer-events="none"
                    />
                }
            }}
        </Show>
    }
}
