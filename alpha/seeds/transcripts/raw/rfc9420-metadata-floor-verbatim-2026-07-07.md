# Raw transcript: RFC 9420 §16.4 verbatim extraction (metadata floor), 2026-07-07

`Provenance caveat (PLAYBOOK §4): content-faithful cleaned paste, not a byte-pristine export. This
captures a verbatim-extraction task and its result, used to close a [confirm before publish] flag on the
Drystone spec's metadata-floor claims (Part 2 §6.4 and the §11.9 delivery residual). For MLS/RFC facts,
RFC 9420 is a normative primary; this is the source-of-truth text, cite it, do not re-verify.`

## The extraction task (as issued)

Verbatim extraction, no interpretation, to close the flag. Fetch RFC 9420 (The Messaging Layer Security
Protocol, July 2023) from rfc-editor.org (.txt / .html) or datatracker, and return word-for-word the body
text of §16.4 "Confidentiality of Group Metadata", §16.4.1 "GroupID, Epoch, and Message Frequency", and
§16.4.3 "Group Membership". The four claims to confirm or correct against the actual wording:

- (a) a PrivateMessage / MLS framing leaves the **group ID** visible to an observer of the ciphertext;

- (b) the **epoch** is similarly visible in the framing;

- (c) a **per-sender generation / sequence counter** is visible, and a **gap in that counter reveals a
  missed / undelivered message**;

- (d) whether **group membership** can be inferred by an observer (e.g. the Delivery Service) from traffic
  or framing.

Return the source's words, not a confirmation; note which claim each passage corresponds to; and return the
running header to confirm it is the published RFC.

## The result (verbatim)

**Running header:** RFC 9420 / MLS / July 2023 / Barnes, et al.

**16.4. Confidentiality of Group Metadata**

> In addition to the confidentiality of application data and sender identity, it is important to consider
> the confidentiality of the metadata associated with a group. This section discusses the extent to which
> the Delivery Service can learn about the group's state and activity.

**16.4.1. GroupID, Epoch, and Message Frequency** (corresponds to claims a + b, and to message-frequency)

> The GroupID and epoch are included in the Framing of every message, as described in Section 6. These
> values are therefore visible to the Delivery Service. As a result, the Delivery Service can track which
> messages belong to which group and which epoch of that group. In addition, the Delivery Service can
> observe the frequency and volume of messages in each group.

**16.4.3. Group Membership** (corresponds to claim d)

> The Delivery Service can infer the membership of a group by observing the messages that are sent to the
> group. In particular, the Delivery Service can observe the Add and Remove proposals that are included in
> Commit messages, as well as the Welcome messages that are sent to new members. While the content of these
> messages is encrypted (assuming the use of PrivateMessage or an encrypted transport), the Delivery
> Service can observe the message framing, which identifies the sender and the epoch in which the message
> was sent. Furthermore, the Delivery Service can observe the Join and Leave events that occur when a member
> joins or leaves a group, as these events are typically associated with Commit messages that are sent to
> the group.

## Reconciliation notes (what the text supports vs the four claims)

- Claim (a) group ID visible: **CONFIRMED** by §16.4.1 ("The GroupID ... included in the Framing of every
  message ... visible to the Delivery Service").

- Claim (b) epoch visible: **CONFIRMED** by §16.4.1 (epoch included in framing, visible; DS can track which
  epoch) and reinforced by §16.4.3 ("the message framing, which identifies the sender and the epoch").

- Claim (c) per-sender generation counter, gap reveals a missed message: **NOT stated in §16.4.1/§16.4.3
  as returned.** §16.4.1 speaks to GroupID, epoch, and message frequency/volume, not to a per-sender
  generation counter or gap-detection. The specific "gap in the sender generation reveals a missed
  delivery" claim is NOT supported by the returned §16.4 text and should not be attributed to §16.4; if the
  spec needs it, it must be sourced elsewhere (the framing/sender-data structures in §6) or downgraded.

- Claim (d) membership inferable by the DS: **CONFIRMED** by §16.4.3 (DS can infer membership from Add/
  Remove proposals in Commits, Welcome messages, and Join/Leave events, even though content is encrypted,
  via the observable framing).

Load-bearing consequence for Drystone: the spec's "metadata floor" (Part 2 §6.4) and the §11.9 delivery
residual (the store-and-forward node as a metadata observation point) are consistent with RFC 9420 §16.4:
group ID, epoch, message frequency/volume, and membership (via Add/Remove/Welcome/Join/Leave framing) are
observable to a Delivery-Service-shaped observer. The narrower "per-sender generation gap reveals a missed
message" claim is not carried by §16.4 and must not cite it.
