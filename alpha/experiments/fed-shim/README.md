# fed-shim — the fediverse-wire conformance shim

A slim Rust library that speaks a narrow subset of ActivityPub over the
wire with **byte-exact fidelity** to Mastodon behavior. Purpose: give other
Croft crates (starting with `ap-ambassador`) a deterministic in-env
integration-test surface — no outbound network, no docker-compose Mastodon.

**Read `FED-SHIM.md` first.** The charter's §0 governing principle and
§3 firm non-goals are load-bearing: this is a wire-conformance surface,
not a Mastodon replica. What the shim models is specimen-anchored (see
`tests/specimens/`); what it doesn't model is FIRM non-goal territory
that the attended live leg (real Mastodon or GoToSocial on a real host)
has to close.

- Charter + scope walls + fidelity discipline: `FED-SHIM.md`
- Findings ledger: `FINDINGS-FED-SHIM.md`
- Wire specimens (single source of truth for byte shape): `tests/specimens/`
