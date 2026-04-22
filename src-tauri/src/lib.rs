#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use gadgets_maigret::{CheckOptions, Progress, ProgressBatch, SiteCheckResult};
use tauri::{AppHandle, Emitter, Window};
use tauri_plugin_dialog::DialogExt;

// ---------------------------------------------------------------------------
// Window-control commands (for the custom frameless title bar).
// ---------------------------------------------------------------------------

#[tauri::command]
fn minimize_window(window: Window) {
    let _ = window.minimize();
}

#[tauri::command]
fn toggle_maximize(window: Window) {
    if window.is_maximized().unwrap_or(false) {
        let _ = window.unmaximize();
    } else {
        let _ = window.maximize();
    }
}

#[tauri::command]
fn close_window(window: Window) {
    let _ = window.close();
}

#[tauri::command]
fn start_dragging(window: Window) {
    // Fallback when `data-tauri-drag-region` misbehaves.
    let _ = window.start_dragging();
}

// ---------------------------------------------------------------------------
// Graph persistence (.altego.json).
// ---------------------------------------------------------------------------

/// Result payload for `load_graph` — returns both the chosen path (for
/// display) and the raw file contents (JSON string).
#[derive(serde::Serialize)]
struct LoadedGraph {
    path: String,
    contents: String,
}

/// Show a save-file dialog and write `contents` (a JSON string built in the
/// frontend) to the chosen path. Returns the chosen path on success, or
/// `None` if the user cancelled the dialog.
#[tauri::command]
async fn save_graph(app: AppHandle, contents: String) -> Result<Option<String>, String> {
    let chosen = app
        .dialog()
        .file()
        .set_title("Save AltEgo Graph")
        .add_filter("AltEgo Graph", &["altego.json", "json"])
        .set_file_name("untitled.altego.json")
        .blocking_save_file();

    let Some(file_path) = chosen else {
        return Ok(None);
    };

    let path_buf = file_path
        .into_path()
        .map_err(|e| format!("Invalid file path: {e}"))?;

    std::fs::write(&path_buf, contents).map_err(|e| format!("Write failed: {e}"))?;
    Ok(Some(path_buf.display().to_string()))
}

/// Write `contents` to an already-known path with no dialog. Used by the
/// "Save" menu item once the user has already named the file once.
#[tauri::command]
async fn save_graph_to(path: String, contents: String) -> Result<(), String> {
    std::fs::write(&path, contents).map_err(|e| format!("Write failed: {e}"))
}

/// Show an open-file dialog and read the chosen file. Returns `None` if the
/// user cancelled.
#[tauri::command]
async fn load_graph(app: AppHandle) -> Result<Option<LoadedGraph>, String> {
    let chosen = app
        .dialog()
        .file()
        .set_title("Open AltEgo Graph")
        .add_filter("AltEgo Graph", &["altego.json", "json"])
        .blocking_pick_file();

    let Some(file_path) = chosen else {
        return Ok(None);
    };

    let path_buf = file_path
        .into_path()
        .map_err(|e| format!("Invalid file path: {e}"))?;

    let contents =
        std::fs::read_to_string(&path_buf).map_err(|e| format!("Read failed: {e}"))?;

    Ok(Some(LoadedGraph {
        path: path_buf.display().to_string(),
        contents,
    }))
}

// ---------------------------------------------------------------------------
// Gadgets — backend wrappers around `gadgets-maigret`.
// ---------------------------------------------------------------------------

/// One-shot username sweep. Blocks (async) until every site is checked and
/// returns the full list. Kept mainly for tests / automation; the UI uses
/// the streaming variant below so the user sees hits arrive in real time.
#[tauri::command]
async fn gadget_check_username(username: String) -> Result<Vec<SiteCheckResult>, String> {
    gadgets_maigret::check_username(&username).await
}

/// Streaming username sweep. Emits batched `gadget-progress::<run_id>`
/// Tauri events as sites finish, then resolves with the final sorted
/// `Vec`.
///
/// The frontend supplies `run_id` (any opaque string) before invoking, so
/// it can subscribe to the per-run event channel *before* results start
/// arriving. We spawn a forwarder task that drains the mpsc channel,
/// coalesces up to `FLUSH_INTERVAL_MS` worth of results into one
/// `ProgressBatch`, and emits that — ~30 Tauri IPC calls/second instead
/// of ~50–100/s, cutting webview jank.
#[tauri::command]
async fn gadget_check_username_streaming(
    app: AppHandle,
    run_id: String,
    username: String,
) -> Result<Vec<SiteCheckResult>, String> {
    /// Flush cadence for batched progress events. 33 ms ≈ 30 Hz, which is
    /// smoother than any human can perceive and also matches a 30 FPS
    /// animation budget. Dropping to 16 ms (60 Hz) spends too much CPU on
    /// IPC serialisation for negligible perceptual gain.
    const FLUSH_INTERVAL_MS: u64 = 33;

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Progress>(128);
    let progress_event = format!("gadget-progress::{run_id}");
    let app_bg = app.clone();
    tokio::spawn(async move {
        let mut interval =
            tokio::time::interval(std::time::Duration::from_millis(FLUSH_INTERVAL_MS));
        // Don't fire make-up ticks after a blocking period — we only care
        // about the *next* flush window.
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        // Skip the immediate first tick; otherwise we'd flush an empty
        // batch before any results exist.
        let _ = interval.tick().await;

        let mut pending: Vec<SiteCheckResult> = Vec::new();
        let mut completed = 0usize;
        let mut total = 0usize;
        let mut dirty = false;
        let mut closed = false;

        while !closed {
            tokio::select! {
                biased;
                msg = rx.recv() => {
                    match msg {
                        Some(p) => {
                            completed = p.completed;
                            total = p.total;
                            if let Some(r) = p.last {
                                pending.push(r);
                            }
                            dirty = true;
                        }
                        None => {
                            closed = true;
                        }
                    }
                }
                _ = interval.tick() => {
                    if dirty {
                        let _ = app_bg.emit(
                            &progress_event,
                            &ProgressBatch {
                                completed,
                                total,
                                new_results: std::mem::take(&mut pending),
                            },
                        );
                        dirty = false;
                    }
                }
            }
        }

        // Trailing flush — make sure any last results queued between the
        // final tick and channel close arrive at the UI. The frontend
        // also syncs against the invoke's final Vec, so this is belt-
        // and-braces, but keeps the live feed in-sync if the user is
        // watching at the moment the run finishes.
        if dirty || !pending.is_empty() {
            let _ = app_bg.emit(
                &progress_event,
                &ProgressBatch {
                    completed,
                    total,
                    new_results: std::mem::take(&mut pending),
                },
            );
        }
    });
    gadgets_maigret::check_username_streaming(&username, CheckOptions::default(), tx).await
}

// ---------------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            minimize_window,
            toggle_maximize,
            close_window,
            start_dragging,
            save_graph,
            save_graph_to,
            load_graph,
            gadget_check_username,
            gadget_check_username_streaming,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
