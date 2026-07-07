# Raw transcript: RFC diagram tooling, ASCII <-> SVG, and Mermaid to both, 2026-07-07

`Provenance caveat (PLAYBOOK §4): content-faithful cleaned paste, not a byte-pristine export. A toolchain
note relevant to the shared doc-writing method in impl (how to author spec diagrams that survive as both
plain-text and vector, and stay legible in terminals and AI/text interfaces).`

## Do RFCs have images?

Yes, but format depends on era:

- **Classic era: ASCII art.** RFCs were strictly plain-text ASCII; authors drew packet headers, state
  machines, and topologies character-by-character with `-`, `|`, `+`. Highly accessible, searchable, and
  identical on any terminal.

- **Modern era: HTML + SVG.** RFC 7990 (late 2016) moved the RFC series to an XML source format, published
  as HTML/PDF/EPUB alongside plain text. Authors can embed XML-based vector graphics (SVG) in the HTML/PDF.
  Archival/accessibility rule: any RFC with an SVG must also carry an ASCII-art equivalent or text
  description for the plain-text rendering.

## Is there a natural SVG-to-ASCII conversion?

Mostly no, for a real structural conversion. Most "SVG to ASCII" tools cheat by rasterizing the SVG to
pixels and mapping brightness to character density; that works for a stylized picture but destroys a
technical diagram (crisp lines become muddy character soup). The core difficulty is the monospace grid: an
SVG lives in a continuous coordinate system with infinite precision, while text art is bound to a chunky
fixed grid (typically 1:2 aspect), so snapping arbitrary vectors/diagonals to the grid causes distortion
and aliasing (the algorithm guesses `/` vs `\` vs `.` and often wrong).

The technical-docs world therefore works in reverse: **draft in ASCII, upscale to SVG** (text art is
already grid-aligned, so upscaling is clean and predictable). Tools: `aasvg` / `asciitosvg` (CLI, parse a
text diagram, detect boxes/arrows/lines, emit SVG); interactive editors like Asciiflow (web) or Monodraw
(macOS) that output grid-clean text. IETF Internet-Draft author resources use this path.

## Mermaid to both ASCII art and SVG (the useful part)

This is practical, because Mermaid is a structured text diagram language (`A --> B`), so tools parse the
logical structure rather than guessing at pixels.

- **Mermaid -> SVG (native):** the Mermaid JS engine renders to SVG in the browser; the CLI
  `@mermaid-js/mermaid-cli` (`mmdc`) compiles a `.mmd` to SVG/PNG/PDF. Pure vector paths, scalable, themable.

- **Mermaid -> ASCII (modern revival):** libraries intercept the Mermaid code before rendering, build a
  layout tree, and map nodes/arrows onto a monospace grid. `graph LR\n A[Client] --> B(Server)` renders to
  a clean boxed ASCII/Unicode diagram. Tools named: `mermaid-ascii` (CLI/lib, originally Go; a hosted
  endpoint accepts `curl ... -d mermaid=...`), "Beautiful Mermaid" (parses the Mermaid AST, emits styled
  SVG and structured ASCII simultaneously), `merman-ascii` (Rust crate for deterministic ASCII/Unicode
  layouts for logs/docs/terminals). Some developer CLIs render Mermaid code blocks inline as ASCII in the
  terminal.

Design takeaway for the doc method: author diagrams as Mermaid (or grid-clean ASCII), then generate BOTH a
plain-text/ASCII form (for the spec's text-first, terminal-legible, archival requirement, matching the RFC
discipline) and an SVG (for HTML rendering), rather than hand-maintaining two forms or trying to downscale
an Illustrator/Inkscape SVG into text.
