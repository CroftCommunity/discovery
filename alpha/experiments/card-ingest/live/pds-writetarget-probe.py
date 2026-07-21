#!/usr/bin/env python3
"""Live WriteTarget probe: validate the content-blind ingest's storage leg against a REAL atproto PDS.

This is the live adapter behind the `WriteTarget` port that the hermetic spike (../card-service) fakes
in memory. It creates, reads back, and deletes one opaque encrypted-contribution record in a dedicated
test collection, proving the storage/transport leg is real and that the PDS holds only ciphertext.

Credentials come from the environment; NONE are stored in this file or its output:

    ATP_IDENTIFIER=<handle-or-email>  ATP_PASSWORD=<app-password-preferred>  python3 pds-writetarget-probe.py

Optional: ATP_ENTRY (default https://bsky.social), ATP_COLLECTION (default ing.croft.cardtest.entry).

Grade note (honest): this validates the STORAGE leg on real infra (create/read/delete of an opaque
contribution; the PDS never receives a key). It does NOT validate (a) the ChaCha20-Poly1305 crypto,
which is proven in the hermetic Rust spike with a compile-time content-blind boundary; nor (b) the
OAuth + DPoP per-collection scoped-delegation path (this uses the legacy app-password/createSession
Bearer flow, acting AS the account, not a mediating service holding a delegated `repo:<NSID>` scope).
Those remain modeled / spec-verified (see the design note).
"""
import base64
import datetime
import json
import os
import sys
import urllib.error
import urllib.parse
import urllib.request

ENTRY = os.environ.get("ATP_ENTRY", "https://bsky.social")
COLL = os.environ.get("ATP_COLLECTION", "ing.croft.cardtest.entry")
IDENT = os.environ.get("ATP_IDENTIFIER")
PW = os.environ.get("ATP_PASSWORD")
if not (IDENT and PW):
    sys.exit("set ATP_IDENTIFIER and ATP_PASSWORD in the environment (app password preferred)")


def call(base, method, body=None, q=None, http="POST", tok=None):
    url = f"{base}/xrpc/{method}"
    if q:
        url += "?" + urllib.parse.urlencode(q)
    headers = {"Content-Type": "application/json"}
    if tok:
        headers["Authorization"] = f"Bearer {tok}"
    data = json.dumps(body).encode() if body is not None else None
    req = urllib.request.Request(url, data=data, method=http, headers=headers)
    try:
        with urllib.request.urlopen(req, timeout=25) as r:
            raw = r.read().decode()
            return r.status, (json.loads(raw) if raw.strip() else {})
    except urllib.error.HTTPError as e:
        raw = e.read().decode()
        return e.code, (json.loads(raw) if raw.strip() else {})


def main():
    st, sess = call(ENTRY, "com.atproto.server.createSession", {"identifier": IDENT, "password": PW})
    if st != 200:
        sys.exit(f"login failed: {sess.get('error')} {sess.get('message')}")
    tok, refresh, did = sess["accessJwt"], sess["refreshJwt"], sess["did"]
    pds = ENTRY
    for s in (sess.get("didDoc") or {}).get("service", []):
        if "atproto_pds" in str(s.get("id", "")):
            pds = s.get("serviceEndpoint", ENTRY)
    print(f"logged in: {sess.get('handle')}  pds={pds}")

    ok = True
    try:
        # opaque encrypted contribution: nonce || ciphertext, base64. no key is ever sent.
        env_b64 = base64.b64encode(os.urandom(12) + os.urandom(48)).decode()
        now = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
        rec = {"$type": COLL, "ciphertext": env_b64, "createdAt": now}
        st, res = call(pds, "com.atproto.repo.createRecord",
                       {"repo": did, "collection": COLL, "record": rec}, tok=tok)
        print("create :", st, res.get("uri"))
        ok &= st == 200
        rkey = res["uri"].rsplit("/", 1)[-1]

        st, res = call(pds, "com.atproto.repo.getRecord",
                       q={"repo": did, "collection": COLL, "rkey": rkey}, http="GET", tok=tok)
        val = res.get("value", {})
        rt = val.get("ciphertext") == env_b64
        blind = "key" not in val and "plaintext" not in val
        print("read   :", st, "round-trips:", rt, "content-blind:", blind, "fields:", sorted(val))
        ok &= st == 200 and rt and blind

        st, _ = call(pds, "com.atproto.repo.deleteRecord",
                     {"repo": did, "collection": COLL, "rkey": rkey}, tok=tok)
        st2, _ = call(pds, "com.atproto.repo.getRecord",
                      q={"repo": did, "collection": COLL, "rkey": rkey}, http="GET", tok=tok)
        print("delete :", st, "gone-after:", st2 != 200)
        ok &= st == 200 and st2 != 200
    finally:
        # revoke the session so no live token outlives the probe.
        call(ENTRY, "com.atproto.server.deleteSession", http="POST", tok=refresh)
        print("session revoked")

    print("\nLIVE WRITE-TARGET LEG:", "PASS" if ok else "FAIL")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
