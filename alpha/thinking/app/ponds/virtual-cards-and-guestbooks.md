# Virtual cards and guestbooks: the buildable design

author: design dialogue (2026-07-21)

scope: how to build the group-greeting-card / guestbook use case (ROADMAP_TODO E43) on Croft's
already-proven mechanics, with a privacy ladder, the no-login constraint, and the one genuinely new
primitive named. The incumbent (GroupGreeting and similar) is weak on UI and functionally basic;
the mechanism is a near-free fit for Croft, so the value is in getting the trust surface right, not
in novel cryptography.

date: 2026-07-21

status: design exploration (alpha). Grades below are honest: "proven" names the experiment and its
environment bound; "new" means not yet built. atproto facts are verified against the primary spec at
edit time (see References), not inferred.

---

## The core realization

A group card is not a new system. It is a **write-restricted scope** (the signers are a `named-set`)
whose **read-grant to the recipient is deferred until a reveal moment**. Everything except the
deferral is already built and proven. So the design work is picking the right privacy tier and
naming the one new primitive, not inventing a protocol.

Guestbooks are the same shape with the recipient dropped and the write policy opened: an append-only,
accreting collection tied to a plot or group. The guestbook is already developed in the corpus three
ways (see References); what the *card* adds is a defined recipient, a scheduled reveal, and
private-by-default.

## The privacy ladder

Encryption is a per-card choice. The honest cost of the private tiers: **encrypt and lose the key,
lose the card.** There is no key escrow, because the point of the private tiers is that no server
ever holds the key. Surface that at create-time ("private means only people with the link can ever
read this, including you").

```
tier                    gate                              vs GroupGreeting        proven?
──────────────────────────────────────────────────────────────────────────────────────────────
1. bearer-link          unguessable scope ref; content    = parity, no login      atproto records +
   (public/plaintext)   is public                         (their whole model)     cross-repo refs
                                                                                   (public-roundtrip)
──────────────────────────────────────────────────────────────────────────────────────────────
2. server-blind         unguessable ref + symmetric key    BETTER: server holds    encrypt→content-
   link-key             K in the URL FRAGMENT; content     only ciphertext, never  address→ref-without-
   (recommended         encrypted client-side; store       reads content            key→fetch→decrypt
    private default)     holds only ciphertext                                       (encrypted-blob-share,
                                                                                     validated on real
                                                                                     iroh-blobs QUIC)
──────────────────────────────────────────────────────────────────────────────────────────────
3. MLS-sealed           full E2EE group; forward secrecy,  STRICTLY BETTER: no      seal/Welcome/re-key
   (high-privacy)       named authorship, revocation       incumbent offers this    Verified at loopback
                                                                                    (croft-group L2a);
                                                                                    browser wasm green-real
                                                                                    (RUN-19)
```

For a casual card, tier 2 is the sweet spot: it beats the incumbent on privacy (they read everything;
Croft does not) without asking anyone to sign in, and its blast radius is gentle (one symmetric key =
read plus produce-content, scoped to this one card; a leaked link exposes this card only, no account,
no identity, no other cards). Reserve tier 3 for cards that genuinely need forward secrecy and
provable authorship.

### The one make-or-break detail: fragment, not query param

The symmetric key rides in the URL **fragment** (`#key`), never a query **param** (`?key=`):

```
correct:  https://card.croft.ing/c/<ref>#<key>    fragment: browsers do NOT send it to the
                                                   server, so the store stays blind
wrong:    https://card.croft.ing/c/<ref>?key=<key> query param: sent in the request line,
                                                   logged by server/proxy/referrer; blindness gone
```

The whole "server only ever holds ciphertext" guarantee lives or dies on this. The client reads
`location.hash`, fetches ciphertext by `<ref>`, and decrypts locally.

## The no-login constraint (the hard part)

The GroupGreeting property people actually value is: **contributors add an entry with no account.**
This collides with atproto at the root: **anonymous append is not PDS-native.** Every atproto write
goes to the author's own repo and is authenticated as that DID (`com.atproto.repo.createRecord` /
`uploadBlob` all require a session). There is no write-into-someone-else's-repo and no anonymous-write
endpoint anywhere in the protocol. No login means no DID means no repo to write to.

