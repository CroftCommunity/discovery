# RUN-FS-01 — the XMTP-ambassador receipt lane: open questions for owner walk

`Draft, 2026-07-20. This is a **pre-brief** — the questions the owner needs
to walk BEFORE RUN-FS-01 can execute red-first. The RUN-AP-01 pattern
(brief §1 "Settled inputs — the five walked verdicts") requires that the
five verdicts land as OWNER DECISIONS, not as agent guesses. This document
enumerates the XMTP-specific angles for each verdict and proposes options
(with the tradeoffs named) so the owner walk is fast.`

## Why we need the walk

The RUN-AP-01 charter's governing principle applies verbatim to XMTP:
"the ambassador respects the customs of the protocol federated with. It
is a delivery-plane role in the A.7 sense: it holds no ordering
authority, no membership authority, and — pinned this run — no
governance conductivity." The generalization is the reason to run
RUN-FS-01 at all — a second instance validates the abstraction.

**What differs** — XMTP is not ActivityPub. It is a message-carrier
protocol built on MLS with an explicit consent model (allowed / denied /
unknown per conversation), inbox_ids that fan across devices, and a
signal-shape (there is no "follow" primitive; the analogue is "opening
a conversation"). The verdicts must be walked in that world.

## The five questions (mirror of AP-V1..V5)

### FS-V1 — register

The AP-V1 answer was: **no** pre-registration of `ap_signed_follow` in
the qualifying-antecedent register. The ambassador's facts sit BELOW
native two-sided facts and cannot participate in R7 quorum antecedents.

**XMTP question.** XMTP's equivalent of a Follow is either:

- (a) a **`ConsentGranted`** event — the persona has moved a peer's
  inbox_id from `unknown` to `allowed` on their consent record;
- (b) an **`InboxOpened`** event — a first message received on a new
  peer inbox_id (with `unknown` consent still, no explicit grant);
- (c) an **`InboxMessage`** event — every received message (spammy at
  Follow-grain but the most-general choice).

**Recommend (a) `ConsentGranted`**: closest semantic parallel to
Follow in that it is an EXPLICIT, subject-authored fact-of-consent, not
just an incidental observation. `InboxOpened` and `InboxMessage` are
observation-shaped; they don't carry the intent that AP-V1's "reception
relationship over an interval" implies. If (a): no pre-registration of
`xmtp_consent_grant` in the qualifying-antecedent register (mirrors
AP-V1 exactly).

**Owner call needed on:** whether the ambassador observes ALL three
event kinds (as separate receipt types) or only the ConsentGranted /
ConsentDenied pair. `InboxMessage` is potentially high-volume and the
ambassador is not a message store; the delivery run's message-carrier
role probably owns that.

### FS-V2 — record composition

AP-V2 answer was: evidence-complete = (full AP JSON, HTTP-signature
headers, actor public key pinned at verify time) + posture-conditional
blinded form (commitment + body hash).

**XMTP shape.** An XMTP message envelope is an MLS-encrypted payload
whose signature verification runs on the group's MLS state, not on
per-message asymmetric keys. Evidence-complete on XMTP looks like:

1. the raw XMTP envelope bytes as received (`Envelope { topic,
   timestamp_ns, message }` in the XMTP wire format);
2. the MLS group id + epoch at receipt time (so a re-verification
   knows which group state to load);
3. the sender's XMTP `installation_key` (the public key that signs
   MLS messages), pinned at verify time;
4. the associated `inbox_id` for the sender, pinned at verify time.

**Blinded form.** The MLS ciphertext is already opaque; the ambassador
can hold the ciphertext + a commitment `H(salt ‖ ciphertext)` and
degrade to `attested-redacted` for the ConsentGranted/Revoked half
(the consent fact itself is the observation the ambassador attests).

**Owner call needed on:** what "the actor's public key" means when
XMTP's identity model is `inbox_id → 1..N installation_keys` (i.e.
the same inbox spans multiple devices, each with its own key). The
ambassador pins **which key signed THIS envelope** at verify time.
That's likely the right call — but XMTP's inbox_id-vs-installation_key
distinction deserves an explicit verdict.

### FS-V3 — undo / delete

AP-V3 answer was: Undo = second receipt; Delete = redact body + keep
skeleton (masked never-was-world equality).

**XMTP shape.** XMTP consent has three states (`unknown / allowed /
denied`), not a Follow/Undo pair. The ambassador's transitions are:

- `unknown → allowed` (ConsentGranted): opens an interval.
- `allowed → denied` (ConsentRevoked): closes the interval, mirror of
  Undo Follow. **Second receipt, nothing deleted.**
- `allowed → unknown` (ConsentReset): distinct from revoke. Rare in
  practice; XMTP's UI treats it as "start over".
- MLS-side removal of a member from a group: content deletion is
  NOT supported by MLS (forward secrecy makes the ciphertext
  undecryptable after group rekey; the RECEIVER has no obligation to
  redact what was already decrypted).

