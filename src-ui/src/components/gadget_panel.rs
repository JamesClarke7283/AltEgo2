//! Bottom-right overlay panel that displays live gadget results.
//!
//! Deliberately not a modal — the canvas stays fully interactive while the
//! sweep runs (which takes ~60 s for Maigret's 3 000-site DB). The user
//! can dismiss via the × button, in which case the run keeps going in the
//! background and the results remain available via `state.gadget_runs`.
//!
//! ## Reactivity notes
//!
//! Every mutable field on `GadgetRun` is an `RwSignal`, so a progress
//! update only invalidates the specific signal(s) that changed — not the
//! outer `gadget_runs` map. Combined with a Leptos `<For/>` keyed by site
//! name, a new result causes O(1) DOM work (append one row) instead of
//! O(N) (rebuild the entire list).
//!
//! `counts` is kept as a cached struct on the run and incrementally
//! bumped as each result arrives, so filter-pill badges are O(1) reads.

use leptos::prelude::*;

use gadgets_maigret::{CheckStatus, SiteCheckResult, StatusCounts};

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

/// Look up the currently-active `GadgetRun` by id. Returns `None` if the
/// panel is closed or the run was cleared from the map. Only tracks
/// *structural* changes to `gadget_runs` (add/remove), not the signals
/// inside each run.
fn active_run(state: AppState) -> Option<GadgetRun> {
    let id = state.active_gadget_run.get()?;
    state.gadget_runs.with(|m| m.get(&id).cloned())
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

    // Header-level readouts. Each of these only re-runs when *its* signal
    // changes — e.g. the title closure never re-runs during a sweep.
    let title = move || active_run(state).map(|r| r.title.clone()).unwrap_or_default();
    let completed = move || active_run(state).map(|r| r.completed.get()).unwrap_or(0);
    let total = move || active_run(state).map(|r| r.total.get()).unwrap_or(0);
    let finished = move || active_run(state).map(|r| r.finished.get()).unwrap_or(false);
    let error = move || active_run(state).and_then(|r| r.error.get());

    let progress_fraction = move || {
        let t = total();
        if t == 0 {
            0.0
        } else {
            (completed() as f64 / t as f64).clamp(0.0, 1.0)
        }
    };

    let close = move |_| state.active_gadget_run.set(None);

    // Cached counts — O(1) pill updates.
    let counts: Signal<StatusCounts> = Signal::derive(move || {
        active_run(state).map(|r| r.counts.get()).unwrap_or_default()
    });

    view! {
        <div
            class="fixed bottom-4 right-4 w-[420px] max-h-[70vh] \
                   flex flex-col rounded-md shadow-2xl \
                   bg-white dark:bg-zinc-900 \
                   border border-zinc-200 dark:border-zinc-700 \
                   text-sm text-zinc-700 dark:text-zinc-200 \
                   z-[999]"
            style="backdrop-filter: blur(6px); contain: layout paint;"
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

            // Progress bar — CSS transition smooths the jumps between
            // 30 Hz backend flushes.
            <div class="h-1 bg-zinc-200 dark:bg-zinc-800 overflow-hidden">
                <div
                    class="h-full bg-orange-500"
                    style=move || format!(
                        "width: {:.1}%; transition: width 120ms linear;",
                        progress_fraction() * 100.0
                    )
                />
            </div>

            // Filter pills — each reads the pre-computed count signal in
            // O(1); no iteration over the result list.
            <div class="flex flex-wrap gap-1.5 px-3 py-2 \
                        border-b border-zinc-200 dark:border-zinc-700 \
                        text-[11px]">
                <FilterPill label="Claimed" color="emerald" signal=show_claimed
                    count=Signal::derive(move || counts.get().claimed) />
                <FilterPill label="Available" color="zinc" signal=show_available
                    count=Signal::derive(move || counts.get().available) />
                <FilterPill label="Unknown" color="amber" signal=show_unknown
                    count=Signal::derive(move || counts.get().unknown) />
                <FilterPill label="Error" color="rose" signal=show_error
                    count=Signal::derive(move || counts.get().error) />
            </div>

            // Body
            <div class="flex-1 overflow-y-auto"
                 style="overscroll-behavior: contain; scrollbar-gutter: stable;">
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

    // Memo: the filtered list. Re-evaluates only when:
    //   * `active_gadget_run` changes (user switched run / closed panel)
    //   * the run's `results` signal changes (new batch arrived)
    //   * any of the four filter toggles flips
    //
    // `<For/>` then diffs this Vec against its previous value by key, so
    // DOM work is proportional to the *delta*, not the whole list.
    let filtered = Memo::new(move |_| -> Vec<SiteCheckResult> {
        let Some(run) = active_run(state) else {
            return Vec::new();
        };
        let (c, a, u, e) = (claimed.get(), available.get(), unknown.get(), error.get());
        run.results.with(|all| {
            all.iter()
                .filter(|r| match r.status {
                    CheckStatus::Claimed => c,
                    CheckStatus::Available => a,
                    CheckStatus::Unknown { .. } | CheckStatus::Invalid { .. } => u,
                    CheckStatus::Error { .. } => e,
                })
                .cloned()
                .collect()
        })
    });

    view! {
        {move || {
            let is_empty = filtered.with(|v| v.is_empty());
            if is_empty {
                view! {
                    <div class="p-3 italic text-zinc-400 dark:text-zinc-500">
                        "No results match the current filters."
                    </div>
                }.into_any()
            } else {
                view! {
                    <ul class="divide-y divide-zinc-200 dark:divide-zinc-800">
                        <For
                            each=move || filtered.get()
                            key=|r| r.site.clone()
                            let:result
                        >
                            <ResultRow result=result />
                        </For>
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

    // Reactive: is this site currently materialised as a child node? Only
    // re-runs when `spawned_nodes` or `nodes` change — not when new
    // results arrive on other sites.
    let site_for_lookup = result.site.clone();
    let is_spawned: Signal<bool> = Signal::derive(move || {
        let Some(run) = active_run(state) else {
            return false;
        };
        let node_id = run
            .spawned_nodes
            .with(|m| m.get(&site_for_lookup).copied());
        let Some(id) = node_id else { return false };
        state.nodes.with(|nodes| nodes.contains_key(&id))
    });

    let site_for_click = result.site.clone();
    let url_for_click = result.url.clone();
    let url_for_fav = result.url.clone();
    let on_click = move |_: web_sys::MouseEvent| {
        if is_claimed {
            toggle_spawn(state, &site_for_click, &url_for_click, &url_for_fav);
        } else if let Some(w) = web_sys::window() {
            let _ = w.open_with_url_and_target(&url_for_click, "_blank");
        }
    };

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
/// active run's source Alias. Details + four cases: see previous commit
/// (file history) — behaviour is unchanged, this version only swaps the
/// `results`/`spawned_nodes` reads from struct fields to signals.
fn toggle_spawn(state: AppState, site: &str, url: &str, favicon_source_url: &str) {
    let Some(run_id) = state.active_gadget_run.get_untracked() else {
        return;
    };

    let Some(run) = state
        .gadget_runs
        .with_untracked(|m| m.get(&run_id).cloned())
    else {
        return;
    };

    let tracked_id: Option<NodeId> = run
        .spawned_nodes
        .with_untracked(|m| m.get(site).copied());
    let spawn_count: usize = run.spawned_nodes.with_untracked(|m| m.len());
    let source_id: NodeId = run.source_node_id;

    let node_still_exists = tracked_id
        .map(|id| state.nodes.with_untracked(|n| n.contains_key(&id)))
        .unwrap_or(false);

    if tracked_id.is_some() && node_still_exists {
        // --- remove ---
        if let Some(id) = tracked_id {
            state.remove_node(id);
        }
        run.spawned_nodes.update(|m| {
            m.shift_remove(site);
        });
        return;
    }

    // --- add ---
    let source_pos = state
        .nodes
        .with_untracked(|n| n.get(&source_id).map(|node| node.position));
    let Some(source_pos) = source_pos else {
        if let Some(w) = web_sys::window() {
            let _ = w.alert_with_message(
                "The source Alias node has been deleted — can't attach a new child node.",
            );
        }
        if tracked_id.is_some() {
            run.spawned_nodes.update(|m| {
                m.shift_remove(site);
            });
        }
        return;
    };

    // Golden-angle arc around the source Alias; rings out every 8 spawns
    // so later rounds don't overlap earlier ones.
    let count = spawn_count as f64;
    let angle_deg = (count * 137.5) % 360.0;
    let angle = angle_deg.to_radians();
    let radius = 150.0 + (spawn_count / 8) as f64 * 50.0;
    let pos = (
        source_pos.0 + radius * angle.cos(),
        source_pos.1 + radius * angle.sin(),
    );

    let favicon = crate::gadgets::favicon_url(favicon_source_url);
    let new_id = state.add_node_with_properties(
        EntityType::Affiliation,
        pos,
        &[
            ("Name", site),
            ("Network", site),
            ("Profile URL", url),
            ("Favicon", favicon.as_deref().unwrap_or("")),
        ],
    );
    let _ = state.add_edge(source_id, new_id);

    run.spawned_nodes.update(|m| {
        m.insert(site.to_string(), new_id);
    });
}
