//! The central SVG canvas: dotted grid, pan/zoom viewport, drop target,
//! and host for nodes, edges, and the edge-drag preview.

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::canvas_edge::{EdgePreview, EdgeView};
use crate::components::canvas_grid::Grid;
use crate::components::canvas_node::NodeView;
use crate::components::icons::IconSpiderWeb;
use crate::components::palette::DRAG_MIME;
use crate::components::zoom_controls::ZoomControls;
use crate::state::{AppState, DragKind, EntityType, NodeId, Selection};

const ZOOM_MIN: f64 = 0.1;
const ZOOM_MAX: f64 = 4.0;

#[component]
pub fn Canvas() -> impl IntoView {
    let state = AppState::expect();

    // ---- transform string for the <g> pan/zoom group ----
    let group_transform = move || {
        let vp = state.viewport.get();
        format!("translate({},{}) scale({})", vp.pan.0, vp.pan.1, vp.zoom)
    };

    // ------------ drop target (palette entity → new node) ------------
    let on_dragover = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        if let Some(dt) = ev.data_transfer() {
            dt.set_drop_effect("copy");
        }
    };
    let on_drop = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        if state.viewport.get_untracked().locked {
            return;
        }
        let kind_str = ev
            .data_transfer()
            .and_then(|dt| dt.get_data(DRAG_MIME).ok())
            .filter(|s| !s.is_empty())
            .or_else(|| {
                ev.data_transfer()
                    .and_then(|dt| dt.get_data("text/plain").ok())
            })
            .unwrap_or_default();
        let Some(entity) = EntityType::from_str(&kind_str) else {
            return;
        };
        let world = client_to_world(ev.client_x() as f64, ev.client_y() as f64, state);
        state.add_node(entity, world);
    };

    // ------------ pointer events on the SVG root (pan + drag follow) ------------
    let on_pointer_down = move |ev: web_sys::PointerEvent| {
        if state.viewport.get_untracked().locked {
            return;
        }
        // Only pan when the primary target is the background (svg itself or
        // the grid rect). If a node handled it, it stopped propagation.
        let Some(target) = ev.target() else { return };
        let Ok(el) = target.dyn_into::<web_sys::Element>() else {
            return;
        };
        let tag = el.tag_name().to_ascii_lowercase();
        if !(tag == "svg" || tag == "rect") {
            return;
        }
        ev.prevent_default();
        let _ = el.set_pointer_capture(ev.pointer_id());
        state.selection.set(Selection::None);
        let vp = state.viewport.get_untracked();
        state.drag.set(DragKind::PanCanvas {
            start_pan: vp.pan,
            start_client: (ev.client_x() as f64, ev.client_y() as f64),
        });
    };

    let on_pointer_move = move |ev: web_sys::PointerEvent| {
        match state.drag.get_untracked() {
            DragKind::PanCanvas { start_pan, start_client } => {
                if state.viewport.get_untracked().locked {
                    return;
                }
                let dx = ev.client_x() as f64 - start_client.0;
                let dy = ev.client_y() as f64 - start_client.1;
                state.viewport.update(|v| {
                    v.pan = (start_pan.0 + dx, start_pan.1 + dy);
                });
            }
            DragKind::MoveNode { id, offset } => {
                if state.viewport.get_untracked().locked {
                    return;
                }
                let world = client_to_world(ev.client_x() as f64, ev.client_y() as f64, state);
                let new_pos = (world.0 - offset.0, world.1 - offset.1);
                state.nodes.update(|m| {
                    if let Some(node) = m.get_mut(&id) {
                        node.position = new_pos;
                    }
                });
            }
            DragKind::NewEdge { from, .. } => {
                let world = client_to_world(ev.client_x() as f64, ev.client_y() as f64, state);
                state.drag.set(DragKind::NewEdge { from, cursor_world: world });
            }
            DragKind::None => {}
        }
    };

    let on_pointer_up = move |ev: web_sys::PointerEvent| {
        match state.drag.get_untracked() {
            DragKind::NewEdge { from, .. } => {
                if let Some(target_id) =
                    node_id_at(ev.client_x() as f64, ev.client_y() as f64)
                {
                    if target_id != from {
                        let _ = state.add_edge(from, target_id);
                    }
                }
            }
            _ => {}
        }
        state.drag.set(DragKind::None);
    };

    // ------------ wheel zoom ------------
    let on_wheel = move |ev: web_sys::WheelEvent| {
        if state.viewport.get_untracked().locked {
            return;
        }
        ev.prevent_default();
        let vp = state.viewport.get_untracked();
        let delta = ev.delta_y();
        let factor = (-delta * 0.0015).exp();
        let new_zoom = (vp.zoom * factor).clamp(ZOOM_MIN, ZOOM_MAX);
        // Keep cursor's world point anchored.
        let (local_x, local_y) = client_to_local(ev.client_x() as f64, ev.client_y() as f64);
        let world_x = (local_x - vp.pan.0) / vp.zoom;
        let world_y = (local_y - vp.pan.1) / vp.zoom;
        let new_pan = (local_x - world_x * new_zoom, local_y - world_y * new_zoom);
        state.viewport.update(|v| {
            v.zoom = new_zoom;
            v.pan = new_pan;
        });
    };

    let empty = move || state.nodes.with(|m| m.is_empty());

    view! {
        <div
            class="relative flex-1 overflow-hidden \
                   bg-white dark:bg-zinc-900"
            on:dragover=on_dragover
            on:drop=on_drop
        >
            <svg
                id="altego-canvas"
                xmlns="http://www.w3.org/2000/svg"
                class="w-full h-full"
                style="touch-action: none;"
                on:pointerdown=on_pointer_down
                on:pointermove=on_pointer_move
                on:pointerup=on_pointer_up
                on:pointercancel=on_pointer_up
                on:wheel=on_wheel
            >
                <Grid />
                <g transform=group_transform>
                    <EdgesLayer />
                    <NodesLayer />
                    <EdgePreview />
                </g>
            </svg>
            <Show when=empty fallback=|| ()>
                <EmptyState />
            </Show>
            <ZoomControls />
            <AttributionLabel />
        </div>
    }
}

