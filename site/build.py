#!/usr/bin/env python3
"""Drystone spec-site builder.

Renders the published Drystone corpus to a static site whose every cross-reference
is a followable link, and fails the build on any broken section reference in the
spec proper (Part 1 / Part 2). See site/README.md for the full contract.

Sources are canonical: this script makes ZERO edits to the markdown. Every anchor
and link is generated here at build time.

Usage:
    python3 site/build.py            # build into site/_site/
    python3 site/build.py --check    # run the broken-ref gate only, write nothing
    python3 site/build.py -o DIR     # build into DIR

Dependencies: markdown==3.7 (pinned in site/requirements.txt). Everything else is
the standard library, including the resolver (site/resolver.py), which is unit
tested in site/test_resolver.py.
"""

import argparse
import html as _html
import os
import re
import sys

import markdown as md_lib

from resolver import Ctx, Doc, parse_headings, autolink_html


REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SPEC_DIR = "beta/drystone-spec"
THINKING_DIR = "alpha/thinking"

# ---- The published set (requirement 5). Nothing outside this is published. ----

# The eight named spec-tier documents, in reading order, with explicit output
# names and the doc-id the resolver keys cross-references on.
SPEC_DOCS = [
    # (source relpath, doc_id, output_name, nav_title)
    (f"{SPEC_DIR}/part-1-reasoning-underpinnings.md", "part-1", "part-1.html",
     "Part 1 — Reasoning Underpinnings"),
    (f"{SPEC_DIR}/part-2-certifiable-design.md", "part-2", "part-2.html",
     "Part 2 — The Certifiable Design"),
    (f"{SPEC_DIR}/EVIDENCE-MAP.md", "evidence-map", "EVIDENCE-MAP.html",
     "Evidence Map (spec ↔ experiment)"),
    (f"{SPEC_DIR}/conventions-and-decisions.md", "conventions-and-decisions",
     "conventions-and-decisions.html", "Conventions & Decisions"),
    (f"{SPEC_DIR}/open-threads.md", "open-threads", "open-threads.html",
     "Open Threads"),
    (f"{SPEC_DIR}/part-2-changelog.md", "part-2-changelog", "part-2-changelog.html",
     "Part 2 — Changelog"),
    (f"{SPEC_DIR}/dag-cbor-and-content-addressing.md", "dag-cbor",
     "dag-cbor-and-content-addressing.html", "DAG-CBOR & Content Addressing (primer)"),
    (f"{SPEC_DIR}/proposed-changes-2026-07-experiment-reconciliation.md",
     "proposed-changes", "proposed-changes.html",
     "Proposed Part 2 Changes (2026-07, historical)"),
]

# Companions annotate Part 2, so their bare  section  refs fall back to Part 2 then Part 1.
SPEC_FALLBACKS = {
    "evidence-map": ["part-2", "part-1"],
    "conventions-and-decisions": ["part-2", "part-1"],
    "open-threads": ["part-2", "part-1"],
    "part-2-changelog": ["part-2", "part-1"],
    "dag-cbor": ["part-2", "part-1"],
    "proposed-changes": ["part-2", "part-1"],
}
# Part 1 and Part 2 are the hard-gated spec: a broken  section  ref in either fails the build.
HARD_GATED = {"part-1", "part-2"}


def thinking_output_name(relpath):
    """Flat, collision-free output name for a thinking doc (path -> dashes)."""
    inner = relpath[len(THINKING_DIR) + 1:]  # strip 'alpha/thinking/'
    stem = inner[:-3] if inner.endswith(".md") else inner
    return "thinking-" + stem.replace("/", "-") + ".html"


def discover_thinking():
    """All markdown under alpha/thinking (the Exploratory tier), sorted."""
    out = []
    base = os.path.join(REPO_ROOT, THINKING_DIR)
    for dirpath, _dirs, files in os.walk(base):
        for fn in sorted(files):
            if fn.endswith(".md"):
                abs_p = os.path.join(dirpath, fn)
                rel = os.path.relpath(abs_p, REPO_ROOT).replace(os.sep, "/")
                out.append(rel)
    out.sort()
    return out


