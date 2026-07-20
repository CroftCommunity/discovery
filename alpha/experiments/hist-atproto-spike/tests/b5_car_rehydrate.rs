//! B5 — CAR re-hydration (RUN-HIST-01).
//!
//! Serves history-durability.md §D (helper removal / store replacement) via
//! HIST-ATPROTO-MATCHUP.md row 4: from a fixture CAR (constructed in-crate —
//! the fixture-grade CARv1 codec in src/car.rs, the dependency choice named
//! in the run summary), rebuild the envelope index and show convergence
//! equality with the original store state. RED-able: a missing block yields
//! a NAMED, not silent, incompleteness — the staged red skips missing blocks
//! silently.

use hist_atproto_spike::car::{drop_block, export_car, fold_envelopes, record_cid, rehydrate, CarError};
use hist_atproto_spike::envelope::fixture_chain;
use hist_atproto_spike::record::to_record;

#[test]
fn full_car_rehydrates_to_convergence_equality() {
    // Two subspaces, exported in rkey order.
    let a = fixture_chain("b5-a", 9);
    let b = fixture_chain("b5-b", 4);
    let records: Vec<_> = a
        .iter()
        .chain(b.iter())
        .map(|(e, blob)| to_record(e, blob))
        .collect();
    let car = export_car(&records);

    let rebuilt = rehydrate(&car).expect("complete CAR re-hydrates");
    let original = fold_envelopes(a.iter().chain(b.iter()).map(|(e, _)| e.clone()));

    assert_eq!(
        rebuilt.digest(),
        original.digest(),
        "re-hydrated store state converges byte-identically with the original"
    );
    assert_eq!(rebuilt, original);
    assert_eq!(rebuilt.pending_count(), 0, "nothing dangles after re-hydration");
}

#[test]
fn missing_block_is_named_never_silent() {
    let chain = fixture_chain("b5-missing", 6);
    let records: Vec<_> = chain.iter().map(|(e, blob)| to_record(e, blob)).collect();
    let car = export_car(&records);

    // Drop entry 3's record block from the CAR (an incomplete export — the
    // repository spec's dangling-reference caution made concrete).
    let victim = record_cid(&records[3]);
    let truncated = drop_block(&car, &victim);

    match rehydrate(&truncated) {
        Err(CarError::MissingBlock { cid }) => {
            assert_eq!(cid, victim, "the incompleteness is named by the missing CID");
        }
        Ok(state) => panic!(
            "silent incompleteness: re-hydration of a CAR missing a \
             root-referenced block returned a state ({} chained entries) \
             instead of naming the missing block",
            state.chains().values().map(Vec::len).sum::<usize>()
        ),
        Err(other) => panic!("wrong rejection shape: {other:?}"),
    }
}

#[test]
fn car_roundtrip_preserves_blocks() {
    use hist_atproto_spike::car::{parse_car, write_car};
    let chain = fixture_chain("b5-codec", 3);
    let records: Vec<_> = chain.iter().map(|(e, blob)| to_record(e, blob)).collect();
    let car = export_car(&records);
    let (roots, blocks) = parse_car(&car).expect("fixture CAR parses");
    assert_eq!(roots.len(), 1);
    assert_eq!(blocks.len(), 4, "root index + 3 entry records");
    // Re-serialize → re-parse → identical block map (codec fidelity).
    let car2 = write_car(&roots, &blocks.clone().into_iter().collect::<Vec<_>>());
    let (roots2, blocks2) = parse_car(&car2).expect("re-serialized CAR parses");
    assert_eq!(roots, roots2);
    assert_eq!(blocks, blocks2);
}
