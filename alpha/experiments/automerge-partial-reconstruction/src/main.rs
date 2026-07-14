// Automerge (Rust crate) partial-reconstruction and snapshot verification.
//
// Mirrors the four JS scenarios on the real `automerge` crate using the
// AutoCommit ergonomic API. Each scenario prints labeled, unambiguous results.

use automerge::transaction::Transactable;
use automerge::{AutoCommit, Change, ObjId, ObjType, ReadDoc, ROOT};

const CRATE_VERSION: &str = env!("AM_VERSION_NOTE");

// Read the `messages` list. None => field absent; Some(vec) => present.
fn read_messages(doc: &AutoCommit) -> Option<Vec<String>> {
    match doc.get(ROOT, "messages") {
        Ok(Some((value, list_id))) => {
            if value.is_object() {
                let n = doc.length(&list_id);
                let mut out = Vec::with_capacity(n);
                for i in 0..n {
                    match doc.get(&list_id, i) {
                        Ok(Some((v, _))) => {
                            out.push(v.into_string().unwrap_or_else(|v| format!("{v:?}")))
                        }
                        _ => out.push("<none>".to_string()),
                    }
                }
                Some(out)
            } else {
                Some(vec![format!("<scalar: {value:?}>")])
            }
        }
        Ok(None) => None,
        Err(e) => {
            println!("    [read_messages] get() returned Err: {e}");
            None
        }
    }
}

fn current_list(doc: &mut AutoCommit) -> ObjId {
    match doc.get(ROOT, "messages").expect("get messages") {
        Some((_, id)) => id,
        None => panic!("messages list missing"),
    }
}

// Build canonical four-change history; return discrete changes [c1,c2,c3,c4].
fn build_four_changes() -> Vec<Change> {
    let mut doc = AutoCommit::new();

    let list = doc
        .put_object(ROOT, "messages", ObjType::List)
        .expect("put_object messages");
    doc.insert(&list, 0, "e1-m0").expect("insert e1-m0");
    doc.commit();

    let list = current_list(&mut doc);
    doc.insert(&list, 1, "e1-m1").expect("insert e1-m1");
    doc.commit();

    let list = current_list(&mut doc);
    doc.insert(&list, 2, "e2-m2").expect("insert e2-m2");
    doc.commit();

    let list = current_list(&mut doc);
    doc.insert(&list, 3, "e2-m3").expect("insert e2-m3");
    doc.commit();

    // 0.7: get_changes returns owned Vec<Change> (0.6.x returned Vec<&Change>).
    doc.get_changes(&[]).into_iter().collect()
}

fn fmt(opt: &Option<Vec<String>>) -> String {
    match opt {
        None => "ABSENT (get returned Ok(None))".to_string(),
        Some(v) => format!("{:?} (len {})", v, v.len()),
    }
}

fn pass(b: bool) -> &'static str { if b { "PASS" } else { "FAIL" } }

fn short(h: &automerge::ChangeHash) -> String {
    let s = h.to_string();
    format!("{}…", &s[..s.len().min(8)])
}