#[component]
fn EdgesLayer() -> impl IntoView {
    let state = AppState::expect();
    let edge_ids = move || {
        state
            .edges
            .with(|m| m.keys().copied().collect::<Vec<_>>())
    };
    view! {
        <For
            each=edge_ids
            key=|id| id.0
            children=move |id| {
                let e = state.edges.with_untracked(|m| m.get(&id).cloned());
                match e {
                    Some(edge) => view! { <EdgeView edge=edge /> }.into_any(),
                    None => ().into_any(),
                }
            }
        />
    }
}

#[component]
fn NodesLayer() -> impl IntoView {
    let state = AppState::expect();
    let node_ids = move || {
        state
            .nodes
            .with(|m| m.keys().copied().collect::<Vec<_>>())
    };
    view! {
        <For
            each=node_ids
            key=|id| id.0
            children=move |id| view! { <NodeView node_id=id /> }
        />
    }
}

#[component]
fn EmptyState() -> impl IntoView {
    view! {
        <div class="absolute inset-0 pointer-events-none flex flex-col items-center justify-center \
                    text-zinc-300 dark:text-zinc-700 select-none">
            <IconSpiderWeb class="w-56 h-56 opacity-60".to_string() />
            <div class="mt-8 text-xl tracking-[0.3em] font-light \
                        text-zinc-400 dark:text-zinc-500">
                "DRAG ENTITIES TO START ANALYSIS"
            </div>
            <div class="mt-3 text-xs tracking-[0.3em] \
                        text-zinc-300 dark:text-zinc-600">
                "ALTEGO 2 LIBRE OSINT PLATFORM"
            </div>
        </div>
    }
}

#[component]
fn AttributionLabel() -> impl IntoView {
    view! {
        <div class="absolute right-3 bottom-2 text-[10px] \
                    text-zinc-400 dark:text-zinc-600 select-none pointer-events-none">
            "Leptos Flow"
        </div>
    }
}

// -----------------------------------------------------------------------------
// coordinate helpers
// -----------------------------------------------------------------------------

fn canvas_rect() -> Option<web_sys::DomRect> {
    let doc = web_sys::window()?.document()?;
    Some(doc.get_element_by_id("altego-canvas")?.get_bounding_client_rect())
}

fn client_to_local(cx: f64, cy: f64) -> (f64, f64) {
    match canvas_rect() {
        Some(r) => (cx - r.left(), cy - r.top()),
        None => (cx, cy),
    }
}

fn client_to_world(cx: f64, cy: f64, state: AppState) -> (f64, f64) {
    let (lx, ly) = client_to_local(cx, cy);
    let vp = state.viewport.get_untracked();
    ((lx - vp.pan.0) / vp.zoom, (ly - vp.pan.1) / vp.zoom)
}

/// Walk up the DOM tree from the element under `(cx, cy)` looking for a
/// `[data-node-id]` ancestor; return the parsed `NodeId` if found.
fn node_id_at(cx: f64, cy: f64) -> Option<NodeId> {
    let doc = web_sys::window()?.document()?;
    let mut el = doc.element_from_point(cx as f32, cy as f32)?;
    loop {
        if let Some(v) = el.get_attribute("data-node-id") {
            if let Ok(id) = v.parse::<u64>() {
                return Some(NodeId(id));
            }
        }
        let Some(parent) = el.parent_element() else {
            return None;
        };
        el = parent;
    }
}
