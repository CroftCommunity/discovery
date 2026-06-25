# 06 — Safety without surveillance

date: 2026-06-24

status: synthesis (graduated to its own theme). Mixed maturity: several mechanisms are green-real or
DECIDED, others are spec or design — flagged per point.

verification: design-synthesis with proof statuses carried from `04`'s ledger where they exist
(green-real / green-model / DECIDED / spec). The two mechanisms shared with `04` — the freshness signal
and revocation authority — are stated there as crypto facts and reasoned here as adversarial/social
mechanisms.

---

## Theme narrative (overview)

Croft is blind by construction: the always-on superpeer (the **meer**) forwards and reconciles ciphertext
plus unavoidable routing metadata, and holds no payload key. That single property is the whole
confidentiality thesis — and it means Croft *cannot* content-moderate at the transport layer the way a
gatekeeper demands. The naive reaction is to add a "central hand on the wheel" that reads messages, scans
the camera buffer, and reports upstream. That is precisely the extractive choke point Croft exists to
remove, so it is refused outright. The question this theme answers is: if you cannot inspect, how do you
stay safe?

The answer is to attack the *distribution topology*, which the system can see (membership count, fan-out,
namespace, rate — the AR-4 metadata bound), rather than the *content*, which it cannot and will not. The
cautionary case is Rave — a blind, publicly-discoverable, unbounded-broadcast co-viewing app pulled from
the App Store. Its trap was Telegram's surface with Signal's blindness: mass public broadcast you cannot
moderate. Croft's discipline is to stay Signal-shaped — no public discovery, scale caps on the broadcast
tier, accountability through membership and standing, report-plus-revoke — because that is the *only*
abuse-resistance available to a blind system, not a preference.

Where a community genuinely wants content moderation, the theme offers a consented exception: the **geer**,
an opt-in, disclosed, scoped, revocable content-gating role that *labels, not enforces* — the protocol
expression of an elected club officer, never a state wiretap. The system default stays firmly blind,
because the geer's existence weakens the "we cannot comply" shield and so must never become default or
mandatory.

Two cross-cutting mechanisms make the social safety honest. First, **freshness gates authority**: a peer
never displays "current" when it cannot prove it, and an admin act requires a strictly-current, corroborated
view — because acting on stale membership is the dangerous, potentially-irrecoverable case. Second, the
social graph is a graph *you hold, not one held about you*: visibility is structural, not a runtime gate
(the Twitter Circles lesson), and membership-as-stake is decoupled from access-as-entry, so a stakeholder
can hold a public door open without diluting governance or creating a Sybil-capture surface.

These ideas share invariants with `04` (the cryptographic layer): the freshness signal and revocation
authority are stated there as crypto facts; here they are reasoned as social/adversarial mechanisms, with
"freshness gates authority" as the bridge sentence. This theme owns the adversarial and social problem;
`04` owns the invariants, `08` the product surface, `07` the cooperative.

## Charter — what this theme covers

**In scope.**

- Blind-by-default as an abuse-resistance posture (the rave-trap argument; Signal vs Telegram poles).
- Scale + peer + membership-metadata levers as the *only* blind-compatible enforcement surface.
- The geer: consented content-visible moderation as a governance dial; label-not-enforce; the
  compellability tension.
- The failed-op response dial (LOUD/SILENT/BLACKHOLE/COMBINED) and the immune-signal pattern.
- The *social reasoning* for why freshness gates authority and why revocation is threshold-based.
- Membership ≠ access (the public door; guest carries no governance weight; Sybil softening).
- Social-layer visibility regimes (S1–S5) and group-privacy-lanes routing as the structural-not-runtime move.
- Anti-entrenchment of delegated roles (meer/geer) as a governance/social property.

**Out of scope (and where it lives).**

- The wire format of the tip beacon, the `meets_threshold_by_lineage` validator, the `RemovedThenIncluded`
  hard-stop, MLS epoch mechanics → `04` (shared docs: freshness-signal + revocation-authority).
- The UI of freshness labels, the invite/onboarding UX, the ten-second-door engineering, label-subscription
  UX → `08`.
- Co-op governance, who runs meers, multi-class membership economics, legal/jurisdiction posture → `07`.

**Boundary calls.**

- `freshness-signal` and `revocation-authority` are SHARED with `04`: `04` carries MEMBERSHIP-FRESH and the
  ADMIN FLOOR as decided crypto/protocol facts; this theme carries the *adversarial rationale*. The geer
  lives here; its cryptographically-enforced scope (rung-1) is an open item `04` would own if built. The
  CSAM posture and jurisdiction are named here as stated limitations, but the *legal review* is the user's /
  `07`'s, not engineering's.

## 1. Blind-by-default forecloses inspection-based moderation

> "an always-on node that shuffles encrypted state without reading it."

> "Croft must take Signal's path, because it is blind by design — Telegram's readable-moderation path is
> not even available to us."

