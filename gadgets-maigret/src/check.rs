//! Per-site check logic. Takes a `SiteDef` + username and produces one
//! `SiteCheckResult`.
//!
//! The three Maigret check types map to:
//!
//! * `"status_code"`  — 2xx/3xx → `Claimed`, 404 (and other 4xx) → `Available`,
//!                      5xx → `Unknown`. `ignore403` sites treat 403 like 404.
//! * `"message"`      — read body (capped at 256 KiB). Any `absenceStrs`
//!                      substring wins (Available). Otherwise any
//!                      `presenseStrs` match → Claimed. If neither matched,
//!                      a 2xx body falls through to Claimed.
//! * `"response_url"` — a site that redirects unknown profiles elsewhere.
//!                      If the final URL's path still contains the username
//!                      → Claimed, else Available.

use std::time::{Duration, Instant};

use futures::StreamExt;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use crate::sites::SiteDef;
use crate::types::{CheckStatus, SiteCheckResult};

/// Soft cap on bytes we read from a "message"-type response. Some sites
/// return multi-MB HTML; we don't need the whole thing to look for a
/// ~20-char absence/presence string.
const MAX_BODY_BYTES: usize = 256 * 1024;

/// Run a single check. This fn is infallible at the type level — every
/// failure mode is encoded into `CheckStatus`.
pub(crate) async fn check_site(
    client: &reqwest::Client,
    name: &str,
    def: &SiteDef,
    username: &str,
    per_request_timeout: Duration,
) -> SiteCheckResult {
    let encoded_username: String = utf8_percent_encode(username, NON_ALPHANUMERIC).to_string();
    let display_url = def.url.replace("{username}", &encoded_username);
    let probe_url = match def.url_probe.as_deref() {
        Some(p) => p.replace("{username}", &encoded_username),
        None => display_url.clone(),
    };

    // regexCheck: if present *and* compiles, enforce it. Rust's regex crate
    // is stricter than Python's `re`; several upstream patterns use look-
    // arounds and won't compile. In that case we *skip* the gate rather
    // than reporting Invalid — falling through to a real HTTP check is
    // strictly more informative than a false Invalid.
    if let Some(pat) = def.regex_check.as_deref() {
        match regex::Regex::new(pat) {
            Ok(re) => {
                if !re.is_match(username) {
                    return SiteCheckResult {
                        site: name.to_string(),
                        url: display_url,
                        tags: def.tags.clone(),
                        status: CheckStatus::Invalid {
                            reason: "regexCheck rejected username".into(),
                        },
                        elapsed_ms: 0,
                    };
                }
            }
            Err(e) => {
                log::debug!(
                    "gadgets-maigret: skipping unparseable regexCheck for {name}: {e}"
                );
            }
        }
    }

    let started = Instant::now();
    let method = def
        .request_method
        .as_deref()
        .unwrap_or("GET")
        .to_ascii_uppercase();
    let mut req = match method.as_str() {
        "HEAD" => client.head(&probe_url),
        "POST" => client.post(&probe_url),
        _ => client.get(&probe_url),
    };
    // Default to a desktop UA if the site didn't pin one — some servers
    // 403 bare reqwest clients.
    if !def
        .headers
        .keys()
        .any(|k| k.eq_ignore_ascii_case("user-agent"))
    {
        req = req.header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) \
             Chrome/122.0 Safari/537.36 AltEgo/2.0",
        );
    }
    for (k, v) in &def.headers {
        req = req.header(k, v);
    }
    req = req.timeout(per_request_timeout);

    let send_result = req.send().await;

    let resp = match send_result {
        Ok(r) => r,
        Err(e) => {
            let elapsed_ms = started.elapsed().as_millis() as u64;
            let status = if e.is_timeout() {
                CheckStatus::Unknown {
                    reason: "timeout".into(),
                }
            } else if e.is_connect() {
                CheckStatus::Error {
                    reason: format!("connect: {e}"),
                }
            } else {
                CheckStatus::Error {
                    reason: e.to_string(),
                }
            };
            return SiteCheckResult {
                site: name.to_string(),
                url: display_url,
                tags: def.tags.clone(),
                status,
                elapsed_ms,
            };
        }
    };

    let status = classify(resp, def, username, &display_url).await;
    let elapsed_ms = started.elapsed().as_millis() as u64;

    SiteCheckResult {
        site: name.to_string(),
        url: display_url,
        tags: def.tags.clone(),
        status,
        elapsed_ms,
    }
}

/// Turn an owned HTTP response into a `CheckStatus`. Consumes `resp` so it
/// can stream the body when needed without an extra clone.
async fn classify(
    resp: reqwest::Response,
    def: &SiteDef,
    username: &str,
    expected_url: &str,
) -> CheckStatus {
    let code = resp.status().as_u16();

    match def.check_type.as_str() {
        "status_code" => {
            if code == 403 && def.ignore_403 {
                return CheckStatus::Available;
            }
            if code == 429 {
                return CheckStatus::Unknown {
                    reason: "rate_limited".into(),
                };
            }
            if (200..400).contains(&code) {
                CheckStatus::Claimed
            } else if (400..500).contains(&code) {
                CheckStatus::Available
            } else {
                CheckStatus::Unknown {
                    reason: format!("http_{code}"),
                }
            }
        }

        "response_url" => {
            let final_url = resp.url().to_string();
            // A Maigret response_url site claims the username when the
            // server does NOT redirect away — i.e. the final URL still
            // contains the username in path/query.
            if final_url.contains(username) || final_url == expected_url {
                CheckStatus::Claimed
            } else {
                CheckStatus::Available
            }
        }

        _ => {
            // "message" (and anything unrecognised) inspects the body.
            if code == 403 && def.ignore_403 {
                return CheckStatus::Available;
            }
            if code == 429 {
                return CheckStatus::Unknown {
                    reason: "rate_limited".into(),
                };
            }
            if code >= 500 {
                return CheckStatus::Unknown {
                    reason: format!("http_{code}"),
                };
            }
            let body = match read_capped_body(resp).await {
                Ok(b) => b,
                Err(reason) => {
                    return CheckStatus::Unknown { reason };
                }
            };
            let body_str = String::from_utf8_lossy(&body);
            if def
                .absence_strs
                .iter()
                .any(|s| !s.is_empty() && body_str.contains(s.as_str()))
            {
                return CheckStatus::Available;
            }
            if def
                .presense_strs
                .iter()
                .any(|s| !s.is_empty() && body_str.contains(s.as_str()))
            {
                return CheckStatus::Claimed;
            }
            // Heuristic: a 2xx with neither marker present is most likely
            // a claimed profile on a site whose markers have drifted.
            if (200..400).contains(&code) {
                CheckStatus::Claimed
            } else {
                CheckStatus::Available
            }
        }
    }
}

/// Stream the response body, accumulating at most `MAX_BODY_BYTES`. Stops
/// early once the cap is reached or the stream ends.
async fn read_capped_body(resp: reqwest::Response) -> Result<Vec<u8>, String> {
    // Early abort on egregious Content-Length.
    if let Some(len) = resp.content_length() {
        if len as usize > MAX_BODY_BYTES * 4 {
            return Err("body_too_large".into());
        }
    }
    let mut buf: Vec<u8> = Vec::with_capacity(16 * 1024);
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("body_read: {e}"))?;
        let take = MAX_BODY_BYTES.saturating_sub(buf.len());
        if take == 0 {
            break;
        }
        if chunk.len() <= take {
            buf.extend_from_slice(&chunk);
        } else {
            buf.extend_from_slice(&chunk[..take]);
            break;
        }
    }
    Ok(buf)
}
