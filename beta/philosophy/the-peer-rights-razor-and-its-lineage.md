# The peer-rights razor and its lineage

`Status: philosophy layer (Layer 2, the intellectual history). Register: lineage / grounding — the legal
ancestor and the civic motivation beneath a rule stated elsewhere. Resolution: supplies the *why beneath the
why*; it does not re-derive the razor or the form. External facts carry verification flags; the load-bearing
legal standard is flagged for a primary-source pass, and refuted or unverifiable quotations from the source
material are deliberately not reproduced.`

## Overview

Croft carries a single non-negotiable rule — the *peer-rights razor*: **no participant may remove the rights
of others.** The rule is stated and put to work in the operating structure (the compatibility badge and the
cooperative bylaws are where it gets its teeth). What that statement does not carry, by design, is where the
rule *comes from* — and a rule whose lineage is dropped cannot be re-derived or defended when it is
contested. This document supplies that lineage in two threads.

The first is the **legal ancestor**: a line the U.S. courts and the FCC drew across the middle of the
twentieth century — that a user may attach a device that helps them so long as it does not harm the network —
which is the precise legal shape of Croft's razor a half-century before the razor was written. The second is
the **civic origin**: the 1970s–90s "crypto wars," the fight over whether ordinary people could hold strong
cryptography at all, which is the digital-liberty story the project's motivation descends from.

The two threads meet at one idea. The failure mode both guard against is *rollup* — one party enclosing a
capability that others depend on and then dictating the terms of access. The razor is the anti-rollup rule
stated as a right; the lineage below is where that rule was already being fought for and won, under other
names, in other decades.

## Charter: what this document covers

- **In scope:** the legal ancestor of the razor (the Hush-A-Phone / Carterfone line and Bazelon's standard,
  read through the Ma Bell → Apple parallel), and the civic motivation the project descends from (the
  crypto-wars lineage), told with verified facts only.
- **Out of scope (and where it lives):** the *structural* argument for why a broker of peer relationships
  must be a cooperative (`peer-standing-and-the-cooperative-form.md`); the razor's operational teeth — the
  compatibility badge and the cooperative bylaws (`../governance/foundation-cooperative-and-sustainability.md`).
- **Boundary call:** this is the "where the rule comes from" register. It grounds the razor; it does not
  re-argue it and does not restate the form built on top of it.

## The legal ancestor: a user may help themselves without harming the network

For most of the twentieth century the American Telephone and Telegraph Company (AT&T, "Ma Bell") ran the
telephone network as a closed system. Under its *foreign attachment* tariffs a subscriber did not own the
handset — they leased it — and connecting any device not made by AT&T's own equipment arm, Western Electric,
was grounds for cutting off service. AT&T's justification was the one every closed platform reaches for: that
outside equipment would degrade or endanger the network. `[historical context; the specific wording of the
"network integrity" argument is not pinned to a verified source]`

The line that broke that logic was drawn in **Hush-A-Phone Corp. v. United States**. The Hush-A-Phone was a
purely mechanical cup that clipped over a telephone mouthpiece to muffle the speaker's voice — no electrical
connection to the network at all. AT&T sued to ban it as an unauthorized foreign attachment. In 1956 the U.S.
Court of Appeals for the D.C. Circuit reversed the FCC and struck the tariff down. Writing for the court,
Judge David L. Bazelon set the standard that is the direct legal ancestor of Croft's razor:

> "…an unwarranted interference with the telephone subscriber's right reasonably to use his telephone in ways
> which are privately beneficial without being publicly detrimental."
>
> — Judge David L. Bazelon, *Hush-A-Phone Corp. v. United States*, 238 F.2d 266 (D.C. Cir. 1956)
> `[confirm against primary]`

The move in that sentence is the whole of the razor. It splits "harm" into two categories the platform had
been conflating on purpose. An act that is *privately beneficial* — one that helps the owner and affects only
the owner — the provider has no standing to block. An act that is *publicly detrimental* — one that harms the
network or the other people on it — the provider may legitimately regulate. The right stops exactly where it
would start subtracting from someone else's.

Twelve years later, **Carterfone** carried the principle from mechanical to electrical attachment. The
Carterfone let a two-way radio couple acoustically to the telephone line for remote oil-field crews; AT&T
blocked it; the FCC ruled unanimously against AT&T in 1968, establishing that a subscriber may connect any
lawful device that does not physically harm the network. `[confirm against primary]` That ruling is why the
consumer modem — and therefore the consumer internet — was legally allowed to exist. AT&T's monopoly was
itself later unwound by antitrust consent decree (the 1982 settlement, divestiture in 1984). `[confirm
against primary]`

The reason this lineage is load-bearing rather than decorative: the same argument recurs whenever a network
becomes essential and its owner conflates *private benefit* with *public detriment* to protect revenue. The
modern instance is the mobile platform — a device you buy but do not fully control, where installing an
alternative store or a privacy-respecting client is framed as a security threat to the "ecosystem." The
Ma Bell → Apple parallel is not an analogy Croft invents; it is the through-line of a settled legal
principle. Croft's razor states as a *right of participants* what Bazelon stated as a *limit on a network
owner*: you may do what benefits you; you may not do what removes the rights of others.

