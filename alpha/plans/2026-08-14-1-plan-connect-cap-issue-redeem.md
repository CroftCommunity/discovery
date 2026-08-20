# Plan — connect cap issue/redeem: one grant, two matchers

- **Repo:** `CroftCommunity/connect` (public, disjoint write-set — the Phase 10 parallel track)
- **Status:** **EXECUTED 2026-08-16** — M0–M5 landed on connect `main`, released as
  **v0.2.0** (contract v2, `app-debug.apk` served from GitHub Releases). See "Execution status" below.
- **Relates to:** `discovery/alpha/plans/2026-08-07-1-plan-croft-relay-tiered-admission.md`
  → "Phase 10 green-lit as a parallel track; listRecords verified — 2026-08-11".
  This plan is Phase 10's detailed design.
- **Prerequisite (settled):** `com.atproto.repo.listRecords` verified against the atproto lexicon
  source (no-auth query, `repo`+`collection`, `limit`/`cursor`/`reverse`, returns `records`).

## Execution status (2026-08-16)

All milestones landed on connect `main` and released as **v0.2.0**. TDD RED→GREEN
throughout; the security-shaped paths mutation-audited (`npm run mutate`, stryker).

| Milestone | Status | Notes |
|---|---|---|
| M0 contract v2 | ✅ | grants/matchers/policies + per-device; ownership note + `CLAUDE.md` added |
| M1 ticket path | ✅ | web redeem (secret-in-fragment, verify, expires-only); android device+grant capture (9/9 green locally) |
| M2 listEndpoints | ✅ | per-device enumeration via `listRecords` |
| M3+M4 matcher engine | ✅ | `areMutuals` (getRelationships), `evaluateMatcher` — ticket/mutuals/registeredCallers, fails closed |
| M5 call-time evaluator | ✅ | `evaluateRules`/`evaluateGrant` — §7 reference impl (relay mirrors in Phase 11) |

- **71 web tests**; resolver.js mutation score 93.3% (remaining survivors equivalent/boundary, triaged).
- **Decisions settled** (see "Open decisions" below): tiered confidentiality, one-use-as-revocation-rule,
  separate policy record, open tagged unions.
- **Not in scope (Phase 11):** identity-proof *acquisition* (OAuth to obtain `provenDid`), the relay's
  call-time wiring of `evaluateGrant`, and the real client — which lives in `croft/android`, not the
  `connect/android` stopgap.
- **Release:** `Croft Connect v0.2.0`, `app-debug.apk` (debug-signed) served from GitHub Releases.
  Versioning/process: `connect/docs/RELEASING.md`.

## Problem Statement