*Verification:* design; the Tier-0 blind broker is green-real (E3.4), and the running blind meer binary is
green-real (meer P0+P1). *Grounds:* the meer holds no payload key, so content moderation at the transport layer is
structurally unavailable. The question is not whether to inspect, but how to stay safe without it.

## 2. The rave-trap: answered by scale and membership metadata, never content

> "the worst combination is **Telegram's surface with Signal's blindness** — public/mass broadcast + large
> media distribution, but blind so you *cannot* moderate it. That is exactly what Rave was"

> "the dangerous variable was never 'public vs private' — it is the *combination* of (a) anonymous access,
> (b) stranger-discovery, and (c) unbounded broadcast scale."

*Verification:* design — the physics caps (mesh dies ~5 peers, an SFU holds dozens) are architectural;
the relay lab proves the blind metadata-admission lever (E11, characterized) and the blind meer (green-real),
not the capacity numbers. *Grounds:* remove public discovery and cap broadcast fan-out / rate / audience, and you remove
the abuse surface without inspecting a byte. Mass distribution *physically needs* a meer, so the
distribution vector is exactly the controllable, blind, metadata-only choke. The category to hold onto:
"a club is members who *joined* (a recorded, attributable, standing-bearing act), not anonymous consumers of
a feed."

## 3. The geer: consented content-gating that labels, not enforces

> "no peer sees content because it *can*; it sees content because the group *granted the role*."

> "The geer *labels* (sees content by elected right, emits a judgment); the **group's governance, or each
> member's client policy, decides the action**... The geer cannot unilaterally enforce — it advises."

*Verification:* design (the atproto labeler primitive it mirrors is real/external). *Grounds:* four
properties are all required — voted in by group governance (default stays blind), disclosed as a named
visible role, scoped to the least-invasive rung, accountable and revocable. It is separation of powers: the
geer labels, governance or each client decides the action. The compellability tension is the load-bearing
caution:

> "Signal's real power is that it *cannot* comply — there is no content to hand over, so it cannot be
> *ordered* to. The moment a content-visible geer *can* exist in the system, the 'we cannot' shield is
> gone... Capability invites compulsion... *you can, therefore you must.*"

And a scope-tightening conclusion that keeps the geer rare:

> "banning/blocking does NOT require content visibility."

*Grounds:* a blind meer + member governance already bans/blocks on reports + attestation + behavioral
metadata; content-visibility adds *only* proactive detection. Reserve the geer's eyes for that, never for
what governance does blind.

## 4. Failed-op response is a dial

