# Readmission, `GroupInfo`, and the admission surface

date: 2026-08-14
status: **planned, not started.** Grounded in S15–S18 (Rung A) and a full read of
`beta/drystone-spec/part-2-certifiable-design.md` §11.6–§11.8, §11.11.
origin: `discovery/alpha/experiments/meer-queue/TEST-LOG.md` (S15, S16, S18), `ROADMAP_TODO.md`
E105, E106, E107

---

> **REFRAMED 2026-08-16 (owner).** *"We've said in Drystone that several things are dials for
> implementation, both in terms of product and user choice. This is feeling like a dial here, where we
> need to define the setups and test and validate them, and note what's enforceable and possible in the
> Drystone spec, and then also decide what Croft does within those protocol dials by default and makes
> available to users, which needs grounding in tests and functional reality."*
>
> **This plan predates that framing and is superseded in shape, not in content.** Its phases remain
> valid work items, but they should be re-read as *"grounding the positions of the readmission dial"*
> rather than *"reworking §11.6–§11.8"*. The dial — four positions, what is enforceable at each, what
> is impossible at every one, and the three that are not yet grounded — is specified in
> `beta/drystone-spec/DOSSIER-exclusion-and-readmission-2026-08-16.md` §3.2, with Croft's separate
> product-layer decision in §4. **Read the dossier first.**

## Problem statement

Four experiments (S15, S16, S17, S18) went looking for gaps in the delivery design and found that
**most of what they surfaced was already specified — in a part of the spec the delivery work had not
been reading.** §11.8 anticipates the admission problem precisely; §11.11 already flags the
resumption-PSK pattern as unproven; §11.6 already names history-convergence nodes as community
infrastructure.

So the real problem is **not** a pile of unknown gaps. It is three specific things:

1. **One normative claim in the spec is false, and it was measured false.** §11.8 (and Appendix E,
   L6) assert that a returning lineage admitted before a ban propagates gains *"zero marginal
   exposure, since a returned lineage briefly re-holds only keys it already had."* **S18 measured the
   opposite:** a re-seated member is a full member **at the current epoch** and read a message sealed
   *after* her return. She is not re-holding old keys; she is receiving new entropy.
2. **One spec mechanism is not constructible on the reference library.** §11.7's key half is the
   resumption PSK. **S16 measured that a resumption PSK cannot be attached to an external commit at
   all on openmls 0.8.1** — not "unsanctioned" (which §11.11 item 6 already says) but *unreachable
   through the public API*.
3. **The delivery design and the scaling spec have not been connected.** `meer-two-target-delivery.md`
   concluded "nothing serves `GroupInfo`" (E105). **That is true of the delivery doc and false of the
   spec**, which has the history-convergence node supplying exactly this. The gap is an unwired seam,
   not a missing capability.

## What was measured, and what it changes

| finding | S | spec position | effect |
|---|---|---|---|
| MLS admits a total stranger on a `GroupInfo` alone | S16 | §11.8 **already says this** (*"a leaf MLS has no reason to reject"*) | **confirms** the spec at Rung A; moves a ***Design*** claim to measured |
| A **removed** member re-seats herself the same way | S18 | implied by §11.8, never stated | confirms, and names the exposure window |
| The re-seated member is **current**, not a stale-lineage ghost | S18 | **§11.8 says the opposite** | **normative correction** |
| She recovers **no** history through MLS | S18 | §11.7 supplies it via the DAG + archival key | confirms the DAG mechanism is **load-bearing, not optional** |
| Refusal holds at two layers (keys, addressing) | S18 | not stated | **new**; supports the §11.8 admission gate |
| A fork is **invisible in the epoch counter** | S18 | §11.11 item 3 knows forks can happen | **sharpens** the failure mode — no cheap local signal |
| Resumption PSK unattachable on openmls 0.8.1 | S16 | §11.11 item 6 flags the pattern as unproven | **sharpens** unproven → **blocked in the reference impl** |
| Admission surface is the **ratchet tree**, not the `GroupInfo` | S18 | §11.11 item 6 notes the HCS conveys the tree | **promotes** a plumbing detail to a governance control point |

## Approach

### Phase 1 — Correct the spec (normative, small, blocking)

**1a. §11.8's exposure claim.** Replace *"zero marginal exposure, since a returned lineage briefly
re-holds only keys it already had"* with the measured statement:

> A returning lineage admitted before a ban propagates re-keys to the **current** epoch and receives
> new entropy. Its exposure is therefore **everything sent between admission and re-exclusion**, not
> zero. The window is bounded by ban-propagation latency and by hot-Group commit liveness — the same
> floor §11.8 already owns — and the design's safety argument rests on that window being short, not
> on the exposure being absent.

The same correction applies to **Appendix E, L6** (line 2885), which restates it.

**This does not break the surrounding argument.** "Dormancy cannot be used to evade a ban" still
holds; "no consensus on membership required" still holds. What changes is that exclusion latency is
now a *confidentiality* parameter, not only a *liveness* one — which strengthens §11.8's own
conclusion that commit liveness is the thing to monitor.

**1b. §11.7's key half.** Add an implementability note: the resumption PSK is ***Verified-RFC*** as a
primitive, but **openmls 0.8.1 exposes no path to attach one to an external commit** — resumption
PSKs resolve from the group's own `ResumptionPskStore`, an external-commit group initialises that
store empty, and its `add` is `pub(crate)`. Record the **governance-issued external PSK** as the
constructible form, and note it collapses both halves into one artifact.

**1c. §11.11 item 6.** Sharpen from "not established as a sanctioned pattern" to "**blocked in the
reference implementation**, with a measured workaround."

