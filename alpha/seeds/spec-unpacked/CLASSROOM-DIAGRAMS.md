# Classroom seed diagrams (RUN-13 Part 3.2)

Drop each fenced block into its chapter skeleton's DIAGRAM beat, verbatim. These are the two seeds;
every other chapter's DIAGRAM beat stays a placeholder until its prose is drafted in conversation.

## Chapter 01 — Two people in a room: no clock, only who-references-whom

```mermaid
flowchart LR
    A["Alice's device<br/><i>her log is her whole world</i>"]
    B["Bob's device<br/><i>his log is his whole world</i>"]
    A -->|"fact 1: 'hello' — signed by Alice"| B
    B -->|"fact 2: 'hey!' — signed by Bob,<br/><b>references fact 1</b>"| A
```

## Chapter 05 — The split room: the group never notices the boundary

```mermaid
flowchart LR
    subgraph R1["Room 1 — fourteen people, one wifi"]
        a1((Ana)) --- a2((Ben))
        a2 --- a3((Cy))
        a1 --- a3
    end
    subgraph R2["Room 2 — seven people, hotel wifi"]
        b1((Pia)) --- b2((Quinn))
    end
    RLY[/"relay<br/>(coordination only —<br/>reads nothing)"/]
    a3 -.->|outward connection| RLY
    b1 -.->|outward connection| RLY
    a3 ==>|"holepunched direct path<br/>(falls back to relay)"| b1
```

Both blocks validated in Mermaid as drawn (flowchart LR; circles, subgraphs, dotted/thick edges,
HTML line breaks in labels). If the site's chosen renderer version rejects the `<br/>`/`<i>`/`<b>`
label markup, simplify the labels to plain text rather than changing the graph shape, and note the
simplification in the summary.