def load(relpath):
    with open(os.path.join(REPO_ROOT, relpath), encoding="utf-8") as fh:
        return fh.read()


# --------------------------------------------------------------------------- #
#  Build model                                                                 #
# --------------------------------------------------------------------------- #

class Page:
    def __init__(self, relpath, doc_id, out_name, title, tier, md_text, fallbacks):
        self.relpath = relpath
        self.doc_id = doc_id
        self.out_name = out_name
        self.title = title
        self.tier = tier              # 'spec' | 'exploratory'
        self.md_text = md_text
        self.fallbacks = fallbacks
        self.registry = parse_headings(md_text)
        self.base_html = None         # rendered + ids injected
        self.unresolved = []


def build_pages():
    pages = []
    for rel, doc_id, out_name, title in SPEC_DOCS:
        pages.append(Page(rel, doc_id, out_name, title, "spec", load(rel),
                          SPEC_FALLBACKS.get(doc_id, [])))
    for rel in discover_thinking():
        out_name = thinking_output_name(rel)
        doc_id = out_name[:-5]  # strip '.html'
        title = rel[len(THINKING_DIR) + 1:]
        pages.append(Page(rel, doc_id, out_name, title, "exploratory", load(rel),
                          ["part-2", "part-1"]))
    return pages


_SEC_TOKEN_RE = re.compile(r"§(\d+(?:\.\d+)*)")
_RFC_BLOCK_RE = re.compile(r"\bRFC\s*\d|\bBCP\s*\d|Verified-RFC")


def _external_sections(md_text, chain_regs):
    """Section numbers cited in an RFC/BCP context block that do NOT resolve via
    this document's own resolution chain (its own headings + its fallbacks) — i.e.
    references into RFC 9420/9750/8446/8126, which also use §. A number that IS a
    Drystone section reachable from this doc is left alone (it resolves normally)."""
    ext = set()
    for block in re.split(r"\n[ \t]*\n", md_text):
        if not _RFC_BLOCK_RE.search(block):
            continue
        for m in _SEC_TOKEN_RE.finditer(block):
            sec = m.group(1)
            if not any(reg.anchor_for(sec) for reg in chain_regs):
                ext.add(sec)
    return ext


def make_ctx(pages):
    docs = {}
    pathmap = {}
    by_id = {p.doc_id: p for p in pages}
    for p in pages:
        gate = "hard" if p.doc_id in HARD_GATED else "soft"
        # Resolution chain for a bare §: this doc, then its fallbacks.
        chain_regs = [p.registry] + [by_id[fb].registry for fb in p.fallbacks if fb in by_id]
        d = Doc.__new__(Doc)           # build Doc without re-parsing the markdown
        d.id = p.doc_id
        d.href = p.out_name
        d.registry = p.registry
        d.gate = gate
        d.fallbacks = p.fallbacks
        d.external_sections = _external_sections(p.md_text, chain_regs)
        docs[p.doc_id] = d
        # Any citation of this source path (repo-root-relative) becomes a link.
        pathmap[p.relpath] = p.out_name
    return Ctx(docs, pathmap)


_H_OPEN_RE = re.compile(r"<h([1-6])>")


def inject_heading_ids(html_text, registry):
    """Assign the section-number-derived id to each heading, in document order.

    markdown emits headings with no id; we replace each opening <hN> tag with the
    next anchor from the registry. If the rendered heading count does not match the
    parsed count (e.g. a stray setext heading), we fail loud rather than mis-anchor.
    """
    anchors = [h[3] for h in registry.headings]  # ordered anchor ids
    rendered = _H_OPEN_RE.findall(html_text)
    if len(rendered) != len(anchors):
        raise SystemExit(
            f"heading count mismatch: rendered {len(rendered)} vs parsed "
            f"{len(anchors)} — anchor alignment unsafe")
    it = iter(anchors)

    def repl(m):
        return f'<h{m.group(1)} id="{next(it)}">'

    return _H_OPEN_RE.sub(repl, html_text)


