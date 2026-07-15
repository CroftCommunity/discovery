//! **E12.7 — Rung-B governance continuity.** The bridge that closes the re-plant loop: it
//! proves the member set the governance fold *derives* (the real `DerivedFold::ingest` path,
//! with authorization and thresholds enforced) is exactly the member set the MLS re-plant
//! *stamps* — and that this correspondence survives governance changes (add / remove) and
//! *rejects* unauthorized ones.
//!
//! This is the only place the two crates meet. The substrate (`local_storage_projection`) stays
//! openmls-free; `mls-replant` stays fold-free; the bridge depends on both by path. A member has
//! two faces here: a **governance identity** (a device signer + a principal the fold reasons
//! over) and an **MLS identity** (an `mls_replant::Persona`). The experiment asserts the two
//! never drift: whoever the fold seats is exactly whom the stamp seats.

pub mod dataplane;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use local_storage_projection::fold_derived::{DerivedFold, FoldError, GroupState, IngestResult};
use local_storage_projection::tables::Db;
use local_storage_projection::traits::{
    CredentialError, CredentialResolver, DeviceId as TraitsDeviceId,
    PrincipalId as TraitsPrincipalId, VerifyError, Verifier,
};
use local_storage_projection::{
    AssertionEnvelope, AssertionType, DeviceId, GroupId, Hash, PrincipalId,
};

use mls_replant::{membership, stamp, Persona, Stamp};
use redb::TableDefinition;

/// A deterministic, self-contained MAC standing in for a real device signature — the bridge
/// crate does not reach into the substrate's test mocks (they are feature-gated) and does not
/// need real crypto to exercise the *governance-continuity* claim: it needs a signer the paired
/// verifier accepts and nothing else rejects. Key-and-message dependent, reproducible.
fn mac(key: &[u8; 32], message: &[u8]) -> Vec<u8> {
    let mut sig = [0u8; 64];
    for (i, b) in sig[..32].iter_mut().enumerate() {
        let mb = if message.is_empty() { 0u8 } else { message[i % message.len()] };
        *b = key[i] ^ mb ^ (i as u8);
    }
    for i in 0..32 {
        sig[32 + i] = !sig[i];
    }
    sig.to_vec()
}

/// Mirror of the fold's derived-state table (see `fold_derived`): the persisted `GroupState`
/// keyed by group id. Stable name — the bridge only reads it.
const STATE_GROUP: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("state_group_v1");

/// The fixed group the experiments operate on.
#[must_use]
pub fn group() -> GroupId {
    GroupId::new([0x67; 32])
}

/// A member with both faces: its governance identity (device signer + principal, what the fold
/// authenticates and reasons over) and its MLS identity (a `Persona`, what the stamp seats).
pub struct Member {
    /// The device signing key (governance identity).
    key: [u8; 32],
    device: DeviceId,
    /// The principal the fold reasons over (governance identity).
    pub principal: PrincipalId,
    /// The MLS crypto identity the stamp seats.
    pub persona: Persona,
}

impl Member {
    /// A fresh member for slot `i`. Device key and principal are derived from `i` (distinct
    /// tags so device ≠ principal), and an independent MLS persona is generated.
    #[must_use]
    pub fn new(i: u8) -> Self {
        let mut dk = [0xD0; 32];
        dk[1] = i;
        let mut pk = [0xC0; 32];
        pk[1] = i;
        Self {
            key: dk,
            device: DeviceId::new(dk),
            principal: PrincipalId::new(pk),
            persona: Persona::new(&format!("member-{i}")),
        }
    }

    /// Author a data-plane [`dataplane::Record`] as this member: a conversation entry carrying the
    /// author's principal, the governance-generation stamp it held, its causal antecedent, and the
    /// (opaque) body. The message-side face of the same member the fold seats.
    #[must_use]
    pub fn record(
        &self,
        gen_stamp: u64,
        antecedent: Option<[u8; 32]>,
        body: &[u8],
    ) -> dataplane::Record {
        dataplane::Record {
            author: *self.principal.as_bytes(),
            gen_stamp,
            antecedent,
            body: body.to_vec(),
        }
    }
}

