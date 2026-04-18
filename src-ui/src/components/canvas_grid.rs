//! Dotted-grid background for the canvas.
//!
//! Defined as an SVG pattern so the grid scrolls/zooms with the viewport
//! transform it's referenced from. For Phase 1 we keep the dots at world
//! scale and render the pattern on an un-transformed `<rect>` that fills the
//! visible surface — zoom/pan cosmetics can be added later.

use leptos::prelude::*;

#[component]
pub fn Grid() -> impl IntoView {
    view! {
        <defs>
            <pattern
                id="altego-grid"
                width="20"
                height="20"
                patternUnits="userSpaceOnUse"
            >
                <circle
                    cx="10"
                    cy="10"
                    r="1"
                    class="fill-zinc-300 dark:fill-zinc-700"
                />
            </pattern>
        </defs>
        <rect width="100%" height="100%" fill="url(#altego-grid)" />
    }
}
