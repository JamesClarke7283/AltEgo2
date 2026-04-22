//! Gadget registry — AltEgo's equivalent of Maltego *transforms*.
//!
//! A gadget is a named action that, given a node, can enrich the graph
//! (fetch data, add child nodes, run OSINT sweeps, …). Each `GadgetDef`
//! advertises which `EntityType`s it applies to; the right-click context
//! menu uses `gadgets_for(entity)` to decide what to offer.
//!
//! Gadgets are plain `const` data with a function pointer, so they're
//! `Copy + 'static` and safe to stash in Leptos signal contexts.

use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use gadgets_maigret::{ProgressBatch, StatusCounts};
use indexmap::IndexMap;

use crate::state::{AppState, EntityType, GadgetRun, Node};
use crate::tauri_bridge;

/// Static description of a gadget — what it's called, who it applies to,
/// and the function that kicks it off.
pub struct GadgetDef {
    /// Stable ID (used for logging / future persistence; not user-visible).
    pub id: &'static str,
    /// Short label shown in the context menu.
    pub label: &'static str,
    /// Tooltip / one-line description.
    pub description: &'static str,
    /// Entity types this gadget is applicable to.
    pub applies_to: &'static [EntityType],
    /// Entry point — gets a snapshot of app state + the target node.
    pub run: fn(AppState, &Node),
}

/// The sole gadget for now — the Maigret username sweep. Future gadgets
/// append here with their own `applies_to` array.
pub const GADGETS: &[GadgetDef] = &[GadgetDef {
    id: "maigret_check_username",
    label: "Check Username",
    description: "Test this alias against ~3000 websites (Maigret).",
    applies_to: &[EntityType::Alias],
    run: run_check_username,
}];

/// Which gadgets apply to this entity type? Returned in declaration order.
pub fn gadgets_for(entity: EntityType) -> Vec<&'static GadgetDef> {
    GADGETS
        .iter()
        .filter(|g| g.applies_to.contains(&entity))
        .collect()
}

// ---------------------------------------------------------------------------
// Gadget implementations
// ---------------------------------------------------------------------------

