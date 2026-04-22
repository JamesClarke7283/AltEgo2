//! Plain serde types shared between the native runner and the Leptos/WASM
//! frontend. Nothing in this module pulls tokio or reqwest, so it is safe to
//! compile for `wasm32-unknown-unknown` (the frontend depends on this crate
//! with `default-features = false`).

use serde::{Deserialize, Serialize};

/// Result of a single site check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteCheckResult {
    /// Human-readable site name, e.g. `"GitHub"`.
    pub site: String,
    /// URL we actually hit (with `{username}` substituted + percent-encoded).
    pub url: String,
    /// Tags from the Maigret DB, e.g. `["coding", "us"]`. Empty when absent.
    pub tags: Vec<String>,
    /// Classification of the response.
    pub status: CheckStatus,
    /// Wall-clock round trip (ms). `0` when the check short-circuited before
    /// a request was ever dispatched (e.g. `CheckStatus::Invalid`).
    pub elapsed_ms: u64,
}

/// Classification of a single site check.
///
/// Variants are serialised as `{ "kind": "claimed" }`, `{ "kind": "unknown",
/// "reason": "rate_limited" }`, etc. — i.e. a tagged enum with snake-cased
/// discriminants — so the frontend can `match` on `kind` without worrying
/// about how serde serialises the variants.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CheckStatus {
    /// Positive evidence that the username exists on this site.
    Claimed,
    /// Site definitively says the username does not exist (404, absence
    /// string matched, …).
    Available,
    /// Check couldn't decide — rate-limited, blocked, 5xx, timeout, body
    /// too large, challenge page, etc.
    Unknown { reason: String },
    /// The username failed the site's `regexCheck`, so no request was sent.
    Invalid { reason: String },
    /// Transport-level error (DNS, TLS, connection refused, …).
    Error { reason: String },
}

/// Progress update emitted while a run is in flight.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    /// How many sites have finished so far.
    pub completed: usize,
    /// Total sites in the run (known up-front once the DB is loaded).
    pub total: usize,
    /// Most recent result, so the UI can stream a live feed. `None` for the
    /// initial "we've loaded the DB, here's the total" message.
    pub last: Option<SiteCheckResult>,
}

/// Tunables for a username check. `Default` is what the UI uses; the public
/// API also exposes `check_username_with` for callers that want to override.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckOptions {
    /// Maximum concurrent in-flight HTTP requests. 50 keeps most home
    /// connections happy without tripping obvious rate-limiters.
    pub concurrency: usize,
    /// Per-request timeout (milliseconds).
    pub per_request_timeout_ms: u64,
    /// Overall deadline for the whole sweep. `None` = no overall cap; slow
    /// sites trickle in.
    pub overall_timeout_ms: Option<u64>,
    /// Restrict to sites whose `tags` intersect this list. Empty = every
    /// site passes the tag gate.
    pub tags: Vec<String>,
    /// When `true`, skip sites flagged `"disabled": true` in the Maigret DB.
    pub skip_disabled: bool,
}

impl Default for CheckOptions {
    fn default() -> Self {
        Self {
            concurrency: 50,
            per_request_timeout_ms: 10_000,
            overall_timeout_ms: None,
            tags: Vec::new(),
            skip_disabled: true,
        }
    }
}
