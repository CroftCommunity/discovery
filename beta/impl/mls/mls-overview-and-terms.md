# MLS: Concepts, Terms, and Architecture

`Status: working reference`

`Scope: RFC 9420 (protocol) and RFC 9750 (architecture), read for Drystone's purposes`

`Purpose: fix the concrete meaning of the terms we kept tripping on, then ground them in a narrative`

This document is the shared vocabulary. It exists because MLS prose uses "client," "leaf," and "device" loosely, and a wrong mental model here is expensive to unlearn later. Every term below is pinned to its normative definition and then placed in an Alice-and-Bob narrative so the relationships are concrete rather than abstract.

Sources are the two RFCs directly (RFC 9420, the protocol; RFC 9750, the architecture). Section numbers are given so each claim can be checked against the primary text. Nothing here rests on secondary analysis.

---

## 1. What MLS is, in one paragraph

MLS is a group key agreement protocol, not a messaging application. Its single job is to get a group of participants to agree on a shared secret key, and to keep that agreement secure as members join and leave. Everything a messaging app also needs (accounts, message delivery, storage, user interface) sits around MLS and is left to the application. The RFC is explicit that MLS is meant to be embedded in a concrete protocol, not used as one. The output of MLS is a shared group secret; secure messaging is just one thing you can build on that secret.

Two properties are the reason MLS exists rather than simpler schemes:

- It scales. Re-keying the whole group on a membership change costs on the order of log(N) work, not N, so groups can reach tens or hundreds of thousands of members.

- It heals. It provides forward secrecy (past messages stay safe if you are compromised now) and post-compromise security (future messages become safe again after you update, even if you were compromised).

---

## 2. The terms, pinned

This is the section to return to. Each term gets its concrete definition, then the distinction from the term it is most often confused with.

### 2.1. Client

A client is the atomic unit of MLS. It is not the user and not the device. It is a set of cryptographic objects: an identity, a public encryption key, and a public signature key, with the matching secret keys held by whoever runs the client.

RFC 9420 §2 defines it directly: a client is an agent that uses the protocol to establish shared cryptographic state with other clients, and a client is defined by the cryptographic keys it holds. RFC 9750 §3.7 adds that the basic unit of operation is not the user but the client. Software runs a client, but MLS defines the client by its keys, not by the software or the hardware.

### 2.2. Member

A member is a role, not a separate object. Per RFC 9420 §2, a member is a client that is included in the shared state of a group and hence has access to the group's secrets. The same client is "a client" in the abstract and "a member" when viewed relative to a specific group. One client that belongs to three groups is a member three times over, once per group.

### 2.3. Group

A group is defined by shared secret knowledge, not by a roster. RFC 9420 §2: a group is a logical collection of clients that share a common secret value at any given time. Membership is a cryptographic fact. RFC 9750 adds that until a client has been added and has contributed to the group secret in a verifiable way, the other members cannot assume it is a member. There is no separate "list of members" that constitutes the group; the group is constituted by who holds the secret.

### 2.4. Epoch

An epoch is a version of the group between two changes. RFC 9420 §2 fixes both the term and its linearity: an epoch is a state of a group in which a specific set of authenticated clients hold shared cryptographic state, and a group's state is a linear sequence of epochs in which each epoch depends on its predecessor. Every membership change or key update advances the group from one epoch to the next. This linearity is built into the cryptography (the transcript hash chain, §8.2), and it is the single most important fact to carry into any decentralized design.

### 2.5. Leaf, and LeafNode

These are two different things, and conflating them is the most common error.

The leaf is a position. Members sit at the leaves of the group's ratchet tree (Section 3). "Leaf" names the slot in a specific group's tree. (RFC 9420 §4.1.)

The LeafNode is the object that sits at that position. Per RFC 9420 §7.2, a LeafNode describes all the details of an individual client's appearance in the group, signed by that client. It holds the client's encryption key, signature key, credential, capabilities, and a field recording how it came to be in the tree (the leaf_node_source).

So the relationship is: leaf is where you stand, LeafNode is the signed statement of how you appear while standing there, and that statement contains keys. A leaf is not "a key." The distinction matters because a LeafNode is group-scoped and re-signed as the tree changes, whereas the underlying client identity is stable.

### 2.6. Device (the term MLS deliberately avoids)

"Device" is not a protocol-level MLS concept. The protocol layer knows only client, member, leaf, LeafNode, group, and tree. "Device" appears only at the application layer, and the RFC uses it loosely and by example (a user with a mobile and a desktop app is described as having multiple clients).

The layering that resolves the leaf/client/device confusion:

- Protocol layer (normative): client, member, leaf, LeafNode, group, tree.

- Application layer (out of scope for the RFC): users, devices, and the mapping of clients onto them.

