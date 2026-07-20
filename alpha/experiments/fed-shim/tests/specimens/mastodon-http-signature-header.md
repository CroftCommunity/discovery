# Specimen: Mastodon HTTP-Signature header shape

**Date:** 2026-07-20
**Source:** Mastodon `app/lib/request.rb` (`Mastodon::Request#build_signature_header`).
Cross-checked against draft-cavage-http-signatures-12 §4 (Signature Parameters).

## The header value shape (single-line, comma-separated key="value" pairs)

```
Signature: keyId="https://alice.example/users/alice#main-key",algorithm="rsa-sha256",headers="(request-target) host date digest",signature="<base64-of-rsa-pkcs1v15-sha256-signature-over-the-signing-string>"
```

## Key-order

`keyId`, `algorithm`, `headers`, `signature` — in exactly that order in
Mastodon's emit. (draft-cavage §4 does not mandate an order, but
Mastodon's implementation is stable at this order and downstream
consumers may — incorrectly — rely on it. The shim matches the order for
maximum interoperability.)

## Covered-headers list

Mastodon covers exactly:
- `(request-target)` — special pseudo-header, value = `<method-lowercase> <path>`.
- `host` — the receiver's host header.
- `date` — RFC 7231 date, e.g. `Mon, 20 Jul 2026 12:00:00 GMT`.
- `digest` — value = `SHA-256=<base64>` over the raw body bytes.

The signing string is these four values joined by `\n`, in the order
listed in `headers`. No trailing newline.

## Byte-fidelity notes

- No whitespace between `key=` and the opening `"`. No whitespace
  after the closing `"` before the comma. Whitespace INSIDE quoted
  values (e.g. in `date`) is preserved verbatim.
- The `signature` value is standard-base64 (with `+`, `/`, `=` padding)
  of the raw RSA-PKCS1-v1_5-SHA256 signature bytes.
- No BOM, no trailing newline; the header VALUE is a single line.

## Verify direction

Receiving side (which the shim reuses from `ap-ambassador::verify`):

1. Parse the header into `{keyId, algorithm, headers, signature}`.
2. Reconstruct the signing string from the covered headers.
3. Fetch the public key at `keyId` (the shim's KeyResolver stub
   pre-populates this; real Mastodon fetches the actor document).
4. Verify RSA-PKCS1-v1_5-SHA256 over the signing string.
5. Verify the Digest header matches `SHA-256(<raw body>)`.

The shim's SIGN direction produces exactly the same shape by inverting
the above.