So "no login" is only reachable one way: a **mediating service that holds a write credential and
authorizes the anonymous contributor by the bearer link**, then appends on their behalf. That is not
a workaround; it is exactly what GroupGreeting is. The Croft difference is that the service is
**content-blind** (it stores fragment-key ciphertext it cannot read), which is the write-side mirror
of the proven content-blind *offer* (RUN-14 EXP-B, green-real at loopback).

The fork, stated plainly, because "no login" and "self-authored / portable" are mutually exclusive:

```
                        no-login (parity)                 sign in with your PDS (upgrade)
contributor auth        none, bearer link                 their atproto DID
entry authorship        display-name only, unauth         self-authored, verifiable, portable
storage                 mediating service writes to a     contributor's own repo; card = cross-repo
                        collection it is scoped to         assembly (public-roundtrip pattern)
ties to                 E29 (guest = pure access, no       the proven cross-repo social graph
                        governance weight, softens Sybil)
```

Default to no-login (it is the property that makes these things spread); offer sign-in as the upgrade
for people who want their entry to be provably theirs.

## Why the write credential cannot ride in the URL

The tempting shortcut is to put the delegated write token in the URL as a shared bearer token, cutting
out the service. atproto OAuth is designed to prevent exactly this, three ways (verified against the
spec):

1. **Not a bearer token: DPoP sender-constrained.** Every request needs a DPoP proof signed by the
   client's private key. The spec: "Tokens are always bound to a unique session DPoP key. Tokens must
   not be shared or reused across client devices." The token string alone is useless without the key.
2. **Short-lived.** Access tokens should be under 30 minutes, 5 recommended. A card collects entries
   over days; a URL-embedded access token is dead within minutes.
3. **Single-use rotating refresh.** "refresh tokens are generally single-use, with the new refresh
   token replacing that used." One link handed to many signers, each refreshing, invalidates the
   others immediately.

The clean line this draws:

```
belongs in the URL fragment:   the symmetric content key K (static secret, no rotation, no binding)
CANNOT go in the URL:          the PDS write credential (DPoP-bound + short TTL + single-use refresh)
```

The write authority has to live somewhere that can hold a private key and rotate refresh tokens:
either the contributor's own logged-in client (back to the no-login fork) or the mediating service.
That is the irreducible reason the content-blind ingest service is not optional for the no-login case:
it is the DPoP-key-and-refresh custodian, not bureaucracy.

## Delegation: scope, and how it is bounded

The organizer logs in to create (confirmed against the live product: creating needs an account,
adding an entry does not), and OAuth-authorizes the service with a **per-collection scope**, e.g.
`repo:ing.croft.card.entry`. Fine-grained per-collection scopes are in production now, not just
proposed. The service can then write card-entry records into the organizer's repo and nothing else
(not their posts, follows, or profile).

Two honesty points on the scope:

- **Granularity is per-collection (by NSID), not per-card.** A token scoped to the entry collection
  can write any card-entry in the organizer's repo, not just one card. Narrow enough to wall off the
  rest of the repo, but not per-card. Per-card content isolation comes from the **fragment key**, not
  the scope: without K, neither the service nor a leaked-token holder can produce a *readable* entry,
  only junk ciphertext that decrypts to nothing.
- **Delete-scope is a `[confirm]`.** Whether collection write can be create-only or necessarily
  includes delete affects whether a bad service could remove entries. Confirm against current atproto.

How the grant is bounded is the subtle part, and the answer is **revocation, not a clock**:

```
client type          overall session          refresh token         access token
confidential (shim)  MAY BE UNLIMITED          <= 180 days each      <= 15-30 min
public (browser)     <= 2 weeks                <= 2 weeks            <= 15-30 min
```

A confidential shim can keep the grant alive indefinitely by refreshing; there is no protocol clock
that ends it. It is bounded by **explicit revocation** (the account can view and delete active OAuth
sessions) and by refresh lapse (180 days of not refreshing). So the "ephemeral shim" property below
is **a discipline the shim must enforce, not a guarantee atproto hands you.**

## State decomposition: PDS is truth, the shim is a time-boxed blind writer