fn main() {
    println!("=================================================================");
    println!("Automerge Rust crate verification");
    println!("Resolved crate version: {CRATE_VERSION}");
    println!("Toolchain: rustc/cargo 1.94.1");
    println!("=================================================================\n");

    let changes = build_four_changes();
    println!("Captured {} discrete changes via get_changes(&[]).", changes.len());
    println!("  API: AutoCommit::get_changes(&self, have_deps: &[ChangeHash]) -> Vec<Change>");
    println!("  Order: c1=init+e1-m0, c2=e1-m1, c3=e2-m2, c4=e2-m3\n");

    let c1 = changes[0].clone();
    let c2 = changes[1].clone();
    let c3 = changes[2].clone();
    let c4 = changes[3].clone();

    // ---- Scenario A ------------------------------------------------
    println!("---------------------------------------------------------------");
    println!("SCENARIO A — partial application with MISSING dependencies");
    println!("---------------------------------------------------------------");
    let mut doc_a = AutoCommit::new();
    println!("  API: AutoCommit::apply_changes(impl IntoIterator<Item=Change>) -> Result<(), AutomergeError>");
    let res_a = doc_a.apply_changes(vec![c3.clone(), c4.clone()]);
    println!("  apply_changes([c3, c4]) returned: {res_a:?}");
    let missing = doc_a.get_missing_deps(&[]);
    println!("  get_missing_deps(&[]) -> {} hash(es): {:?}",
        missing.len(), missing.iter().map(short).collect::<Vec<_>>());
    let read_a = read_messages(&doc_a);
    println!("  read messages -> {}", fmt(&read_a));
    println!("  doc heads -> {} (empty => nothing visible)", doc_a.get_heads().len());
    let a_ok = res_a.is_ok() && read_a.is_none() && !missing.is_empty();
    println!("  EXPECTED: apply Ok, messages ABSENT, deps buffered, no error.");
    println!("  RESULT: {}", if a_ok { "MATCH — changes held inert, no partial state" }
        else { "*** DIVERGENCE FROM JS — INVESTIGATE ***" });

    // ---- Scenario B ------------------------------------------------
    println!("\n---------------------------------------------------------------");
    println!("SCENARIO B — dependencies arrive afterward (continues from A)");
    println!("---------------------------------------------------------------");
    let res_b = doc_a.apply_changes(vec![c1.clone(), c2.clone()]);
    println!("  apply_changes([c1, c2]) returned: {res_b:?}");
    let missing_b = doc_a.get_missing_deps(&[]);
    println!("  get_missing_deps(&[]) -> {} hash(es)", missing_b.len());
    let read_b = read_messages(&doc_a);
    println!("  read messages -> {}", fmt(&read_b));
    let expected_b = vec!["e1-m0", "e1-m1", "e2-m2", "e2-m3"];
    let b_ok = read_b.as_deref().map(|v| v == expected_b).unwrap_or(false);
    println!("  EXPECTED: {expected_b:?} in causal order.");
    println!("  RESULT: {}", if b_ok { "MATCH — buffered changes materialized once deps satisfied" }
        else { "*** DIVERGENCE — INVESTIGATE ***" });

    // ---- Scenario C ------------------------------------------------
    println!("\n---------------------------------------------------------------");
    println!("SCENARIO C — snapshot via save/load");
    println!("---------------------------------------------------------------");
    let mut full = AutoCommit::new();
    full.apply_changes(vec![c1.clone(), c2.clone(), c3.clone(), c4.clone()])
        .expect("apply full history");
    println!("  API: AutoCommit::save(&mut self) -> Vec<u8>");
    let bytes = full.save();
    println!("  save() -> {} bytes", bytes.len());
    println!("  API: AutoCommit::load(&[u8]) -> Result<AutoCommit, AutomergeError>");
    match AutoCommit::load(&bytes) {
        Ok(loaded_doc) => {
            // 0.7: get_missing_deps takes &self (0.6.x took &mut self).
            let missing_c = loaded_doc.get_missing_deps(&[]);
            let read_c = read_messages(&loaded_doc);
            println!("  loaded doc read messages -> {}", fmt(&read_c));
            println!("  loaded doc get_missing_deps -> {} (0 => self-contained)", missing_c.len());
            let c_ok = read_c.as_deref().map(|v| v == expected_b).unwrap_or(false)
                && missing_c.is_empty();
            println!("  EXPECTED: {expected_b:?}, zero missing deps (compacted).");
            println!("  RESULT: {}", if c_ok { "MATCH — snapshot is self-contained" }
                else { "*** DIVERGENCE — INVESTIGATE ***" });
        }
        Err(e) => println!("  load() ERROR: {e}  *** DIVERGENCE ***"),
    }

    // ---- Scenario D ------------------------------------------------
    println!("\n---------------------------------------------------------------");
    println!("SCENARIO D — incremental changes since a known point (heads)");
    println!("---------------------------------------------------------------");
    let mut epoch1 = AutoCommit::new();
    epoch1.apply_changes(vec![c1.clone(), c2.clone()]).expect("apply epoch1");
    println!("  API: AutoCommit::get_heads(&mut self) -> Vec<ChangeHash>");
    let heads = epoch1.get_heads();
    println!("  epoch-1 heads -> {} hash(es): {:?}", heads.len(),
        heads.iter().map(short).collect::<Vec<_>>());

    let mut newer = AutoCommit::new();
    newer.apply_changes(vec![c1.clone(), c2.clone(), c3.clone(), c4.clone()])
        .expect("apply four");
    let list = current_list(&mut newer);
    newer.insert(&list, 4, "e2-m4").expect("insert e2-m4");
    newer.commit();

    println!("  API: AutoCommit::get_changes(&self, have_deps: &[ChangeHash]) -> Vec<Change>");
    let incremental: Vec<Change> = newer.get_changes(&heads).into_iter().collect();
    println!("  get_changes(since epoch-1 heads) -> {} change(s)", incremental.len());

    epoch1.apply_changes(incremental.clone()).expect("apply incremental");
    let read_d = read_messages(&epoch1);
    println!("  epoch-1 doc after incremental apply -> {}", fmt(&read_d));
    let expected_d = vec!["e1-m0", "e1-m1", "e2-m2", "e2-m3", "e2-m4"];
    let d_ok = incremental.len() == 3
        && read_d.as_deref().map(|v| v == expected_d).unwrap_or(false);
    println!("  EXPECTED: 3 incremental changes (c3, c4, c5); final {expected_d:?}.");
    println!("  RESULT: {}", if d_ok { "MATCH — clean incremental sync" }
        else { "*** DIVERGENCE — INVESTIGATE ***" });

    println!("\n=================================================================");
    println!("SUMMARY");
    println!("  A (missing deps held inert):     {}", pass(a_ok));
    println!("  B (resolves when deps arrive):   {}", pass(b_ok));
    println!("  D (incremental since heads = 3): {}", pass(d_ok));
    println!("=================================================================");
}
