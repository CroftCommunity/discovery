"""Reference resolver for the Drystone spec site.

Pure, dependency-free logic (stdlib only) so it is unit-testable in isolation
from the HTML rendering. Two responsibilities:

  1. Stable anchors from section numbers.  `heading_id` turns a heading's visible
     text into a (section-key, anchor-id) pair derived from its *number*, never its
     title text, so links survive a title rewording.  `## 7.6.2. Foo` -> `#s7-6-2`;
     `## Appendix B. ...` -> `#appendix-b`.

  2. Reference autolinking + the broken-ref gate.  `autolink_text` turns every
     inline section reference into a link to the resolved anchor, and returns the
     list of references that resolved to no heading (the offenders the build gate
     fails on).  `autolink_html` wraps it so references inside <code>/<pre>/<a>
     are left untouched.

Resolution rules (see RUN-10 Part 1, requirement 3):
  - `Part 1 §N…` / `Part 2 §N…`     -> that document's anchor (cross-doc).
  - `Appendix X`                     -> the appendix anchor (appendices live in Part 2;
                                        resolved in-doc for Part 2, else in Part 2).
  - bare `§N…`                       -> the same document if it has that section,
                                        otherwise the document's declared fallbacks
                                        (companions annotate Part 2, so they fall back
                                        to Part 2 then Part 1).
  - `§N.N Rk`                        -> the R-bullet anchor `#sN-N-rk` if present,
                                        else the section anchor `#sN-N`.
  - a repo-relative `alpha/…` / `beta/…` `.md` path that names a *published* file
                                        -> a relative link; an unpublished path is
                                        left literal (intentionally-not-a-link, not
                                        a broken ref).
"""

import html as _html
import re
from html.parser import HTMLParser


# --------------------------------------------------------------------------- #
#  Anchor id derivation                                                        #
# --------------------------------------------------------------------------- #

_APPENDIX_RE = re.compile(r"^Appendix\s+([A-Z])\b\.?\s*(.*)$")
# A leading structured section token: numeric (7 / 7.6 / 7.6.2), a letter+number
# subsection (A.1 / B.2.1), or a called-out change id (F1). Trailing '.' optional.
_NUMERIC_RE = re.compile(r"^(\d+(?:\.\d+)*)\.?(?:\s|$)")
_LETTER_SUB_RE = re.compile(r"^([A-Z]\.\d+(?:\.\d+)*)\.?(?:\s|$)")


def _slugify(text):
    """Fallback anchor for a prose heading: lowercase, non-alnum -> hyphen."""
    text = _html.unescape(text)
    text = text.lower()
    text = re.sub(r"[^a-z0-9]+", "-", text).strip("-")
    return text or "section"


def _num_to_anchor(num):
    return "s" + num.replace(".", "-")


def heading_id(heading_text):
    """Return (section_key, anchor_id) for a heading's visible text.

    section_key is the normalized reference key ('7.6', 'appendix-b', 'c.4',
    'a.1') or None for a prose heading. anchor_id is the stable DOM id.
    """
    text = heading_text.strip()

    m = _APPENDIX_RE.match(text)
    if m:
        letter = m.group(1).lower()
        return ("appendix-" + letter, "appendix-" + letter)

    m = _NUMERIC_RE.match(text)
    if m:
        num = m.group(1)
        return (num, _num_to_anchor(num))

    m = _LETTER_SUB_RE.match(text)
    if m:
        key = m.group(1).lower()  # 'C.4' -> 'c.4'
        return (key, key.replace(".", "-"))

    return (None, _slugify(text))


# --------------------------------------------------------------------------- #
#  Per-document heading registry                                               #
# --------------------------------------------------------------------------- #

_HEADING_LINE_RE = re.compile(r"^(#{1,6})\s+(.*?)\s*#*\s*$")
_FENCE_RE = re.compile(r"^\s*(```+|~~~+)")
# A bold-leading R-bullet list item: '- **R7, ...' or '* **R7 ...'
_R_BULLET_RE = re.compile(r"^\s*[-*]\s+\*\*R(\d+)\b")
# A markdown ATX section number at the head of the current section, to scope R-bullets.


