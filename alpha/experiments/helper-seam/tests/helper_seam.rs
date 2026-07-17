//! EXP-C acceptance tests — RED first, then green.
//!
//! The three claims:
//!  1. Helper as member feeds the index; the SAME index/serve path handles a
//!     public-source row and a helper-fed (private-source) row.
//!  2. Revocation makes the helper forward-blind (MLS forward secrecy): frames
//!     sealed after the epoch roll do not decrypt and produce no index rows;
//!     pre-revocation rows remain.
//!  3. No-authority: the helper's outward write surface is the index only; it
//!     holds no invite/remove/govern capability.

use group_core::ChatMessage;
use group_seal::Sealer;
use helper_seam::{ContentHelper, Index, NormalizedEvent};

const GROUP: &str = "group:stellin-eng";

fn chat(sender: &str, text: &str) -> ChatMessage {
    ChatMessage { sender: sender.to_string(), text: text.to_string() }
}

/// Found a group and admit the helper by grant (real Welcome). Returns the
/// founder (group authority) and the admitted helper, in the same epoch.
fn founded_with_helper() -> (Sealer, ContentHelper) {
    let mut founder = Sealer::found("did:plc:founder").expect("found group");
    let mut helper = ContentHelper::enroll("did:plc:helper").expect("enroll helper");
    let (_commit, welcome) = founder
        .invite(&[helper.key_package().expect("helper kp")])
        .expect("admit helper by grant");
    helper.accept_welcome(&welcome).expect("helper joins from Welcome");
    (founder, helper)
}

// ── Step 1: helper feeds the index; one path serves both sources ──────────────
#[test]
fn helper_feeds_index_and_path_is_source_agnostic() {
    let (mut founder, mut helper) = founded_with_helper();
    let index = Index::open_in_memory().expect("index");

    // A public-source row through the SAME apply path.
    let public_ev = NormalizedEvent::public_post("did:plc:public", "p1", "hiring platform engineers");
    index.apply(&public_ev).expect("apply public row");

    // The founder seals a group message; the helper decrypts it as a member,
    // normalizes it, and feeds the identical index.
    let sealed = founder.seal(&chat("did:plc:founder", "internal: platform team is hiring")).expect("seal");
    let helper_ev = helper.ingest(GROUP, 0, &sealed).expect("helper decrypts + normalizes");
    index.apply(&helper_ev).expect("apply helper row");

    // Source-agnostic shape: the two events expose the identical key set.
    assert_eq!(
        public_ev.key_set(),
        helper_ev.key_set(),
        "public and helper-fed events normalize to the same shape"
    );

    // A search over group content returns the helper-fed hit …
    let hits = index.search("platform").expect("search");
    assert_eq!(hits.len(), 2, "one query returns both a public and a helper-fed hit: {hits:?}");
    let sources: Vec<&str> = hits.iter().map(|h| h.source.as_str()).collect();
    assert!(sources.contains(&"public:jetstream"));
    assert!(sources.contains(&"private:group-helper"));
}

// ── Step 2: revocation → forward-blind ────────────────────────────────────────
#[test]
fn revocation_makes_helper_forward_blind() {
    let (mut founder, mut helper) = founded_with_helper();
    let index = Index::open_in_memory().expect("index");

    // Pre-revocation: the helper indexes a message.
    let before = founder.seal(&chat("did:plc:founder", "pre-revocation secret")).expect("seal");
    let ev_before = helper.ingest(GROUP, 0, &before).expect("helper decrypts pre-revocation");
    index.apply(&ev_before).expect("index pre-revocation row");
    assert_eq!(index.count().unwrap(), 1);

    // The authority revokes the helper (membership remove + epoch roll). The
    // helper does NOT drive this — the founder does.
    let _commit = founder.remove_member("did:plc:helper").expect("revoke helper");

    // Post-revocation: the founder seals under the new epoch.
    let after = founder.seal(&chat("did:plc:founder", "post-revocation secret")).expect("seal after");

    // The helper cannot decrypt the later frame (MLS forward secrecy) → no row.
    let attempt = helper.ingest(GROUP, 1, &after);
    assert!(attempt.is_err(), "a revoked helper cannot decrypt a later-epoch frame");
    if let Ok(ev) = attempt {
        index.apply(&ev).ok();
    }

    // The index still holds exactly the pre-revocation row: revocation is
    // forward-only. What the helper was shown, it was shown.
    assert_eq!(index.count().unwrap(), 1, "no post-revocation row; pre-revocation row remains");
    let hits = index.search("secret").expect("search");
    assert_eq!(hits.len(), 1);
    assert!(hits[0].text.contains("pre-revocation"));
}

// ── Step 3: no-authority (executable where possible) ──────────────────────────
#[test]
fn helper_holds_no_authority_surface() {
    // By construction: ContentHelper wraps a Sealer privately and re-exports only
    // join + ingest. This test asserts the source of the public API carries no
    // membership-mutation / governance method on ContentHelper (mirrors the L2a
    // firewall guard). Where the claim is only assertable by construction, this
    // is the construction it rests on.
    let src = include_str!("../src/helper.rs");

    // Isolate the `impl ContentHelper { … }` block's public method names.
    let impl_block = src
        .split_once("impl ContentHelper")
        .expect("impl block")
        .1;
    for forbidden in ["fn invite", "fn remove_member", "fn grant", "fn revoke", "fn found("] {
        assert!(
            !impl_block.contains(forbidden),
            "ContentHelper must expose no authority method, found `{forbidden}`"
        );
    }

    // And its single outward product is a NormalizedEvent for the index — it
    // returns no commit, key, or membership handle from ingest.
    let (mut founder, mut helper) = founded_with_helper();
    let sealed = founder.seal(&chat("did:plc:founder", "x")).expect("seal");
    let out: NormalizedEvent = helper.ingest(GROUP, 0, &sealed).expect("ingest");
    assert_eq!(out.collection, "app.stellin.groupPost");
}
