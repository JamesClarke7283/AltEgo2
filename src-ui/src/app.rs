//! Top-level `<App/>`: provides the global `AppState` context, initialises
//! theme, and lays out the frameless window contents.

use leptos::prelude::*;

use crate::components::canvas::Canvas;
use crate::components::context_menu::NodeContextMenu;
use crate::components::gadget_panel::GadgetPanel;
use crate::components::menu_bar::MenuBar;
use crate::components::palette::EntityPalette;
use crate::components::right_sidebar::RightSidebar;
use crate::components::title_bar::TitleBar;
use crate::hotkeys;
use crate::state::AppState;
use crate::theme;

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);
    theme::init(state);
    hotkeys::install(state);

    view! {
        <div class="h-screen w-screen flex flex-col \
                    bg-white dark:bg-zinc-900 \
                    text-zinc-900 dark:text-zinc-100 \
                    overflow-hidden antialiased">
            <TitleBar />
            <MenuBar />
            <div class="flex-1 flex min-h-0">
                <EntityPalette />
                <Canvas />
                <RightSidebar />
            </div>
            // Overlay layer — context menu + gadget results panel.
            // Rendered as siblings of the main layout so their
            // `position: fixed` is relative to the viewport, not the flex
            // row that holds the canvas and sidebars.
            <NodeContextMenu />
            <GadgetPanel />
        </div>
    }
}
