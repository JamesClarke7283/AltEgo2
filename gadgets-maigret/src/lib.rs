//! Username sweep gadget for AltEgo 2, backed by the Maigret site database.
//!
//! Two build flavours:
//!
//! * `native` (default) — full HTTP/async stack. Used by the Tauri backend
//!   and by unit tests.
//! * `default-features = false` — only the serde types compile. Used by the
//!   Leptos/WASM frontend so the `SiteCheckResult` type can flow end-to-end
//!   without dragging reqwest+tokio into the browser bundle.
//!
//! ## Quick start (native)
//!
//! ```no_run
//! # #[cfg(feature = "native")]
//! # async fn demo() -> Result<(), String> {
//! let results = gadgets_maigret::check_username("torvalds").await?;
//! for r in results.iter().filter(|r| matches!(
//!     r.status,
//!     gadgets_maigret::CheckStatus::Claimed
//! )) {
//!     println!("{} -> {}", r.site, r.url);
//! }
//! # Ok(()) }
//! ```

pub mod types;

pub use types::{CheckOptions, CheckStatus, Progress, ProgressBatch, SiteCheckResult, StatusCounts};

#[cfg(feature = "native")]
mod check;
#[cfg(feature = "native")]
mod runner;
#[cfg(feature = "native")]
mod sites;

/// One-shot sweep using default options. Internally just defers to
/// `check_username_with(Default::default())`.
#[cfg(feature = "native")]
pub async fn check_username(username: &str) -> Result<Vec<SiteCheckResult>, String> {
    check_username_with(username, CheckOptions::default()).await
}

/// One-shot sweep with caller-supplied options.
#[cfg(feature = "native")]
pub async fn check_username_with(
    username: &str,
    opts: CheckOptions,
) -> Result<Vec<SiteCheckResult>, String> {
    runner::run(username, opts, None).await
}

/// Streaming sweep: sends a `Progress` to `tx` per completed site, and
/// resolves with the full sorted list once every site has finished (or
/// `opts.overall_timeout_ms` elapses).
#[cfg(feature = "native")]
pub async fn check_username_streaming(
    username: &str,
    opts: CheckOptions,
    tx: tokio::sync::mpsc::Sender<Progress>,
) -> Result<Vec<SiteCheckResult>, String> {
    runner::run(username, opts, Some(tx)).await
}

#[cfg(all(test, feature = "native"))]
mod tests {
    use super::*;

    /// Integration smoke test: torvalds exists on GitHub.
    ///
    /// Gated on the `MAIGRET_LIVE_TESTS` env var so CI doesn't hammer
    /// thousands of public sites on every push. Run locally with
    /// `MAIGRET_LIVE_TESTS=1 cargo test --release -- --nocapture`.
    #[tokio::test]
    async fn torvalds_on_github_is_claimed() {
        if std::env::var_os("MAIGRET_LIVE_TESTS").is_none() {
            eprintln!("skipping: set MAIGRET_LIVE_TESTS=1 to run live network tests");
            return;
        }
        let opts = CheckOptions {
            tags: vec!["coding".into()],
            concurrency: 10,
            per_request_timeout_ms: 15_000,
            ..Default::default()
        };
        let results = check_username_with("torvalds", opts).await.unwrap();
        let github = results
            .iter()
            .find(|r| r.site == "GitHub")
            .expect("GitHub should be in results");
        assert!(
            matches!(github.status, CheckStatus::Claimed),
            "expected GitHub Claimed, got {:?}",
            github.status
        );
    }
}