```
PDS (organizer's repo)  ALL durable state, user-owned, portable/exportable
  - the card anchor record (policy, reveal-time)
  - every entry record (ciphertext under K)
  -> if the shim vanishes, the card survives intact in the PDS

shim (ingest service)   ephemeral, time-boxed, NO durable content
  - the collection-scoped DPoP write token + refresh (the thing that cannot live in a URL)
  - the contribution window ("accept appends until T; reveal at T")
  - transient abuse/rate state
  -> on delivery it stops refreshing or self-revokes and holds nothing
```

Two properties fall out. First, the residue at the intermediary is *just the window plus the
credential to act during it*; everything with lasting value is in the PDS, which inverts
GroupGreeting (there the service is the permanent custodian of everything). Second, because entries
are encrypted under K and the shim never has K, **the shim is a blind writer**: it cannot read what
it appends and cannot forge a readable entry, only spam junk-ciphertext or withhold. Producing a
believable message requires the link, not the write token. Even the write-proxy is low-trust.

To make the time-box real (matching "residue = just the window"), enforce it, since the protocol will
not: **per-card confidential session + auto-revoke on delivery.** The grant dies when the window
closes, and per-card sessions keep the blast radius per-card rather than per-collection.

## The reveal: the one genuinely new primitive

Reveal is a **deferred read-grant**: the recipient must stay blind until moment T. Three ways to do
it, cheapest first:

1. **Withhold the link** from the recipient until T (out-of-band scheduling). Trivial; the shim or a
   notification job delivers the link at T.
2. **Withhold the offer**: the store gates *delivery* of the ciphertext to the recipient until T
   without ever reading it (content-blind offer-gating, RUN-14 EXP-B). Structural, so an early ref
   leak still does not open it.
3. **Withhold the key** until T (release K on a schedule).