_R_LI_RE = re.compile(r"<li>(\s*(?:<p>)?\s*<strong>R(\d+))")
_CODE_PATH_RE = re.compile(
    r"(?<!<pre>)<code>(?P<p>(?:alpha|beta)/[\w./-]+\.md)(?P<f>#[\w.-]+)?</code>")


def inject_r_bullet_ids(html_text, registry):
    """Give each §N.N R-bullet list item its stable id (s N - N -r K ), in order,
    so the §N.N Rk links resolve to a real DOM anchor. Fails loud on desync."""
    bullets = registry.r_bullets
    if not bullets:
        return html_text
    found = _R_LI_RE.findall(html_text)
    if len(found) != len(bullets):
        raise SystemExit(
            f"R-bullet count mismatch: rendered {len(found)} vs parsed {len(bullets)}")
    it = iter(bullets)

    def repl(m):
        rnum, anchor = next(it)
        if m.group(2) != rnum:
            raise SystemExit(f"R-bullet order desync at R{m.group(2)} (expected R{rnum})")
        return f'<li id="{anchor}">{m.group(1)}'

    return _R_LI_RE.sub(repl, html_text)


def link_code_paths(html_text, ctx):
    """Turn a backticked repo-path citation to a PUBLISHED file into a link while
    keeping its monospace rendering: <code>path</code> -> <a><code>path</code></a>.
    Fenced code blocks (<pre><code>) are excluded by the lookbehind."""
    def repl(m):
        path, frag = m.group("p"), m.group("f") or ""
        href = ctx.pathmap.get(path)
        if not href:
            return m.group(0)
        return f'<a href="{href}{frag}"><code>{path}{frag}</code></a>'

    return _CODE_PATH_RE.sub(repl, html_text)


def render_markdown(md_text):
    converter = md_lib.Markdown(
        extensions=["fenced_code", "tables", "sane_lists"],
        output_format="html5",
    )
    return converter.convert(md_text)


# --------------------------------------------------------------------------- #
#  Page template                                                               #
# --------------------------------------------------------------------------- #

CSS = """
:root { --fg:#1a1a1a; --bg:#fff; --muted:#666; --rule:#e2e2e2; --link:#0b5cad;
        --code-bg:#f4f4f4; --band-spec:#0b5cad; --band-exp:#8a6d0b; }
@media (prefers-color-scheme: dark) {
  :root { --fg:#e6e6e6; --bg:#141414; --muted:#9a9a9a; --rule:#333; --link:#6db3f2;
          --code-bg:#1e1e1e; --band-spec:#6db3f2; --band-exp:#d9b544; } }
* { box-sizing:border-box; }
body { margin:0; color:var(--fg); background:var(--bg);
       font:16px/1.62 -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Helvetica,Arial,sans-serif; }
.wrap { max-width:52rem; margin:0 auto; padding:1.5rem 1.25rem 6rem; }
nav.top { max-width:52rem; margin:0 auto; padding:0.75rem 1.25rem; font-size:0.86rem;
          color:var(--muted); display:flex; gap:0.75rem; flex-wrap:wrap; align-items:center; }
nav.top a { color:var(--link); text-decoration:none; }
nav.top a:hover { text-decoration:underline; }
.tier { display:inline-block; font-size:0.72rem; text-transform:uppercase; letter-spacing:0.06em;
        border-radius:0.2rem; padding:0.1rem 0.45rem; color:#fff; }
.tier.spec { background:var(--band-spec); }
.tier.exp  { background:var(--band-exp); }
.banner { border-left:4px solid var(--band-exp); background:var(--code-bg);
          padding:0.6rem 0.9rem; margin:0 0 1.5rem; font-size:0.9rem; color:var(--muted); border-radius:0 0.25rem 0.25rem 0; }
h1,h2,h3,h4,h5,h6 { line-height:1.25; scroll-margin-top:1rem; }
h1 { font-size:1.9rem; } h2 { font-size:1.5rem; border-bottom:1px solid var(--rule); padding-bottom:0.25rem; margin-top:2.5rem; }
h3 { font-size:1.2rem; margin-top:2rem; } h4 { font-size:1.03rem; } h5,h6 { font-size:0.95rem; }
h2 a[id], h3 a[id] { text-decoration:none; }
a { color:var(--link); }
code { background:var(--code-bg); padding:0.1em 0.35em; border-radius:0.2rem;
       font:0.86em/1.4 ui-monospace,SFMono-Regular,Menlo,Consolas,monospace; }
pre { background:var(--code-bg); padding:0.9rem 1rem; overflow-x:auto; border-radius:0.35rem; }
pre code { background:none; padding:0; }
blockquote { margin:1rem 0; padding:0.2rem 1rem; border-left:4px solid var(--rule); color:var(--muted); }
table { border-collapse:collapse; display:block; overflow-x:auto; max-width:100%; margin:1rem 0; font-size:0.92rem; }
th,td { border:1px solid var(--rule); padding:0.4rem 0.6rem; text-align:left; vertical-align:top; }
th { background:var(--code-bg); }
hr { border:none; border-top:1px solid var(--rule); margin:2rem 0; }
img { max-width:100%; }
footer { max-width:52rem; margin:0 auto; padding:2rem 1.25rem; color:var(--muted);
         font-size:0.82rem; border-top:1px solid var(--rule); }
"""

