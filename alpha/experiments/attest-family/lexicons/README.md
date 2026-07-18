# attest-family draft lexicons — DRAFT, NON-NORMATIVE

Status: **DRAFT** (RUN-ATTEST-03 Part B.2; revised RUN-ATTEST-04 B.4). These
are sketches, not shipped schemas: the crate's canonical dag-cbor
(`src/canonical.rs`) remains the source of truth, and nothing consumes these
files at runtime except the `atproto_map` mapping spike (T-A3.7/T-A3.8), which
mirrors their shapes in pure code.

V5 revision (RUN-ATTEST-04): `ing.croft.attest.treeHead` supersedes
`ing.croft.attest.commitmentEpoch` (kept visible, marked superseded — never
deleted); `credential` gains the required `era` anchor (era-graded membership,
V6/era-reissue).

Design rules carried into the schemas:

- **Closed sets use lexicon `enum`, never `knownValues`** — `knownValues` is
  advisory-open ("values are not limited to this set" per the lexicon spec);
  `enum` is "a closed set of allowed values". Every closed vocabulary from the
  crate (consent modes, predicate kinds, methods, roles, and V1's
  `AntecedentKind`) is an `enum` here, or the T-AT6.1/T-A3.2 compile boundary
  would silently reopen at the schema layer.
- **No numeric score field anywhere** (the T-AT0.2 invariant expressed as
  schema absence). The only integers are date-claim components ({y,m,d},
  mirroring the crate's `DateClaim`) and the commitment epoch's number/total
  (the T-PA3.3-audited totals-only numerics).
- **The co-signed edge join is the shared core hash**, not mutual CID citation
  (which is impossible — see ATTEST-ATPROTO-MATCHUP.md row 2). `edgeHalf`
  carries the full canonical core; the optional `counterpart` strong ref is a
  one-directional convenience locator, never the edge's identity.
- **What has NO lexicon here is deliberate**: resolvability policy, ceremony
  session privates, and seam-typed issuer state stay on the Drystone/private
  tier; withdraw/dissolve/supersede realize as author-sovereign record
  operations (delete / same-rkey replace), not as record kinds. The
  mechanical, test-enforced list is `atproto_map::fields_without_lexicon_home`
  (T-A3.8).
