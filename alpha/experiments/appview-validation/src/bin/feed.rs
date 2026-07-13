//! Real-time trending feed with identity hydration (no credentials needed):
//!   #3  TRENDING FEED: ingest posts AND likes concurrently for a window, then
//!       rank the most-liked subjects observed — real feed-generator logic over
//!       the live like firehose.
//!   #4  IDENTITY + CONTENT HYDRATION: for each trending post, resolve the author
//!       DID -> handle/displayName (PLC + profile record) and fetch the post text
//!       (com.atproto.repo.getRecord), assembling a human-readable view from the
//!       opaque `at://did/...` references the firehose carries.
//!
//! This is what an AppView does beyond storage: turn DIDs and URIs into a view a
//! human can read. It also fixes the earlier "likes rarely reference posts we
//! indexed" gap — we hydrate trending subjects on demand by fetching them.

use std::time::Duration;

use anyhow::Result;

use appview_validation::atproto;
use appview_validation::index::{Index, IndexStats};
use appview_validation::jetstream::JetstreamSource;
use appview_validation::record_source::{ParseOutcome, RecordSource};

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("install rustls ring crypto provider");

    let mut index = Index::open("feed.sqlite")?;
    let mut stats = IndexStats::default();

    // ───────────── #3: ingest posts + likes together, then rank ─────────────
    println!("\n############ #3: real-time trending feed ############");
    println!("  ingesting app.bsky.feed.post + app.bsky.feed.like for ~25s…");
    let mut src = JetstreamSource::new(
        &["app.bsky.feed.post", "app.bsky.feed.like"],
        20_000,
        Duration::from_secs(25),
    );
    let (mut posts, mut likes) = (0u64, 0u64);
    src.run(|outcome| {
        if let ParseOutcome::Commit { event, .. } = outcome {
            if event.collection == "app.bsky.feed.like" {
                likes += 1;
            } else {
                posts += 1;
            }
            let _ = index.apply(&event, &mut stats);
        }
        true
    })
    .await?;
    println!("  ingested {posts} posts and {likes} likes.");

    let trending = index.top_liked_subjects(8)?;
    println!("  top liked subjects this window: {} distinct", trending.len());

    // ───────────── #4: hydrate each trending post (identity + content) ─────────────
    println!("\n############ #4: hydrate trending posts (DID -> handle, fetch text) ############");
    let http = atproto::client();
    let mut rank = 0;
    for (subject_uri, like_count) in trending {
        let Some((did, collection, rkey)) = atproto::parse_at_uri(&subject_uri) else {
            continue;
        };
        if collection != "app.bsky.feed.post" {
            continue; // likes can target other things; we hydrate posts
        }
        rank += 1;

        // Identity hydration: DID -> PDS + handle (and displayName from profile).
        let (handle, display, text) = match atproto::resolve_identity(&http, &did).await {
            Ok((pds, handle)) => {
                let (display, _desc) = atproto::get_profile(&http, &pds, &did).await;
                let text = atproto::get_record(&http, &pds, &did, &collection, &rkey)
                    .await
                    .ok()
                    .and_then(|r| r.get("text").and_then(|t| t.as_str()).map(str::to_string));
                (handle, display, text)
            }
            Err(_) => ("<unresolved>".to_string(), None, None),
        };

        let who = match &display {
            Some(d) => format!("@{handle} ({d})"),
            None => format!("@{handle}"),
        };
        let snippet = text
            .map(|t| {
                let one_line = t.replace('\n', " ");
                if one_line.chars().count() > 100 {
                    format!("{}…", one_line.chars().take(100).collect::<String>())
                } else {
                    one_line
                }
            })
            .unwrap_or_else(|| "<text unavailable (deleted/blocked/no access)>".to_string());

        println!("\n  #{rank}  ♥ {like_count}   {who}");
        println!("       \"{snippet}\"");
        println!("       {subject_uri}");
    }
    if rank == 0 {
        println!("  (no post-subject likes in this window — rerun; volume varies.)");
    }

    println!("\n  => trending detected from the live like firehose, then hydrated: the");
    println!("     opaque at://did/... references became real authors + readable text.");
    println!("\n############ DONE (feed) ############");
    Ok(())
}
