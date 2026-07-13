//! Performing effects via the browser's fetch (spec 1.5). The browser does TLS,
//! so there is no Rust TLS crate and nothing to license-flag. Reads public,
//! unauthenticated feeds (spec 0b); parsing reuses the native `bluesky::wire`
//! parser. Also hydrates a pin by address through the Bluesky module's hooks
//! (Phase 2 M2.4).

use app_core::{Cursor, Intent};
use bluesky::pins::hydrate_path;
use bluesky::wire::{parse_posts, parse_timeline};
use wasm_bindgen::{JsCast, JsValue};

const APPVIEW: &str = "https://public.api.bsky.app";
const LIMIT: u32 = 25;

/// When running inside a Tauri webview, perform the fetch through a native Rust
/// HTTP handler (`desktop/effects.rs`) via `invoke`, returning the raw response
/// body. Returns `None` when not under Tauri (the web build), so the caller
/// falls back to the browser's fetch. This is the one place the effect handler
/// differs per platform (spec 2.1); parsing stays shared (`bluesky::wire`).
async fn tauri_invoke(cmd: &str, args: JsValue) -> Option<String> {
    use js_sys::{Function, Promise, Reflect};
    use wasm_bindgen_futures::JsFuture;

    let window = web_sys::window()?;
    let tauri = Reflect::get(&window, &JsValue::from_str("__TAURI__")).ok()?;
    if tauri.is_undefined() {
        return None; // the web build: fall back to browser fetch
    }
    let core = Reflect::get(&tauri, &JsValue::from_str("core")).ok()?;
    let invoke: Function = Reflect::get(&core, &JsValue::from_str("invoke"))
        .ok()?
        .dyn_into()
        .ok()?;
    let promise: Promise = invoke
        .call2(&core, &JsValue::from_str(cmd), &args)
        .ok()?
        .dyn_into()
        .ok()?;
    JsFuture::from(promise).await.ok()?.as_string()
}

/// Perform one feed fetch for `actor` and translate the outcome into the
/// follow-up intent the core expects.
pub async fn fetch_feed(actor: &str, cursor: Option<Cursor>) -> Intent {
    // Desktop (Tauri): perform the GET natively and parse the returned body.
    let args = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&args, &"actor".into(), &actor.into());
    if let Some(c) = &cursor {
        let _ = js_sys::Reflect::set(&args, &"cursor".into(), &c.0.as_str().into());
    }
    if let Some(body) = tauri_invoke("fetch_feed", args.into()).await {
        return match parse_timeline(&body) {
            Ok(page) => Intent::FeedPageLoaded {
                posts: page.posts,
                next_cursor: page.next_cursor.map(Cursor),
            },
            Err(reason) => Intent::FeedLoadFailed { reason },
        };
    }

    // Web: the browser's fetch (browser does TLS; no Rust TLS crate).
    let mut url = format!(
        "{APPVIEW}/xrpc/app.bsky.feed.getAuthorFeed?actor={}&limit={LIMIT}",
        encode(actor)
    );
    if let Some(c) = &cursor {
        url.push_str("&cursor=");
        url.push_str(&encode(&c.0));
    }

    match gloo_net::http::Request::get(&url).send().await {
        Ok(resp) if resp.ok() => match resp.text().await {
            Ok(body) => match parse_timeline(&body) {
                Ok(page) => Intent::FeedPageLoaded {
                    posts: page.posts,
                    next_cursor: page.next_cursor.map(Cursor),
                },
                Err(reason) => Intent::FeedLoadFailed { reason },
            },
            Err(_) => Intent::FeedLoadFailed {
                reason: "couldn't read the response".to_string(),
            },
        },
        Ok(resp) => Intent::FeedLoadFailed {
            reason: format!("the feed service replied {}", resp.status()),
        },
        Err(_) => Intent::FeedLoadFailed {
            reason: "couldn't reach the feed".to_string(),
        },
    }
}

/// Hydrate a pin by its address (Bluesky URI) through the module's hooks.
/// Returns `(author, snippet)` when live, or `None` when the target is gone
/// (deleted / unavailable) so the strip can degrade gracefully.
pub async fn hydrate_pin(address: &str) -> Option<(String, String)> {
    // Desktop (Tauri) first, then the browser's fetch.
    let body = {
        let args = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&args, &"address".into(), &address.into());
        match tauri_invoke("hydrate_pin", args.into()).await {
            Some(b) => b,
            None => {
                let url = format!("{APPVIEW}{}", hydrate_path(&[address]));
                let resp = gloo_net::http::Request::get(&url).send().await.ok()?;
                if !resp.ok() {
                    return None;
                }
                resp.text().await.ok()?
            }
        }
    };
    let post = parse_posts(&body).ok()?.into_iter().next()?;
    let author = post
        .author
        .display_name
        .clone()
        .unwrap_or_else(|| post.author.handle.clone());
    Some((author, post.record.text))
}

/// Minimal percent-encoding for a query value (keeps the unreserved set).
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