EXPLORATORY_BANNER = (
    "Exploratory — design dialogue. This is first-pass, concurrently-discovered "
    "thinking (the <code>alpha/thinking</code> tier), published because the "
    "certifiable design cites it by path. It is not normative; the specification "
    "is Part 1 and Part 2."
)


def page_html(page, nav_links):
    tier_class = "spec" if page.tier == "spec" else "exp"
    tier_label = "Specification" if page.tier == "spec" else "Exploratory"
    nav = " · ".join(nav_links)
    banner = f'<div class="banner">{EXPLORATORY_BANNER}</div>' if page.tier == "exploratory" else ""
    title = _html.escape(page.title)
    return f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title} — Drystone</title>
<style>{CSS}</style>
</head>
<body>
<nav class="top"><a href="index.html">Drystone</a><span class="tier {tier_class}">{tier_label}</span>{nav}</nav>
<main class="wrap">
{banner}{page.body_html}
</main>
<footer>Generated from canonical markdown by <code>site/build.py</code>. Sources are unmodified;
every §-reference is linked at build time and the broken-reference gate guards Part 1 and Part 2.</footer>
</body>
</html>
"""


def index_html(pages):
    spec = [p for p in pages if p.tier == "spec"]
    exp = [p for p in pages if p.tier == "exploratory"]

    def li(p):
        return f'<li><a href="{p.out_name}">{_html.escape(p.title)}</a></li>'

    reading = (
        '<p class="banner" style="border-left-color:var(--band-spec)">'
        '<strong>Reading order:</strong> '
        '<a href="part-1.html">Part 1 — Reasoning Underpinnings</a> → '
        '<a href="part-2.html">Part 2 — The Certifiable Design</a> → '
        '<a href="EVIDENCE-MAP.html">the Evidence Map</a>.</p>'
    )
    spec_items = "\n".join(li(p) for p in spec)
    exp_items = "\n".join(li(p) for p in exp)
    return f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Drystone — specification &amp; design corpus</title>
<style>{CSS}</style>
</head>
<body>
<nav class="top"><a href="index.html">Drystone</a></nav>
<main class="wrap">
<h1>Drystone</h1>
<p>The Drystone specification and its design corpus, published so every cross-reference
is followable — within a document, across Part 1 ↔ Part 2, and into the companions.</p>
{reading}
<h2><span class="tier spec">Specification</span> &nbsp;Published specification set</h2>
<p>The normative design and its immediate companions.</p>
<ul>
{spec_items}
</ul>
<h2><span class="tier exp">Exploratory</span> &nbsp;Design dialogues</h2>
<p>First-pass, concurrently-discovered thinking (the <code>alpha/thinking</code> tier).
Not normative; published because Part 2 cites these notes by path.</p>
<ul>
{exp_items}
</ul>
</main>
<footer>Generated by <code>site/build.py</code> from unmodified markdown sources.</footer>
</body>
</html>
"""