Detection is deterministic (the faithful path's identical-verdict property, green-real in `04`); *response*
is governance. The dial: **LOUD** (a signed rejection-event into "immune memory" — reputation / rate-limit /
removal-vote input); **SILENT** (local reject, denies the attacker feedback); **BLACKHOLE** (engage-then-
void tarpit); **COMBINED** (silent-to-attacker, loud-to-members). The immune signal must require **k
corroborating observers** (the same threshold dial as revocation) or it becomes a framing / injection
vector. *(Seam: failed-op ↔ revocation threshold ↔ freshness — don't act on stale immune memory.)*

## 5. Freshness gates authority (shared with `04`)

> "**A peer never displays 'current' when it cannot prove freshness.**" — and — "There is no path where
> silence is rendered as currency."

> "the danger is not being behind — being behind is recoverable. The danger is *believing you are current
> when you are not*"

*Verification:* **DECIDED**; E2.16 is green-model (specified and modeled). *Grounds:* a periodic signed
content-free tip beacon + a local time-since-heard clock yields BEHIND (comparative) and UNCERTAIN/STALE-RISK
(the load-bearing partition case). MEMBERSHIP-FRESH: originating or co-signing an admin op requires a strict
CURRENT + corroborated view; content carries no such bar. The honest boundary, not to over-claim:

> "MEMBERSHIP-FRESH *narrows* the stale-action window; it does **not** close the **fresh-but-wrong
> partition**... Freshness proves liveness, not global currency."

The fresh-but-wrong partition is left to `04`'s reconcile hard-stop by design. The bridge sentence to the
next point:

> "**Freshness gates authority** — don't act on a removal authorized against a stale epoch."

## 6. Revocation authority: a threshold dial and the admin floor (shared with `04`)

> "Roles are delegated authority, never irrevocable — including the creator's." — and — "The creator holds
> **no** structural superuser right."

*Verification:* the co-signed self-certifying op is **green-real** over iroh-gossip (C-faithful-revoke,
2026-06-17), validated by `meets_threshold_by_lineage`; the floor / role-ladder is **DECIDED**, suite group
I specified-not-run. *Grounds:* the ADMIN FLOOR = threshold-satisfiability (`k ≤ eligible lineages at
set-time`) + the `n ≥ k` floor + a never-irrevocable role ladder (routine revoke → unanimity-of-non-holders
backstop → fork). The floor is **anti-brick only**:

> "The floor is ANTI-BRICK ONLY — capture is not structurally blocked... The recourse for an out-voted
> minority is the §7 **re-formation fork** (vote with your feet), never a structural veto. Capture ≠ brick"

## 7. Meer anti-entrenchment: materially-reversible delegation

> "equal peers delegate revocable authority, and the substrate must make that revocation real — not just by
> allowing exit, but by letting the group *stand up and elect a different holder* — so a delegate never
> hardens into an owner by circumstance."

*Verification:* the re-formation / state-portability result is green-real-multimachine (A1 re-formation
(trap door) in the proof ledger; meer export→import to a replacement, E9), applied to roles by design. *Grounds:* de-jure revocability is necessary
but not sufficient — a resourced always-on state-holder can entrench by *circumstance*. The guards: no state
hostage (encrypted, exportable state → re-host cheaply); blindness caps the weight; stand-up-a-replacement-
and-elect-it (the decisive everyday guard, distinct from the adversarial fork backstop); scoped non-creeping
rights; metadata transparency. The same ladder applies to the geer.

## 8. Membership ≠ access: the public door a stakeholder holds open

> "the anonymous visitor is a **guest in a room, not a member of the co-op** — access without a stake."

> "Frictionless onboarding and Sybil resistance are in direct tension *only when the door also confers a
> stake*; decoupling them lets the public door be cheap because it grants nothing to capture."

*Verification:* design (from the user's own dialogue). *Grounds:* two orthogonal axes Discord conflates —
**membership** (stake + vote, the infra/governance layer, "ponds") vs **access** (who enters/posts, the room
layer, "pads"). A stakeholder can open a public/anonymous door into a pad while the pond stays member-
governed. This keeps the ten-second door without diluting ownership, and softens Sybil: guests carry no
governance weight, so guest-room spam cannot capture the co-op. The access-layer expression of D9 (member ≠
governance-constituent). A guest pad is also the natural place for the geer to moderate,
without touching the member pond's blind-by-default guarantee.

## 9. The social graph is structural, not runtime (S1–S5)

> "**you hold the graph of your life, you see it, you shape it, no one monetizes it, and it is invisible to
> anyone you have not chosen to show it to.**"

The graph layer (structural fact) is separated from the label layer (opt-in social) — most platforms fuse
them, and that fusion is the invasiveness. S1 freeze-by-default; S2 scoped-visibility-not-opaque-structure
(topology deanonymizes — no "visible structure, hidden identities"); S3 quiet/asymmetric membership
(reachable without being mapped — **unsolved**); S4 multi-identity as first-class; and the load-bearing one:

> "**S5 — Private must be structural, not a runtime gate (the Twitter Circles lesson).** A visibility
> 'clause' attached to content that physically lives on a public/broadcast plane is not privacy; it is a
> promise the platform can break the moment its ranking or indexing logic changes." — and — "**never overlay
> a semi-private gate on a public plane — make private data structurally private**"

*Verification:* design, backed by the Twitter Circles billion-user precedent. This is the
social-graph statement of the lineage-group rule V3 — content is born into a visibility regime and cannot
silently change it.

## 10. Group-privacy-lanes: routing prevents the dangerous hybrid

> "A `public: true` flag on an MLS group is a contradiction the crypto cannot honor."

> "the **routing prevents the trap**: a public-follow feed is atproto (public, moderatable); a member club
> is open-join MLS (self-governed); you never get the dangerous middle — *an MLS group acting as a public
> broadcast feed*... That hybrid is simply unrepresentable if 'public' always routes to Lane 3."

*Verification:* design (MLS/atproto mechanics external-real). *Grounds:* a `visibility` flag is the wrong
model; instead route at compose time — Lane 1 closed MLS, Lane 2 open-join MLS ("the club", self-governed,
no global stranger-directory), Lane 3 truly-public atproto records (public by construction, moderatable as
public content). The geer fits Lane 2, never Lane 1.

## 11. The fork reality: bound the blast radius, don't pretend to prevent

Croft is open; a fork *can* strip the limits. The mitigation is not prevention: the mainline ships no abuse
affordance; co-op meers won't subsidize abuse scale (a piracy-CDN fork must run its own meers, its own
liability); and physics caps the pure-P2P case. The topology that makes Croft private — no central forwarder
— is the same topology that stops a no-infra fork from becoming a CDN.

---

## What this theme establishes (and does not)

**Establishes:** a content-blind system has a real, structural safety story — deny the abuse *surface*
(no public discovery, capped broadcast scale), pin accountability to consented attributable revocable roles,
gate authority by freshness, and decouple membership from access. Several pieces are green-real (blind meer,
co-signed revoke over the wire, re-formation) or DECIDED (freshness, admin floor).

**Does not establish:** that freshness "solves" partition (it narrows the stale window; the fresh-but-wrong
partition is left to `04`'s hard-stop by design); that the geer "solves" compellability (content-visible
rungs remain compellable); or that the admin floor prevents capture (it prevents *brick*; capture's recourse
is the fork). S3 quiet membership is unsolved; the E11 ten-second-door engineering is open; CSAM posture and
jurisdiction need legal review, not engineering optimism.
