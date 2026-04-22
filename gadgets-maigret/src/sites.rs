//! Loader for Maigret's `data.json` site database.
//!
//! Maigret stores a bare JSON blob with two top-level keys:
//!   * `sites`   — ~3 100 named site definitions
//!   * `engines` — ~14 shared templates a subset of sites inherit from
//!
//! Engine-based sites carry only their URL + a `"engine": "engineName"`
//! reference; the actual check config (checkType / absenceStrs / headers / …)
//! lives on the engine. We flatten engines into their sites at load time so
//! the rest of the crate never has to think about the distinction.
//!
//! Loading is tried in this order:
//!   1. Network fetch from the upstream Maigret repo (10 s timeout).
//!   2. Bundled snapshot via `include_bytes!` — guaranteed to succeed.
//!
//! The resulting Vec is cached in an async `OnceCell` so a process only pays
//! the parse cost once.

use std::collections::HashMap;
use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::OnceCell;

/// URL we fetch at runtime when online. Matches the path in the plan / task
/// description. If this ever 404s, the bundled snapshot is used.
const UPSTREAM_URL: &str =
    "https://raw.githubusercontent.com/soxoj/maigret/main/maigret/resources/data.json";

/// Bundled snapshot — committed under `resources/data.json`. Guarantees the
/// crate works offline / in CI / in sandboxed Tauri environments.
const BUNDLED_DATA_JSON: &[u8] =
    include_bytes!("../resources/data.json");

/// Flattened site definition — every field we care about, with engine
/// defaults already merged in.
#[derive(Debug, Clone)]
pub(crate) struct SiteDef {
    pub url: String,
    pub url_probe: Option<String>,
    /// Human-readable home-page URL. Kept so future UI affordances (e.g.
    /// "open site" icon when a check is inconclusive) have a target.
    #[allow(dead_code)]
    pub url_main: Option<String>,
    pub regex_check: Option<String>,
    pub check_type: String,
    pub presense_strs: Vec<String>,
    pub absence_strs: Vec<String>,
    pub headers: HashMap<String, String>,
    pub tags: Vec<String>,
    pub disabled: bool,
    pub request_method: Option<String>,
    /// Treat HTTP 403 as "not found" rather than "blocked". Some engines
    /// (XenForo in particular) set this.
    pub ignore_403: bool,
}

static SITES_CACHE: OnceCell<Arc<Vec<(String, SiteDef)>>> = OnceCell::const_new();

/// Raw JSON shape. We keep engines as serde_json values so the eventual
/// merge can copy fields verbatim without re-modelling every engine option.
#[derive(Debug, Deserialize)]
struct RawDb {
    sites: HashMap<String, serde_json::Value>,
    #[serde(default)]
    engines: HashMap<String, RawEngine>,
}

#[derive(Debug, Deserialize)]
struct RawEngine {
    #[serde(default)]
    site: serde_json::Value,
}

/// Load (and cache) the site database. Returns a flat list sorted by site
/// name for deterministic iteration.
pub(crate) async fn load_sites() -> Result<Arc<Vec<(String, SiteDef)>>, String> {
    SITES_CACHE
        .get_or_try_init(|| async {
            let bytes = fetch_db_bytes().await;
            let raw: RawDb = serde_json::from_slice(&bytes)
                .map_err(|e| format!("parse data.json: {e}"))?;
            let flattened = flatten(raw);
            Ok::<_, String>(Arc::new(flattened))
        })
        .await
        .cloned()
}

/// Try the network first, fall back to the bundled snapshot on any failure.
/// We never hard-fail here — the bundled copy is always valid JSON.
async fn fetch_db_bytes() -> Vec<u8> {
    // NOTE: This HTTP call runs in the Tauri *backend* (native), not the
    // WebView. There is no CORS to worry about. Do not be tempted to move
    // this to the WASM frontend — it would then be subject to both CORS
    // and the WebView's more limited TLS stack.
    let timeout = std::time::Duration::from_secs(10);
    let client = match reqwest::Client::builder()
        .timeout(timeout)
        .user_agent("AltEgo2-gadgets-maigret/0.1")
        .build()
    {
        Ok(c) => c,
        Err(_) => return BUNDLED_DATA_JSON.to_vec(),
    };
    match client.get(UPSTREAM_URL).send().await {
        Ok(resp) if resp.status().is_success() => match resp.bytes().await {
            Ok(b) => b.to_vec(),
            Err(_) => BUNDLED_DATA_JSON.to_vec(),
        },
        _ => BUNDLED_DATA_JSON.to_vec(),
    }
}

/// Merge engine defaults into every site that references one, then coerce
/// to `SiteDef`. Sites without a `checkType` after merging are dropped —
/// we can't check them without knowing how.
fn flatten(raw: RawDb) -> Vec<(String, SiteDef)> {
    let mut out: Vec<(String, SiteDef)> = Vec::with_capacity(raw.sites.len());
    for (name, site_val) in raw.sites {
        let mut site_obj = match site_val {
            serde_json::Value::Object(m) => m,
            _ => continue,
        };
        // Engine-merge: engine fields act as *defaults*, so site fields
        // override them. We copy missing keys from the engine's `site`
        // object into the concrete site object.
        let engine_name = site_obj
            .get("engine")
            .and_then(|v| v.as_str())
            .map(str::to_owned);
        if let Some(engine_name) = engine_name {
            if let Some(engine) = raw.engines.get(&engine_name) {
                if let Some(defaults) = engine.site.as_object() {
                    for (k, v) in defaults {
                        site_obj.entry(k.clone()).or_insert_with(|| v.clone());
                    }
                }
            }
        }
        let Some(def) = coerce(site_obj) else { continue };
        out.push((name, def));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// Pull the fields we care about out of a raw JSON object. `None` means
/// the entry is unusable (missing URL or checkType) and should be skipped.
fn coerce(m: serde_json::Map<String, serde_json::Value>) -> Option<SiteDef> {
    let url = m.get("url")?.as_str()?.to_string();
    let check_type = m.get("checkType")?.as_str()?.to_string();

    let str_list = |key: &str| -> Vec<String> {
        m.get(key)
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default()
    };

    let headers: HashMap<String, String> = m
        .get("headers")
        .and_then(|v| v.as_object())
        .map(|o| {
            o.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect()
        })
        .unwrap_or_default();

    Some(SiteDef {
        url,
        url_probe: m.get("urlProbe").and_then(|v| v.as_str()).map(str::to_owned),
        url_main: m.get("urlMain").and_then(|v| v.as_str()).map(str::to_owned),
        regex_check: m
            .get("regexCheck")
            .and_then(|v| v.as_str())
            .map(str::to_owned),
        check_type,
        presense_strs: str_list("presenseStrs"),
        absence_strs: str_list("absenceStrs"),
        headers,
        tags: str_list("tags"),
        disabled: m.get("disabled").and_then(|v| v.as_bool()).unwrap_or(false),
        request_method: m
            .get("requestMethod")
            .and_then(|v| v.as_str())
            .map(str::to_owned),
        ignore_403: m.get("ignore403").and_then(|v| v.as_bool()).unwrap_or(false),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The bundled snapshot always parses. This catches any upstream schema
    /// break the moment we refresh `resources/data.json`.
    #[test]
    fn bundled_snapshot_parses() {
        let raw: RawDb = serde_json::from_slice(BUNDLED_DATA_JSON).expect("bundled parses");
        let flat = flatten(raw);
        assert!(
            flat.len() > 1000,
            "expected >1000 usable sites, got {}",
            flat.len()
        );
    }
}
