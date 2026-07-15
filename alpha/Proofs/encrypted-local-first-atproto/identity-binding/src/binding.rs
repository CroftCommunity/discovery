//! Verifiable binding between a group identity (MLS-derived `did:key`) and a PDS
//! account (`did:plc`) — the issue Phase 6 surfaced (two identifiers for one
//! principal, with no cryptographic link).
//!
//! The binding is **bidirectional and mutually signed**: a single statement
//! naming both DIDs is signed by *both* the account key and the MLS group key.
//! Verifying it requires only:
//!   * the account's verification key (an AppView gets this from the `did:plc`
//!     DID document — the trust root), and
//!   * the group key, which is *embedded in the `did:key` itself* (so no extra
//!     lookup — the verifier extracts it from the binding).
//!
//! This is exactly the property an AppView needs: given a record published under
//! `did:plc:X`, plus this binding, it can prove the same principal controls group
//! key `did:key:Y` — without trusting anyone beyond the DID document.
//!
//! `did:key` here uses a reversible `did:key:z<hex(pubkey)>` form for clarity;
//! real atproto uses multibase/multicodec (`did:key:z6Mk…`), equally reversible.

use ed25519_dalek::{Signature, SigningKey, Verifier, VerifyingKey};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::signatures::Signer;
use rand::RngCore;
use serde::{Deserialize, Serialize};

/// The PDS account side: an Ed25519 key whose DID an AppView resolves to its
/// verification key. (Real `did:plc` is a hash of a genesis op; we derive the
/// DID from the key so it stays self-consistent for the demo.)
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

/// `did:key` for an Ed25519 public key (the group/MLS identity).
pub fn did_key(mls_pubkey: &[u8]) -> String {
    format!("did:key:z{}", hex::encode(mls_pubkey))
}

/// Extract the public key bytes from a `did:key` produced by [`did_key`].
fn pubkey_from_did_key(did: &str) -> Result<VerifyingKey, String> {
    let hexpart = did.strip_prefix("did:key:z").ok_or("not a did:key")?;
    let bytes = hex::decode(hexpart).map_err(|_| "bad did:key hex")?;
    let arr: [u8; 32] = bytes.as_slice().try_into().map_err(|_| "did:key not 32 bytes")?;
    VerifyingKey::from_bytes(&arr).map_err(|_| "did:key not a valid ed25519 key".to_string())
}

/// The exact bytes both parties sign. Versioned + domain-separated.
fn statement(account_did: &str, group_did: &str) -> Vec<u8> {
    format!("atproto-mls-identity-binding|v1|account={account_did}|group={group_did}").into_bytes()
}

/// A publishable binding record (atproto-shaped: carries `$type`).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdentityBinding {
    #[serde(rename = "$type")]
    pub type_: String,
    pub account_did: String,
    pub group_did: String,
    /// Account key's signature over the statement (hex).
    pub sig_account: String,
    /// MLS group key's signature over the statement (hex).
    pub sig_group: String,
}

pub const BINDING_NSID: &str = "org.croftc.experiment.identity.binding";

/// Build a binding: both the account key and the MLS group key sign the
/// statement naming both DIDs.
pub fn create_binding(account: &Account, mls_signer: &SignatureKeyPair) -> IdentityBinding {
    let group_did = did_key(&mls_signer.to_public_vec());
    let stmt = statement(&account.did, &group_did);
    let sig_account = account.signing.sign(&stmt);
    let sig_group = mls_signer.sign(&stmt).expect("MLS sign"); // raw ed25519 bytes
    IdentityBinding {
        type_: BINDING_NSID.to_string(),
        account_did: account.did.clone(),
        group_did,
        sig_account: hex::encode(sig_account.to_bytes()),
        sig_group: hex::encode(sig_group),
    }
}

/// Verify a binding using only the account's verification key (from the DID doc);
/// the group key is taken from the binding's own `did:key`. Returns Ok(()) iff
/// BOTH signatures are valid over the statement naming both DIDs.
pub fn verify_binding(b: &IdentityBinding, account_pub: &VerifyingKey) -> Result<(), String> {
    let stmt = statement(&b.account_did, &b.group_did);

    let sig_account = decode_sig(&b.sig_account)?;
    account_pub
        .verify(&stmt, &sig_account)
        .map_err(|_| "account signature invalid".to_string())?;

    let group_pub = pubkey_from_did_key(&b.group_did)?;
    let sig_group = decode_sig(&b.sig_group)?;
    group_pub
        .verify(&stmt, &sig_group)
        .map_err(|_| "group (MLS) signature invalid".to_string())?;

    Ok(())
}

use ed25519_dalek::Signer as _;

/// ATTACK SIMULATOR: an attacker tries to claim `victim_group_did` under their
/// own account, using the only keys they have (their own account key + their own
/// MLS key). The resulting binding's group signature is over the victim's
/// `did:key` but signed with the attacker's MLS key, so it fails verification.
pub fn forge_binding(
    attacker_account: &Account,
    victim_group_did: &str,
    attacker_mls: &SignatureKeyPair,
) -> IdentityBinding {
    let stmt = statement(&attacker_account.did, victim_group_did);
    let sig_account = attacker_account.signing.sign(&stmt);
    let sig_group = attacker_mls.sign(&stmt).expect("attacker MLS sign");
    IdentityBinding {
        type_: BINDING_NSID.to_string(),
        account_did: attacker_account.did.clone(),
        group_did: victim_group_did.to_string(),
        sig_account: hex::encode(sig_account.to_bytes()),
        sig_group: hex::encode(sig_group),
    }
}

fn decode_sig(h: &str) -> Result<Signature, String> {
    let bytes = hex::decode(h).map_err(|_| "bad signature hex")?;
    let arr: [u8; 64] = bytes.as_slice().try_into().map_err(|_| "signature not 64 bytes")?;
    Ok(Signature::from_bytes(&arr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mls;

    #[test]
    fn valid_binding_verifies_and_tampering_fails() {
        let alice = mls::Member::new("Alice");
        let account = Account::new();
        let binding = create_binding(&account, &alice.signer);
        assert!(verify_binding(&binding, &account.verifying_key()).is_ok());

        // Tamper: flip the group signature -> fails.
        let mut bad = binding.clone();
        bad.sig_group.replace_range(0..1, if bad.sig_group.starts_with('a') { "b" } else { "a" });
        assert!(verify_binding(&bad, &account.verifying_key()).is_err());

        // Wrong account key -> account signature fails.
        let other = Account::new();
        assert!(verify_binding(&binding, &other.verifying_key()).is_err());
    }
}
