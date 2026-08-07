# Cost ceilings and the prepaid meter — the equity dimension of metered storage

date: 2026-08-07

status: thinking-layer distillation. Extends the **"meter the boundary"** charging mechanism
(`crystallized/principles.md` Tier 3; `thinking/cooperative-social-union-model.md`) with the
dimension the earlier work left implicit: not just *how you charge*, but *how you cap*, and why
the cap is an **equity instrument** rather than a convenience.

source: `seeds/transcripts/raw/ciss-cost-ceilings-and-prepaid-meter-equity-2026-08-07.md`
(cleaned-paste, content-faithful — §4). Volatile external facts below are **model-asserted,
`[UNVERIFIED]`** until checked against primary sources; the *design implications* are the durable
takeaway, the citations are supporting color.

Relationship to CISS (the running implementation): the metered byte-path, the signed
receipt-per-transfer, and the signed keep-set manifest already exist (`CISS/docs/SECURITY-POSTURE.md`
billing invariants; `CISS/src/receipts.rs`, `CISS/src/manifest.rs`). This doc is the *thinking-layer*
articulation of the ceiling/prepaid layer those primitives make possible.

---

## 0. The one-line thesis

**A synchronous meter turns a hard spending cap from an impossibility into a ledger comparison —
and once the cap is possible, the question stops being technical and becomes political: who governs
its terms.** The same prepaid meter is an equity instrument or a poverty trap depending on whether
every lever is a co-signed record / member vote, or a provider's discretion.

## 1. The historical anchor — McCarthy's utility was peer-shaped

