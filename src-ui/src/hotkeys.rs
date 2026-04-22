//! Global keyboard shortcuts.
//!
//! Installed once at app-root from `App`. All listeners use
//! `window.add_event_listener` + `Closure::forget`, same pattern as the
//! menu-bar outside-click / Escape handling in `components/menu_bar.rs`.
//!
//! ## Shortcuts
//!
//! | Key                                       | Action                                |
//! |-------------------------------------------|---------------------------------------|
//! | `Backspace` / `Delete`                    | Remove the selected node or edge      |
//! | `Ctrl`/`⌘` + `Z`                          | Undo                                  |
//! | `Ctrl`/`⌘` + `X`, `Ctrl+Shift+Z`, `Ctrl+Y`| Redo                                  |
//! | `Ctrl`/`⌘` + `S`                          | Save the graph                        |
//!
//! `Ctrl+X` is an unusual redo key, but the user asked for it; the
//! conventional alternatives (`Ctrl+Shift+Z` and `Ctrl+Y`) are accepted
//! too so muscle memory also works.
//!
//! ## Input-focus guard
//!
//! A global `keydown` handler can easily hijack typing. For most
//! shortcuts we ignore the event whenever focus is in an `<input>`,
//! `<textarea>`, `<select>`, or anything `contenteditable="true"` — so
//! editing a property in the right sidebar still behaves like a normal
//! text field. That also preserves native `Ctrl+Z`/`Ctrl+X` behaviour
//! (undo typing / cut selection) inside form controls.
//!
//! **`Ctrl+S` is a deliberate exception**: users reach for it while
//! mid-edit in the property sidebar expecting it to save, and the
//! native WebView has no built-in "save this form" action to clash
//! with. We handle it regardless of focus.

use leptos::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;

use crate::actions;
use crate::state::{AppState, Selection};

/// Install the global hotkey listeners. Call once at `App` root.
pub fn install(state: AppState) {
    let Some(window) = web_sys::window() else { return };

    let keydown = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(
        move |ev: web_sys::KeyboardEvent| {
            let key = ev.key();
            // Treat Cmd (macOS) the same as Ctrl. Webviews report it as
            // `metaKey`.
            let mod_key = ev.ctrl_key() || ev.meta_key();
            let shift = ev.shift_key();
            let lower = key.to_ascii_lowercase();

            // ---- Save (handled BEFORE the input-focus guard) ----
            //
            // Users reach for Ctrl+S while mid-edit in the property
            // sidebar; swallowing it to let the form take it would just
            // leave them staring at an unsaved graph. There's also no
            // native WebView action bound to Ctrl+S, so there's nothing
            // useful to fall through to.
            if mod_key && !shift && lower == "s" {
                ev.prevent_default();
                spawn_local(async move {
                    if let Err(msg) = actions::save(state).await {
                        if let Some(w) = web_sys::window() {
                            let _ = w.alert_with_message(&msg);
                        }
                    }
                });
                return;
            }

            // Everything below must NOT fire while the user is typing
            // in a form control.
            if is_editable_focused() {
                return;
            }

            // ---- Undo / redo ----
            if mod_key {
                match lower.as_str() {
                    // Ctrl+Shift+Z → redo; plain Ctrl+Z → undo.
                    "z" => {
                        ev.prevent_default();
                        if shift {
                            state.redo();
                        } else {
                            state.undo();
                        }
                        return;
                    }
                    // User-requested redo binding (non-standard) plus
                    // the conventional Windows redo key.
                    "x" | "y" => {
                        ev.prevent_default();
                        state.redo();
                        return;
                    }
                    _ => {}
                }
            }

            // ---- Delete selection ----
            if key == "Backspace" || key == "Delete" {
                match state.selection.get_untracked() {
                    Selection::Node(id) => {
                        // Browsers may use Backspace to navigate back —
                        // block that whenever we actually consumed the
                        // key.
                        ev.prevent_default();
                        state.remove_node(id);
                    }
                    Selection::Edge(id) => {
                        ev.prevent_default();
                        state.remove_edge(id);
                    }
                    Selection::None => {}
                }
            }
        },
    );
    let _ =
        window.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref());
    keydown.forget();
}

/// `true` when the document's focused element is something the user is
/// actively typing into. Keeps Backspace/Delete working normally inside
/// property inputs.
fn is_editable_focused() -> bool {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
        return false;
    };
    let Some(active) = doc.active_element() else {
        return false;
    };
    if is_text_form_control(&active) {
        return true;
    }
    // contenteditable can be "true", "plaintext-only", or inherit from an
    // ancestor. Walk up checking the attribute until we hit the document
    // root or a node with an explicit "false".
    let mut cur: Option<Element> = Some(active);
    while let Some(el) = cur {
        if let Some(v) = el.get_attribute("contenteditable") {
            return v != "false";
        }
        cur = el.parent_element();
    }
    false
}

fn is_text_form_control(el: &Element) -> bool {
    let tag = el.tag_name().to_ascii_uppercase();
    match tag.as_str() {
        // <textarea> and <select> are always text-editing targets. For
        // <input>, types like "checkbox" / "radio" / "button" don't
        // capture typing, so we could be more surgical — but the common
        // case is text inputs, and being conservative here never hurts
        // (a Backspace on a checkbox does nothing anyway).
        "TEXTAREA" | "SELECT" | "INPUT" => true,
        _ => false,
    }
}
