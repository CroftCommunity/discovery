//! B3 — chain-gap detection over cursored pages (RUN-HIST-01).
//!
//! Serves history-durability.md §I (gap request "naming the two bounding
//! digests") and rbsr-construction req. 3 (stateless responder) via
//! HIST-ATPROTO-MATCHUP.md row 3: a mock responder with the listRecords
//! shape (cursored pages, no session state) serves a chain; the member-side
//! consumer names a gap by its bounding digests exactly as §I specifies.
//! RED-able: gap present, consumer must name it — the staged red is the
//! naive last-page consumer, which walks the cursor to exhaustion and calls
//! the gapped chain complete.

use hist_atproto_spike::envelope::fixture_chain;
use hist_atproto_spike::pages::{assemble_chain, ChainCheck, MockPds};

#[test]
fn gap_is_named_by_bounding_digests_across_pages() {
    // A 23-entry chain served in pages of 5, with entry 11 withheld from the
    // record set (a deleted or withheld index record mid-chain).
    let chain = fixture_chain("b3-gapped", 23);
    let subspace = chain[0].0.subspace;
    let mut pds = MockPds::new();
    for (env, _) in &chain {
        pds.insert(env.clone());
    }
    pds.remove(&subspace, 11).expect("victim entry existed");

    let after = chain[10].0.entry_digest; // last held before the break
    let before = chain[12].0.entry_digest; // first served after it

    match assemble_chain(&pds, &subspace, 5) {
        ChainCheck::Gap {
            subspace: s,
            after_digest,
            before_digest,
        } => {
            assert_eq!(s, subspace);
            assert_eq!(
                (after_digest, before_digest),
                (after, before),
                "the §I gap request names exactly the two bounding digests"
            );
        }
        ChainCheck::Complete(entries) => panic!(
            "naive last-page consumption: a {}-entry span with a mid-chain \
             gap was called complete — the gap MUST be named",
            entries.len()
        ),
    }
}

#[test]
fn complete_chain_across_pages_is_complete() {
    let chain = fixture_chain("b3-complete", 17);
    let subspace = chain[0].0.subspace;
    let mut pds = MockPds::new();
    for (env, _) in &chain {
        pds.insert(env.clone());
    }
    match assemble_chain(&pds, &subspace, 4) {
        ChainCheck::Complete(entries) => {
            assert_eq!(entries.len(), 17, "every page consumed");
            assert_eq!(
                entries.iter().map(|e| e.counter).collect::<Vec<_>>(),
                (0..17).collect::<Vec<_>>(),
                "rkey-paged reads deliver the chain in counter order"
            );
        }
        gap => panic!("complete chain misreported: {gap:?}"),
    }
}

#[test]
fn responder_is_stateless_and_cursor_driven() {
    // rbsr-construction req. 3: the responder holds no session — the same
    // cursor replayed yields the same page, and interleaved "sessions"
    // cannot disturb each other.
    let chain = fixture_chain("b3-stateless", 9);
    let subspace = chain[0].0.subspace;
    let mut pds = MockPds::new();
    for (env, _) in &chain {
        pds.insert(env.clone());
    }
    let p1 = pds.list_records(&subspace, None, 4);
    let p1_replay = pds.list_records(&subspace, None, 4);
    assert_eq!(p1.items, p1_replay.items, "no session state: replay is identical");
    assert_eq!(p1.cursor, p1_replay.cursor);

    // Two consumers at different positions, interleaved.
    let c1 = p1.cursor.clone().expect("more pages follow");
    let p2a = pds.list_records(&subspace, Some(&c1), 4);
    let _other_consumer = pds.list_records(&subspace, None, 2);
    let p2b = pds.list_records(&subspace, Some(&c1), 4);
    assert_eq!(p2a.items, p2b.items, "another consumer's reads change nothing");

    // Final page carries no cursor (the XRPC convention: continue until the
    // cursor is not included any longer).
    let c2 = p2a.cursor.expect("one more page");
    let p3 = pds.list_records(&subspace, Some(&c2), 4);
    assert!(p3.cursor.is_none(), "exhausted range ends the cursor chain");
}
