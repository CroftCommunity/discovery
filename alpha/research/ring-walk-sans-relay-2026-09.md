# Walking the social tree in rings without a relay: what a browser can reach, measured

author: Claude Code session (research + live probes), commissioned by the owner 2026-09-08

date: 2026-09-08

status: measured findings + recommendation, for owner decision. Every number below was
produced by the probe scripts in the appendix on 2026-09-08 against production hosts;
third-party facts carry a `[source]` or `[secondary]` label.

`Answers the owner's question: forage (and the coming social-tree visualization site)
read the follow graph through the Bluesky AppView today; when that "worldwide view" is
down, could a browser walk the rings (me → mutuals → follows → follows-of-follows) direct
from PDSs, how efficient can that be, should it be a Rust→wasm core shared across sites,
and what is actually usable with no relay at all? Also retires E146's open measurement
("squaring the hop is the thing to measure rather than predict"). Prior corpus thinking it
extends: the rev-gated revalidation design in
seeds/wiki-unpacked/RUN-BUNDLE-PRECACHE.md (E58) and the radius model of E62.`

---

## TL;DR — the decision and its options

**Recommendation: build one shared "rev-gated ring walker", in TypeScript, not Rust→wasm,
and give each ring its own refresh cadence.** The measurements say the cost of walking
rings is network requests, not CPU: a 14.6 MB repo export decodes in plain JavaScript in
50 ms, while the same account's follow list takes 12.5 s to page through. A wasm core would
speed up the part that is already free.

What a browser can reach with **no relay and no AppView** (PDS + `plc.directory` only):