### Phase 2 — Wire the history-convergence node into the delivery design

The spec already has the server the delivery doc thought was missing. Amend
`meer-two-target-delivery.md` to name the **history-convergence node** as the party that serves
`GroupInfo` and conveys the ratchet tree, and retire the "nothing serves it" framing in E105.

This also resolves the S15 limbo finding without a new component: an always-on HCS in the returner's
own device pool never falls out of the liveness window, so it can always answer.

**Design deliberately, per the owner's 2026-08-14 framing:** if the same party runs your meer *and*
your history-convergence node there is a natural continuity worth building on — but that is also the
case where one operator holds both roles, and the meer's value is that it holds no keys while an HCS
in your pool holds group secrets. **Record the trust asymmetry explicitly rather than letting it
arrive by convenience.**

### Phase 3 — Make the admission surface a deliberate control

**S19 settled which dial, by eliminating the obvious one.** `export_group_info` offers exactly one
option, `with_ratchet_tree`, and every `GroupInfo` a member can produce carries the `external_pub`
external-join key —`export_group_info_with_additional_extensions` *errors* if a `RatchetTreeExtension`
or `ExternalPubExtension` is supplied directly, so neither can be hand-managed.

> **There is no "safe" `GroupInfo`** — no way to prove current group state without also admitting
> its holder. So the surface cannot be narrowed by exporting a weaker one.

**The ratchet tree is therefore the only dial** (S18, S19). Serve `GroupInfo` **without** the bundled
ratchet tree by default; convey the tree only after the admission gate passes.

Two properties make this cheap:

- The `with_ratchet_tree` export flag is **per call**, independent of the group's
  `use_ratchet_tree_extension` config — so this is enforced at the serving node, not in group config,
  and no group-wide setting can defeat it.
- **Bandwidth agrees with governance.** S8 measured the tree roughly doubling `Welcome` (330 vs 152
  bytes/member) and crossing the 2 MiB cap first at N ≈ 6,350.

### Phase 4 — Readmission as a group-context policy, not a prompt

S18's fork finding forces this. Members must reach the **same** answer without a negotiation round,
so the rule is a **group-context extension** agreed in advance. The check runs pre-merge on the
`ProcessedMessage`, where S16 measured all four inputs available:

```
   external commit arrives
     ├─ sender == NewMemberCommit ?          (is this a self-join)
     ├─ psk_proposals() non-empty & known ?  (governance token — S16)
     ├─ aad → attestation                    (standing claim — S16)
     └─ credential → lineage                 (resolve against ban ceiling, §11.8)
          └─ merge, or drop
```

**A per-member dialog box is a partition generator** — and S18 measured that the resulting fork is
invisible in the epoch counter, so it hides its own damage.

### Phase 5 — A fork signal that is not the epoch number

S18 measured two branches at the **same epoch number** with different secrets. Nothing in the
delivery layer surfaces this. Cheapest candidate: peers compare a derived value they already have —
the group queue name is exactly such a value, and divergence is detectable by comparing it rather
than the epoch. Specify a signal; do not leave "peers silently stop reading each other" as the only
symptom.

## Reasoning

**Why the spec correction is first and blocking.** §11.8's exposure claim is load-bearing for the
"eventual consistency is safe" argument, and it is the one thing here that is *wrong* rather than
*unfinished*. A reader who trusts it will under-scope exclusion latency, and the failure is silent.
Everything else in this plan is design work; this is a correction, and corrections go first.

**Why this plan is smaller than it looks.** Three of the four experiments largely **confirmed** the
spec. The delivery work had been treating §11 as out of scope and re-derived several of its
conclusions independently — which is a useful corroboration but produced a false sense of missing
capability. Reading the spec removed more work than the experiments added.

**Why the ratchet tree rather than the `GroupInfo` is the control point.** Because it is what was
measured, and because S19 eliminated the alternative: there is no export flag that withholds the
external-join key, so every `GroupInfo` admits its holder. Withholding the `GroupInfo` itself is
impractical (§11.7 needs the returner to fetch one, and §7.4.2 already treats it as a *claim* rather
than authority), while withholding the tree is both effective and independently justified by size.

**And the deeper reason the key layer cannot carry this at all (S19).** An epoch roll's exclusion is
real — measured at the strong grade, a removed member who advances her own branch to the same epoch
number is refused on the **key**, with an AEAD decryption failure. But that lock is on
**derivation**, and the external-join path **never derives**: the joiner does a KEM against the
published `external_pub` to obtain the current epoch's `init_secret`. **So the roll excludes passive
reading and does nothing about active re-entry** — the two are separate doors, and no amount of
re-keying closes the second. This is the mechanical reason §11.8 is right to put ban enforcement in
an application-layer gate, and the reason no key-layer fix exists to look for.

**Why refusal being two-layered matters more than the side door.** The alarming finding is that a
removed member can walk back in. The finding that *defuses* it is that anyone who declines is
mechanically protected at both the key layer and the addressing layer, without cooperation from
anyone. **Who you key for is your group** — the owner's framing, confirmed at Rung A. That reframes
the work from "close the hole" to "make sure the group can agree, in advance, on when to refuse."

**Honest limit of this plan.** It does not address §11.11's gap-completeness beam, which is the
property that decides whether two nodes agree on standing at all. Everything in Phase 4 is sound
*given* gap-completeness and unproven without it. That beam is Part 2 Appendix B's and is not
discharged here.

## What this does not change

- The two-target delivery shape (group queue + personal inbox) is unaffected.
- Custodial write (meer-lane Phase 1) and object lifecycle (E95) remain the standing blockers.
- Nested sealing (E96/S17) remains an adoption decision with measured cost, independent of this plan.
