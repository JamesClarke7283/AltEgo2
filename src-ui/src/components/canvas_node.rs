//! Individual graph node rendered as an SVG `<g>`.

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::icons::EntityIcon;
use crate::state::{AppState, ContextMenuState, DragKind, Node, NodeId, Selection};

#[component]
pub fn NodeView(node_id: NodeId) -> impl IntoView {
    let state = AppState::expect();

    let node = move || -> Option<Node> { state.nodes.with(|m| m.get(&node_id).cloned()) };

    let selected = move || state.selection.get() == Selection::Node(node_id);
    let hovered = move || state.hovered_node.get() == Some(node_id);

    let transform = move || {
        let (x, y) = node().map(|n| n.position).unwrap_or((0.0, 0.0));
        format!("translate({x},{y})")
    };

    let label_text = move || -> String {
        node()
            .map(|n| {
                let name = n
                    .properties
                    .values()
                    .next()
                    .cloned()
                    .filter(|s| !s.is_empty());
                name.unwrap_or_else(|| n.entity_type.label().to_string())
            })
            .unwrap_or_default()
    };

    let entity_kind = move || node().map(|n| n.entity_type);

    // Favicon URL stored on the node, if any. Gadgets (e.g. the Maigret
    // username sweep) set a `Favicon` property when spawning a child
    // node so the canvas can render the site's favicon in place of the
    // generic entity glyph.
    let favicon = move || -> Option<String> {
        node().and_then(|n| {
            n.properties
                .get("Favicon")
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
    };

    // ------------- interactions -------------

    let on_node_pointer_down = move |ev: web_sys::PointerEvent| {
        if state.viewport.get_untracked().locked {
            return;
        }
        ev.stop_propagation();
        // Capture pointer so we still receive move/up after leaving the
        // element (or even the window).
        if let Some(target) = ev.target() {
            if let Ok(el) = target.dyn_into::<web_sys::Element>() {
                let _ = el.set_pointer_capture(ev.pointer_id());
            }
        }
        state.selection.set(Selection::Node(node_id));
        let (nx, ny) = state
            .nodes
            .with_untracked(|m| m.get(&node_id).map(|n| n.position))
            .unwrap_or((0.0, 0.0));
        // Snapshot the whole drag as one undo step, and flip the dirty
        // flag once at drag start. The subsequent per-pointer-move
        // `nodes.update(...)` calls in `canvas.rs` do NOT snapshot and
        // do NOT re-mark dirty — they'd otherwise create an undo entry
        // per pixel and churn the dirty signal every frame.
        state.push_undo_snapshot();
        state.mark_dirty();
        let world = client_to_world(ev.client_x() as f64, ev.client_y() as f64, state);
        state.drag.set(DragKind::MoveNode {
            id: node_id,
            offset: (world.0 - nx, world.1 - ny),
        });
    };

    let on_handle_pointer_down = move |ev: web_sys::PointerEvent| {
        if state.viewport.get_untracked().locked {
            return;
        }
        ev.stop_propagation();
        if let Some(target) = ev.target() {
            if let Ok(el) = target.dyn_into::<web_sys::Element>() {
                let _ = el.set_pointer_capture(ev.pointer_id());
            }
        }
        state.selection.set(Selection::Node(node_id));
        let world = client_to_world(ev.client_x() as f64, ev.client_y() as f64, state);
        state.drag.set(DragKind::NewEdge {
            from: node_id,
            cursor_world: world,
        });
    };

    let on_mouse_enter = move |_| state.hovered_node.set(Some(node_id));
    let on_mouse_leave = move |_| {
        if state.hovered_node.get_untracked() == Some(node_id) {
            state.hovered_node.set(None);
        }
    };

    // Right-click: open the gadgets context menu anchored at the cursor.
    let on_context_menu = move |ev: web_sys::MouseEvent| {
        // Suppress the native WebView context menu — we render our own.
        ev.prevent_default();
        ev.stop_propagation();
        let entity = state
            .nodes
            .with_untracked(|m| m.get(&node_id).map(|n| n.entity_type));
        let Some(kind) = entity else { return };
        // Select the node so the user has visual confirmation of the
        // target, and so the results panel can cross-reference selection.
        state.selection.set(Selection::Node(node_id));
        state.context_menu.set(Some(ContextMenuState {
            node_id,
            entity_type: kind,
            screen_pos: (ev.client_x() as f64, ev.client_y() as f64),
        }));
    };

    let ring_visible = move || selected();
    let handle_visible = move || selected() || hovered();

    view! {
        <g
            transform=transform
            data-node-id=node_id.0.to_string()
            on:mouseenter=on_mouse_enter
            on:mouseleave=on_mouse_leave
            on:contextmenu=on_context_menu
        >
            // Selection ring
            <Show when=ring_visible fallback=|| ()>
                <circle r="32" class="stroke-orange-500" fill="none" stroke-width="2" />
            </Show>
            // Filled main circle
            <circle
                r="28"
                class="fill-white dark:fill-zinc-800 stroke-zinc-400 dark:stroke-zinc-600 cursor-pointer"
                stroke-width="1.5"
                on:pointerdown=on_node_pointer_down
            />
            // Node glyph: prefer the per-node favicon (set by gadgets
            // like the Maigret username sweep) and fall back to the
            // entity-type SVG icon. `pointer-events:none` keeps clicks
            // reaching the main circle.
            {move || {
                if let Some(url) = favicon() {
                    view! {
                        <image
                            href=url
                            x="-16" y="-16" width="32" height="32"
                            style="pointer-events:none"
                            preserveAspectRatio="xMidYMid meet"
                            // Rounded clip so the favicon visually fits
                            // inside the node circle.
                            clip-path="inset(0 round 6px)"
                        />
                    }.into_any()
                } else {
                    entity_kind().map(|kind| view! {
                        <g transform="translate(-12,-12)" style="pointer-events:none"
                           class="text-zinc-700 dark:text-zinc-200">
                            <EntityIcon entity=kind class="w-6 h-6".to_string() />
                        </g>
                    }).into_any()
                }
            }}
            // Label below
            <text
                y="48"
                text-anchor="middle"
                class="fill-zinc-700 dark:fill-zinc-200 pointer-events-none"
                style="font-size: 12px; font-family: inherit;"
            >
                {label_text}
            </text>
            // Edge-start handle
            <Show when=handle_visible fallback=|| ()>
                <circle
                    cx="28" cy="0" r="6"
                    class="fill-orange-500 stroke-white dark:stroke-zinc-900 cursor-crosshair"
                    stroke-width="2"
                    data-node-handle=node_id.0.to_string()
                    on:pointerdown=on_handle_pointer_down
                />
            </Show>
        </g>
    }
}

fn client_to_world(cx: f64, cy: f64, state: AppState) -> (f64, f64) {
    // Look up the SVG wrapper by well-known id (set on the <svg> element).
    let doc = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return (cx, cy),
    };
    let rect = doc
        .get_element_by_id("altego-canvas")
        .and_then(|el| Some(el.get_bounding_client_rect()));
    let (lx, ly) = match rect {
        Some(r) => (cx - r.left(), cy - r.top()),
        None => (cx, cy),
    };
    let vp = state.viewport.get_untracked();
    ((lx - vp.pan.0) / vp.zoom, (ly - vp.pan.1) / vp.zoom)
}