class Registry:
    """Anchors for one document, keyed by normalized reference key."""

    def __init__(self):
        self._by_key = {}          # section_key -> anchor_id
        self.headings = []         # ordered list of (level, text, section_key, anchor_id)
        self.r_bullets = []        # ordered list of (rnum, anchor_id) for  R-bullets
        self._used_anchors = set()

    def _dedupe(self, anchor):
        if anchor not in self._used_anchors:
            self._used_anchors.add(anchor)
            return anchor
        i = 2
        while f"{anchor}-{i}" in self._used_anchors:
            i += 1
        anchor = f"{anchor}-{i}"
        self._used_anchors.add(anchor)
        return anchor

    def add_heading(self, level, text):
        key, anchor = heading_id(text)
        anchor = self._dedupe(anchor)
        self.headings.append((level, text, key, anchor))
        if key is not None and key not in self._by_key:
            self._by_key[key] = anchor
        return anchor

    def add_key(self, key, anchor):
        self._by_key.setdefault(key, anchor)

    def anchor_for(self, key):
        return self._by_key.get(key)


def parse_headings(md_text):
    """Build a Registry from markdown source.

    Skips headings inside fenced code blocks. Registers R-bullet anchors
    (`§N.N Rk` -> `sN-N-rk`) for bold-leading `- **Rk, ...` list items that
    sit under a numbered section.
    """
    reg = Registry()
    in_fence = False
    fence_marker = None
    current_section_num = None  # the nearest enclosing numeric section key

    for line in md_text.splitlines():
        fence = _FENCE_RE.match(line)
        if fence:
            marker = fence.group(1)[:3]
            if not in_fence:
                in_fence = True
                fence_marker = marker
            elif marker == fence_marker:
                in_fence = False
                fence_marker = None
            continue
        if in_fence:
            continue

        hm = _HEADING_LINE_RE.match(line)
        if hm:
            text = hm.group(2)
            reg.add_heading(len(hm.group(1)), text)
            key, _ = heading_id(text)
            # Track the enclosing numeric section for R-bullet scoping.
            if key is not None and re.match(r"^\d+(?:\.\d+)*$", key):
                current_section_num = key
            continue

        rm = _R_BULLET_RE.match(line)
        if rm and current_section_num is not None:
            rnum = rm.group(1)
            key = f"{current_section_num} r{rnum}"
            anchor = reg._dedupe(f"s{current_section_num.replace('.', '-')}-r{rnum}")
            reg.add_key(key, anchor)
            reg.r_bullets.append((rnum, anchor))

    return reg


# --------------------------------------------------------------------------- #
#  Site context                                                                #
# --------------------------------------------------------------------------- #

class Doc:
    def __init__(self, doc_id, href, md_text, gate="soft", fallbacks=(),
                 external_sections=()):
        self.id = doc_id
        self.href = href
        self.registry = parse_headings(md_text)
        self.gate = gate                 # 'hard' -> unresolved refs fail the build
        self.fallbacks = list(fallbacks)  # doc ids to try for a bare  section
        # Section numbers used in this doc as external (RFC/BCP/MLS) citations,
        # not Drystone sections. A bare  section  in this set is left literal and
        # is NOT a broken-ref gate failure (it points into an RFC, not the spec).
        self.external_sections = set(external_sections)


class Ctx:
    def __init__(self, docs, pathmap):
        self.docs = docs                 # doc_id -> Doc
        self.pathmap = pathmap           # repo-relative source path -> output href


# --------------------------------------------------------------------------- #
#  Reference matching + resolution                                             #
# --------------------------------------------------------------------------- #

# One combined scanner, alternatives tried left-to-right at each position.
_SECTION = r"\d+(?:\.\d+)*"
_REF_RE = re.compile(
    r"(?P<partsec>\bPart\s+(?P<pnum>[12])\s+§(?P<psec>" + _SECTION + r"))"
    r"|(?P<partapp>\bPart\s+2\s+Appendix\s+(?P<pappl>[A-G])(?:\.(?P<pappn>\d+(?:\.\d+)*))?)"
    r"|(?P<appx>\bAppendix\s+(?P<appl>[A-G])(?:\.(?P<appn>\d+(?:\.\d+)*))?)"
    r"|(?P<sec>§(?P<snum>" + _SECTION + r")(?:\s+R(?P<rnum>\d+))?)"
    r"|(?P<path>\b(?:alpha|beta)/[\w./-]+\.md(?:#[\w.-]+)?)"
)


def _href(target_doc_id, current_doc_id, ctx, anchor):
    if target_doc_id == current_doc_id:
        return "#" + anchor
    return ctx.docs[target_doc_id].href + "#" + anchor


