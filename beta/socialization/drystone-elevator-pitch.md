# Drystone: the elevator pitch

`Resolution: elevator pitch, the shortest telling, in a plain-spoken and a technical register. For a grounded overview, read the coffee-shop; for the complete reference, the library (Parts 1 and 2). Where this and the library diverge, the library governs.`

`Status: proposal, derived from Part 1 (§1, §2). To reconcile with the founding tagline and paragraph already in hand; replace freely.`

## Plain-spoken

Most apps put a company's server in the middle of your relationships, and that server quietly owns the
truth: it decides what was said, who you are, and who is allowed to do what. The day the company's
interests stop matching yours, you find out none of it was ever really yours. Drystone lets a group talk
and govern itself with no one in the middle. Your copy of the conversation lives with you and works on its
own, you can check the record yourself and no one can silently rewrite it, everyone counts equally, and
when a group genuinely splits, it splits cleanly, with everyone walking away whole instead of one side
being erased. It runs on an ordinary phone, and it is built to sit alongside networks like Bluesky rather
than replace them.

## Technical

Drystone is a center-free, peer-symmetric protocol for group messaging and governance: no node holds
canonical or privileged authority over shared state, because no node in a distributed system can prove its
knowledge is complete or current, and the design treats that epistemic limit as generative rather than as
something to coordinate away. It builds on MLS (RFC 9420/9750) for the cryptographic group, a local-first
append-only governance log that folds to compute current authority, and an iroh transport; a conflict
either resolves by a party-neutral deterministic tiebreak or, when it is a genuine social dispute,
hard-stops to human adjudication instead of auto-merging. Its distinctive primitive is fork-not-verdict:
the substrate computes provenance but never renders the social verdict, so the harshest act available to
any authority is to force a fork, leaving the excluded party whole in its own lineage. It complements
ATProto-class networks by carrying the no-coordination premise of local-first up into the governance
layer.

## Read on

For a grounded overview, read the **coffee-shop**; for the complete, definitive reference, the
**library** (Part 1 and Part 2).
