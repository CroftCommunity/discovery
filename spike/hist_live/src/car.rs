//! Minimal CAR v1 reader.  Enough to extract blocks {cid → bytes} from the
//! CAR returned by `com.atproto.sync.getRepo` and `getRecord`.  We do NOT
//! implement the writer, we do NOT parse CAR v2, and we do NOT re-validate the
//! MST here — the point is to hand the raw dag-cbor block bytes back to E1/E3
//! so byte-identity assertions can be made.

use cid::Cid;
use std::collections::HashMap;
use std::io::{Cursor, Read};

pub struct CarBlock {
    pub cid: Cid,
    pub data: Vec<u8>,
}

pub struct Car {
    pub roots: Vec<Cid>,
    pub blocks: Vec<CarBlock>,
    /// Convenience lookup.
    pub by_cid: HashMap<Cid, Vec<u8>>,
}

pub fn parse_car(bytes: &[u8]) -> Result<Car, String> {
    let mut cur = Cursor::new(bytes);
    let header_len = read_uvarint(&mut cur)?;
    let mut header = vec![0u8; header_len as usize];
    cur.read_exact(&mut header)
        .map_err(|e| format!("read header: {}", e))?;
    let header_val: HeaderV1 = serde_ipld_dagcbor::from_slice(&header)
        .map_err(|e| format!("decode car header: {}", e))?;
    if header_val.version != 1 {
        return Err(format!("unsupported car version {}", header_val.version));
    }
    let roots = header_val
        .roots
        .into_iter()
        .map(|c| c.0)
        .collect::<Vec<_>>();

    let mut blocks = Vec::new();
    let mut by_cid = HashMap::new();
    loop {
        let pos = cur.position();
        if pos as usize >= bytes.len() {
            break;
        }
        let block_len = match read_uvarint(&mut cur) {
            Ok(n) => n,
            Err(_) => break, // EOF
        };
        if block_len == 0 {
            break;
        }
        let mut block = vec![0u8; block_len as usize];
        cur.read_exact(&mut block)
            .map_err(|e| format!("read block: {}", e))?;
        // The CID at the front of the block is length-prefixed by its own
        // bytes; `Cid::read_bytes` handles all this.
        let mut bcur = Cursor::new(&block[..]);
        let cid = Cid::read_bytes(&mut bcur).map_err(|e| format!("read cid: {}", e))?;
        let data_start = bcur.position() as usize;
        let data = block[data_start..].to_vec();
        by_cid.insert(cid, data.clone());
        blocks.push(CarBlock { cid, data });
    }

    Ok(Car { roots, blocks, by_cid })
}

fn read_uvarint<R: Read>(r: &mut R) -> Result<u64, String> {
    let mut result: u64 = 0;
    let mut shift = 0u32;
    let mut buf = [0u8; 1];
    loop {
        r.read_exact(&mut buf)
            .map_err(|e| format!("uvarint read: {}", e))?;
        let b = buf[0];
        result |= ((b & 0x7f) as u64) << shift;
        if b & 0x80 == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift >= 64 {
            return Err("uvarint too long".into());
        }
    }
}

// Minimal Header type; we ignore any additional fields.
#[derive(serde::Deserialize)]
struct HeaderV1 {
    #[serde(default)]
    version: u64,
    #[serde(default)]
    roots: Vec<CidBytes>,
}

// Serde helper for a CID inside dag-cbor (tag 42 wrap).
struct CidBytes(Cid);

impl<'de> serde::Deserialize<'de> for CidBytes {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // serde_ipld_dagcbor decodes CIDs as ipld_core::cid::Cid values wrapped
        // in tag 42 automatically when the field type is Cid, but for our own
        // structs we need to use ipld_core's serde newtype.  Easiest: decode
        // via ipld_core::ipld::Ipld and pattern-match.
        let ipld = ipld_core::ipld::Ipld::deserialize(d)?;
        match ipld {
            ipld_core::ipld::Ipld::Link(c) => Ok(CidBytes(c)),
            other => Err(serde::de::Error::custom(format!(
                "expected cid, got {:?}",
                other
            ))),
        }
    }
}
