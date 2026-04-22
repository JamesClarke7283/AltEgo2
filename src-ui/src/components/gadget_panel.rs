//! Bottom-right overlay panel that displays live gadget results.
//!
//! Deliberately not a modal — the canvas stays fully interactive while the
//! sweep runs (which takes ~60 s for Maigret's 3 000-site DB). The user
//! can dismiss via the × button, in which case the run keeps going in the
//! background and the results remain available via `state.gadget_runs`.

use leptos::prelude::*;

use gadgets_maigret::{CheckStatus, SiteCheckResult};

use crate::state::{AppState, EntityType, GadgetRun, NodeId};

#[component]
pub fn GadgetPanel() -> impl IntoView {
    let state = AppState::expect();

    let visible = move || state.active_gadget_run.get().is_some();

    view! {
        <Show when=visible fallback=|| ()>
            <PanelBody />
        </Show>
    }
}

#[component]
fn PanelBody() -> impl IntoView {
    let state = AppState::expect();

    // Filter toggles — default to Claimed only (users care about hits,
    // not 2 900 "Available" entries that would drown the UI).
    let show_claimed = RwSignal::new(true);
    let show_available = RwSignal::new(false);
    let show_unknown = RwSignal::new(false);
    let show_error = RwSignal::new(false);

    let current = move || -> Option<GadgetRun> {
        let id = state.active_gadget_run.get()?;
        state.gadget_runs.with(|m| m.get(&id).cloned())
    };

    let title = move || current().map(|r| r.title).unwrap_or_default();
    let completed = move || current().map(|r| r.completed).unwrap_or(0);
    let total = move || current().map(|r| r.total).unwrap_or(0);
    let finished = move || current().map(|r| r.finished).unwrap_or(false);
    let error = move || current().and_then(|r| r.error.clone());

    let progress_fraction = move || {
        let t = total();
        if t == 0 {
            0.0
        } else {
            (completed() as f64 / t as f64).clamp(0.0, 1.0)
        }
    };

    let close = move |_| state.active_gadget_run.set(None);

    let counts = move || -> (usize, usize, usize, usize) {
        let Some(run) = current() else {
            return (0, 0, 0, 0);
        };
        let mut claimed = 0;
        let mut available = 0;
        let mut unknown = 0;
        let mut error = 0;
        for r in &run.results {
            match r.status {
                CheckStatus::Claimed => claimed += 1,
                CheckStatus::Available => available += 1,
                CheckStatus::Unknown { .. } | CheckStatus::Invalid { .. } => unknown += 1,
                CheckStatus::Error { .. } => error += 1,
            }
        }
        (claimed, available, unknown, error)
    };

    view! {
        <div
            class="fixed bottom-4 right-4 w-[420px] max-h-[70vh] \
                   flex flex-col rounded-md shadow-2xl \
                   bg-white dark:bg-zinc-900 \
                   border border-zinc-200 dark:border-zinc-700 \
                   text-sm text-zinc-700 dark:text-zinc-200 \
                   z-[999]"
            style="backdrop-filter: blur(6px);"
        >
            // Header
            <div class="flex items-center gap-2 px-3 py-2 \
                        border-b border-zinc-200 dark:border-zinc-700">
                <span class="font-medium truncate">{title}</span>
                <span class="ml-auto text-xs text-zinc-500 dark:text-zinc-400">
                    {move || format!("{}/{}", completed(), total())}
                </span>
                <button
                    class="text-zinc-500 hover:text-zinc-900 dark:hover:text-white \
                           rounded px-1 leading-none"
                    title="Close"
                    on:click=close
                >"×"</button>
            </div>

            // Progress bar
            <div class="h-1 bg-zinc-200 dark:bg-zinc-800 overflow-hidden">
                <div
                    class="h-full bg-orange-500 transition-all"
                    style=move || format!("width: {:.1}%", progress_fraction() * 100.0)
                />
            </div>

            // Filter pills
            <div class="flex flex-wrap gap-1.5 px-3 py-2 \
                        border-b border-zinc-200 dark:border-zinc-700 \
                        text-[11px]">
                <FilterPill label="Claimed" color="emerald" signal=show_claimed count=Signal::derive(move || counts().0) />
                <FilterPill label="Available" color="zinc" signal=show_available count=Signal::derive(move || counts().1) />
                <FilterPill label="Unknown" color="amber" signal=show_unknown count=Signal::derive(move || counts().2) />
                <FilterPill label="Error" color="rose" signal=show_error count=Signal::derive(move || counts().3) />
            </div>

            // Body
            <div class="flex-1 overflow-y-auto">
                {move || {
                    if let Some(err) = error() {
                        view! {
                            <div class="p-3 text-rose-600 dark:text-rose-400">
                                "Error: "{err}
                            </div>
                        }.into_any()
                    } else if total() == 0 && !finished() {
                        view! {
                            <div class="p-3 italic text-zinc-500 dark:text-zinc-400">
                                "Loading site database…"
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <ResultsList
                                claimed=show_claimed.into()
                                available=show_available.into()
                                unknown=show_unknown.into()
                                error=show_error.into()
                            />
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn FilterPill(
    label: &'static str,
    color: &'static str,
    signal: RwSignal<bool>,
    #[prop(into)] count: Signal<usize>,
) -> impl IntoView {
    let cls = move || {
        let base = "px-2 py-0.5 rounded-full cursor-pointer border transition-colors";
        let (on, off) = match color {
            "emerald" => (
                "bg-emerald-500/15 text-emerald-700 dark:text-emerald-300 border-emerald-500/40",
                "bg-transparent text-zinc-500 dark:text-zinc-400 border-zinc-300 dark:border-zinc-700",
            ),
            "amber" => (
                "bg-amber-500/15 text-amber-700 dark:text-amber-300 border-amber-500/40",
                "bg-transparent text-zinc-500 dark:text-zinc-400 border-zinc-300 dark:border-zinc-700",
            ),
            "rose" => (
                "bg-rose-500/15 text-rose-700 dark:text-rose-300 border-rose-500/40",
                "bg-transparent text-zinc-500 dark:text-zinc-400 border-zinc-300 dark:border-zinc-700",
            ),
            _ => (
                "bg-zinc-500/15 text-zinc-700 dark:text-zinc-200 border-zinc-500/40",
                "bg-transparent text-zinc-500 dark:text-zinc-400 border-zinc-300 dark:border-zinc-700",
            ),
        };
        format!("{base} {}", if signal.get() { on } else { off })
    };
    let on_click = move |_: web_sys::MouseEvent| signal.update(|v| *v = !*v);
    view! {
        <span class=cls on:click=on_click>
            {label} " (" {move || count.get()} ")"
        </span>
    }
}

#[component]
fn ResultsList(
    claimed: Signal<bool>,
    available: Signal<bool>,
    unknown: Signal<bool>,
    error: Signal<bool>,
) -> impl IntoView {
    let state = AppState::expect();
    let run_results = move || -> Vec<SiteCheckResult> {
        let Some(id) = state.active_gadget_run.get() else {
            return Vec::new();
        };
        state.gadget_runs.with(|m| {
            m.get(&id).map(|r| r.results.clone()).unwrap_or_default()
        })
    };

    let filtered = move || -> Vec<SiteCheckResult> {
        let mut items = run_results();
        items.retain(|r| match r.status {
            CheckStatus::Claimed => claimed.get(),
            CheckStatus::Available => available.get(),
            CheckStatus::Unknown { .. } | CheckStatus::Invalid { .. } => unknown.get(),
            CheckStatus::Error { .. } => error.get(),
        });
        items
    };

    view! {
        {move || {
            let items = filtered();
            if items.is_empty() {
                view! {
                    <div class="p-3 italic text-zinc-400 dark:text-zinc-500">
                        "No results match the current filters."
                    </div>
                }.into_any()
            } else {
                view! {
                    <ul class="divide-y divide-zinc-200 dark:divide-zinc-800">
                        {items.into_iter().map(|r| view! {
                            <ResultRow result=r />
                        }).collect_view()}
                    </ul>
                }.into_any()
            }
        }}
    }
}

#[component]
fn ResultRow(result: SiteCheckResult) -> impl IntoView {
    let state = AppState::expect();

    let (dot_class, default_status_label) = match &result.status {
        CheckStatus::Claimed => ("bg-emerald-500", "claimed"),
        CheckStatus::Available => ("bg-zinc-400", "available"),
        CheckStatus::Unknown { .. } => ("bg-amber-500", "unknown"),
        CheckStatus::Invalid { .. } => ("bg-zinc-400", "invalid"),
        CheckStatus::Error { .. } => ("bg-rose-500", "error"),
    };
    let reason = match &result.status {
        CheckStatus::Unknown { reason }
        | CheckStatus::Invalid { reason }
        | CheckStatus::Error { reason } => Some(reason.clone()),
        _ => None,
    };
    let is_claimed = matches!(result.status, CheckStatus::Claimed);

    // Reactive: is this site currently materialised as a child node? The
    // panel re-renders any time `state.gadget_runs` changes, so the
    // badge / background flip automatically after a toggle.
    //
    // `Signal::derive` gives us a `Copy` handle, so the three view
    // closures below can each call `is_spawned.get()` without needing to
    // clone the underlying closure.
    let site_for_lookup = result.site.clone();
    let is_spawned: Signal<bool> = Signal::derive(move || {
        let Some(rid) = state.active_gadget_run.get() else {
            return false;
        };
        state
            .gadget_runs
            .with(|m| {
                m.get(&rid)
                    .map(|r| r.spawned_nodes.get(&site_for_lookup).copied())
            })
            .flatten()
            .map(|id| state.nodes.with(|nodes| nodes.contains_key(&id)))
            .unwrap_or(false)
    });

    // Click: Claimed → toggle spawn/despawn. Otherwise → open URL in a
    // browser tab (informational rows stay clickable as a convenience).
    let site_for_click = result.site.clone();
    let url_for_click = result.url.clone();
    let on_click = move |_: web_sys::MouseEvent| {
        if is_claimed {
            toggle_spawn(state, &site_for_click, &url_for_click);
        } else if let Some(w) = web_sys::window() {
            let _ = w.open_with_url_and_target(&url_for_click, "_blank");
        }
    };

    // Row chrome shifts when spawned so users can see at a glance which
    // claimed results they've pulled into the graph.
    let row_class = move || {
        let base = "px-3 py-2 cursor-pointer transition-colors";
        if is_spawned.get() {
            format!(
                "{base} bg-emerald-500/10 hover:bg-emerald-500/15 \
                 border-l-2 border-emerald-500"
            )
        } else {
            format!("{base} hover:bg-zinc-50 dark:hover:bg-zinc-800/50")
        }
    };

    let status_label_text = move || -> &'static str {
        if is_claimed && is_spawned.get() {
            "in graph"
        } else {
            default_status_label
        }
    };

    view! {
        <li class=row_class on:click=on_click>
            <div class="flex items-center gap-2">
                <span class=format!("w-2 h-2 rounded-full shrink-0 {}", dot_class)></span>
                <span class="font-medium truncate">{result.site.clone()}</span>
                <span class="ml-auto text-[10px] uppercase tracking-wider \
                             text-zinc-500 dark:text-zinc-400">
                    {status_label_text}
                </span>
            </div>
            <div class="text-[11px] text-zinc-500 dark:text-zinc-400 truncate">
                {result.url.clone()}
            </div>
            {reason.map(|r| view! {
                <div class="text-[11px] text-zinc-400 italic">{r}</div>
            })}
            {(!result.tags.is_empty()).then(|| view! {
                <div class="flex flex-wrap gap-1 mt-1">
                    {result.tags.iter().map(|t| view! {
                        <span class="text-[10px] px-1.5 py-0.5 rounded \
                                     bg-zinc-100 dark:bg-zinc-800 \
                                     text-zinc-600 dark:text-zinc-300">
                            {t.clone()}
                        </span>
                    }).collect_view()}
                </div>
            })}
            {move || is_claimed.then(|| view! {
                <div class="text-[10px] mt-1 text-zinc-400 dark:text-zinc-500">
                    {if is_spawned.get() {
                        "Click to remove from graph"
                    } else {
                        "Click to add as child node"
                    }}
                </div>
            })}
        </li>
    }
}

/// Toggle whether the given site is materialised as a child node under the
/// active run's source Alias.
///
/// Four cases:
///   1. Tracked & node still exists on canvas → remove node + untrack.
///   2. Tracked but node was manually deleted  → untrack + add fresh.
///   3. Not tracked, source still exists       → add + track.
///   4. Not tracked, source was deleted        → alert user, no-op.
///
/// New child nodes are placed in an expanding arc around the source so the
/// first 8 spawn without overlapping, and overflow rings out further.
fn toggle_spawn(state: AppState, site: &str, url: &str) {
    let Some(run_id) = state.active_gadget_run.get_untracked() else {
        return;
    };

    // Read run context (source id, existing spawn, spawn count).
    let (tracked_id, source_id, spawn_count): (Option<NodeId>, Option<NodeId>, usize) = state
        .gadget_runs
        .with_untracked(|m| match m.get(&run_id) {
            Some(r) => (
                r.spawned_nodes.get(site).copied(),
                Some(r.source_node_id),
                r.spawned_nodes.len(),
            ),
            None => (None, None, 0),
        });

    // Decide whether we're removing or adding.
    let node_still_exists = tracked_id
        .map(|id| state.nodes.with_untracked(|n| n.contains_key(&id)))
        .unwrap_or(false);

    if tracked_id.is_some() && node_still_exists {
        // --- Case 1: remove ---
        if let Some(id) = tracked_id {
            state.remove_node(id);
        }
        state.gadget_runs.update(|m| {
            if let Some(r) = m.get_mut(&run_id) {
                r.spawned_nodes.shift_remove(site);
            }
        });
        return;
    }

    // --- Cases 2 / 3 / 4: we need to add ---
    let Some(source_id) = source_id else { return };
    let source_pos = state
        .nodes
        .with_untracked(|n| n.get(&source_id).map(|node| node.position));
    let Some(source_pos) = source_pos else {
        // Case 4: source was deleted; can't attach.
        if let Some(w) = web_sys::window() {
            let _ = w.alert_with_message(
                "The source Alias node has been deleted — can't attach a new child node.",
            );
        }
        // Clear any stale tracking so the UI doesn't keep insisting the
        // row is spawned.
        if tracked_id.is_some() {
            state.gadget_runs.update(|m| {
                if let Some(r) = m.get_mut(&run_id) {
                    r.spawned_nodes.shift_remove(site);
                }
            });
        }
        return;
    };

    // Arc placement: golden-angle increments keep the first ~20 children
    // from colliding without any bookkeeping per slot. Radius bumps out
    // every 8 spawns so later rounds don't overlap earlier ones.
    let count = spawn_count as f64;
    let angle_deg = (count * 137.5) % 360.0;
    let angle = angle_deg.to_radians();
    let radius = 150.0 + (spawn_count / 8) as f64 * 50.0;
    let pos = (
        source_pos.0 + radius * angle.cos(),
        source_pos.1 + radius * angle.sin(),
    );

    let new_id = state.add_node_with_properties(
        EntityType::Affiliation,
        pos,
        &[("Name", site), ("Network", site), ("Profile URL", url)],
    );
    let _ = state.add_edge(source_id, new_id);

    state.gadget_runs.update(|m| {
        if let Some(r) = m.get_mut(&run_id) {
            r.spawned_nodes.insert(site.to_string(), new_id);
        }
    });
}