John McCarthy's 1961 MIT-centennial "computing as a public utility" line survives through Waldrop's
*The Dream Machine* `[UNVERIFIED — cite Waldrop; wording "computation may someday be organized… computing
utility," not the popular "computing… computer utility"]`. The sentence the short version always drops:
subscribers **selling services to each other** (weather prediction, programming, renting one's compiler).
The 1961 utility was **peer-shaped** — which the hyperscalers never quote. That peer-shape is exactly the
co-op posture: the utility is a substrate its members transact *over*, not a landlord they rent *from*.

## 2. What the incumbent meter actually is (the contrast) `[UNVERIFIED]`

Three structural facts, offered as competitive context (a fuller cloud-metering comparison could seed a
`research/` note — see ROADMAP_TODO E89):

1. **Asymmetry.** Ingress free, storage cheap, **egress is the margin**; requests metered per-thousand
   calls. (Zero-egress challengers: Cloudflare R2, Backblaze B2, Wasabi.)
2. **The meter is one-sided, batch, and unverifiable.** It is the provider's pipeline; you see totals
   after the fact; there is a delay between incurring a charge and being notified, during which cost
   keeps accruing past any threshold.
3. **Hard caps do not exist.** The account is an open credit line; a leaked key or runaway process is
   discovered when the invoice lands. Budgets are **alerts wearing enforcement's costume.**

The regulatory lever landed where the thesis predicts: egress-fee waivers (2024) and the EU Data Act
`[UNVERIFIED — switching charges capped Sep 2025, banned 12 Jan 2027]` force *switching* charges down,
but only on full exit, at provider discretion, leaving operational egress and the "trust the invoice"
gap untouched. **A verifiable meter is on no regulator's roadmap.** It is the co-op's alone.

## 3. Why a hard ceiling is mechanical for CISS and impossible for them

A hard cap requires metering **at request time**. Incumbent billing is batch reconciliation hours behind
reality, so a cap would either overshoot or lie. CISS meters **synchronously** — a signed receipt per
byte-crossing at the boundary (`CISS/src/receipts.rs`) — so a ceiling is just a **ledger comparison
before serving**:

- Express it in the existing **dial pattern**: a co-signed setting (declared, signed, priced) —
  *"spend stops at X this period."*
- Behavior past X is **throttle or defer, never bill**. The honest failure mode is *"service paused at
  your limit,"* not *"surprise invoice."*
- **Storage rent cannot spike by definition** — it is `manifest × tariff`, and only the owner's signature
  changes the manifest (`CISS/src/manifest.rs`, invariant I5). So the ceiling really governs **postage**,
  which is precisely the volatile axis.

**"The bill cannot surprise you" belongs next to the slogan** — and it is a capability the hyperscalers
structurally cannot match without rebuilding their billing planes.

### 3a. The client-side cost twin (two-sided enforcement)

Because postage/rent are **boundary-observable units the customer computes from their own signed
manifest**, the *client* can price any operation **pre-flight** from the same tariff tables. Ceilings then
hold on **both sides of the wire**: the client refuses/defer before sending; the server refuses at the
boundary. This is a direct build target for the **CISS file-sync client** (its phase-plan Phase 6 —
"cost twin + ceiling"; ROADMAP_TODO E90). Note the current gaps it must design around: CISS exposes **no
pre-flight estimate endpoint** (the client computes `du`-sizes × tariff), and wire receipts are
**Unilateral only** (`Bilateral` co-signing → `501`), so a genuinely *co-signed* ceiling wants the
bilateral-receipt seam closed first (E82 lane).

## 4. The inversion — three reversals stacked

Prepaid does not just cap cost; it **inverts the incumbent power geometry**:

| Axis | Incumbent | Prepaid / co-op |
|---|---|---|
| **Who carries risk** | Customer holds unbounded liability (leaked key, runaway process); provider holds none | Bounded on **both** sides |
| **What the seller monetizes** | **Variance** — bill shock, overage, egress surprise are revenue lines | **Predictability** — selling the *absence* of surprise |
| **What mercy is** | **Discretionary** — you must ask, they may grant (a power instrument) | **On-book** — a co-signed [grace-ledger](cooperative-social-union-model.md) entry with a reason code, a rule not a favor |

The mercy row is already built in the corpus: the **grace ledger** ("mercy is on-book, not off-book")
in `cooperative-social-union-model.md`. This doc supplies the *why it matters* argument.

## 5. The prepaid meter's dual history → a design checklist

Honesty requires the meter's full record: the UK 2022–23 prepayment-meter scandal `[UNVERIFIED — ~3.2M
disconnections in 2022; standing charges accruing while disconnected; prepay unit-rate premium until
Ofgem equalized caps Jul 2023; British Gas £20M fine; failings known since 2018]` is the **punitive**
version of the same mechanism. The lesson is not that prepayment is bad — it is that **prepayment is
equitable only under specific conditions**, and when the operator holds every lever the same mechanism
becomes a poverty trap. Each documented abuse maps to a CISS ledger rule:

1. **Tariff parity as bylaw.** Prepaid and postpaid pay identical rates. The prepay premium was pure
   extraction from those least able to refuse it.
2. **Meter mode changes only with the customer's signature.** Remote credit→prepay switching was the
   ugliest mechanism; in CISS the ceiling value and payment mode are **co-signed dial settings** — the
   provider cannot flip your meter.
3. **A ceiling must never mint debt.** Hitting the cap **throttles**; it does not accrue standing charges
   into a hole you must climb out of before service resumes. (The grace machinery's throttle-not-cutoff
   posture, now with its historical argument.)
4. **Exit is unconditionally cap-exempt.** The deepest possible storage-utility abuse is the analog of
   *"pay your debt before reconnection"* — data held hostage against a balance. The foreclosing rule:
   **reading out your own manifest and blobs to leave must work at zero balance, always.** *They can stop
   selling you service; they can never keep your furniture.* → candidate **new CISS invariant** (self-egress
   is not a metered/gated surface; see §6).
5. **Throttle events are ledgered and counted, not euphemized.** The UK fight was partly over
   "self-disconnection" laundering harm into a lifestyle choice. A co-op surfaces **cap-hit counts to
   governance as a first-class metric** — a tariff that throttles many members is a tariff the members
   should be voting on.

## 6. Where this lands against CISS today (bug vs design-gap)

Per `CISS/CLAUDE.md`'s classification discipline, most of §5 is **design intent not yet an invariant** —
work to add to the posture doc, not code that violates it:

- **Exit-exempt self-egress (rule 4)** — today reads are metered (download receipts) and gated. Making
  *self-directed egress-to-leave* explicitly cap-exempt and un-gated for the owner is a **new invariant**
  (write it into `SECURITY-POSTURE.md`, likely a small ADR). This is the highest-value structural rule.
- **Co-signed ceiling + mode (rules 2, and 3's throttle)** — depends on the **bilateral receipt** seam
  (currently `501`) and a dial-setting record with I5-style monotonic anti-rollback (the manifest already
  demonstrates the pattern).
- **Tariff parity + throttle-count-as-governance-metric (rules 1, 5)** — co-op **bylaw / governance**
  surface, not server code; belongs in the cooperative model doc + the governance-telemetry lane.
- **Client cost twin (§3a)** — client-side, buildable now against `du` + a published tariff; the CISS
  sync-client's Phase 6.

## 7. The general principle

**Same meter, opposite power geometry.** The scandal happened because the operator held the tariff, the
warrant, the remote switch, and the standing charge, while the customer held nothing. The co-op inversion
is that **every one of those levers is a document requiring the member's signature or the membership's
vote.** That — not the meter — is the whole difference between equity and extraction across a century of
utility history.

---

### Cross-references

- `crystallized/principles.md` — "meter the boundary" (Tier 3) + the new ceiling/exit/governance
  principles added alongside it.
- `thinking/cooperative-social-union-model.md` — the grace ledger, sealed/tombstone tiers, two-plane split.
- `CISS/docs/SECURITY-POSTURE.md` — billing invariants (the enforcement home for §6).
- ROADMAP_TODO **E89** (this design, open items) · **E90** (CISS file-sync client + cost-twin) · **E82**
  (cooperative metered-storage lane / bilateral-receipt seam) · **D5** (sustainability-as-mechanism) · **E25**.
- COHESION §67.