The RUN-18 reception-completeness limit ("a withheld tail is undetectable until something newer
arrives") is a *feature* here, not a bug: blind-until-reveal is exactly what a surprise card wants.
The scheduled/withheld read-grant is the net-new piece; it is small (a durable timer plus a grant or
key release) and composes with the proven offer-gating. See the adjacent `fair-reveal-primitive-spec.md`
for the commit-reveal shape used elsewhere; the card reveal is time-gated rather than commitment-gated,
so it is simpler.

## What is proven vs what is new

- **Proven, reusable as-is:** encrypt-then-content-address with the reference carrying hash and nonce
  but no key, fetch and decrypt (encrypted-blob-share, real iroh-blobs QUIC); content-blind offer
  against a verified caller (RUN-14 EXP-B, green-real loopback); cross-repo record assembly by
  strong-ref (public-roundtrip); MLS seal/Welcome/re-key for tier 3 (croft-group L2a Verified at
  loopback; browser wasm green-real, RUN-19).
- **New, small (now spiked green):** the content-blind *ingest* (write-side mirror of the proven
  offer) and the scheduled read-grant (the reveal) are proven in a hermetic Rust spike,
  `../../experiments/card-ingest/` (10 tests green; content-blindness proven as a dependency fact by
  `cargo tree`, the service's runtime graph carries no cipher). Still design, not code: the card UX
  and the per-card OAuth session lifecycle (create, delegate scope, ingest, reveal, auto-revoke).
- **Live storage leg now validated:** the `WriteTarget` storage path ran against a real bsky PDS
  (`../../experiments/card-ingest/live/pds-writetarget-probe.py`): an encrypted contribution
  create/read/delete round-trips as opaque bytes, the PDS holds only ciphertext, and the custom NSID
  needs no pre-registration. **Still modeled / spec-verified:** the OAuth + DPoP per-collection
  scoped-delegation write (the live leg used the legacy app-password Bearer flow acting as the account,
  not a mediating service holding a delegated `repo:<NSID>` scope) and the anonymous-contributor
  mediation.
- **Capability register (reasoning, filed):** `../../experiments/card-ingest/CAPABILITIES.md` probes
  what the model learns (metadata residue), what the bearer authorizes (append-only, content-addressed),
  what the reveal trusts (the clock source), authorship as an opt-in content-blind-verifiable signature,
  and revocation as all-or-nothing (escalating to the sealed tier). Five hermetic probes plus the live
  leg.
- **Not a Croft build:** the atproto storage substrate itself (PDS, OAuth, scopes) is upstream and
  used as-is.

## Open calls and residuals (honest)

- **[decision]** default private arm: tier-2 link-key vs tier-3 sealed. Recommend tier-2 for cards.
- **[decision]** public-write abuse model for open-link signing, shared with the Croft.ing
  footprints/guestbook widget (E33 open call a): what the client filters, rate limits, allow/deny.
- **[explore]** the scheduled/withheld read-grant primitive (not built).
- **[confirm]** whether atproto collection write can be create-only (no delete) for the delegated
  scope.
- **liveness**: during the contribution window the shim is an uptime dependency the pure-public case
  does not have. Bounded (short window), but real.
- **authorship**: none in no-login mode (display-name only); the sign-in upgrade is the only path to
  verifiable authorship.

## Sane build order

1. **Public + link-key card** on the atproto-pad path (arecipe/skylite lineage): reuses the proven
   crypto path, cross-repo refs, and the content-blind offer. This is the MVP and beats the incumbent
   on day one.
2. **Content-blind ingest + per-card session lifecycle** (create/delegate/append/auto-revoke): the
   net-new service piece for the no-login property.
3. **Scheduled read-grant** (the reveal): spike it small against the offer-gating harness.
4. **Sign-in-with-Bluesky** entry mode (self-authored, portable): the authorship upgrade.
5. **MLS-sealed** card variant: later, for high-privacy cards only.

## Beyond cards: a general collaborative primitive

Nothing in the spiked shape is card-specific. What it actually is: **delegated per-collection write +
content-blind ingest + an optional scheduled grant.** That serves any "many contributors, one owner's
repo, mediated, optionally server-blind" surface, so the card is only the first instance of a reusable
collaborative primitive. Other instances that ride the same shim with no new mechanism:

- shared lists and collaborative notes (open or named-set write into one repo);
- polls and votes (each ballot an encrypted contribution; tally after a scheduled close, which is the
  reveal by another name);
- RSVP rosters and sign-up sheets (the auditable-count wedge from D11 rides this directly);
- a group playlist, a public suggestion box, a condolence/tribute wall (the guestbook generalized).

The two dials that specialize the primitive per use case are the **write policy** (open link vs
named-set vs sign-in) and **whether there is a reveal** (a card and a poll have one; a guestbook does
not). This is worth its own consideration when the card MVP lands: the ingest is infrastructure, not a
single feature. Tracked loosely against E8 (ponds/pads catalog) and E43.

## References

- ROADMAP_TODO **E43** (the backlog item this note develops); **E8** (ponds/pads catalog), **E39**
  (aggregator pond), **E29** (membership-vs-access / the public door), **E33** (Croft.ing
  guestbook/footprints + abuse-model open call).
- Tier model: `../../../experiments/appview-infra/GROUPS.md` (two-axis membership/write policy);
  write-restricted scopes + the degeneration principle: `../../../experiments/appview-infra/PUBLICATIONS.md`.
- Proven pieces: `../../../experiments/encrypted-blob-share/` (encrypt→content-address→decrypt on real
  iroh-blobs); `../../../experiments/public-roundtrip/` (cross-repo strong-refs); RUN-14 EXP-B
  (content-blind offer against a verified caller); `../../../experiments/croft-group/` L2a (MLS seal,
  Verified loopback); RUN-19 (`../../../experiments/wasm-seal/`, browser seal green-real).
- Guestbook, already developed: `../../../../beta/impl/mls/side-histories-and-threading.md` (tier-2
  side-history guestbook); `../../../../beta/croft/presence-ritual-and-composed-ponds.md` (the Presence
  & Ritual guestbook as connective tissue); `../../../../beta/croft/croft-ing-the-website-and-the-plot.md`
  (guestbook = replies on an Anchor Post).
- Adjacent primitive: `fair-reveal-primitive-spec.md` (commit-reveal; the card reveal is the simpler
  time-gated cousin).
- atproto facts, verified against the primary at edit time (2026-07): OAuth is DPoP sender-constrained,
  access tokens <= 15-30 min, refresh single-use rotating, confidential sessions may be unlimited /
  180-day refresh, public sessions <= 2 weeks (https://atproto.com/specs/oauth); fine-grained
  per-collection scopes `repo:<NSID>` are in production (https://atproto.com/guides/permission-sets).
