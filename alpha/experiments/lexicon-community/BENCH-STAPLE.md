# EXP-LEX-04 benchmark — staple vs status-by-callback

`Measured local (Modeled grade, loopback), dev/debug (unoptimized — a release build is ~1-2 orders faster) build. Era size = 1024 credentials; audit path = 10 nodes.`

| metric | holder-stapled inclusion proof | status-by-callback (OCSP-shaped) |
|---|---|---|
| verifier→issuer network | **none** | 1 round trip per check |
| privacy | issuer learns nothing | issuer learns (verifier, credential) every check |
| offline verify | **yes** | no |
| wire bytes (holder→verifier) | **870 B** | ~0 (request) + status response |
| verify time (measured) | **10.136785ms** | dominated by network RTT (tens of ms) |
| trust base | 2 CID-first sigs + Merkle path | issuer liveness + TLS + honest-answer |

Head record 208 B · binding 166 B · path 320 B (10 nodes) · sigs 128 B.
The staple is a strict improvement on the axis the spec leaves open: freshness
WITHOUT the (verifier, subject) capture leak OCSP taught the web to avoid.