**Owner call needed on:** whether the ambassador respects an in-band
"delete this message" signal (XMTP has no first-class such signal —
the closest is a client-side message-hidden hint). Recommend: NO
Delete custom rider on XMTP; the MLS forward-secrecy pattern already
solves the "make old messages unreadable" concern for the FUTURE, and
there is no protocol handle for the PAST. Stated firmly this becomes
FS-V3.

### FS-V4 — governance

AP-V4 answer was: hard exclusion as a ROLE boundary; structural +
behavioral permanent-red.

**XMTP shape.** Same principle applies verbatim. The xmtp-ambassador
crate must:

- **Structurally:** be un-importable by any R7 / governance crate; a
  distinct `XmtpReceiptId` newtype from `attest_family::ObjectId`;
  no ambassador variant on the closed `AntecedentKind` enum.
- **Behaviorally:** provide `reject_governance_use()` returning `Err`
  unconditionally.

**No owner call needed** — this is the abstraction we're validating.
FS-V4 = AP-V4, mutatis mutandis (structural + behavioral permanent-
red pair).

### FS-V5 — identity upgrade

AP-V5 answer was: fresh-start default; only upgrade path is subject-
initiated dual-proof binding (DID repo signature + AP-side origin
proof).

**XMTP shape.** XMTP's identity model is `inbox_id`, a stable
identifier that spans devices. The AT-proto side is a `did:plc:…` DID.
The binding proves that the same subject controls both. XMTP's
equivalent of "the AP-side origin proof" is:

- an **`AssociationState`** entry — XMTP's own attested link between an
  `inbox_id` and a `signature_public_key` (which could be the DID's
  repo key, if the subject added it explicitly).

**Owner call needed on:** whether the ambassador honors XMTP's own
`AssociationState` (subject-signed via the XMTP identity flow) as the
"XMTP-side origin proof" leg of the dual-proof, or requires a
separate handshake. Recommend: honor `AssociationState` — it IS the
subject-initiated attestation on the XMTP side. The dual-proof
becomes:

- DID repo signature over `{DID, xmtp_inbox_id, xmtp_association_state,
  antecedent = H(old xmtp_receipt)}`;
- `xmtp_association_state` is XMTP-native and verified against the
  XMTP identity update log (fixture-level this run; live leg fetches
  from XMTP's identity service).

**No live rel="me" leg on XMTP** — XMTP doesn't have HTML pages; the
association state IS the identity proof. Simpler than AP-V5's
`rel="me"` in this respect.

## What the walk produces

Five verdicts (FS-V1..V5) landing as SETTLED INPUTS in the eventual
`RUN-FS-01` brief, mirroring RUN-AP-01's §1. The brief then specifies
seven parts (P1..P7) that build on those verdicts red-first:

| Part | Analogue of | XMTP-specific shape |
|---|---|---|
| P1 | AP P1 verify | verify_xmtp_envelope — MLS signature check against a pinned installation_key. Reuses libxmtp mechanics (fixture-level this run). |
| P2 | AP P2 record | canonical dag-cbor envelope with XMTP-specific fields (inbox_id, installation_key, mls_group_id, epoch); blinded form same as AP-V2. |
| P3 | AP P3 fold | interval fold on ConsentGranted/Revoked pairs (or the FS-V1 choice). |
| P4 | AP P4 redact | FS-V3 rider — likely NO redact rider on XMTP (see FS-V3). Test-pinned negative: `apply_delete` doesn't exist or is unreachable. |
| P5 | AP P5 boundary | XmtpReceiptId newtype; structural (no import from R7); behavioral (reject_governance_use always Err). Same shape, different name. |
| P6 | AP P6 binding | subject-signed binding over {DID, xmtp_inbox_id, xmtp_association_state, antecedent}. |
| P7 | AP P7 non-touch | attest-family/src/{fold,types}.rs untouched; beta/drystone-spec/ untouched. |

## Recommended sequence after the walk

1. Owner walk FS-V1..V5 (this doc → 30–60 min of decision review).
2. Draft the RUN-FS-01 brief in the RUN-AP-01 shape with the settled
   verdicts as §1.
3. Execute RUN-FS-01 red-first — mirror RUN-AP-01's commit shape
   (RED landing + per-part GREEN commits + close-out docs).

## Blocker

**XMTP dev-dependency: `libxmtp` (Rust)**. Compiles cleanly? Fits the
env's disk allowance? Verify before authoring P1's tests. If the Rust
crate is heavy, an alternative is to model XMTP wire-shape at the byte
level (like fed-shim does for Mastodon) and skip the actual MLS
verify path — but that changes the run's grade and needs owner sign-
off (adds an FS-OC).

## Non-goals (recorded, out of RUN-FS-01 scope)

- **XMTP DMs / group-chat semantics** — the ambassador is a
  delivery-plane role; the fact family is consent-and-interval, not
  message-content. Same reasoning as AP-OC-9 (inbound non-follow
  activities on the AP side).
- **XMTP → AP interoperability** — a separate run's problem.
- **The XMTP node-selection / gateway-relay choice** — infrastructure,
  not this lane.