def _link(href, text):
    return f'<a href="{href}">{text}</a>'


def _resolve_partsec(m, current, ctx):
    doc_id = "part-1" if m.group("pnum") == "1" else "part-2"
    sec = m.group("psec")
    doc = ctx.docs.get(doc_id)
    anchor = doc.registry.anchor_for(sec) if doc else None
    if anchor:
        return _link(_href(doc_id, current, ctx, anchor), m.group("partsec")), None
    return m.group("partsec"), f'Part {m.group("pnum")} §{sec}'


def _resolve_appendix(letter, num, label, current, ctx):
    key = "appendix-" + letter.lower()
    # Sub-appendix like C.4 is registered under its own letter-sub key 'c.4'.
    sub_key = f"{letter.lower()}.{num}" if num else None
    # Appendices live in Part 2 (also present in-doc when current IS part-2).
    for doc_id in ([current] if current in ctx.docs else []) + ["part-2"]:
        doc = ctx.docs.get(doc_id)
        if not doc:
            continue
        if sub_key and doc.registry.anchor_for(sub_key):
            return _link(_href(doc_id, current, ctx, doc.registry.anchor_for(sub_key)), label), None
        if doc.registry.anchor_for(key):
            return _link(_href(doc_id, current, ctx, doc.registry.anchor_for(key)), label), None
    return label, label


def _resolve_bare_section(m, current, ctx):
    sec = m.group("snum")
    rnum = m.group("rnum")
    label = m.group("sec")
    # Try the current document, then its declared fallbacks.
    order = [current] + (ctx.docs[current].fallbacks if current in ctx.docs else [])
    for doc_id in order:
        doc = ctx.docs.get(doc_id)
        if not doc:
            continue
        if rnum:
            r_anchor = doc.registry.anchor_for(f"{sec} r{rnum}")
            if r_anchor:
                return _link(_href(doc_id, current, ctx, r_anchor), label), None
        anchor = doc.registry.anchor_for(sec)
        if anchor:
            return _link(_href(doc_id, current, ctx, anchor), label), None
    return label, f"§{sec}" + (f" R{rnum}" if rnum else "")


# An RFC / BCP citation immediately before a  section  token (no intervening
#  section ) marks it as an external reference, e.g. "RFC 9420 §8.2",
# "RFC 8126 (BCP 26), §5.2" -> not a Drystone anchor.
_RFC_ADJ_RE = re.compile(r"(?:RFC\s*\d{3,4}|BCP\s*\d{1,3})[^§]{0,40}$")


def _is_external_section(sec, preceding, current, ctx):
    if _RFC_ADJ_RE.search(preceding):
        return True
    doc = ctx.docs.get(current)
    return bool(doc) and sec in doc.external_sections


def _resolve_path(m, ctx):
    raw = m.group("path")
    frag = ""
    path = raw
    if "#" in raw:
        path, frag = raw.split("#", 1)
        frag = "#" + frag
    href = ctx.pathmap.get(path)
    if href:
        return _link(href + frag, raw), None
    return raw, None  # unpublished path: literal, not a broken ref


def _bump(counter, key):
    if counter is not None:
        counter[key] = counter.get(key, 0) + 1


def autolink_text(text, current_doc_id, ctx, counter=None):
    """Linkify one plain-text run (no HTML). Returns (html_out, unresolved).

    If a `counter` dict is passed it is tallied: found / linked / external /
    unresolved (for the build's reference stats)."""
    unresolved = []
    out = []
    pos = 0
    for m in _REF_RE.finditer(text):
        out.append(_html.escape(text[pos:m.start()]))
        _bump(counter, "found")
        external = False
        if m.group("partsec"):
            rep, bad = _resolve_partsec(m, current_doc_id, ctx)
        elif m.group("partapp"):
            label = m.group("partapp")
            rep, bad = _resolve_appendix(m.group("pappl"), m.group("pappn"), label,
                                         current_doc_id, ctx)
            if bad:
                bad = label
        elif m.group("appx"):
            label = m.group("appx")
            rep, bad = _resolve_appendix(m.group("appl"), m.group("appn"), label,
                                         current_doc_id, ctx)
            if bad:
                bad = label
        elif m.group("sec"):
            if _is_external_section(m.group("snum"), text[:m.start()],
                                    current_doc_id, ctx):
                rep, bad, external = m.group("sec"), None, True  # RFC/BCP: literal
            else:
                rep, bad = _resolve_bare_section(m, current_doc_id, ctx)
        else:  # path
            rep, bad = _resolve_path(m, ctx)
            external = bad is None and "<a " not in rep  # unpublished path: literal
        out.append(rep)
        if bad:
            unresolved.append(bad)
            _bump(counter, "unresolved")
        elif external:
            _bump(counter, "external")
        else:
            _bump(counter, "linked")
        pos = m.end()
    out.append(_html.escape(text[pos:]))
    return "".join(out), unresolved