# --------------------------------------------------------------------------- #
#  Driver                                                                      #
# --------------------------------------------------------------------------- #

def run(out_dir, check_only):
    pages = build_pages()
    ctx = make_ctx(pages)

    # Pass 1: render + inject ids.
    total_headings = 0
    for p in pages:
        base = render_markdown(p.md_text)
        base = inject_heading_ids(base, p.registry)
        p.base_html = inject_r_bullet_ids(base, p.registry)
        total_headings += len(p.registry.headings)

    # Pass 2: autolink references against the full corpus registry.
    counter = {}
    code_path_links = 0
    hard_unresolved = {}   # doc_id -> [offenders]
    soft_unresolved = {}
    for p in pages:
        linked, unresolved = autolink_html(p.base_html, p.doc_id, ctx, counter)
        before = linked.count("<a ")
        linked = link_code_paths(linked, ctx)
        code_path_links += linked.count("<a ") - before
        p.body_html = linked
        p.unresolved = unresolved
        if unresolved:
            (hard_unresolved if p.doc_id in HARD_GATED else soft_unresolved)[p.doc_id] = unresolved

    # ----- The broken-ref gate (requirement 4) -----
    n_soft = sum(len(v) for v in soft_unresolved.values())
    n_hard = sum(len(v) for v in hard_unresolved.values())
    print(f"documents built            : {len(pages)}  ({sum(1 for p in pages if p.tier=='spec')} spec, "
          f"{sum(1 for p in pages if p.tier=='exploratory')} exploratory)")
    print(f"headings anchored          : {total_headings}")
    print(f"§-references found          : {counter.get('found', 0)}")
    print(f"  resolved -> links         : {counter.get('linked', 0)}")
    print(f"  external (RFC/BCP) literal : {counter.get('external', 0)}")
    print(f"  skipped in code spans      : {counter.get('skipped_code', 0)}")
    print(f"  unresolved                 : {counter.get('unresolved', 0)}  "
          f"(hard-gated {n_hard}, companion {n_soft})")
    print(f"repo-path citation links   : {code_path_links}")

    if soft_unresolved:
        print("\n-- companion/exploratory unresolved refs (reported, non-fatal) --")
        for doc_id, offenders in sorted(soft_unresolved.items()):
            for off in offenders:
                print(f"   [{doc_id}] {off}")

    if hard_unresolved:
        print("\nBROKEN-REF GATE FAILED — Part 1 / Part 2 references that resolve to no heading:")
        for doc_id, offenders in sorted(hard_unresolved.items()):
            for off in offenders:
                print(f"   [{doc_id}] {off}")
        raise SystemExit(1)

    if check_only:
        print("\ngate OK (check-only; no site written)")
        return

    # ----- Emit the site -----
    os.makedirs(out_dir, exist_ok=True)
    nav_links = [
        '<a href="part-1.html">Part 1</a>',
        '<a href="part-2.html">Part 2</a>',
        '<a href="EVIDENCE-MAP.html">Evidence Map</a>',
    ]
    for p in pages:
        with open(os.path.join(out_dir, p.out_name), "w", encoding="utf-8") as fh:
            fh.write(page_html(p, nav_links))
    with open(os.path.join(out_dir, "index.html"), "w", encoding="utf-8") as fh:
        fh.write(index_html(pages))
    # .nojekyll so GitHub Pages serves files with leading underscores / as-is.
    open(os.path.join(out_dir, ".nojekyll"), "w").close()
    print(f"\nsite written to {out_dir} ({len(pages) + 1} html files)")


def main():
    ap = argparse.ArgumentParser(description="Build the Drystone spec site.")
    ap.add_argument("--check", action="store_true",
                    help="run the broken-ref gate only; write nothing")
    ap.add_argument("-o", "--out", default=os.path.join(os.path.dirname(__file__), "_site"),
                    help="output directory (default: site/_site)")
    args = ap.parse_args()
    run(args.out, args.check)


if __name__ == "__main__":
    main()