| Ring | Meaning | Reachable sans AppView? | Measured cost |
|---|---|---|---|
| 0 | me | yes | one `getLatestCommit` (0.13 s, ~200 B) per poll; a diff CAR of 8–300 KB when the rev moved |
| 1 | my follows | yes | 701 follows: 9 pages, 2.7 s. 4,007 follows: 42 pages, 12.5 s — or one 14.6 MB CAR in 1.7 s + 50 ms parse |
| mutuals | my follows who follow me | **partly** — my *followers* live in other people's repos; no PDS can list them. Two ways: Constellation's backlink index (third-party, 0.1 s), or as a **free by-product of the ring-2 walk** (every followee's follow list is read anyway, so "does X follow me" falls out) | see ring 2 |
| 2 | follows of follows | yes, as a background job | 701 follows fan out to **440,424 edges = 4,755 pages** (median 337 follows per followee, p90 1,530, max 20,142). ~5 k requests: about 70 s at 10-parallel, spread over 25–29 PDS hosts |
| 3 | follows of ring 2 | **no** | ≈ 440 k × 654 ≈ 290 M edges. Not a client-side walk at any cadence |
| ∞ | world, search, follower counts | **no** | AppView (or a third-party index) only |

The live-update tier follows the same shape: **Jetstream accepts up to 10,000 DIDs per
socket** [source], so ring 1 fits one browser WebSocket and ring 2 does not. Ring 2 is a
walk, refreshed on a cadence, never a stream.

Options the owner should pick between (details in § 6):

- **Placement (A):** a new small shared package consumed by forage, pdsview, and the
  social-tree site — *recommended*; vs **(B)** grow it inside forage's `lens.js` and copy
  it later — the pattern that produced eight atproto OAuth implementations
  (`.claude/DECISIONS.md` § "Do NOT write a ninth").
- **Diff apply (v1 vs v2):** v1 re-lists a repo's follows with `listRecords` when its rev
  moved (no CBOR in the bundle — the choice RUN-BUNDLE-PRECACHE already made); v2 applies
  `getRepo?since=` diff CARs against a persisted block store. Recommend v1 first, with the
  v2 trigger stated below.
- **Followers source:** Constellation (third party, one machine) with an honest
  "followers unknown" state when it is unreachable; vs followers-only-from-AppView.

---

## 1. Problem statement

forage's ring scopes (`me` / `mutuals` / `follows` / `hop` / `world`, `forage/js/rings.js`)
are computed by `ringGraph()` in `forage/js/substrates/lens.js` from
`app.bsky.graph.getFollows` and `getFollowers` on the Bluesky AppView
(`public.api.bsky.app` unauthenticated, PDS-proxied when signed in). Nothing persists the
graph across sessions and there is no degraded mode: when the AppView is down, the ring is
gone. The AppView has had multi-hour outages in 2026 (an 11-hour `api.pop1/pop2.bsky.app`
outage in April, another in August) [secondary: StatusGator / Tom's Guide].

The social-tree visualization site needs the same graph, and the owner's E146 question
("is `+2` a rung or a different feature?") has been waiting on a measured fan-out.

## 2. Approach

Probe the actual surfaces from this machine, unauthenticated, with `curl`, on two real
accounts of different sizes (`pfrazee.com`: 701 follows / 35 k followers;
`jay.bsky.team`: 4,007 follows / 13.3 M followers), plus a 40-followee sample of each for
the second ring; then parse the resulting repo exports with the JavaScript CAR/CBOR decoder
forage would ship (`@atcute/car` 6.0.2 + `@atcute/cbor`) to find out whether decoding is a
cost at all. Scripts are in the appendix and are re-runnable.

## 3. What each surface actually offers (verified 2026-09-08)

### 3.1 The PDS — everything a browser needs for *outbound* edges, unauthenticated

| Endpoint | Auth | CORS | Latency | Notes |
|---|---|---|---|---|
| `com.atproto.repo.listRecords?collection=app.bsky.graph.follow&limit=100` | none | `access-control-allow-origin: *` | ~0.2 s / page | cursor-chained — pages of one repo cannot be fetched in parallel. Rate-limit headers are exposed to the browser (`RateLimit-Remaining` etc.) |
| `com.atproto.sync.getLatestCommit` | none | `*` | 0.13 s | returns `{cid, rev}` — the cheapest possible "has anything changed?" |
| `com.atproto.sync.getRepo?did=` | none | `*` | 0.26 s – 62 s | full CAR; sizes in this sample 10 KB → 81 MB (one 57 MB export took 62 s from a slow host) |
| `com.atproto.sync.getRepo?did=&since=<rev>` | none | `*` | 0.14 – 0.45 s | **diff CAR** — only blocks newer than `rev`. An arbitrary TID works as `since` (no need for a rev the PDS has seen) |

Rate limit: **3,000 requests per 5 minutes per IP, per PDS host** [source:
bsky.network rate-limits; header `ratelimit-policy: 3000;w=300` observed]. The 40-followee
samples touched **25–29 distinct PDS hosts**, so a ring-2 walk's budget is roughly
3,000 × (number of hosts) per 5 minutes, and a per-host concurrency cap is the right
throttle.

Diff CAR sizes, the number that makes polling cheap:

| Account | Full CAR | since 1 day | since 7 days | since 30 days |
|---|---|---|---|---|
| jay.bsky.team (4,007 follows) | 14.6 MB | **8.4 KB** (0.14 s) | 16.7 KB | 19.8 KB |
| pfrazee.com (701 follows) | 81.4 MB | **287 KB** (0.30 s) | 1.1 MB | 2.1 MB |

Decoding those diffs in JavaScript: 0.3 MB in 1 ms, 2.1 MB in 6 ms. The 30-day diff for
pfrazee held 32 new follows and 14 new blocks among 6,773 blocks; the 1-day diff held 1
follow. **The follow collection is a tiny fraction of a repo's churn — likes dominate** (4,043
of the 6,773 blocks), which is why per-collection `listRecords` re-listing (v1 below) is
competitive with the diff path for *follows* specifically.

Caveats the walker must handle:

- A **diff CAR only reveals additions directly.** Deletions (unfollows) appear as changed
  MST nodes; detecting them needs the previous MST blocks, i.e. a persisted block store
  keyed by CID. Without one, the honest fallback on rev change is a full re-list.
- **Dead hosts are normal in ring 2.** Of 40 followees, one PDS answered 502 (a
  self-hosted `*.bluesky.page`), and three were 403 — from this machine's OpenDNS
  interception, not the PDS (the recorded "this machine's resolver lies" trap). Treat
  every host as optional and every failure as "unknown", never as "empty".
- **PDS answers differ from AppView answers** — deactivated, blocked and taken-down
  subjects are still follow records in the repo (bluesky-social/atproto#3508) [source].
  Status must be resolved per DID (a `getLatestCommit` 4xx on the subject's PDS is the
  cheap tell).

### 3.2 Identity — `plc.directory`, and a faster mirror

`https://plc.directory/<did>` answers unauthenticated with CORS `*`; 40 DID documents at
10-parallel took 0.6–0.7 s. Its rate limits are "generous" and unstated [source: bsky docs].
Slingshot (`slingshot.microcosm.blue`, `com.bad-example.identity.resolveMiniDoc`) returns
`{did, handle, pds, signing_key}` in 0.27 s, bi-directionally verified — an optional
second resolver, not a dependency.

### 3.3 Followers — the one thing no PDS can give you

A follow is a record in the *follower's* repo. "Who follows me" is a backlink question,
answerable only by something that has read the whole network:

- **The AppView** (`getFollowers`) — today's path.
- **Constellation** (`constellation.microcosm.blue`, microcosm's backlink index): CORS
  `*`, unauthenticated, "community-supported infrastructure ready for production" [source],
  running on **one Raspberry Pi 5 with a 1 TB NVMe** [source: microcosm.blue]. Measured:
  followers count 0.10 s (35 k) to 2.4 s (13.3 M); "does X follow me" via
  `getBacklinks?subject=<me>&source=app.bsky.graph.follow:subject&did=<X>` in **0.11 s**;
  follower enumeration 100 DIDs/page with a cursor (350 pages for 35 k). The documented
  `getDistinct` XRPC name returned 404 on 2026-09-08; the deprecated `/links/distinct-dids`
  route worked. A single-machine third party is a dependency to *degrade around*, not to
  build on — the same posture forage's ADR-004 already takes for backlink counts.
- **The ring-2 walk itself.** Walking every followee's follow list to build ring 2 reads
  exactly the records that say whether each followee follows *me*. Mutuals-among-my-follows
  therefore cost nothing extra once ring 2 is walked — and that is the mutual set forage's
  scopes use (`mut ⊂ fol`). Followers who are *not* my follows still need an index.

### 3.4 Live updates — Jetstream fits ring 1, not ring 2

Jetstream (`wss://jetstream.us-east.bsky.network` and siblings) is a browser-reachable
WebSocket that filters by `wantedCollections` (≤ 100) and `wantedDids` (**≤ 10,000**)
[source: bsky.network/docs/jetstream]. Ring 1 for the accounts probed (701 / 4,007 DIDs)
fits one socket; ring 2 (~440 k DIDs) does not, and Jetstream cannot filter by *subject*
(E139 already recorded this), so "someone new followed me" is not a Jetstream question.
Spacedust (microcosm's interactions firehose) *does* filter by subject, but its readme
states it "offers no replay window, and cannot emit delete events" [source] — usable as a
notification hint, not as a source of truth.

### 3.5 Rust→wasm — the toolchain exists; the case does not

- Decoding: `@atcute/car` + `@atcute/cbor` parsed jay's 14.6 MB / 51,033-block export in
  **50 ms** (Node 22, one core), diffs in 1–6 ms. The walker's wall-clock is 99 % network.
- The Rust side is possible: `atrium-xrpc-client`'s reqwest client "automatically switches
  to the WASM one" on wasm32 [source: docs.rs]; `atrium-repo` pulls tokio and is not a
  browser crate; `atproto-repo` / `atmst` are pure-Rust MST/CAR readers. `fun/` already
  ships ~18 Rust→wasm crates (raw C-ABI + serde-JSON, no wasm-bindgen) and `croft`'s core
  is "WASM-clean" by ADR — but `croft/web/` is empty and the three consumers of a ring
  walker today (forage, pdsview, bluebird, and the social-tree site to come) are all TypeScript
  with hand-rolled fetch layers.
- So wasm buys nothing measurable and costs a second toolchain in three repos. The place
  Rust earns its keep is the **pure set math** (ring nesting, containment — what
  `forage/js/rings.js` does in 130 lines) *if* croft's Rust core ever needs the same ring
  semantics; that is a port of a pure function, not of the walker.

## 4. The ring-scaled cadence, with numbers

The owner's instinct — immediacy expectations scale with ring distance — is what the
budgets force anyway. For an account with N₁ follows on H hosts:

| Ring | Refresh | Cost per refresh | Cadence that fits the budget |
|---|---|---|---|
| 0 | `getLatestCommit` on me; Jetstream `wantedDids=[me]` when a socket is open | 1 request | continuous / every minute |
| 1 | `getLatestCommit` per followee; diff or re-list only where rev moved | N₁ requests of ~200 B (701 → ~9 s at 10-parallel); typically < 5 % of repos moved in a day | every 5–15 min in the foreground; Jetstream socket instead when signed in and N₁ ≤ 10 k |
| mutuals | by-product of ring 2, or Constellation | 0 extra, or 1 request per followee | inherits ring 2's cadence; a single "does X follow me" is 0.11 s on demand |
| 2 | walk each followee's follows once; then rev-gate them like ring 1 | first walk ~4.8 k pages ≈ 70 s at 10-parallel, ≈ 25 s at 30-parallel across hosts; steady state N₂ `getLatestCommit`s (~440 k → not per-session) | first walk in the background on first use; re-walk **daily or on demand**, per-followee lazily when that node is looked at |
| 3+ | never walked | — | AppView / index only, labelled as such |

The steady-state figure for ring 2 is the important one: 440 k rev checks is ~25 minutes
at 30-parallel and would burn the 3,000/5-min budget on the big shared hosts. **Ring 2 is
refreshed by re-walking the followees whose rev moved (ring 1's diff tells you which), not
by polling ring-2 nodes.** A followee whose rev has not moved has not changed their follows.

E146's question is answered by the ratio: ring 2 is **~630× ring 1's edge count** for this
account (440,424 / 701). `+2` is a different feature (a background-walked, hours-stale set
with an honest "as of" stamp), not another stop on a slider that recomputes on tap.

## 5. What is usable with no relay and no AppView

- **Yes:** ring 0, ring 1, ring 2 by follows; mutuals *among my follows*; each
  followee's posts (the same rev-gated walker over `app.bsky.feed.post` — the diff CAR
  already contains them); identity resolution; block/mute records I wrote myself.
- **Only through a third-party index (degrade honestly):** my followers who are not my
  follows; follower counts; "who newly followed me".
- **No:** ring 3+, search, the world view, anything ranked over the whole network.

A social-tree visualization drawn from rings 0–2 with a per-ring "as of" timestamp is fully
buildable from PDSs alone. Its follower-side half should render as "unknown" rather than
"none" when the index is unreachable — the same rule forage applies to a rejected ring
promise ("a transient 502 must never be remembered as an empty ring").

## 6. Recommendation, and the reasoning

**Build one walker, in TypeScript, as a shared package, and reuse it in three places.**

1. **One home, not three copies.** forage's `pagedGraph`/`ringGraph`, pdsview's
   `listRecords`/`getRepo` primitives, bluebird's PDS `getRecord`/`listRecords` transport and
   the social-tree site all want the same three calls plus the same cache. The workspace's
   own record says what happens otherwise (eight OAuth clients). Placement is a
   REPO-GRAMMAR / ARCHITECTURE decision for the owner: a package in the shared chassis
   (`croft-pwa`) or its own repo consumed as a pinned git dependency per the
   dependency-sourcing rule.
2. **TypeScript, because the measured cost is requests, not cycles.** 50 ms to decode a
   14.6 MB CAR; 12.5 s to page the same account's follows. Rust→wasm would add a toolchain
   to forage and pdsview to optimise the free part.
3. **Rev-gated, per repo** (the RUN-BUNDLE-PRECACHE design, generalised to any repo):
   store `{did, pds, rev, follows[], fetchedAt}` in IndexedDB; poll with
   `getLatestCommit`; act only when `rev` moved. v1 acts by re-listing the collection with
   `listRecords` (no CBOR in the bundle). v2 acts by applying `getRepo?since=` diffs against
   a persisted block store, which also yields unfollows without a re-list. **v2 trigger:**
   when a session's measured re-list volume exceeds what its diff volume would have been —
   the appendix script prints both — or when posts join the walker (post churn is 5–10×
   follow churn in the diffs above).
4. **Per-ring cadence as in § 4**, exposed to the UI as a per-ring "as of" stamp. The
   walker owns the schedule; the sites own the meaning.
5. **Per-host concurrency and honest failure.** Cap in-flight requests per PDS host (the
   rate limit is per host), read the exposed `RateLimit-Remaining`, and represent a host's
   failure as "unknown since <time>", never as an empty set.
6. **Followers via Constellation, degraded around,** exactly as ADR-004 does for counts;
   mutuals-among-follows from the ring-2 walk so the core scopes need no index at all.
7. **Jetstream for ring 1 only**, when signed in and under 10 k follows; it replaces the
   ring-1 poll, not ring 2's walk.

What this does *not* recommend: a Rust core for I/O, a relay dependency, a per-session
ring-2 recompute, or trusting any PDS-side answer about *followers*.

## 7. Open questions for the owner

- Placement of the shared package (§ 6.1).
- Whether Constellation is acceptable as a *degraded-around* dependency for followers in
  the social-tree site (it already is for forage counts), or whether followers stay
  AppView-only.
- Whether ring 2's "as of" honesty is enough UX for the visualization, or the site should
  refuse to draw ring 2 until the first walk completes.

## 8. Sources

- PDS rate limits: https://bsky.network/docs/rate-limits/ (3,000 / 5 min per IP; headers observed live)
- `getRepo` lexicon (`since`: "The revision ('rev') of the repo to create a diff from"): https://github.com/bluesky-social/atproto/blob/main/lexicons/com/atproto/sync/getRepo.json
- Jetstream limits (100 collections, 10,000 DIDs per subscription): https://bsky.network/docs/jetstream/
- Constellation: https://constellation.microcosm.blue/ and https://www.microcosm.blue/ (single Pi 5; "ready for production")
- Spacedust readme ("no replay window … cannot emit delete events"): https://github.com/at-microcosm/microcosm-rs/tree/main/spacedust
- PDS vs AppView follow lists differ: https://github.com/bluesky-social/atproto/issues/3508
- atrium wasm client: https://docs.rs/atrium-xrpc-client ; atrium-repo (tokio): https://docs.rs/atrium-repo
- AppView outages 2026 [secondary]: https://statusgator.com/services/bluesky/apipop1bskyapp , https://www.tomsguide.com/news/live/bluesky-down-live-outage-updates-4-16-26
- Corpus: `seeds/wiki-unpacked/RUN-BUNDLE-PRECACHE.md` (E58), `ROADMAP_TODO.md` E62 / E139 / E146, `forage/plans/2026-08-26-4-plan-views-and-the-ring-ladder.md`, `forage/plans/2026-09-03-plan-ring-as-display-scope.md`, `forage/docs/adr/0002-wide-lens-intake.md`, `forage/docs/adr/0004-constellation-backlinks.md`, `.claude/DECISIONS.md` (atproto OAuth ×8).

---

## Appendix A — probe scripts (as run 2026-09-08)

`ring.sh <handle> <N>` — ring 1 by PDS-direct paging, then ring-2 identity resolution and
first pages for N followees:

```bash
#!/bin/bash
H=$1; N=${2:-40}
DID=$(curl -s "https://public.api.bsky.app/xrpc/com.atproto.identity.resolveHandle?handle=$H" | python3 -c 'import json,sys;print(json.load(sys.stdin)["did"])')
PDS=$(curl -s https://plc.directory/$DID | python3 -c 'import json,sys; d=json.load(sys.stdin); print([s["serviceEndpoint"] for s in d["service"] if s["id"]=="#atproto_pds"][0])')
echo "handle=$H did=$DID pds=$PDS"
t0=$(date +%s.%N); cur=""; pages=0; : > follows.txt
while :; do
  r=$(curl -s "$PDS/xrpc/com.atproto.repo.listRecords?repo=$DID&collection=app.bsky.graph.follow&limit=100${cur:+&cursor=$cur}")
  pages=$((pages+1))
  echo "$r" | python3 -c 'import json,sys; d=json.load(sys.stdin); [print(r["value"]["subject"]) for r in d["records"]]' >> follows.txt
  cur=$(echo "$r" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("cursor") or "")')
  [ -z "$cur" ] && break
done
t1=$(date +%s.%N)
echo "ring1: $(wc -l < follows.txt) follows in $pages pages, $(echo "$t1-$t0"|bc)s sequential"
head -$N follows.txt > sample.txt
t2=$(date +%s.%N)
cat sample.txt | xargs -P 10 -I{} sh -c 'curl -s https://plc.directory/{} | python3 -c "import json,sys; d=json.load(sys.stdin); print(\"{}\", [s[\"serviceEndpoint\"] for s in d[\"service\"] if s[\"id\"]==\"#atproto_pds\"][0])"' > sample_pds.txt
t3=$(date +%s.%N)
echo "ring2 resolve: $(wc -l < sample_pds.txt) DID docs in $(echo "$t3-$t2"|bc)s (10 parallel); distinct PDS hosts: $(awk '{print $2}' sample_pds.txt | sort -u | wc -l)"
t4=$(date +%s.%N)
while read d p; do echo "$p/xrpc/com.atproto.repo.listRecords?repo=$d&collection=app.bsky.graph.follow&limit=100"; done < sample_pds.txt | xargs -P 10 -I{} sh -c 'curl -s -w "%{http_code} %{time_total}\n" -o /dev/null "{}"' > r2.txt
t5=$(date +%s.%N)
echo "ring2 first-page listRecords: $(wc -l < r2.txt) requests in $(echo "$t5-$t4"|bc)s (10 parallel); status histogram: $(awk '{print $1}' r2.txt | sort | uniq -c | tr '\n' ' ')"
```

Ring-2 edge volume (research-only probe; it uses the AppView for counts because a count is
what it needs, the walk above is what production would do):

```python
import json, urllib.request, statistics
dids = [l.strip() for l in open('follows.txt') if l.strip()]
counts = []
for i in range(0, len(dids), 25):
    q = '&'.join('actors=' + d for d in dids[i:i+25])
    with urllib.request.urlopen('https://public.api.bsky.app/xrpc/app.bsky.actor.getProfiles?' + q) as r:
        counts += [p.get('followsCount', 0) for p in json.load(r)['profiles']]
print(len(counts), statistics.median(counts), statistics.mean(counts), sorted(counts)[int(.9*len(counts))], max(counts), sum(counts), sum(-(-c//100) for c in counts))
```

Diff-CAR sizes — a TID for "N days ago" is a valid `since`:

```python
import time
B32 = '234567abcdefghijklmnopqrstuvwxyz'
def tid(us):
    v = us << 10; s = ''
    for _ in range(13): s = B32[v & 31] + s; v >>= 5
    return s
now = int(time.time() * 1e6)
for d in (1, 7, 30): print(d, tid(now - d * 86400 * 10**6))
# curl "$PDS/xrpc/com.atproto.sync.getRepo?did=$DID&since=$TID" -o diff.car
```

JavaScript decode benchmark (`@atcute/car` 6.0.2, `@atcute/cbor`; Node 22):

```js
import { readFileSync } from 'node:fs';
import { fromUint8Array } from '@atcute/car';
import { decode } from '@atcute/cbor';
for (const f of process.argv.slice(2)) {
  const buf = new Uint8Array(readFileSync(f));
  const t0 = performance.now();
  let blocks = 0, follows = 0, mst = 0;
  for (const { bytes } of fromUint8Array(buf)) {
    blocks++;
    const v = decode(bytes);
    if (v && typeof v === 'object' && '$type' in v) { if (v.$type === 'app.bsky.graph.follow') follows++; }
    else if (v && typeof v === 'object' && 'e' in v) mst++;
  }
  console.log(`${f}: ${(buf.length/1e6).toFixed(1)}MB blocks=${blocks} mst=${mst} follows=${follows} ${(performance.now()-t0).toFixed(0)}ms`);
}
```

Output: `jay.car: 14.6MB blocks=51033 mst=10674 follows=4007 50ms` — likes 28,500, posts
4,113, follows 4,007, reposts 3,678.