/// A roster of members whose two identities the bridge keeps mapped, so the MLS membership of a
/// stamp can be translated back to the fold's principals for comparison.
pub struct Roster {
    pub members: Vec<Member>,
}

impl Roster {
    /// A roster of `n` fresh members.
    #[must_use]
    pub fn of(n: u8) -> Self {
        Self { members: (0..n).map(Member::new).collect() }
    }

    fn by_principal(&self, p: &PrincipalId) -> Option<&Member> {
        self.members.iter().find(|m| &m.principal == p)
    }

    fn by_persona_id(&self, id: &[u8]) -> Option<&Member> {
        self.members.iter().find(|m| m.persona.id.as_slice() == id)
    }
}

/// A verifier that authenticates every roster device. Establishing device→signature authenticity
/// for the whole roster is orthogonal to *group membership*, which the fold governs — exactly as a
/// real deployment separates "who is a real device" from "who is in this group".
struct RosterVerifier {
    keys: Vec<[u8; 32]>,
}

impl Verifier for RosterVerifier {
    fn verify(
        &self,
        device_id: &TraitsDeviceId,
        message: &[u8],
        signature: &[u8],
    ) -> Result<(), VerifyError> {
        for k in &self.keys {
            if *k == device_id.0 {
                return if mac(k, message) == signature {
                    Ok(())
                } else {
                    Err(VerifyError::InvalidSignature { device_id: *device_id })
                };
            }
        }
        Err(VerifyError::UnknownDevice(*device_id))
    }
}

/// A credential resolver over the roster's registered (device, principal) pairs.
struct RosterCred {
    allowed: HashSet<([u8; 32], [u8; 32])>,
}

impl CredentialResolver for RosterCred {
    fn resolve(
        &self,
        device: &TraitsDeviceId,
        principal: &TraitsPrincipalId,
    ) -> Result<(), CredentialError> {
        if self.allowed.contains(&(device.0, principal.0)) {
            Ok(())
        } else {
            Err(CredentialError::NotFound { device: *device, principal: *principal })
        }
    }
}

/// The governance chain: a real `DerivedFold` over an in-memory store, driven by hand-crafted,
/// signed, authorized envelopes — the same ingest path the substrate's own tests exercise.
pub struct Chain {
    db: Arc<Db>,
    fold: DerivedFold<RosterVerifier, RosterCred>,
    /// Per-device next lamport (governance facts are per-device monotonic).
    next_lamport: HashMap<[u8; 32], u64>,
    /// The backing file is unlinked but held open by redb for the fold's lifetime; keep the
    /// handle so the path is not reused.
    _tmp: tempfile::NamedTempFile,
}

impl Chain {
    /// Build a chain that authenticates and credential-resolves the whole roster. Group
    /// membership itself is decided by the governance facts ingested later, not here.
    #[must_use]
    pub fn new(roster: &Roster) -> Self {
        let verifier =
            RosterVerifier { keys: roster.members.iter().map(|m| m.key).collect() };
        let mut allowed = HashSet::new();
        for m in &roster.members {
            allowed.insert((*m.device.as_bytes(), *m.principal.as_bytes()));
        }
        let cred = RosterCred { allowed };
        let tmp = tempfile::NamedTempFile::new().expect("tempfile");
        let db = Arc::new(Db::open(tmp.path()).expect("open db"));
        let fold = DerivedFold::new(Arc::clone(&db), verifier, cred);
        Self { db, fold, next_lamport: HashMap::new(), _tmp: tmp }
    }

    fn lamport_for(&mut self, device: &DeviceId) -> u64 {
        let e = self.next_lamport.entry(*device.as_bytes()).or_insert(0);
        let l = *e;
        *e += 1;
        l
    }

