# Blacksky: community infrastructure on atproto

date: 2026-07-07

**Source.** This profile distills Thread D of the raw transcript
`../../alpha/seeds/transcripts/raw/mls-blacksky-modular-prior-art-2026-07-06.md` (batch eight), where
grounded research on Blacksky was preserved but not distilled into a standalone doc. It sits in cairn because
Blacksky is shippable, running infrastructure Drystone can learn from, and because it lives squarely in the
atproto/AT ecosystem cairn already surveys.

## What it is

Blacksky (Rudy Fraser) is an atproto community-infrastructure project. It began as a custom Bluesky feed
centering Black voices and grew into full atproto infrastructure: its own relay, PDS, AppView, moderation
service, feeds, and client. Fraser frames it as "infrastructure for interdependence," organized around
"what's good for us" rather than "what's good for engagement." The distinction matters for cairn's purposes,
because it is a case of a community assembling the whole atproto stack for itself rather than renting a slice
of someone else's.

## Governance, in three layers

Blacksky's governance is worth studying as a working model, not a proposal. The transcript records three
distinct layers.

- **Deliberation** runs through a **People's Assembly** built on **Polis**, the vTaiwan consensus-mapping
  tool, self-hosted by Blacksky. The transcript notes recent participation in the hundreds, with tens of
  thousands of votes, on questions such as community guidelines and an AI-coding policy.

- **Moderation labor** is done by paid, community-recruited, publicly-credited moderators. The
  `@blacksky.app` moderation service is default-on and non-disableable inside their own client. The
  transcript preserves the protocol-layer nuance: at the atproto layer, another client still chooses whether
  to subscribe to those labels, so the "non-disableable" property is a property of Blacksky's client, not of
  the network.

- **Funding** is entirely subscription-backed through Open Collective, with no ads, at roughly $6,500 per
  month per the transcript.

## The AppView story

Blacksky's AppView is a deliberately-thin fork of `bluesky-social/atproto`, hosted at
`blacksky-algorithms/atproto` and not accepting contributions. Changes are confined to `packages/bsky`,
`services/bsky`, and one migration, which keeps the fork close to upstream and cheap to track.

Three changes stand out.

- **A Rust indexer for throughput.** Blacksky ripped out the TypeScript indexer and replaced it with a Rust
  one, `rsky-wintermute`. The transcript records the motivating gap: roughly 90 records per second on the
  TypeScript backfill (about 6.5 years for the full network) against a target of 10,000-plus records per
  second.

- **Self-hosting-at-scale fixes**, needed to run the stack independently rather than as a hobby deployment.

- **Community Posts**, a `community.blacksky.feed.*` lexicon backed by a `community_post` table, for private,
  membership-gated posts that live on the AppView rather than on the PDS. This inverts atproto's
  PDS-as-source-of-truth model for that one record type: the authoritative copy of a Community Post is
  AppView-resident, not repo-resident. The transcript frames this as an explicit, hard-isolated exception
  rather than a pretense that private data fits the repo model.

The fork's README is candid about real failure modes, which cairn should record because honest operational
postmortems are rare in this space: a 1.3-billion-row / 663 GB notification table caused by a missing unique
constraint; roughly 66,000 records corrupted by COPY escaping; and Redis disabled over a serialization bug.

## Scale

The transcript records growth from 0 to 2 million users, "$0 out of pocket," and "100% organic." Blacksky
runs a **full-network** AppView: it indexes the whole atproto network (36M-plus accounts, roughly 18.5B
records), with recommended hardware of 48-plus cores, 256 GB RAM, and 28-plus TB NVMe, and a two-to-four-week
backfill. The stated rationale is strategic independence plus ecosystem redundancy: a second full index of
the network that no single operator controls.

The transcript also notes that Blacksky is named by Bluesky (alongside Northsky and Habitat) as a team
implementing non-public-data extensions ahead of Bluesky's official "Permissioned Data" work. That naming is
attributed to Bluesky in the source, so it is preserved here as Bluesky's characterization rather than an
independent finding.

## What is transferable to Drystone

Two ideas carry over, per the transcript.

- **Traffic-class queue separation with a replayable upstream cursor as the real durability guarantee.**
  Blacksky separates live traffic from backfill traffic into distinct queues. The durability guarantee does
  not live in the local queue at all: the local queue (a Fjall embedded LSM store, with roughly 72 hours of
  relay retention) is treated as disposable, and the actual guarantee is the ability to replay from a cursor
  against the upstream. This is the load-bearing insight: durability is the replayable position, not the
  buffer.

- **The honesty about inverting your own source-of-truth invariant.** Community Posts break atproto's
  PDS-as-source-of-truth invariant on purpose, and Blacksky documents that inversion as an explicit,
  hard-isolated exception rather than hiding it. For a spec like Drystone that also carries source-of-truth
  invariants, the transferable practice is the disclosure discipline: when you break your own invariant, name
  it, scope it, and isolate it, rather than pretending the exception fits the model.

## The tension worth flagging

The transcript flags one gap directly relevant to Drystone's governance-layer questions: Fraser is
founder and CEO of **Black Sky Algorithms**, a company steering community-governed infrastructure, not a
formal cooperative or nonprofit. The participatory-governance rhetoric (People's Assembly, community-recruited
moderators, subscription-not-ads funding) sits atop a corporate form. That participatory-governance-versus-
corporate-form gap is exactly the seam Drystone has to reason about when it asks who holds the capital and
legal authority underneath a governed commons.

## What Drystone takes / what to watch

**Takes:** the queue-separation-plus-replayable-cursor durability pattern, and the disclosure discipline
around a deliberately-inverted source-of-truth invariant. Both are concrete, shippable practices rather than
aspirations, which is why they belong in cairn.

**Watches:** whether participatory governance can hold when the legal shell is a company. Blacksky is the
best available evidence that community-governed atproto infrastructure can reach real scale, and also a
live demonstration that governance rhetoric and corporate form can coexist without being reconciled. Drystone
should track how (or whether) that gap closes, because the governance layer inherits the same question.
