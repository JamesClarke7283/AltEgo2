//! High-level user-triggered actions.
//!
//! Kept separate from UI components so the same entry-points can be
//! invoked from the menu bar, the `Ctrl+S` hotkey, right-click menus,
//! future toolbars, etc. — each caller site just calls one of these
//! async functions and handles the `Result` however it likes (an alert,
//! a toast, a logged warning, …).

use leptos::prelude::*;

use crate::state::AppState;
use crate::tauri_bridge;

/// Outcome of a [`save`] call.
pub enum SaveOutcome {
    /// File was written. Carries the absolute path we wrote to — handy
    /// for toast messages like "Saved to /foo/bar.altego.json".
    Saved(String),
    /// User dismissed the Save-As dialog. No error, nothing to do.
    Cancelled,
}

/// Serialise the graph and write it to disk.
///
/// * If a `current_file_path` is already set, we write there silently
///   (matches the existing File → Save behaviour).
/// * Otherwise we pop the native Save-As dialog and remember the chosen
///   path on success.
///
/// On a successful write we clear `is_dirty` so the title-bar dot flips
/// green. Cancellation does NOT clear dirty — nothing was written.
pub async fn save(state: AppState) -> Result<SaveOutcome, String> {
    let graph = state.snapshot_graph_file();
    let json = serde_json::to_string_pretty(&graph)
        .map_err(|e| format!("Failed to serialise graph:\n{e}"))?;

    let current_path = state.current_file_path.get_untracked();
    match current_path {
        Some(path) => {
            tauri_bridge::save_graph_to(path.clone(), json)
                .await
                .map_err(|e| format!("Save failed:\n{e}"))?;
            state.mark_clean();
            Ok(SaveOutcome::Saved(path))
        }
        None => match tauri_bridge::save_graph(json).await {
            Ok(Some(path)) => {
                state.current_file_path.set(Some(path.clone()));
                state.mark_clean();
                Ok(SaveOutcome::Saved(path))
            }
            Ok(None) => Ok(SaveOutcome::Cancelled),
            Err(e) => Err(format!("Save failed:\n{e}")),
        },
    }
}
