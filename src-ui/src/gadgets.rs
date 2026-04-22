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

use gadgets_maigret::Progress;
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
    state.gadget_runs.update(|runs: &mut IndexMap<String, GadgetRun>| {
        runs.insert(
            run_id_for_state.clone(),
            GadgetRun {
                run_id: run_id_for_state,
                title: title.clone(),
                source_node_id,
                completed: 0,
                total: 0,
                results: Vec::new(),
                spawned_nodes: IndexMap::new(),
                finished: false,
                error: None,
            },
        );
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
        // IMPORTANT: subscribe BEFORE invoking, or we risk missing the
        // first few progress events. `listen` resolves once the
        // subscription is confirmed with the webview.
        let pump = async {
            match tauri_wasm_rs::api::event::listen::<Progress>(&event_name).await {
                Ok(mut stream) => {
                    use futures::StreamExt;
                    while let Some(ev) = stream.next().await {
                        let p = ev.payload;
                        state.gadget_runs.update(|runs| {
                            if let Some(run) = runs.get_mut(&run_id_for_pump) {
                                run.completed = p.completed;
                                run.total = p.total;
                                if let Some(r) = p.last {
                                    run.results.push(r);
                                }
                            }
                        });
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
                    state.gadget_runs.update(|runs| {
                        if let Some(run) = runs.get_mut(&run_id_for_invoke) {
                            run.results = final_results;
                            run.finished = true;
                            // If the backend's tally raced ahead of our
                            // event stream (we dropped events), reconcile.
                            run.completed = run.results.len();
                            if run.total < run.completed {
                                run.total = run.completed;
                            }
                        }
                    });
                }
                Err(e) => {
                    state.gadget_runs.update(|runs| {
                        if let Some(run) = runs.get_mut(&run_id_for_invoke) {
                            run.error = Some(e);
                            run.finished = true;
                        }
                    });
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
