//! Desktop effect handlers, exposed as Tauri commands. These are the native
//! analogue of the web shell's browser-fetch effects: same architecture, same
//! intent/effect loop in the (shared) WASM frontend, only the performer differs.
//! Each command performs an HTTPS GET with a normal Rust HTTP client (reqwest,
//! platform TLS) and returns the raw response body; the frontend parses it with
//! the shared `bluesky::wire` parser, so parsing is identical on both platforms.

const APPVIEW: &str = "https://public.api.bsky.app";
const LIMIT: u32 = 25;

/// Fetch a page of an actor's public feed (the Phase 2 stand-in for the authed
/// timeline). Returns the raw JSON body for the frontend to parse.
#[tauri::command]
pub async fn fetch_feed(actor: String, cursor: Option<String>) -> Result<String, String> {
    let mut url = format!(
        "{APPVIEW}/xrpc/app.bsky.feed.getAuthorFeed?actor={}&limit={LIMIT}",
        encode(&actor)
    );
    if let Some(c) = cursor {
        url.push_str("&cursor=");
        url.push_str(&encode(&c));
    }
    get(&url).await
}

/// Hydrate a pin by its address, through the Bluesky module's addressing hook.
#[tauri::command]
pub async fn hydrate_pin(address: String) -> Result<String, String> {
    let url = format!("{APPVIEW}{}", bluesky::pins::hydrate_path(&[&address]));
    get(&url).await
}

/// One blocking-free HTTPS GET. Errors are sanitized (no URL/body echo).
async fn get(url: &str) -> Result<String, String> {
    let resp = reqwest::get(url)
        .await
        .map_err(|_| "feed request failed".to_string())?;
    if !resp.status().is_success() {
        return Err(format!("feed service replied {}", resp.status().as_u16()));
    }
    resp.text()
        .await
        .map_err(|_| "couldn't read the response".to_string())
}

/// Minimal percent-encoding for a query value (unreserved set kept).
fn encode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for &b in value.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
