//! B5 — CAR export / re-hydration: rebuild the envelope index from a CAR and
//! converge to the original store state; a missing block is NAMED, not
//! silently absorbed.
//!
//! Matchup row 4: getRepo serves "all repo records, MST nodes, and the
//! current signed commit object, all in a single CAR file" (sync spec), and
//! the repository spec cautions that imported CARs may carry dangling
//! references and un-referenced blocks — so a re-hydrator treats the CAR as
//! untrusted input and rebuilds by verification.
//!
//! The codec here is a minimal **fixture-grade CARv1**: a varint-framed
//! header (dag-cbor `{roots, version}`, the CARv1 layout) followed by
//! varint-framed blocks, constructed in-crate — a deliberate fixture
//! dependency choice (named in the run summary), NOT a wire pin and NOT a
//! full IPLD implementation. The root block is a dag-cbor list of links to
//! entry-record blocks (a fixture index shape standing in for the MST walk).

use crate::envelope::Digest;
use crate::fold::{fold, FoldState};
use crate::record::{from_record, record_bytes};
use ipld_core::cid::multihash::Multihash;
use ipld_core::cid::Cid;
use ipld_core::ipld::Ipld;
use std::collections::BTreeMap;
use std::io::Cursor;

const DAG_CBOR_CODEC: u64 = 0x71;
const SHA2_256: u64 = 0x12;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarError {
    Truncated,
    BadHeader(String),
    BadCid(String),
    /// Re-hydration found a root-referenced block absent from the CAR — the
    /// named, never-silent incompleteness (B5).
    MissingBlock { cid: Cid },
    BadRecord(String),
}

fn cid_for_dag_cbor(bytes: &[u8]) -> Cid {
    use sha2::Digest as _;
    let digest = sha2::Sha256::digest(bytes);
    let mh = Multihash::<64>::wrap(SHA2_256, &digest).expect("sha-256 digest fits");
    Cid::new_v1(DAG_CBOR_CODEC, mh)
}

fn write_varint(out: &mut Vec<u8>, mut v: u64) {
    loop {
        let byte = (v & 0x7f) as u8;
        v >>= 7;
        if v == 0 {
            out.push(byte);
            break;
        }
        out.push(byte | 0x80);
    }
}

fn read_varint(cur: &mut Cursor<&[u8]>) -> Result<u64, CarError> {
    let mut out: u64 = 0;
    let mut shift = 0;
    loop {
        let pos = cur.position() as usize;
        let data = *cur.get_ref();
        let byte = *data.get(pos).ok_or(CarError::Truncated)?;
        cur.set_position(pos as u64 + 1);
        out |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Ok(out);
        }
        shift += 7;
        if shift > 63 {
            return Err(CarError::BadHeader("varint overflow".into()));
        }
    }
}

/// Serialize (roots, blocks) as fixture-grade CARv1 bytes.
pub fn write_car(roots: &[Cid], blocks: &[(Cid, Vec<u8>)]) -> Vec<u8> {
    let mut header = BTreeMap::new();
    header.insert(
        "roots".to_string(),
        Ipld::List(roots.iter().map(|c| Ipld::Link(*c)).collect()),
    );
    header.insert("version".to_string(), Ipld::Integer(1));
    let header_bytes =
        serde_ipld_dagcbor::to_vec(&Ipld::Map(header)).expect("dag-cbor encode of pure map cannot fail");
    let mut out = Vec::new();
    write_varint(&mut out, header_bytes.len() as u64);
    out.extend_from_slice(&header_bytes);
    for (cid, data) in blocks {
        let cid_bytes = cid.to_bytes();
        write_varint(&mut out, (cid_bytes.len() + data.len()) as u64);
        out.extend_from_slice(&cid_bytes);
        out.extend_from_slice(data);
    }
    out
}

/// A parsed CAR: (roots, block map).
pub type CarContents = (Vec<Cid>, BTreeMap<Cid, Vec<u8>>);

