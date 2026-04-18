//! Top-level `<App/>`: provides the global `AppState` context, initialises
//! theme, and lays out the frameless window contents.

use leptos::prelude::*;

use crate::components::canvas::Canvas;
use crate::components::menu_bar::MenuBar;
use crate::components::palette::EntityPalette;
use crate::components::right_sidebar::RightSidebar;
use crate::components::title_bar::TitleBar;
use crate::state::AppState;
use crate::theme;

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);
    theme::init(state);

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
        </div>
    }
}