Croft splits a call into two independent "yes" votes: **Membership** ("will the relay carry the
traffic?" — the CISS accounting backbone, now built and merged) and **Cap** ("may this caller reach
me?" — the callee's own decision). The Cap side does not exist yet. `connect/docs/contract.md`
today defines only the endpoint record (a single `rkey=self` per repo), the `croftcall://` deep
link, and the resolution pipeline. There is no way to express, publish, hand out, check, or revoke
"you may call me."

That credential has to work under two constraints that would be trivial with a private server and
are the whole difficulty without one:

1. **It is world-readable.** Grants live as public records in the caller-ee's own atproto repo.
   The record must not announce *who* was invited.
2. **There is no backend to check it.** The redeem/exchange page is static `web/`; it can *read*
   public records (`getRecord`, `listRecords`) but cannot run server logic or hold private state.

## Approach

**One mechanism, two matchers.** A *grant* is always "a public record that resolves to yes/no for a
caller at call time." The only thing that varies is **how a caller qualifies**:

```
                 A GRANT  (public record in the callee's atproto repo)
                 ─────────────────────────────────────────────────────
                 matcher: how does a caller qualify?
                        │
            ┌───────────┴────────────┐
            ▼                         ▼
       TICKET matcher            RULE matcher
       qualify by POSSESSION     qualify by IDENTITY
       (holds a secret)          (is something)
       ─ handed-out invite       ─ preset: mutuals (no list)
       ─ expirable / one-use     ─ registered callers (DID list)
       ─ record stores           ─ v1: list stored plainly;
         hash(secret) only         confidentiality is a later dial
       revoke: delete record     revoke: edit/delete the rule
       no caller identity        caller must prove DID X
         needed                    (sign a challenge)

Matchers combine by OR: a caller is admitted if *any* grant says yes (a preset
grant and a registered-list grant coexist — mutuals get through, and so does
anyone on the list). "Tiered" = presets for the common case, explicit lists for
the named case, more attribute checks addable later.
```

Both are grants. Both are public records. Both are evaluated the same way at call time: *does the
caller satisfy any grant that says yes?* The two matchers serve two real product needs the owner
named — a standing default ("mutuals can call me") and handed-out tokens ("here's a one-use invite
that expires").

**Three record shapes** (extending the contract's existing endpoint record):

| shape | collection (proposed) | answers | opacity |
|---|---|---|---|
| **endpoint** (per-device) | `ing.croft.iroh.endpoint` | *where* to dial | n/a (your own address) |
| **grant** | `ing.croft.call.grant` | *who* may call | must not name the grantee |
| **policy** | `ing.croft.call.policy` | *under what limits* | n/a (rules, not identities) |

- **endpoint** grows from single `rkey=self` to **rkey-per-device** (several endpoints per person).
  This is what pulls in `listRecords` (enumerate a repo's devices) + the deep-link `?device=` hint.
- **grant** carries a `matcher` (discriminated union: `ticket` | `mutuals` (preset) |
  `registeredCallers` (DID list)), the device(s) it grants, and an optional `policyRef`.
- **policy** carries `expiresAt` / `oneUse` / `maxUses` / rate, referenced by many grants so limits
  tune without reissuing grants.

**Redemption is pure-read.** The invite link carries the grant's rkey (so the page reads it with
`getRecord`, not a scan) and — for a ticket — the secret in the URL **fragment** (`#`), never the
query. The page verifies `sha256(secret) == grant.secretHash`, reads the referenced endpoint(s),
and builds the `croftcall://` deep link. It writes nothing.

**Revocation is composable rules at call time, callee-side.** A static page cannot mark a ticket
spent, and a caller who already redeemed still holds the endpoint + secret. So a grant is a
**conditional token** whose continued validity is decided by composable revocation rules the callee/
relay evaluates at call time — grant-still-exists, `expiresAt`, `maxUses`, manual revoke, and "burn
after first *successful* call." "One-use" is just that last rule; "used" means a call succeeded,
which only the call-time side can observe. Deleting a grant stops *new* redemptions immediately
(readers see it gone via `listRecords`); stopping an *already-issued* one is the call-time re-check.

## Reasoning

- **Why one shape, not two flows.** Ticket and rule differ only in the match predicate; everything
  else (public record, call-time evaluation, revoke-by-delete) is shared. Modeling them as one
  `matcher` union from day one costs nothing in the contract and avoids a reshape when the second
  matcher lands. The owner's framing — "two different use cases for the same mechanism" — is the
  design.
- **Why secret-in-fragment.** The fragment never leaves the browser and never hits a server log,
  which is what keeps the exchange page *honestly* backendless and the bearer secret private. A
  query param would leak it to every proxy and referrer. The record stores only `sha256(secret)`,
  so the public grant reveals nothing usable.
- **Why confidentiality is a dial, not a gate (Decision 1, settled).** `ticket` (hash) and the
  `mutuals` preset (names no one) are already opaque. The `registeredCallers` list is the one shape
  that could leak a guest list, and the owner's call is **tiered**: ship the explicit DID list in
  v1 stored **plainly**, and treat hiding it as a later dial rather than blocking on it. Parked dials
  (named, not designed away): hash the list (resists browsing, not a confirmed guess); hash PDS
  content to define "registered callers" without an explicit list; fold list-membership into a
  broader attribute check. Matchers OR-combine, so a preset grant and a list grant admit
  independently — that is the "tiered" behavior.
- **Why ticket first.** It needs no identity-proof or signature machinery, so it is the shortest
  path to a working end-to-end invite and exercises the entire read-record-verify-build-deep-link
  pipeline. The rule matcher then slots in as "a second matcher type" once identity proof exists.
- **Why call-time for revoke/one-use.** Backendless redemption structurally cannot enforce
  single-use; pretending otherwise would be a silent-failure trap. Locating spent-tracking and
  revocation at the callee's call-time check is the honest home and reuses the Membership seam.

## Milestones

Ticket-first is milestone **M1**; **both matchers are finished by M5.** M0 settles the contract for
*both* up front so no shape is reworked later.

### M0 — Contract: the grant model (both matchers, up front)

`docs/contract.md` v2, the source of truth both halves agree on before either is built.
- Extend §1 endpoint: `rkey`-per-device; enumerate via `listRecords`; `?device=` hint semantics.
- §3 grant record: the `matcher` union (`ticket` | `mutuals` (preset) | `registeredCallers` (DID
  list)), `devices`, `policyRef`; `secretHash` for tickets; a plaintext `dids` list for
  `registeredCallers` (confidentiality is a parked dial per Decision 1).
- §4 policy record: `expiresAt` / `oneUse` / `maxUses` / rate.
- §5 invite link: `https` exchange URL, grant rkey in query, ticket secret in `#fragment`.
- §6 call-time check: grant-still-exists + one-use spent-set, callee-side (interface only; the relay
  wiring is Phase 11 / Membership-side, out of this repo).
- **Deliverable:** contract.md v2 merged. No code yet — the contract is the artifact.

### M1 — Ticket matcher, end to end (the opening buildable milestone)

Prove the whole pipeline with the matcher that needs no identity.
- `web/`: mint an invite (generate secret, store `sha256(secret)` in a grant record, build link with
  secret in fragment); exchange page redeems (getRecord grant, verify secret, build deep link).
- `android/`: `DeepLink.kt` already parses `croftcall://`; extend for any new params.
- Expirable via policy `expiresAt`; **not** one-use yet (deferred to M5's call-time locus).
- Both halves unit-tested against the contract's cases, per the repo's stated convention.

### M2 — Per-device endpoints + `listRecords` enumeration

- Move endpoint from `rkey=self` to per-device rkeys; page lists a repo's endpoints and selects by
  `?device=`. Wire the resolution pipeline (§ informative) to the multi-record world.

### M3 — Identity-proof scaffolding

- Caller proves "I am DID X" (sign a challenge with X's key), backendless-verifiable on the page.
  The machinery the rule matcher requires and the ticket matcher does not. No product behavior
  change yet — it is the substrate for M4.

### M4 — Rule matcher: `mutuals` preset + `registeredCallers` list (finishes the second model)

- `mutuals` (preset): read the two public `app.bsky.graph.follow` records (mine→them, them→mine)
  via `listRecords`/`getRecord`; both present ⇒ qualify. Names no one.
- `registeredCallers` (list): `provenDid ∈ grant.dids` (plaintext list in v1; hashing is a parked
  dial per Decision 1).
- Both gated on the M3 proof. This closes "finish both models." The union stays open for further
  presets/attribute checks later.

### M5 — Composable revocation rules at call time

Not a one-use boolean. A grant is a **conditional token**; whether it still admits a caller is
decided by **composable revocation rules** evaluated when a call arrives, callee-side:
- grant-still-exists (delete = revoke), `expiresAt` (also enforceable earlier at redeem),
  `maxUses`, manual revoke, and "burn after first **successful call**" — the last is why this is
  call-time only: "used" means *a call succeeded*, which only the callee/relay can observe, not the
  static redeem page.
- The rules compose (a token can carry several); the interface takes a caller + grant + observed
  history and returns admit/deny.
- This is the seam into the Membership backbone (relay/CISS side) — the connect repo defines the
  interface; the wiring is Phase 11.

## Open decisions (flag before M0 merges)

1. ~~**`registeredCallers` opacity.**~~ **SETTLED 2026-08-16 — tiered.** Ship the explicit DID list
   in v1 stored **plainly** (don't block on hiding it), plus the `mutuals` preset. Confidentiality
   is a parked dial (hash the list; hash PDS content for "registered"; broader attribute check), to
   revisit later. Matchers OR-combine.
2. ~~**One-use now or deferred.**~~ **SETTLED 2026-08-16 — A, and reframed.** Expiry ships in M1
   (a static page can enforce it honestly). "One-use" is not a primitive: it is a conditional token
   plus a composable **revocation rule** ("burn after first *successful* call"), evaluated call-time
   at M5 — because "used = a call succeeded" is only observable there, never on the static redeem
   page. Revocation rules compose (expiry, maxUses, manual, burn-on-success).
3. ~~**Policy: separate record vs inlined.**~~ **SETTLED 2026-08-16 — A.** Separate `policy` record
   from the start (`policyRef` on the grant). One policy governs many grants; edit conditions/
   revocation rules once. Redeem/call-time reads two records (grant, then policy).
4. ~~**Rule expressiveness ceiling.**~~ **SETTLED 2026-08-16 — A.** `matcher` is an open tagged
   union. Build `mutuals` + `registeredCallers` now; new rule types (`followsMe`, group/attribute
   checks) append cleanly with no reshape. Same for the revocation-rule set (Decision 2) — both are
   extensible unions.

---

**All four settled 2026-08-16.** Through-line: keep the *shapes* open (tagged unions for matchers
and revocation rules, a referenced policy record) so v1 stays small while nothing needs reshaping
later; the only real values call was Decision 1 (guest-list confidentiality), settled as a parked
dial rather than a v1 gate.