/// Launch the Maigret username sweep for an Alias node.
///
/// Flow:
///   1. Read the Alias `Handle` property; bail with an alert if empty.
///   2. Seed a fresh `GadgetRun` in AppState and focus the panel on it.
///   3. Subscribe to `gadget-progress::<run_id>`; patch the run as events
///      arrive (this is what produces the live feed).
///   4. Invoke `gadget_check_username_streaming`; when it resolves, stash
///      the final `Vec<SiteCheckResult>` into the run and mark it done.
fn run_check_username(state: AppState, node: &Node) {
    // ---- 1. extract the username ----
    let username = node
        .properties
        .get("Handle")
        .cloned()
        .unwrap_or_default()
        .trim()
        .to_string();
    if username.is_empty() {
        if let Some(w) = web_sys::window() {
            let _ = w.alert_with_message(
                "Set a Handle on this Alias before running the username check.",
            );
        }
        return;
    }

    // ---- 2. seed state ----
    let run_id = generate_run_id();
    let title = format!("Check Username: {username}");
    let run_id_for_state = run_id.clone();
    let source_node_id = node.id;
    // All the rapidly-changing fields are RwSignals, so the outer
    // IndexMap only invalidates on structural change (run added/removed).
    let new_run = GadgetRun {
        run_id: run_id_for_state.clone(),
        title: title.clone(),
        source_node_id,
        completed: RwSignal::new(0),
        total: RwSignal::new(0),
        results: RwSignal::new(Vec::new()),
        counts: RwSignal::new(StatusCounts::default()),
        spawned_nodes: RwSignal::new(IndexMap::new()),
        finished: RwSignal::new(false),
        error: RwSignal::new(None),
    };
    state.gadget_runs.update(|runs: &mut IndexMap<String, GadgetRun>| {
        runs.insert(run_id_for_state, new_run);
    });
    state.active_gadget_run.set(Some(run_id.clone()));

    // ---- 3. subscribe + invoke in a single task ----
    //
    // We own the event-name String for the whole task scope, then drive
    // both the progress-listen and the invoke concurrently via
    // `futures::join!`. This avoids a nested spawn_local (which would
    // require `'static` captures and trip the Rust 2024 `impl Trait`
    // capture rule on the stream's lifetime).
    let event_name = format!("gadget-progress::{run_id}");
    let run_id_for_invoke = run_id.clone();
    let run_id_for_pump = run_id.clone();

    spawn_local(async move {
        // Snapshot the RwSignal handles for this run once. They're Copy
        // so we can move them freely into each sub-task.
        let run_signals = state
            .gadget_runs
            .with_untracked(|m| m.get(&run_id_for_pump).cloned());

        // IMPORTANT: subscribe BEFORE invoking, or we risk missing the
        // first few progress events. `listen` resolves once the
        // subscription is confirmed with the webview.
        let pump = async {
            let Some(run) = run_signals.clone() else { return };
            match tauri_wasm_rs::api::event::listen::<ProgressBatch>(&event_name).await {
                Ok(mut stream) => {
                    use futures::StreamExt;
                    while let Some(ev) = stream.next().await {
                        let batch = ev.payload;
                        // One reactive update per batch (typically 10–50
                        // results), not one per site. This is the main
                        // perf win: 3 000 events → ~60 update cycles.
                        run.completed.set(batch.completed);
                        run.total.set(batch.total);
                        if !batch.new_results.is_empty() {
                            run.counts.update(|c| {
                                for r in &batch.new_results {
                                    c.bump(&r.status);
                                }
                            });
                            run.results.update(|v| v.extend(batch.new_results));
                        }
                    }
                }
                Err(e) => {
                    log::warn!(
                        "gadget listen failed, proceeding without live updates: {e:?}"
                    );
                }
            }
        };

        // ---- 4. invoke the command and await the final Vec ----
        let invoke = async {
            match tauri_bridge::gadget_check_username_streaming(
                run_id_for_invoke.clone(),
                username,
            )
            .await
            {
                Ok(final_results) => {
                    if let Some(run) = run_signals.clone() {
                        // Recompute counts from scratch on the final list
                        // so we can't drift from dropped events.
                        let mut counts = StatusCounts::default();
                        for r in &final_results {
                            counts.bump(&r.status);
                        }
                        let len = final_results.len();
                        run.results.set(final_results);
                        run.counts.set(counts);
                        run.completed.set(len);
                        run.total.update(|t| {
                            if *t < len {
                                *t = len;
                            }
                        });
                        run.finished.set(true);
                    }
                }
                Err(e) => {
                    if let Some(run) = run_signals.clone() {
                        run.error.set(Some(e));
                        run.finished.set(true);
                    }
                }
            }
        };

        // Run both concurrently. The pump loop exits naturally when the
        // backend stops emitting (which happens after `invoke` resolves
        // and the forwarder task drops its mpsc sender).
        futures::join!(pump, invoke);
    });
}

/// Cheap process-local unique id. We don't need cryptographic uniqueness;
/// just something the backend can route progress events to and that's
/// distinct between concurrent runs.
fn generate_run_id() -> String {
    let ms = js_sys::Date::now() as u64;
    let rand = (js_sys::Math::random() * 1_000_000_000.0) as u64;
    format!("run-{ms}-{rand}")
}

/// Build a favicon URL for a given site URL.
///
/// Uses Google's public `s2/favicons` service, which:
///   * works for any public site without CORS fuss (browsers treat it as
///     an `<img>` source, not an API call);
///   * falls back to a sensible placeholder when the site doesn't serve a
///     favicon at a discoverable location;
///   * normalises size — we always get a 64 × 64 PNG.
///
/// Returns `None` if we can't extract a host from the input URL (unlikely;
/// Maigret URLs always look like `https://host/{username}`).
pub fn favicon_url(site_url: &str) -> Option<String> {
    let host = host_of(site_url)?;
    Some(format!(
        "https://www.google.com/s2/favicons?domain={host}&sz=64"
    ))
}

/// Extract the host component from a URL without pulling in the `url`
/// crate (which isn't in the frontend dep graph). Handles `http://` and
/// `https://` prefixes; everything else returns `None`.
fn host_of(url: &str) -> Option<&str> {
    let after_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let end = after_scheme
        .find(|c: char| c == '/' || c == ':' || c == '?' || c == '#')
        .unwrap_or(after_scheme.len());
    let host = &after_scheme[..end];
    if host.is_empty() {
        None
    } else {
        Some(host)
    }
}
