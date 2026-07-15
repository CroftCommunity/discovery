//! Phase 11: binding lifecycle — validity windows, revocation, and key rotation.
//! Extends Phase 8's binding (which had no expiry/revocation) with the controls
//! a real identity system needs.
//!
//! * **Validity window**: the binding carries `not_before`/`not_after`, and both
//!   are inside the signed statement (so they can't be silently extended).
//! * **Revocation**: a separate account-signed revocation record supersedes a
//!   binding; an AppView checks revocations before trusting a binding.
//! * **Key rotation**: a compromised MLS key is handled by revoking the old
//!   binding and issuing a new one for the new key.

use ed25519_dalek::{Signature, Signer as _, SigningKey, Verifier, VerifyingKey};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::signatures::Signer;
use rand::RngCore;
use serde::{Deserialize, Serialize};

pub const BINDING_NSID: &str = "org.croftc.experiment.identity.binding";
pub const REVOCATION_NSID: &str = "org.croftc.experiment.identity.revocation";

pub struct Account {
    signing: SigningKey,
    pub did: String,
}

impl Account {
    pub fn new() -> Self {
        let mut seed = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut seed);
        let signing = SigningKey::from_bytes(&seed);
        let did = format!("did:plc:{}", hex::encode(&signing.verifying_key().to_bytes()[..12]));
        Self { signing, did }
    }
    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing.verifying_key()
    }
}

pub fn did_key(mls_pubkey: &[u8]) -> String {
    format!("did:key:z{}", hex::encode(mls_pubkey))
}

fn pubkey_from_did_key(did: &str) -> Result<VerifyingKey, String> {
    let hexpart = did.strip_prefix("did:key:z").ok_or("not a did:key")?;
    let bytes = hex::decode(hexpart).map_err(|_| "bad did:key hex")?;
    let arr: [u8; 32] = bytes.as_slice().try_into().map_err(|_| "did:key not 32 bytes")?;
    VerifyingKey::from_bytes(&arr).map_err(|_| "did:key invalid".to_string())
}

fn decode_sig(h: &str) -> Result<Signature, String> {
    let bytes = hex::decode(h).map_err(|_| "bad sig hex")?;
    let arr: [u8; 64] = bytes.as_slice().try_into().map_err(|_| "sig not 64 bytes")?;
    Ok(Signature::from_bytes(&arr))
}

/// The statement both keys sign — now includes the validity window.
fn binding_statement(account_did: &str, group_did: &str, nbf: &str, naf: &str) -> Vec<u8> {
    format!("atproto-mls-identity-binding|v2|account={account_did}|group={group_did}|nbf={nbf}|naf={naf}").into_bytes()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdentityBinding {
    #[serde(rename = "$type")]
    pub type_: String,
    pub account_did: String,
    pub group_did: String,
    pub not_before: String,
    pub not_after: String,
    pub sig_account: String,
    pub sig_group: String,
}

pub fn create_binding(
    account: &Account,
    mls_signer: &SignatureKeyPair,
    not_before: &str,
    not_after: &str,
) -> IdentityBinding {
    let group_did = did_key(&mls_signer.to_public_vec());
    let stmt = binding_statement(&account.did, &group_did, not_before, not_after);
    let sig_account = account.signing.sign(&stmt);
    let sig_group = mls_signer.sign(&stmt).expect("MLS sign");
    IdentityBinding {
        type_: BINDING_NSID.to_string(),
        account_did: account.did.clone(),
        group_did,
        not_before: not_before.to_string(),
        not_after: not_after.to_string(),
        sig_account: hex::encode(sig_account.to_bytes()),
        sig_group: hex::encode(sig_group),
    }
}

/// An account-signed revocation that supersedes a binding for (account, group).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Revocation {
    #[serde(rename = "$type")]
    pub type_: String,
    pub account_did: String,
    pub group_did: String,
    pub revoked_at: String,
    pub reason: String,
    pub sig_account: String,
}

fn revocation_statement(account_did: &str, group_did: &str, revoked_at: &str, reason: &str) -> Vec<u8> {
    format!("atproto-mls-identity-revocation|v1|account={account_did}|group={group_did}|at={revoked_at}|reason={reason}").into_bytes()
}

pub fn create_revocation(account: &Account, group_did: &str, revoked_at: &str, reason: &str) -> Revocation {
    let stmt = revocation_statement(&account.did, group_did, revoked_at, reason);
    let sig = account.signing.sign(&stmt);
    Revocation {
        type_: REVOCATION_NSID.to_string(),
        account_did: account.did.clone(),
        group_did: group_did.to_string(),
        revoked_at: revoked_at.to_string(),
        reason: reason.to_string(),
        sig_account: hex::encode(sig.to_bytes()),
    }
}

/// Full check an AppView performs: signatures valid, `now` inside the window,
/// and no valid revocation (effective at/before `now`) supersedes it.
/// atproto datetimes are RFC-3339 UTC, so lexicographic compare == chronological.
pub fn verify(
    b: &IdentityBinding,
    account_pub: &VerifyingKey,
    now: &str,
    revocations: &[Revocation],
) -> Result<(), String> {
    // 1. signatures over the windowed statement
    let stmt = binding_statement(&b.account_did, &b.group_did, &b.not_before, &b.not_after);
    account_pub.verify(&stmt, &decode_sig(&b.sig_account)?).map_err(|_| "account signature invalid".to_string())?;
    let group_pub = pubkey_from_did_key(&b.group_did)?;
    group_pub.verify(&stmt, &decode_sig(&b.sig_group)?).map_err(|_| "group signature invalid".to_string())?;

    // 2. validity window
    if now < b.not_before.as_str() {
        return Err(format!("not yet valid (now {now} < not_before {})", b.not_before));
    }
    if now >= b.not_after.as_str() {
        return Err(format!("expired (now {now} >= not_after {})", b.not_after));
    }

    // 3. revocation: any account-signed revocation for this (account, group)
    //    effective at or before `now` invalidates the binding.
    for r in revocations {
        if r.account_did == b.account_did && r.group_did == b.group_did {
            let rstmt = revocation_statement(&r.account_did, &r.group_did, &r.revoked_at, &r.reason);
            if account_pub.verify(&rstmt, &decode_sig(&r.sig_account)?).is_ok() && r.revoked_at.as_str() <= now {
                return Err(format!("revoked at {} ({})", r.revoked_at, r.reason));
            }
        }
    }
    Ok(())
}