# --------------------------------------------------------------------------- #
#  HTML-aware autolinking (skip code / pre / existing anchors)                 #
# --------------------------------------------------------------------------- #

_SKIP_TAGS = {"code", "pre", "a", "script", "style"}


_SEC_TOKEN_RE = re.compile(r"§\d")


class _AutolinkHTML(HTMLParser):
    def __init__(self, current_doc_id, ctx, counter=None):
        super().__init__(convert_charrefs=False)
        self.current = current_doc_id
        self.ctx = ctx
        self.counter = counter
        self.out = []
        self.skip_depth = 0
        self.unresolved = []

    def handle_starttag(self, tag, attrs):
        if tag in _SKIP_TAGS:
            self.skip_depth += 1
        self.out.append(self.get_starttag_text())

    def handle_startendtag(self, tag, attrs):
        self.out.append(self.get_starttag_text())

    def handle_endtag(self, tag):
        if tag in _SKIP_TAGS and self.skip_depth > 0:
            self.skip_depth -= 1
        self.out.append(f"</{tag}>")

    def handle_data(self, data):
        if self.skip_depth > 0:
            self.out.append(data)
            if self.counter is not None:
                self.counter["skipped_code"] = (
                    self.counter.get("skipped_code", 0) + len(_SEC_TOKEN_RE.findall(data)))
            return
        linked, unresolved = autolink_text(data, self.current, self.ctx, self.counter)
        self.out.append(linked)
        self.unresolved.extend(unresolved)

    def handle_entityref(self, name):
        self.out.append(f"&{name};")

    def handle_charref(self, name):
        self.out.append(f"&#{name};")

    def handle_comment(self, data):
        self.out.append(f"<!--{data}-->")

    def handle_decl(self, decl):
        self.out.append(f"<!{decl}>")


def autolink_html(html_text, current_doc_id, ctx, counter=None):
    """Linkify references in rendered HTML, leaving code/pre/anchors untouched.

    Returns (html_out, unresolved). If a `counter` dict is passed, tallies
    found / linked / external / unresolved / skipped_code reference counts."""
    p = _AutolinkHTML(current_doc_id, ctx, counter)
    p.feed(html_text)
    p.close()
    return "".join(p.out), p.unresolved


# --------------------------------------------------------------------------- #
#  Mermaid diagram blocks (the diagram gate, RUN-13 Part 4)                    #
# --------------------------------------------------------------------------- #

class MermaidError(Exception):
    """A mermaid block failed to render. The build gate treats this as fatal."""


# What python-markdown's fenced_code extension emits for a ```mermaid fence.
# A mermaid example QUOTED inside another fenced block renders as escaped text
# inside the OUTER <code> element (which carries the outer fence's language
# class, never language-mermaid), so it can not match here — that is the
# no-double-processing guarantee.
_MERMAID_BLOCK_RE = re.compile(
    r'<pre><code class="language-mermaid">(?P<src>.*?)</code></pre>',
    re.DOTALL,
)


def substitute_mermaid_blocks(html_text, source_relpath, renderer):
    """Replace each rendered ```mermaid fence with build-time SVG.

    `renderer(source_text) -> svg_markup` receives the fence's UNESCAPED source
    and returns SVG markup; it raises MermaidError on a parse/render failure.
    A failure is re-raised naming the offending source file, so the build gate's
    error identifies where the broken diagram lives.

    Returns (html_out, blocks_rendered).
    """
    count = 0

    def repl(m):
        nonlocal count
        count += 1
        source = _html.unescape(m.group("src"))
        try:
            svg = renderer(source)
        except MermaidError as e:
            raise MermaidError(
                f"{source_relpath}: mermaid block {count} failed to render: {e}"
            ) from e
        return f'<div class="mermaid-figure">{svg}</div>'

    return _MERMAID_BLOCK_RE.sub(repl, html_text), count
