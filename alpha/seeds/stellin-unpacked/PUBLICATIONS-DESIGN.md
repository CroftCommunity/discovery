# Publications on the group substrate: what vanilla atproto gives, and where our machinery pays rent

`Design document, 2026-07-17. Standalone corpus payload, written to be placed beside GROUPS.md
(suggested landing: alpha/experiments/appview-infra/PUBLICATIONS.md). It records the
derivation that began with a simple question — "could you do a newsletter with atproto on its
own?" — and ended with the system's thesis. Companion to the group tier model (GROUPS.md /
RUN-16 §A); evidence pointers cite the tier-proof lineage (RUN-17, RUN-18).`

## 0. One sentence

Vanilla atproto validates authors perfectly and always did; what it cannot do is hold a
group, on either side of the page — and the group, not the author, turned out to be the
product.

## 1. What vanilla atproto gives, stated generously and precisely

Every record is content-addressed and lives under a signed Merkle commitment in its author's
repo. From that alone, with no machinery of ours:

- **Authorship and integrity are free.** "Is this post authentic, unmodified, from this DID"
  is a signature-chain check anyone can run.

- **Current-state completeness is free.** A full repo sync commits to the entire record set;
  nothing presently in the repo can be hidden from a verifier.

- **A single-author newsletter is fully expressible.** One account, one collection, readers
  following it: authorship and integrity guaranteed end to end.

**The degeneration principle (binding on our designs).** Because the above is true, our
open-membership, single-writer scopes SHOULD ride ordinary atproto records with exactly one
addition — the per-author chaining rule — and nothing else. We do not rebuild what the
substrate already proves. Every delta below must earn its place by expressing something a
single signed repo structurally cannot.

## 2. The single-agent limit

Every primitive in atproto is single-agent. A repo is one agent's speech acts. Follows and
blocks are unilateral directed edges. Bluesky lists — the closest native thing to a group —
are one curator's record ABOUT other people: conscription-capable (you can be placed on one
without consent), held or released solely by the curator, expressing no joint authority. The
protocol gives you vertices and one-way arrows. There is no native object for a set of people
with agreed boundaries; no collective noun.

**The missing atom, and the only thing our machinery truly adds:** the provable multi-party
fact — co-signed, causally ordered, two-sided where consent matters. Author sets, subscriber
rosters, and steward councils are all this one atom under different policy values (the two
axes: membership policy, write policy).

## 3. The deltas: what a publication gains over a repo

| Delta | Vanilla atproto | On the group substrate |
|---|---|---|
| Authorization vs authorship | proves who WROTE a record; the author set is one account, forever | proves who was AUTHORIZED to write, at which causal position, granted and revoked by governed ops; "valid author" is an electable, removable role; a removed author's earlier issues stay valid, later ones do not |
| Publication history | tamper-FREE current state: the Merkle proof attests what is present now; a silently deleted post is indistinguishable from never-published; retraction is invisible | tamper-EVIDENT history: chaining makes an issue's existence permanent public structure once anything later circulates; a reader distinguishes never-existed, retracted, and being-withheld-from-me (the three-way distinction) |
| Identity | the publication IS an account; succession means surrendering the owner's whole identity | the publication is an institution: a genesis-hash principal under which stewards change, authors rotate, tiers are crossed, and the lineage persists independent of any person |
| Ordering | per-repo revs only; no order across authors' repos | causal antecedents order the stream across any number of repos |
| Reader completeness | depends on an unbroken firehose cursor or a full sync of each author | detection-grade completeness from the chain, identical over DS, backfill, or swarm; gaps are named and repairable |
| History access | all or nothing | membership-interval scoped offering, per the fold |

## 4. The subscriber reframe: two rosters under one lineage

A newsletter is not "an author plus an audience"; it is two rosters hanging off one lineage —
a tiny gated WRITER roster and a large open READER roster — the two-axes model named from the
subscriber side. Managing the reader side as a provable roster is what upgrades the product:

- **The reception guarantee gains a beneficiary.** "Complete up to newest held" is owed to a
  defined set; that is what makes it a guarantee rather than a property.

- **Reach becomes auditable.** Platform subscriber and follower counts are asserted numbers
  taken on faith; a roster folded from public self-registrations is a count anyone
  re-derives, and churn (unsubscribes as authenticated deletions) is equally provable.
  Metrics that cannot be inflated is a claim no incumbent platform can make.

- **Consent is structural.** Against the list model: nobody appears on a roster without a
  record they authored, and nobody is held on it after deleting that record.

- **The paid tier is pre-designed.** A gated reader roster under the same lineage is a
  paywalled publication with zero new machinery; the tier ladder applies to readers exactly
  as to members, because they were never different objects.

**Honest scope of the word "managing."** In open enrollment there is no authority: we fold,
serve, and owe completeness; nobody is admitted or expelled. Management in the strong sense
begins only where a roster goes gated — and then it is governance, with everything that word
means in this corpus (sealed deliberation, public verdicts, silence never a verdict).

## 5. Where the rent begins

The boundary is exact. The moment a publication has an editor who is not the owner, a second
author, a succession plan, or a retraction policy readers can audit, it has left what one
signed repo can express. Below that line, use the substrate bare (§1). Above it, the group
machinery is not an alternative to atproto verification; it is the collective layer atproto
never had, built from atproto's own parts — records, repos, DIDs, the relay — plus the one
atom it lacks.

## 6. Pointers

The mechanics live in the group tier model (GROUPS.md; RUN-16 §A): principals and blinding
(A.1), the two axes and reception completeness (A.2), membership facts and intervals (A.3),
the open tier (A.4), assertion and identity (A.5), governance (A.6), delivery roles (A.7),
transports (A.8). Executable evidence: the tier proof (RUN-17) and the reception/publications
amendment (RUN-18): chaining validation, gap detection and repair, tail honesty, the
retraction three-way distinction, and the auditable-count assertion.