/// Parse fixture-grade CARv1 bytes into (roots, block map).
pub fn parse_car(bytes: &[u8]) -> Result<CarContents, CarError> {
    let mut cur = Cursor::new(bytes);
    let header_len = read_varint(&mut cur)? as usize;
    let start = cur.position() as usize;
    let header_bytes = bytes.get(start..start + header_len).ok_or(CarError::Truncated)?;
    cur.set_position((start + header_len) as u64);
    let header: Ipld =
        serde_ipld_dagcbor::from_slice(header_bytes).map_err(|e| CarError::BadHeader(e.to_string()))?;
    let Ipld::Map(m) = header else {
        return Err(CarError::BadHeader("header not a map".into()));
    };
    let roots = match m.get("roots") {
        Some(Ipld::List(l)) => l
            .iter()
            .map(|v| match v {
                Ipld::Link(c) => Ok(*c),
                _ => Err(CarError::BadHeader("non-link root".into())),
            })
            .collect::<Result<Vec<_>, _>>()?,
        _ => return Err(CarError::BadHeader("missing roots".into())),
    };
    let mut blocks = BTreeMap::new();
    while (cur.position() as usize) < bytes.len() {
        let len = read_varint(&mut cur)? as usize;
        let start = cur.position() as usize;
        let section = bytes.get(start..start + len).ok_or(CarError::Truncated)?;
        let mut section_cur = Cursor::new(section);
        let cid = Cid::read_bytes(&mut section_cur).map_err(|e| CarError::BadCid(e.to_string()))?;
        let data = section[section_cur.position() as usize..].to_vec();
        cur.set_position((start + len) as u64);
        blocks.insert(cid, data);
    }
    Ok((roots, blocks))
}

/// Build the fixture export: entry records as dag-cbor blocks + one root
/// index block (a dag-cbor list of links, in rkey order).
pub fn export_car(records: &[Ipld]) -> Vec<u8> {
    let mut blocks = Vec::new();
    let mut links = Vec::new();
    for r in records {
        let data = record_bytes(r);
        let cid = cid_for_dag_cbor(&data);
        links.push(Ipld::Link(cid));
        blocks.push((cid, data));
    }
    let root_data =
        serde_ipld_dagcbor::to_vec(&Ipld::List(links)).expect("dag-cbor encode of pure list cannot fail");
    let root_cid = cid_for_dag_cbor(&root_data);
    let mut all = vec![(root_cid, root_data)];
    all.extend(blocks);
    write_car(&[root_cid], &all)
}

/// Drop one block (by CID) from a CAR — the fixture for an incomplete export.
pub fn drop_block(car: &[u8], victim: &Cid) -> Vec<u8> {
    let (roots, blocks) = parse_car(car).expect("fixture CAR parses");
    let kept: Vec<(Cid, Vec<u8>)> = blocks.into_iter().filter(|(c, _)| c != victim).collect();
    write_car(&roots, &kept)
}

/// Re-hydrate: walk the root index, decode every referenced entry record,
/// rebuild the envelope index, and fold to converged chain state.
///
/// STAGED-RED (B5, replace at green): a root-referenced block absent from
/// the CAR is **silently skipped** — the incomplete store re-hydrates
/// without a word, exactly the failure the assertion must name.
pub fn rehydrate(car: &[u8]) -> Result<FoldState, CarError> {
    let (roots, blocks) = parse_car(car)?;
    let root = roots.first().ok_or(CarError::BadHeader("no root".into()))?;
    let root_data = blocks.get(root).ok_or(CarError::MissingBlock { cid: *root })?;
    let index: Ipld =
        serde_ipld_dagcbor::from_slice(root_data).map_err(|e| CarError::BadRecord(e.to_string()))?;
    let Ipld::List(links) = index else {
        return Err(CarError::BadRecord("root index not a list".into()));
    };
    let mut deliverer = crate::delivery::Deliverer::new();
    let mut delivered = Vec::new();
    for link in links {
        let Ipld::Link(cid) = link else {
            return Err(CarError::BadRecord("non-link index entry".into()));
        };
        let Some(data) = blocks.get(&cid) else {
            // A root-referenced block absent from the CAR: NAME it. (The B5
            // red run captured the silent-skip form re-hydrating an
            // incomplete store without a word before this went green.)
            return Err(CarError::MissingBlock { cid });
        };
        let record: Ipld =
            serde_ipld_dagcbor::from_slice(data).map_err(|e| CarError::BadRecord(e.to_string()))?;
        let (env, _blob_cid) = from_record(&record).map_err(|e| CarError::BadRecord(e.0))?;
        delivered.push(deliverer.deliver(env));
    }
    Ok(fold(delivered))
}

/// The CID of an entry-record block, for fixtures that need to name a victim.
pub fn record_cid(record: &Ipld) -> Cid {
    cid_for_dag_cbor(&record_bytes(record))
}

/// Fold an envelope set (in any order) for comparison against a re-hydration.
pub fn fold_envelopes(envs: impl IntoIterator<Item = crate::envelope::Envelope>) -> FoldState {
    let mut deliverer = crate::delivery::Deliverer::new();
    fold(envs.into_iter().map(|e| deliverer.deliver(e)).collect::<Vec<_>>())
}

/// Named-incompleteness helper for tests: which subspaces a state covers.
pub fn subspaces(state: &FoldState) -> Vec<Digest> {
    state.chains().keys().copied().collect()
}