MLS fixes exactly one cardinality: a client occupies one leaf per group it is in. Everything above the client (how clients map to devices, how devices map to users) is the application's decision. This is why RFC 9750 has a section titled "Support for Multiple Devices": multi-device is an application concern the protocol accommodates but does not define.

### 2.7. Identity, keys, and credential (three things, not one)

It is tempting to treat identity and keys as the same thing. MLS separates them deliberately, and the separation is what enables key rotation and multi-device.

- A signature key pair is pure cryptographic material. It can produce verifiable signatures and nothing else.

- An identity is an application-meaningful name (an email address, a username, a phone number). It is not cryptographic.

- A credential is the object that binds an identity to a signature key, vouched for by the Authentication Service. Per RFC 9420 §7.2 and §5.3, the LeafNode's signature_key field holds the public signing key and the credential field authenticates both the member's identity and that signing key. The fact that the credential's job is to bind the two together is the proof that they are distinct things.

The consequence that matters: the same identity can be bound to different keys (multi-device, key rotation), and the same key is never the identity itself. The credential is the hinge that lets one change while the other holds.

### 2.8. The message verbs

Defined in RFC 9420 §12 (proposals and commits) and §10 (KeyPackage); Welcome framing in §12.4.3.

- Proposal: describes a change for the next epoch (add a member, remove a member, update a key). It does not enact anything. (§12.1.)

- Commit: enacts a set of proposals and begins a new epoch. This is what actually changes the group secret. (§12.4.)

- KeyPackage: a pre-published advertisement that a client is available to be added. It contains a LeafNode plus an init key. Others fetch it to add that client. (§10.)

- Welcome: sent only to a newly added member, encrypted to them, carrying the secret state they need to join the epoch they were added in. (§12.4.3.)

One structural asymmetry to remember: proposal and commit can be sent by different members. A member can propose their own removal, but cannot commit it; someone else must commit. There is no unilateral "leave." A member is removed by a remaining member.

### 2.9. Linking one group to another: ReInit, branching, and the resumption PSK

A group's epochs are a single linear chain, and groups are normally independent of one another. But MLS provides a way to cryptographically link a new group to an old one, which matters because it is the native shape of operations Drystone leans on heavily.

The link is the resumption PSK. Each epoch derives a resumption pre-shared key, and injecting it into a new group carries one specific guarantee: members entering the new group agree on a key if and only if they were members during the epoch the resumption key came from. In plain terms, the PSK proves co-membership across the boundary, and it does so irrespective of any key changes those members made in between. It proves entitlement, not content.

MLS defines two operations that use this link:

- ReInit closes one group and creates a new group with the same members but different parameters (for example a new protocol version or cipher suite). The old group is closed; the new one continues with the same membership, linked by resumption PSK.

- Branching starts a new group with a subset of the original's participants, leaving the original untouched. The original group continues to exist; the branch is a new group carrying the linked subset.

These two are worth naming because they are the MLS-native forms of Drystone's re-plant family. ReInit is the shape of a same-membership re-key (close and re-form over the same people). Branching is the shape of a legitimate fork (a subset splits off, the original survives). Drystone generalizes these, but the primitives already exist rather than being bespoke. A hazard specific to ReInit (the close and the re-form are not one atomic step) is treated in the hard-cases companion, not here.

---

## 3. The ratchet tree, concretely

The tree is the machine that makes re-keying cost log(N) instead of N. Concepts are in RFC 9420 §4 (terminology) and the operations in §7 (parent and leaf contents, evolution, update paths, tree and parent hashes); the secret tree that derives per-message keys is §9.

### 3.1. Structure and the one invariant

Members are the leaves of a left-balanced binary tree. Every node, leaf and internal alike, holds a key pair. The one rule that everything else follows from:

> A member knows a node's secret key if and only if that member's leaf sits in the node's subtree.

In plain terms: you know a node's secret if and only if you are somewhere underneath it. From this, two pieces of vocabulary:

- Direct path: the chain of nodes from your leaf up to the root. These are the nodes whose secrets you know.

- Copath: the siblings of each node on your direct path. These are the sibling subtrees whose secrets you do not know.

The root secret is known to every member (every leaf is under the root) and to nobody outside the group. That shared root secret seeds the group key. The whole tree exists to distribute a fresh root secret to all N members cheaply.

### 3.2. Why re-keying is log(N)

When a member commits a change, they refresh the secrets along their direct path (an UpdatePath), then encrypt each new secret to the copath sibling subtree. Because one encryption aimed at an internal node reaches everyone beneath it, the number of encryptions equals the height of the tree, which is log(N).

