# Industry research & comparison

This directory holds our **industry research and comparison** of the field — analytical work
that positions our design against what exists, surfaces lessons, and informs decisions.

## Relationship to the ecosystem register (they overlap on purpose)

`research/` and `../ECOSYSTEM.md` cover **much of the same set of projects**, but they are
written for different purposes, audiences, and needs — keep both, and let them overlap:

```
ECOSYSTEM.md (relational)              research/ (analytical / comparative)
  purpose: homage, integration,          purpose: position our design, extract
    partnership, rebroadcast, learn↔       lessons, inform build decisions
  audience: us + future collaborators;   audience: us (design), reviewers, and
    a "movement we're part of" record      re-cuttable for funders / public
  shape: register (org/project/state/    shape: deep comparative analysis along
    relationship tags)                     axes (usability/security/capability, …)
```

Same projects, different lens. A fact about, say, Delta Chat's iroh usage may appear in both:
in ECOSYSTEM.md as "closest Rust+iroh cousin, partner/learn↔"; here as a detailed
usability/security/capability comparison with our stack. That duplication is intended — do
not try to merge them into one document; cross-reference instead.

## Contents

- `messaging-solutions-landscape.md` — three-axis competitive analysis (usability, security,
  capability) of Signal, Delta Chat, SSB, Matrix, Briar, Session, WhatsApp/Telegram, mapped
  to our planned stack. The design-lens comparison.

## Anticipated (different audiences, same underlying research)

The same comparative material will likely be re-cut for different needs — e.g. a
funder/partner-facing brief, a public-facing "why this is different" piece, or a
threat-model-focused security comparison. When that happens, each lives here as its own
audience-targeted document rather than overloading the design-lens analysis.

Refresh discipline matches ECOSYSTEM.md: verify volatile current-state before external use.