## The civic origin: the crypto-wars lineage

The razor answers "what may a participant do." The project's *motivation* — why build a substrate that keeps
capability in ordinary hands at all — descends from an older fight over whether ordinary people could hold
strong cryptography without the state's permission. The following are the load-bearing, verified facts of
that lineage; several widely-circulated *quotations* from the same period do not survive checking and are
deliberately omitted (see the note that closes this section).

- **Diffie–Hellman, 1976.** Whitfield Diffie and Martin Hellman published *New Directions in Cryptography*,
  introducing public-key exchange and breaking what had been an effective state monopoly on the field.
  `[confirmed]`
- **The NSA pressure campaign of the 1970s.** In 1975 the NSA pressed the National Science Foundation to stop
  funding civilian cryptographic research, asserting sole statutory authority over the field; and an NSA
  employee, J.A. Meyer, warned the IEEE (in a real 1977 letter) that presenting cryptographic research to
  audiences including foreign nationals could violate arms-export law. The campaign shifted from intimidation
  toward engagement after Vice Admiral Bobby Ray Inman took over the NSA in late 1977. `[confirmed as events;
  the Meyer letter's exact wording is unverifiable and is not quoted here]`
- **Zimmermann and PGP, 1991.** Phil Zimmermann released Pretty Good Privacy free on the internet in June
  1991 — pushed by Senate Bill 266, which had proposed a government-access ("backdoor") mandate for secure
  communications equipment. Strong cryptography was then classified as "munitions" under export law, and
  Zimmermann became the subject of a criminal investigation (roughly 1993 to January 1996) that ended with
  **no indictment**. `[confirmed]`
- **The MIT-Press export loophole.** To move the code past export restrictions that treated software as a
  weapon but could not touch a printed book, the full PGP C source was published as an OCR-friendly ~600-page
  book by MIT Press (1995): a book may cross borders where a binary may not. This forced the government toward
  a contradiction — is source code a munition, or is it protected speech? `[confirmed]`
- **Bernstein v. United States → "code is speech."** Daniel J. Bernstein (a Berkeley mathematics PhD
  candidate), backed by the Electronic Frontier Foundation, sued the government (filed 1995) over the right to
  publish his "Snuffle" cipher and its paper. A 1999 Ninth Circuit panel held that source code is expression
  protected by the First Amendment. `[partly: the panel win was 2–1 and later vacated for en-banc review; the
  government relaxed the export rules before the point was finally settled — treat "code is speech" as the
  established direction of travel, not a single clean holding]`
- **Barlow and the EFF.** John Perry Barlow — Grateful Dead lyricist and Wyoming rancher — co-founded the
  Electronic Frontier Foundation in 1990 and, in February 1996, issued *A Declaration of the Independence of
  Cyberspace* in response to the Communications Decency Act, framing the early internet as a space outside
  established state sovereignty. `[confirmed]`

Set aside deliberately, so the corpus does not repeat them: a widely-quoted Zimmermann line invoking Stalin,
and a verbatim wording attributed to the Meyer letter, are misattributed or unsourced and are **not**
reproduced here; the underlying events (Zimmermann's 1993 congressional testimony, the real Meyer letter) are
genuine, but the popular quotations are not. Where the record gives only a paraphrase, this document states
the fact and does not dress it as a quotation.

What this thread grounds is not a mechanism but a motivation: the recurring pattern is that a capability
ordinary people rely on gets enclosed by whoever controls the choke point — the network owner, the export
regime, the platform — and the civic response has always been to keep the capability in the participants'
own hands. That is the same anti-rollup instinct the razor encodes, one generation earlier.

## What this establishes (and does not)

**Establishes** that Croft's peer-rights razor — no participant may remove the rights of others — has a
precise legal ancestor in Bazelon's *privately beneficial without being publicly detrimental* standard
(Hush-A-Phone, 1956) and its extension in Carterfone (1968): the courts and the FCC drew exactly the line the
razor draws, decades before it, splitting a user's own benefit (protected) from harm to the network and its
other users (regulable). Establishes that the project's civic motivation descends from a verified crypto-wars
lineage — Diffie–Hellman, the NSA pressure campaign, Zimmermann/PGP and the MIT-Press export loophole,
Bernstein's "code is speech," Barlow and the EFF — the through-line being resistance to *rollup*, the
enclosure of a capability that ordinary people depend on.

**Does not** re-derive the razor or argue the structural case for the cooperative form (that is
`peer-standing-and-the-cooperative-form.md`), and does **not** build the razor's operational teeth — the
compatibility badge and the cooperative bylaws where the rule is actually enforced live in
`../governance/foundation-cooperative-and-sustainability.md`. It supplies the lineage beneath a decision made
elsewhere; it does not reopen the decision. The Bazelon/Carterfone standard carries a `[confirm against
primary]` flag pending a primary-source read; the crypto-wars facts are FACTCHECK-confirmed and flagged
inline, and the refuted or unverifiable quotations from the same material are omitted rather than repeated.
