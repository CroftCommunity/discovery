# Rights vs capabilities — the definitions the boundary depends on

date: 2026-06-24

source: distilled from the 2026-06-24 rights-vs-capabilities definitional dialogue
(`../seeds/transcripts/raw/rights-vs-capabilities-definitions-dialogue-2026-06-24.md`, cleaned-paste).

purpose: make the rights-vs-capability line **sharp**, so the boundary "no right to remove the rights of
others" (beta `01` §5) is load-bearing rather than asserted. A standalone, cite-able block: the Drystone spec
(`drystone-spec/`, §2.3 / §5), beta `01` §5/§6.1, `05`, and `06` all point here. Pairs with the
capabilities-not-rights / data-plane-vs-control-plane framing already in `01` §6.1.

> **Status:** the **discriminating test** and the rights/capability *definitions* are settled and were
> **folded into beta `01` §5 on 2026-06-25 (user-approved, K17)** — a tier-clean grounding paragraph. The
> **four-rights closed set** still carries **two verify-before-hardening items** (share; tenure) — see the
> end — so do **not** harden the *closed set* into the normative spec until both clear (ROADMAP_TODO E32
> b/c). The 01 fold deliberately states the principle without those caveats (they are spec/04/07-level).

## The discriminating test — does removal cancel the conditions of its own contestation?

The distinction is **a property, not a list.** The wolf test supplies the criterion: a right is any standing
whose removal is **self-amplifying toward collapse**, because removing it lowers the variety available to
resist the *next* removal. A capability is a power whose removal leaves contestation intact.

- **A right is standing that must survive in order for any dispute about it to remain contestable.** Remove
  it and the holder loses the very means to object to the removal.
- **A capability is an ability to act or affect outcomes that does not, when removed, disable the holder's
  standing to contest the removal.**

This is the same cut as the local-first derivation (`01` §3): **rights are local-first state the person
holds and so cannot be reached into; capabilities are mediated through roles and relationships and so move
freely.** It is the standing-side companion to the data-plane/control-plane framing in `01` §6.1 — there,
*capabilities* (data plane) vs *rights* (control plane); here, the test that tells which is which.

## The four rights — a closed set, each defined by what its removal forecloses

Defined by irreversibility-from-the-center, not by what they grant:

| Right | What it is | Removal forecloses |
|---|---|---|
| **Tenure** | standing to remain a peer; identity-as-local-first-state (the `05` rights-floor: you cannot be cleared because your standing is not held elsewhere) | erasure of the unit |
| **Exit** | the right to fork — local-first's native move when peers disagree; the dignified alternative to being silenced (the Mill grounding, `01` §2.1) | the dissenter has nowhere to go (what *On Liberty* names as theft) |
| **Voice** | standing to assert into the record and be corroborated or refuted | your assertions made invisible — which defeats provenance itself (`01` §1) |
| **Share** | your claim on the collective's commons | expropriation — the enclosure inversion `02` is built against |

## Capabilities — the open remainder

Roles, delegated authority, moderation powers, admin functions, write-access to a shared resource, vote
weighting in a particular governance scheme, relay. Each can be **granted, narrowed, rotated, or revoked**
without disabling the holder's standing to object. **The tell:** if you lose admin on a collective, you can
still contest that loss using tenure, voice, and exit — so admin was a capability. (This is the
`floor + [capabilities]` decomposition the Drystone §2 spec draft uses for roles.)

## The resolved edge case — voice vs amplification

Voice is the hard one, because the line between *removing voice* (a rights violation) and *declining to
amplify / labeling / refusing to corroborate* (legitimate, per `06`'s label-not-enforce) is exactly where the
content-blind safety posture lives. The clean cut:

> **Voice is the right to assert into your own record and reach willing peers. It is *not* a right to compel
> any specific peer to carry, host, or amplify you.**

Removing your ability to assert at all = a rights violation. A peer declining to relay you = that peer
exercising *their* own standing. This keeps voice from collapsing into "a right to an audience," which would
itself be a clearance of others' standing *not to listen*.

## Two items to verify before this hardens (open) — staged beta `OPEN-THREADS` T21 / T22 (2026-06-26)

1. **Is `share` fully a right, or partly a capability?** (→ `07` / the cooperative model.) If `share` can be
   diluted by legitimate governance / membership class, part of it behaves like a capability — name **which
   portion is the inviolable floor** and which portion is a class-varying entitlement. Until resolved,
   `share` is the least-settled of the four. ROADMAP_TODO.
2. **Does the `04` survivor re-key strand a peer's `tenure`?** (→ `04`.) If a survivor re-key can in practice
   strand a peer, **tenure has an implementation-level exception** that must be named explicitly rather than
   left absolute. A protocol-level check against the re-key/survivor mechanism. ROADMAP_TODO.

## Where this landed

The dialogue produced this block **and** a one-paragraph framing for **`01` §5** that states the cut, ties it
to `01` §3, and hands off here. **Folded into beta `01` §5 on 2026-06-25 (user-approved, K17)** as a
tier-clean paragraph (no pointer back to this alpha block) after the boundary bullets — the discriminating
test, the four rights-by-removal, and the voice-vs-amplification cut. This block remains the fuller alpha
reference and the home of the two open checks above. The original framing paragraph is preserved verbatim in
the raw transcript. Trace: `../BETA-ROLLUP.md` (Beta grounding, K17).
