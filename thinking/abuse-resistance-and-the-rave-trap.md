# Abuse resistance without a central authority — the "Rave trap" and Croft's answer

date: 2026-06-16
status: thinking (design). Problem / Approach / Reasoning. How Croft stays off the
piracy/abuse-hub path **without** adopting the centralized-moderation playbook that contradicts its
thesis. Steers the broadcast tier, the meer admission policy, and the App-Store posture.

## Problem

**Rave (verified 2026-06-16)** is the cautionary case. A high-performance, cross-platform co-viewing
app (it uses iroh + MoQ) was **removed from Apple's App Store in August 2025**; Rave filed **antitrust
suits in five countries (May 2026)** alleging the real motive was protecting Apple's SharePlay. Apple's
stated reason was **repeated guideline breaches — hosting/sharing pornographic and pirated content and
CSAM complaints** — enabled by **unmoderated public rooms**. Both can be true: it was a competitive
threat *and* its friction-free, publicly-discoverable, unmoderated P2P design made it a porn/piracy/
CSAM distribution surface. It "worked too well" in both directions.

The dilemma this poses for Croft is real and unavoidable: **Croft is deliberately blind** (the meer
can't read content) and **non-extractive** (no central authority inspecting users). Those are the whole
point — and they mean Croft **cannot content-moderate at the transport/meer layer** the way Apple
demands. If we also shipped public discovery + unbounded broadcast, we would build exactly the
distribution surface that got Rave pulled, with *less* ability to police it. The user's instinct is the
right lever: **step lightly on large-scale broadcast and the piracy/abuse avenues — mostly via scale
and peer restrictions, not content surveillance.**

## Approach

### The two poles — Signal vs Telegram (verified 2026-06-16), and why Croft takes Signal's path

Signal and Telegram are near-opposite poles, and the contrast is the whole argument:

| axis | **Signal** | **Telegram** | **Croft** |
|---|---|---|---|
| content visibility | E2EE everywhere (blind) | cloud chats/groups/**channels NOT E2EE** (server-readable) | blind (E2EE + blind meer) |
| public discovery | none (contact-based) | **yes** (public channels/groups + search) | none (invite/capability) |
| mass broadcast | no (a messenger) | **yes** (channels, 2 GB file uploads) | capped (tiers) |
| can it content-moderate? | **no** (refuses to scan) | **yes** (readable) | **no** (blind) |
| moderation method | report + block + minimal metadata + account accountability | server-side scan + (post-2024) law-enforcement cooperation | report + revocation + standing + scale caps |
| abuse outcome | low — not a hub | **became the hub → CEO arrested** | designed to stay Signal-shaped |

The two ways to avoid being an abuse hub are **opposite**:
- **Telegram's way:** be *readable* so you *can* moderate centrally. Telegram is not E2EE by default
  (only 1:1 "secret chats" are), so it can — and after **Pavel Durov's arrest in France (Aug 2024,
  charged with complicity incl. CSAM distribution)** it reversed its "never hand over data" stance:
  on 2024-09-23 it began sharing **IP + phone** on valid warrants, deployed AI moderation, and removed
  **15.4M groups/channels** in 2024. It *could* do this only because it reads content. Its public
  channels + 2 GB uploads made it the piracy/CSAM **distribution** surface in the first place.
- **Signal's way:** don't create the surface. **No public discovery** (contact-based), **no mass
  broadcast** (it's a messenger), **blind** (E2EE + sealed sender, >80% of traffic hides the sender).
  It refuses content/client-side scanning on principle; moderation is **report + block + the minimal
  metadata it holds** (registration + last-connection — almost nothing) + account accountability.
  WhatsApp (same E2EE model) bans 300k+ accounts/month for CSAM on **metadata + reports, not content**.

**Croft must take Signal's path, because it is blind by design — Telegram's readable-moderation path
is not even available to us.** So our only viable abuse-resistance is Signal's: **deny the surface
(no public discovery, scale caps) + accountability (standing/membership) + report/revoke.** This is
why the rest of this doc is scale/peer levers, not inspection — it is not a preference, it is the only
path consistent with being blind.

**The trap to name precisely (this is Rave):** the worst combination is **Telegram's surface with
Signal's blindness** — public/mass broadcast + large media distribution, but blind so you *cannot*
moderate it. That is exactly what Rave was, and what gets a platform pulled. So the discipline is:
**do not become Telegram-shaped on the broadcast/media side.** Stay Signal-shaped; cap the broadcast
tier; never ship public discovery.

**Chat vs media impact (the user's point, confirmed):** Telegram's hub problem is driven by **channels
(broadcast) + 2 GB file uploads** — the *mass-distribution* combo — far more than by 1:1 chat. So the
real axis is **conversational vs broadcast-at-scale**, not chat-vs-media: a 1:1 video call is
low-impact (like a DM); a one-to-many broadcast / large-file distribution is high-impact (like a
Telegram channel). This maps exactly onto our **media interaction types** — conversational (RoQ,
low-impact) vs broadcast (MoQ, the high-impact surface to cap).

### What we explicitly REJECT (the pasted "central hand on the wheel" playbook)

The standard advice — *a central control plane that handles auth/room-metadata/link-filtering/reports,
runs content-moderation APIs on messages and URLs, holds central kill-switch authority over identity,
and runs edge-AI that scans the camera buffer and reports to a backend* — is **thesis-incompatible**.
It recreates the extractive choke point Croft exists to remove: a party that inspects content,
identity, and links, that can be subpoenaed, breached, or repurposed. Edge-AI-scanning-then-snitching
is client-side surveillance with a center, and "who defines explicit?" makes the classifier a censor.
We do **not** adopt the golden-rule framing "keep a centralized hand on identity and compliance." That
is the thing we refuse.

This refusal has a real cost — stated plainly: it does **not** make the App-Store gatekeeper risk go
away. Croft could face the same pressure Rave did. Our mitigation is structural (below), not a promise
that a gatekeeper won't object. Signal survives this posture (E2EE + no content scanning + account
accountability); that is the precedent we follow, not Discord's.

### What we DO — thesis-consistent levers (scale + peer + governance, never content)

1. **No public discovery, by default — and this is already our model, not a new feature.** Croft is
   capability-/invite-gated: you join via an explicit ticket/QR/invite shared out-of-band, admitted by
   a member with standing. There is **no public-rooms directory** to turn into a piracy/CSAM index.
   This is the single biggest lever and it *aligns* with the thesis (S2 scoped visibility, quiet
   membership) — a system with no anonymous-stranger discovery surface is structurally not a hub.
2. **Scale caps via the interaction tiers — broadcast is the dangerous tier.** Mass distribution is a
   *broadcast* (one-to-many at scale) phenomenon. Mesh dies at ~5 and SFU at dozens by physics — only a
   **MoQ relay / meer** enables mass fan-out. So the distribution vector is exactly the controllable
   choke: cap broadcast fan-out, rate, and audience size as tier policy.
3. **The meer/relay as a BLIND scale-and-admission choke (not a content choke).** A meer sees the
   metadata it already sees (AR-4: membership count, fan-out, namespace, rate) — *not* content. So a
   co-op-run meer **can** enforce scale/rate/admission policy blindly (refuse to be a mass-distribution
   CDN, cap audience, throttle), and **cannot** enforce content policy (it's blind). That boundary is
   honest and is the enforcement surface: **abuse-at-scale needs a meer, and co-op meers won't serve
   abusive scale.**
4. **Membership accountability via standing/lineage — not anonymity.** Participants are in a group
   because a member with standing admitted them (the proven faithful-path/standing model). Within a
   group, authorship is attributable (signed messages) and removable (revocation, MD-G5). This is
   accountability *without* central identity surveillance — the group governs itself.
5. **Block = revocation; report = client-attested, blind.** A client block terminates the iroh stream
   and blackholes the peer (the failed-op-response dial). A report carries what the *reporter's* client
   can attest from its own decrypted view (the immune-signal pattern) — the meer never snapshots
   content (it can't). Moderation lives at the **client + membership** layer, as it must under E2EE
   (same as Matrix's stated position).

### The fork reality — bound the blast radius, don't pretend to prevent

Croft is open; a fork **can** strip the limits and add public discovery + unbounded broadcast. We
cannot prevent that, and shouldn't pretend to. What we can do is bound the blast radius:
- **The mainline app does not ship the abuse affordance** (no public directory, scale-capped broadcast,
  meer admission policy). A fork that adds it is a *different app the co-op neither ships nor hosts.*
- **Co-op/non-extractive infrastructure does not subsidize abuse scale.** Mass distribution needs
  meers; the co-op's meers enforce scale/admission policy. A fork wanting piracy-CDN scale must run
  **its own meers** — its own infrastructure, its own legal liability. Croft's infra doesn't carry it.
- **Physics caps the worst pure-P2P case.** A mesh-only fork (no meer) cannot reach mass-distribution
  scale — the topology that makes Croft private (no central forwarder) is the same topology that makes
  a no-infrastructure fork unable to become a CDN.
So the worst case is bounded: a fork must take on its own infrastructure *and* its own liability to do
at scale what the mainline structurally prevents.

### The CSAM corner — named, not waved away

CSAM is the category where "we're blind, so we can't scan" is the hardest to stand behind, for any
platform. Croft's honest position: resistance comes from **(a)** no anonymous public-discovery surface,
**(b)** membership/standing accountability (no anonymous-stranger distribution), **(c)** scale caps (no
mass broadcast), and **(d)** client reporting + revocation — **not** from content scanning, which is
impossible while blind. This is the same posture as Signal/Matrix (E2EE + report + account
accountability) and is defensible, but it is a **stated limitation**, not a solved problem, and it must
be in the threat model and the public posture, not hidden.

## Reasoning

- **Why scale/peer restrictions are the right lever (the user's instinct, confirmed):** content
  surveillance is impossible (blind) and thesis-breaking (extractive); the abuse that gets platforms
  pulled is *scaled, public, anonymous* distribution. Remove public discovery and cap broadcast scale,
  and you remove the surface — without inspecting a single byte. You attack the *distribution topology*,
  which we can see (AR-4 metadata), not the *content*, which we can't and won't.
- **Why this aligns rather than conflicts with the thesis:** the same features that make Croft private
  (invite-only, no public directory, no central forwarder, blind meer) are the features that make it a
  poor abuse hub. Privacy and abuse-resistance-by-topology are the same design, not a trade.
- **Why we accept the App-Store risk consciously:** the alternative (central content/identity control)
  is the extraction we exist to refuse. We take the Signal posture and the "private utility, not a
  public broadcast network" framing — which is *also* what survives review best — and we are honest
  that large-scale public broadcast is the riskiest surface, to be approached lightly or not at all.

## Open edges

- **Exact broadcast-tier caps** (max audience, fan-out rate, who may create a broadcast) — undefined;
  needs a policy schema, enforced blindly at the meer from metadata.
- **Meer admission policy language** — how a co-op expresses "I won't serve broadcasts over N / rate R /
  to non-members" without seeing content; ties to `meer-superpeer-design.md` (role 3, MoQ relay).
- **Report integrity** — a client-attested report can itself be abused (false reports); ties to the
  k-observer corroboration in `failed-op-response.md`.
- **The CSAM posture needs legal review**, not just engineering — flag for a human/legal pass, do not
  finalize from the code side.
- **Jurisdictional reality** — five-country litigation (the Rave case) shows abuse/competition policy is
  per-jurisdiction; a co-op running meers inherits the law where the meer runs. Out of scope here,
  named for the roadmap.
