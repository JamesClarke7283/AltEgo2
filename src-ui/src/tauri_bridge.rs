//! Thin async helpers that invoke our own Tauri commands.
//!
//! Using our own commands (rather than `tauri-wasm-rs`'s pre-release window
//! plugin bindings) keeps the IPC surface stable and visible in the
//! `src-tauri/src/lib.rs` invoke_handler list.

use serde::{Deserialize, Serialize};
use tauri_wasm_rs::api::core::invoke;

#[derive(Serialize)]
struct Empty {}

async fn call_unit(cmd: &str) {
    let _ = invoke::<_, ()>(cmd, &Empty {}).await;
}

pub async fn minimize() {
    call_unit("minimize_window").await;
}

pub async fn toggle_maximize() {
    call_unit("toggle_maximize").await;
}

pub async fn close() {
    call_unit("close_window").await;
}

/// Fallback title-bar drag when `data-tauri-drag-region` misbehaves.
#[allow(dead_code)]
pub async fn start_dragging() {
    call_unit("start_dragging").await;
}

// ---------------------------------------------------------------------------
// Graph persistence
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct SaveGraphArgs {
    contents: String,
}

#[derive(Serialize)]
struct SaveGraphToArgs {
    path: String,
    contents: String,
}

/// Show the native save dialog and write `contents` to the chosen path.
/// Returns `Ok(Some(path))` when the user saved, `Ok(None)` when cancelled,
/// and `Err(msg)` on a genuine error (disk full, permission denied, ...).
pub async fn save_graph(contents: String) -> Result<Option<String>, String> {
    let res: Result<Option<String>, String> =
        match invoke::<_, Option<String>>("save_graph", &SaveGraphArgs { contents }).await {
            Ok(opt) => Ok(opt),
            Err(js) => Err(format!("{js:?}")),
        };
    res
}

/// Write `contents` directly to `path` without showing a dialog. Used once
/// the user has already named the file via "Save As".
pub async fn save_graph_to(path: String, contents: String) -> Result<(), String> {
    match invoke::<_, ()>("save_graph_to", &SaveGraphToArgs { path, contents }).await {
        Ok(()) => Ok(()),
        Err(js) => Err(format!("{js:?}")),
    }
}

#[derive(Deserialize)]
pub struct LoadedGraph {
    pub path: String,
    pub contents: String,
}

/// Show the native open dialog and read the chosen file.
pub async fn load_graph() -> Result<Option<LoadedGraph>, String> {
    match invoke::<_, Option<LoadedGraph>>("load_graph", &Empty {}).await {
        Ok(opt) => Ok(opt),
        Err(js) => Err(format!("{js:?}")),
    }
}

// ---------------------------------------------------------------------------
// Gadgets
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct GadgetCheckArgs {
    #[serde(rename = "runId")]
    run_id: String,
    username: String,
}

/// Kick off the Maigret username sweep. The backend emits
/// `gadget-progress::<run_id>` events while it runs, and this future
/// resolves with the final sorted `Vec<SiteCheckResult>` when the sweep
/// finishes (or an `Err(String)` on a hard failure such as malformed
/// site DB).
///
/// The caller is expected to subscribe to the progress event *before*
/// invoking, using `tauri_wasm_rs::api::event::listen`.
pub async fn gadget_check_username_streaming(
    run_id: String,
    username: String,
) -> Result<Vec<gadgets_maigret::SiteCheckResult>, String> {
    match invoke::<_, Vec<gadgets_maigret::SiteCheckResult>>(
        "gadget_check_username_streaming",
        &GadgetCheckArgs { run_id, username },
    )
    .await
    {
        Ok(v) => Ok(v),
        Err(js) => Err(format!("{js:?}")),
    }
}