The mechanical link between the tree and the epoch's key is in the key schedule (RFC 9420 §8). A commit uses the updated ratchet tree to distribute fresh entropy, and per §12 that new entropy is added to the epoch secret for the new epoch so that it is not known to any removed member. The root of the tree feeds the commit secret, which the key schedule (§8) folds together with the prior epoch's init secret to produce the new epoch secret. So re-keying the tree and advancing the epoch key are the same act: refresh the tree, and the new root drives the new epoch's key.

### 3.3. Blank nodes and unmerged leaves (where log(N) degrades)

Two conditions break the "one encryption covers a whole subtree" property, and both push cost back toward N.

- A blank node is a keyless hole, created mainly by removal. When a member is removed, their leaf and the nodes above it are blanked, which is exactly what strips the departed member's key knowledge out of the tree. A blank node has no key to encrypt to, so the sender must instead encrypt to the resolution: the set of non-blank descendants that cover the blank.

- An unmerged leaf is a member sitting under a non-blank node who does not yet know that node's secret. This is the normal state of a freshly added member: they are slotted in, but they were not present when the ancestor secrets were derived. The tree records each node's unmerged leaves explicitly. To reach them, the sender must encrypt separately to each unmerged leaf in addition to the node.

Both heal only when members commit an UpdatePath through the affected nodes. So tree efficiency is a function of recent commit activity, not a fixed guarantee. A group that churns membership without members re-committing accumulates blanks and unmerged leaves and drifts toward O(N) re-keying. In a coordinator-free deployment, nobody owns the job of keeping the tree tight, which makes this a live design concern rather than an edge case.

---

## 4. The architecture: two abstract services

MLS relies on exactly two external services, and treats both as abstract (they can be a cloud provider, a peer-to-peer network, or even manual human action).

### 4.1. Authentication Service (AS)

The AS is the identity authority. It issues credentials binding identities to signature keys, lets a client verify another client's credential against a reference identity, and lets a member verify that two credentials represent the same client. The AS is highly trusted: its compromise would let an adversary impersonate members.

### 4.2. Delivery Service (DS)

The DS does two jobs: it acts as a directory that stores and serves KeyPackages (so you can add an offline client), and it routes MLS messages among clients.

The trust asymmetry between the two services is the central architectural fact:

> MLS confidentiality depends on the AS behaving correctly. It does not depend on the DS behaving correctly. Even a malicious DS cannot add itself to a group or recover the group key.

The DS is untrusted for confidentiality. But it is not harmless: depending on deployment, the DS may learn group membership or block group-change messages. So the DS is a metadata and availability adversary, not a confidentiality one. For threat modeling, that is the line to draw.

### 4.3. The ordering problem the DS quietly solves

Because epochs are a linear chain, the group must agree on which single commit ends each epoch. If two members commit against the same epoch simultaneously, only one can be epoch N+1. RFC 9750 frames this through the CAP theorem and offers two DS designs:

- A strongly consistent DS acts as an ordering server: it imposes a single global order, and clients apply the first valid commit for an epoch and ignore later ones. Most deployments use this.

- An eventually consistent DS (the peer-to-peer case) has no referee. Clients reconcile collisions themselves, using a deterministic tie-break, and must delete stale forked state promptly to preserve forward secrecy.

This is the seam that matters most for any center-free design: the strongly consistent DS is a coordinator, and removing it means inheriting the ordering obligation explicitly.

---

## 5. Alice and Bob: the narrative

This section walks the concepts above through actors, so the relationships are concrete.

### 5.1. Setup

Alice wants to talk securely with Bob. Alice is a person. She runs the app on two devices: a phone and a laptop. Each app instance generates its own keys, so Alice has two clients: Alice-phone and Alice-laptop. Both present a credential for the same identity ("alice"), issued by the Authentication Service, but each credential binds that shared identity to that client's own distinct signature key. Same person, same identity, two clients, two key pairs.

Bob, for simplicity, runs one client on one device: Bob-phone.

### 5.2. Advertising availability

Before any group exists, each client publishes a KeyPackage to the Delivery Service. A KeyPackage says "here is a client you can add, here are its keys." Alice-phone, Alice-laptop, and Bob-phone each upload at least one. These are refreshed regularly so a fresh key is used each time and old keys can be deleted, which is what gives forward secrecy at join time.

### 5.3. Creating the group

Alice-phone creates a group. Creation is unilateral and needs no interaction: Alice-phone simply initializes epoch 0 with only itself as a member. This is the one MLS operation that is free and local.

Now Alice-phone adds Bob. It fetches Bob-phone's KeyPackage from the DS and produces two messages: a Commit that adds Bob (advancing the group to a new epoch), and a Welcome addressed only to Bob-phone, encrypted with Bob-phone's init key from the KeyPackage. The Welcome carries the secret state Bob-phone needs to join. Even if the DS misroutes the Welcome, secrecy holds, because it is encrypted for Bob-phone alone.

Bob-phone processes the Welcome and is now a member. The group has a shared secret known to Alice-phone and Bob-phone and nobody else.

