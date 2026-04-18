//! Floating zoom controls at the bottom-left of the canvas.

use leptos::prelude::*;

use crate::components::icons::{IconLock, IconLockOpen, IconMaximizeBox, IconMinus, IconPlus};
use crate::state::AppState;

const ZOOM_MIN: f64 = 0.1;
const ZOOM_MAX: f64 = 4.0;
const ZOOM_STEP: f64 = 1.2;

#[component]
pub fn ZoomControls() -> impl IntoView {
    let state = AppState::expect();

    let on_plus = move |_| {
        state.viewport.update(|v| {
            v.zoom = (v.zoom * ZOOM_STEP).min(ZOOM_MAX);
        });
    };
    let on_minus = move |_| {
        state.viewport.update(|v| {
            v.zoom = (v.zoom / ZOOM_STEP).max(ZOOM_MIN);
        });
    };
    let on_fit = move |_| {
        let nodes = state.nodes.get_untracked();
        if nodes.is_empty() {
            state.viewport.update(|v| {
                v.pan = (0.0, 0.0);
                v.zoom = 1.0;
            });
            return;
        }
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for n in nodes.values() {
            let (x, y) = n.position;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
        let width = (max_x - min_x).max(1.0) + 120.0;   // node radius padding
        let height = (max_y - min_y).max(1.0) + 120.0;
        // Approximate canvas size from window — this is a Phase 1 heuristic
        // and is plenty good enough to recentre the graph.
        let (vw, vh) = window_inner_size().unwrap_or((1200.0, 800.0));
        let zx = vw / width;
        let zy = vh / height;
        let zoom = zx.min(zy).min(1.5).max(ZOOM_MIN);
        let cx = (min_x + max_x) / 2.0;
        let cy = (min_y + max_y) / 2.0;
        let pan = (vw / 2.0 - cx * zoom, vh / 2.0 - cy * zoom);
        state.viewport.update(|v| {
            v.zoom = zoom;
            v.pan = pan;
        });
    };
    let on_lock = move |_| state.toggle_lock();

    let locked = move || state.viewport.get().locked;

    view! {
        <div class="absolute left-3 bottom-3 flex flex-col gap-1 z-10">
            <ZoomButton title="Zoom in"    on_click=Box::new(on_plus)>
                <IconPlus class="w-4 h-4".to_string()/>
            </ZoomButton>
            <ZoomButton title="Zoom out"   on_click=Box::new(on_minus)>
                <IconMinus class="w-4 h-4".to_string()/>
            </ZoomButton>
            <ZoomButton title="Fit to view" on_click=Box::new(on_fit)>
                <IconMaximizeBox class="w-4 h-4".to_string()/>
            </ZoomButton>
            <ZoomButton title="Toggle lock" on_click=Box::new(on_lock)>
                <Show when=locked fallback=|| view! { <IconLockOpen class="w-4 h-4".to_string()/> }>
                    <IconLock class="w-4 h-4".to_string()/>
                </Show>
            </ZoomButton>
        </div>
    }
}

#[component]
fn ZoomButton(
    title: &'static str,
    on_click: Box<dyn Fn(web_sys::MouseEvent) + 'static>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class="w-8 h-8 flex items-center justify-center \
                   bg-white dark:bg-zinc-900 \
                   border border-zinc-300 dark:border-zinc-700 \
                   text-zinc-700 dark:text-zinc-200 \
                   rounded shadow-sm \
                   hover:bg-zinc-100 dark:hover:bg-zinc-800"
            title=title
            on:click=on_click
        >
            {children()}
        </button>
    }
}

fn window_inner_size() -> Option<(f64, f64)> {
    let w = web_sys::window()?;
    let inner_width = w.inner_width().ok()?.as_f64()?;
    let inner_height = w.inner_height().ok()?.as_f64()?;
    Some((inner_width, inner_height))
}
