//! Concurrent orchestrator. Builds a shared `reqwest::Client`, loads the
//! Maigret DB, and fires off every site check with bounded parallelism.
//!
//! A `tokio::sync::mpsc::Sender<Progress>` (optional) is fed one update per
//! completed site, with `try_send` — drop-on-full, because the UI only
//! needs coarse tick updates, not every result.

use std::time::Duration;

use futures::stream::StreamExt;
use tokio::sync::mpsc;

use crate::check::check_site;
use crate::sites::{load_sites, SiteDef};
use crate::types::{CheckOptions, Progress, SiteCheckResult};

/// Run the full sweep. `tx` is optional; when `None`, progress is silently
/// accumulated and returned as the final `Vec`.
pub(crate) async fn run(
    username: &str,
    opts: CheckOptions,
    tx: Option<mpsc::Sender<Progress>>,
) -> Result<Vec<SiteCheckResult>, String> {
    let sites = load_sites().await?;
    let filtered: Vec<(String, SiteDef)> = sites
        .iter()
        .filter(|(_, def)| !(opts.skip_disabled && def.disabled))
        .filter(|(_, def)| {
            opts.tags.is_empty() || def.tags.iter().any(|t| opts.tags.iter().any(|w| w == t))
        })
        .cloned()
        .collect();

    let total = filtered.len();

    // Preflight Progress so the UI knows the total before any site finishes.
    if let Some(tx) = tx.as_ref() {
        let _ = tx
            .send(Progress {
                completed: 0,
                total,
                last: None,
            })
            .await;
    }

    // Shared HTTP client. `redirect::limited(5)` matches Maigret behaviour
    // and lets `response_url` checks observe the final landing page.
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(opts.per_request_timeout_ms))
        .redirect(reqwest::redirect::Policy::limited(5))
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(8)
        .build()
        .map_err(|e| format!("reqwest client init: {e}"))?;

    let per_req_timeout = Duration::from_millis(opts.per_request_timeout_ms);
    let concurrency = opts.concurrency.max(1);

    let stream = futures::stream::iter(filtered.into_iter().map(|(name, def)| {
        let client = client.clone();
        let username = username.to_string();
        async move { check_site(&client, &name, &def, &username, per_req_timeout).await }
    }))
    .buffer_unordered(concurrency);

    let mut results: Vec<SiteCheckResult> = Vec::with_capacity(total);
    let mut stream = Box::pin(stream);

    let run_all = async {
        while let Some(result) = stream.next().await {
            results.push(result.clone());
            if let Some(tx) = tx.as_ref() {
                // Bounded channel + try_send: UI gets "most recent",
                // nothing stalls the runner if the UI can't keep up.
                let _ = tx.try_send(Progress {
                    completed: results.len(),
                    total,
                    last: Some(result),
                });
            }
        }
    };

    // Optional overall deadline. On timeout we still return whatever
    // completed so far — partial results are more useful than nothing.
    if let Some(ms) = opts.overall_timeout_ms {
        let budget = Duration::from_millis(ms);
        let _ = tokio::time::timeout(budget, run_all).await;
    } else {
        run_all.await;
    }

    // Sort: Claimed first, then Available, then the rest; alphabetical
    // within a status bucket.
    results.sort_by(|a, b| {
        let rank = |s: &crate::types::CheckStatus| match s {
            crate::types::CheckStatus::Claimed => 0,
            crate::types::CheckStatus::Available => 1,
            crate::types::CheckStatus::Unknown { .. } => 2,
            crate::types::CheckStatus::Invalid { .. } => 3,
            crate::types::CheckStatus::Error { .. } => 4,
        };
        rank(&a.status)
            .cmp(&rank(&b.status))
            .then_with(|| a.site.cmp(&b.site))
    });
    Ok(results)
}