### 5.4. Alice's second device

Alice wants her laptop in the group too. From MLS's point of view, Alice-laptop is a separate client, so it is added exactly like any other member: Alice-phone fetches Alice-laptop's KeyPackage and commits an Add. The group's tree now has three leaves: Alice-phone, Alice-laptop, Bob-phone. There are two leaves for "Alice" because there are two clients. The application knows these two clients are one user; MLS does not and does not need to.

This is the concrete meaning of "a user may have multiple clients with the same identity and different keys." Same identity ("alice") in both credentials, different signature keys per client, one leaf per client per group. The "same identity, different keys" fact lives on the user-to-client axis. The "one leaf per group" fact lives on the client-to-group axis. They never collide.

One thing MLS does not do here: the laptop-client gets no back-history. A new device is a new client to the protocol, and it gains no access to messages sent before it joined, even though the same person owns another member. Any "see your old messages on your new laptop" experience is application-layer history sync, not MLS. For Drystone this aligns cleanly: the mechanism that populates the new device is out-of-band history convergence, the same dataplane hash-range exchange used everywhere else. The new device is just a client with no history that converges the dataplane. Note the boundary precisely: the laptop's entitlement to the group was established when it joined as a client (governance layer), and convergence only backfills content (dataplane layer); the two do not mix.

### 5.5. Sending a message

When Bob-phone sends a message, it is encrypted under keys derived from the current epoch's group secret, which was seeded by the root of the ratchet tree. Every current member can derive the same secret and decrypt. After decrypting, each client deletes the per-message key, on the schedule RFC 9420 §9.2 requires. That deletion is what makes the message forward-secret: a later compromise of Bob-phone cannot recover the key to that message, because the key is gone. The ciphertext may live forever; the key does not.

### 5.6. Bob's phone is compromised, then heals

Suppose an attacker briefly compromises Bob-phone and steals its current keys. Post-compromise security says the group can recover. Bob-phone sends an Update (or a Commit), which refreshes the secrets along Bob-phone's direct path in the tree. Once the other members process it, the group secret is one the attacker never saw. Bob-phone is healed within this group.

The sharp caveat: this healing is per-group. If Bob-phone is also in four other groups, updating this one heals only this one. Each group must be updated separately. In a center-free mesh with no coordinator to prompt updates, ensuring members actually update across all their groups is an explicit design obligation.

### 5.7. Alice leaves her laptop behind

Alice stops using her laptop. Alice-laptop goes offline and its keys are never refreshed. This is the persistently-offline-member hazard: stale key material sitting in the tree that, if later compromised, threatens the group's forward and post-compromise security. RFC 9420 §16.6 says updates should be sent at regular intervals and non-updating members should eventually be removed, but "eventually" and "regular" have no enforcer without a coordinator. Deciding when and how to prune Alice-laptop is a governance question, not a protocol one.

### 5.8. Bob loses his state and recovers

Bob's phone is wiped and he loses his local MLS state entirely. MLS's recovery path (RFC 9750 §6.6) is to rejoin as a new member and remove the member representing his lost state, and the application can require him to prove prior membership with a resumption PSK exported from the earlier state. Two things are worth separating precisely here, because they are two independent exchanges carrying two different things.

First, entitlement. Bob proves he was a member using the resumption PSK. This is a governance-layer fact (was Bob a member at epoch n), and it is all the PSK proves.

Second, content. MLS explicitly does not restore the messages Bob missed during the loss window; rejoining proves prior membership but recovers no content. Bob's missed messages come back through out-of-band history convergence, the dataplane hash-range exchange, which carries no entitlement claim at all, only "here are the content hashes I hold, tell me what I am missing."

So recovery is two separate motions: re-prove entitlement (PSK, governance) and backfill content (convergence, dataplane). They do not touch. This is the same convergence that populates a new device (Section 5.4); the difference is that a recovering member must also re-prove entitlement, whereas a new device established its entitlement fresh when it joined.

---

## 6. The one-paragraph summary

A client is a bundle of keys, the atomic actor. A member is a client inside a group. A group is the set of clients holding a shared secret, which rotates every epoch along a strictly linear chain. Each member occupies one leaf per group; the LeafNode at that leaf is a signed, group-scoped object holding the member's keys, distinct from the stable client identity underneath. Identity, keys, and credential are three separate things, bound together by the Authentication Service so that one can change while another holds. The ratchet tree distributes a fresh root secret to all members in log(N) work, degrading toward N as blanks and unmerged leaves accumulate. Two abstract services surround the protocol: the AS, highly trusted for identity, and the DS, untrusted for confidentiality but able to attack metadata and availability. The DS's hidden but essential job is imposing a single order on the linear epoch chain, which is exactly the job a center-free design must replace.