    fn build(&mut self, author: &Member, ty: AssertionType, payload: Vec<u8>) -> AssertionEnvelope {
        let lamport = self.lamport_for(&author.device);
        let mut env = AssertionEnvelope {
            version: 0x01,
            assertion_type: ty,
            author_device: author.device,
            author_principal: author.principal,
            group: group(),
            antecedents: vec![] as Vec<Hash>,
            lamport,
            timestamp: 1_700_000_000 + lamport,
            payload,
            signature: vec![],
        };
        env.signature = mac(&author.key, &env.canonical_bytes());
        env
    }

    /// Genesis: `founder` creates the group (and is seated Owner by the fold) with the given
    /// four thresholds.
    pub fn genesis(
        &mut self,
        founder: &Member,
        thresholds: [u32; 4],
    ) -> Result<IngestResult, FoldError> {
        let mut p = Vec::with_capacity(50);
        p.extend_from_slice(&1u16.to_be_bytes());
        for t in thresholds {
            p.extend_from_slice(&t.to_be_bytes());
        }
        p.extend_from_slice(founder.device.as_bytes());
        let env = self.build(founder, AssertionType::GroupGenesis, p);
        self.fold.ingest(&env)
    }

    /// `author` adds `subject` at `role`. Authorized iff `author`'s role + the group's
    /// add threshold are satisfied — the fold decides.
    pub fn add(
        &mut self,
        author: &Member,
        subject: &Member,
        role: u8,
    ) -> Result<IngestResult, FoldError> {
        let mut p = Vec::with_capacity(33);
        p.extend_from_slice(subject.principal.as_bytes());
        p.push(role);
        let env = self.build(author, AssertionType::MembershipAdd, p);
        self.fold.ingest(&env)
    }

    /// `author` removes `subject`. Authorized iff `author`'s role + the remove threshold are
    /// satisfied — the fold decides.
    pub fn remove(&mut self, author: &Member, subject: &Member) -> Result<IngestResult, FoldError> {
        let env = self.build(author, AssertionType::MembershipRemove, subject.principal.as_bytes().to_vec());
        self.fold.ingest(&env)
    }

    /// Read the fold's derived `GroupState` from the persisted store.
    fn group_state(&self) -> Option<GroupState> {
        let rtxn = self.db.inner().begin_read().ok()?;
        let tbl = rtxn.open_table(STATE_GROUP).ok()?;
        let raw = tbl.get(group().as_bytes().as_ref()).ok()??;
        GroupState::from_bytes(raw.value()).ok()
    }

    /// The member set the fold currently derives, as a sorted principal list — the thing the
    /// re-plant must stamp exactly.
    #[must_use]
    pub fn derived_members(&self) -> Vec<PrincipalId> {
        let mut v: Vec<PrincipalId> =
            self.group_state().map(|s| s.members.iter().map(|(p, _, _)| *p).collect()).unwrap_or_default();
        v.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
        v
    }
}

/// **Stamp a fresh MLS group over the fold's derived member set.** The planter is the
/// lowest-principal member (deterministic; any member yields the same membership — E12.3). This
/// is the re-plant reading the governance chain and re-keying over exactly whom it seats.
///
/// Returns `None` if the derived set is empty (no group).
#[must_use]
pub fn restamp(roster: &Roster, derived: &[PrincipalId]) -> Option<Stamp> {
    let mut members: Vec<&Member> =
        derived.iter().filter_map(|p| roster.by_principal(p)).collect();
    if members.is_empty() {
        return None;
    }
    members.sort_by(|a, b| a.principal.as_bytes().cmp(b.principal.as_bytes()));
    let planter = members[0];
    let others: Vec<&Persona> = members[1..].iter().map(|m| &m.persona).collect();
    Some(stamp(&planter.persona, &others))
}

/// Translate the MLS membership of a stamp back to the fold's principals (sorted), so it can be
/// compared to `Chain::derived_members`. If the stamp ever seats a persona no roster member
/// owns, that persona is dropped — and the mismatch surfaces as a set difference.
#[must_use]
pub fn stamped_principals(roster: &Roster, s: &Stamp) -> Vec<PrincipalId> {
    let mut v: Vec<PrincipalId> = membership(&s.group)
        .iter()
        .filter_map(|id| roster.by_persona_id(id).map(|m| m.principal))
        .collect();
    v.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
    v
}
