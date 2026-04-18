#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{AppHandle, Window};
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
