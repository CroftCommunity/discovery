"""Unit tests for the Drystone spec-site reference resolver (TDD).

Run: python3 -m unittest -v site.test_resolver   (from repo root)
  or: python3 -m unittest -v test_resolver         (from site/)

These tests are the contract the resolver in resolver.py must satisfy. They are
the "resolver red -> green" evidence RUN-10 Part 1 asks for. They cover, per the
run's TDD requirement:
  - a doc with in-doc refs (bare  section  -> same-doc anchor)
  - a cross-doc ref (Part 1 / Part 2 -> the other document's anchor)
  - a ref to a MISSING section (must be reported unresolved, offender named)
  - an appendix ref (Appendix B -> #appendix-b)
  - a ref inside a code span (must NOT be linkified)
  - the R-bullet anchor ( 7.2 R7 -> #s7-2-r7 when present, else the  7.2 target)
  - a repo-path citation to a published vs an unpublished companion
"""

import unittest

from resolver import (
    heading_id,
    parse_headings,
    Doc,
    Ctx,
    autolink_text,
    autolink_html,
)


# A tiny two-document corpus reused across tests.
PART1_MD = """# Drystone, Part 1

## 2. Design Principles

### 2.5. The forced terminus

Body text.
"""

PART2_MD = """# Drystone, Part 2

## 5. Identity

### 5.2. Principal, client, persona

See earlier discussion.

## 7. The realization

### 7.2. The grant-and-revocation interface

- **R1, Unforgeable grant.** ...
- **R7, Content-bound quorum.** ...

### 7.6. Human adjudication

## Appendix B. Open Questions

Body.
"""


def make_ctx():
    part1 = Doc("part-1", "part-1.html", PART1_MD, gate="hard", fallbacks=[])
    part2 = Doc("part-2", "part-2.html", PART2_MD, gate="hard", fallbacks=[],
                # §16.4 is an RFC 9420 section, not a Drystone one.
                external_sections={"16.4"})
    # A companion that annotates Part 2: its bare  section  refs fall back to Part 2.
    companion = Doc("open-threads", "open-threads.html", "# Open threads\n\n## 1. A thread\n",
                    gate="soft", fallbacks=["part-2", "part-1"])
    pathmap = {
        "beta/drystone-spec/EVIDENCE-MAP.md": "EVIDENCE-MAP.html",
        "alpha/thinking/reconciliation-horizon.md": "reconciliation-horizon.html",
    }
    return Ctx({"part-1": part1, "part-2": part2, "open-threads": companion}, pathmap)


class TestHeadingId(unittest.TestCase):
    def test_numeric_two_level(self):
        self.assertEqual(heading_id("7.6. Human adjudication"), ("7.6", "s7-6"))

    def test_numeric_three_level(self):
        self.assertEqual(heading_id("7.6.2. Something"), ("7.6.2", "s7-6-2"))

    def test_top_level(self):
        self.assertEqual(heading_id("7. The realization"), ("7", "s7"))

    def test_appendix(self):
        self.assertEqual(heading_id("Appendix B. Open Questions"), ("appendix-b", "appendix-b"))

    def test_appendix_sub(self):
        self.assertEqual(heading_id("C.4 The governance-as-protocol frontier"), ("c.4", "c-4"))

    def test_prose_heading_has_no_section_key(self):
        key, anchor = heading_id("Why this document exists")
        self.assertIsNone(key)
        self.assertEqual(anchor, "why-this-document-exists")


class TestParseHeadings(unittest.TestCase):
    def test_registers_numbered_sections(self):
        reg = parse_headings(PART2_MD)
        self.assertEqual(reg.anchor_for("5.2"), "s5-2")
        self.assertEqual(reg.anchor_for("7.2"), "s7-2")
        self.assertEqual(reg.anchor_for("appendix-b"), "appendix-b")

    def test_registers_r_bullets_under_section(self):
        reg = parse_headings(PART2_MD)
        # R7 lives under  7.2 as a bold list item -> anchorable id
        self.assertEqual(reg.anchor_for("7.2 r7"), "s7-2-r7")

    def test_ignores_headings_inside_code_fences(self):
        md = "## 3. Real\n\n```\n## 9. Fake heading in code\n```\n"
        reg = parse_headings(md)
        self.assertEqual(reg.anchor_for("3"), "s3")
        self.assertIsNone(reg.anchor_for("9"))


class TestInDocRef(unittest.TestCase):
    def test_bare_section_links_in_same_doc(self):
        out, unresolved = autolink_text("as shown in §7.6 above", "part-2", make_ctx())
        self.assertIn('href="#s7-6"', out)
        self.assertIn("§7.6", out)  # link text preserved
        self.assertEqual(unresolved, [])

    def test_bare_section_appendix(self):
        out, unresolved = autolink_text("see Appendix B", "part-2", make_ctx())
        self.assertIn('href="#appendix-b"', out)
        self.assertEqual(unresolved, [])


class TestCrossDocRef(unittest.TestCase):
    def test_part1_ref_from_part2(self):
        out, unresolved = autolink_text("per Part 1 §2.5 the terminus", "part-2", make_ctx())
        self.assertIn('href="part-1.html#s2-5"', out)
        self.assertEqual(unresolved, [])

    def test_part2_ref_from_companion(self):
        out, unresolved = autolink_text("governed under Part 2 §7.2", "open-threads", make_ctx())
        self.assertIn('href="part-2.html#s7-2"', out)
        self.assertEqual(unresolved, [])

    def test_bare_section_in_companion_falls_back_to_part2(self):
        # open-threads has no  7.6 of its own; its bare  section  annotates Part 2.
        out, unresolved = autolink_text("the §7.6 posture", "open-threads", make_ctx())
        self.assertIn('href="part-2.html#s7-6"', out)
        self.assertEqual(unresolved, [])


class TestMissingSection(unittest.TestCase):
    def test_missing_section_is_reported_with_offender(self):
        out, unresolved = autolink_text("see §9.9.9 which does not exist", "part-2", make_ctx())
        # unresolved names the offending reference
        self.assertEqual(len(unresolved), 1)
        self.assertIn("9.9.9", unresolved[0])
        # and it is left as plain text, not a dangling link
        self.assertNotIn("<a", out)

    def test_missing_cross_doc_section_reported(self):
        out, unresolved = autolink_text("per Part 1 §8.8 nope", "part-2", make_ctx())
        self.assertEqual(len(unresolved), 1)
        self.assertIn("Part 1", unresolved[0])
        self.assertIn("8.8", unresolved[0])


class TestRBullet(unittest.TestCase):
    def test_r_bullet_anchor_when_present(self):
        out, unresolved = autolink_text("the R7 rule (§7.2 R7)", "part-2", make_ctx())
        self.assertIn('href="#s7-2-r7"', out)
        self.assertEqual(unresolved, [])

    def test_section_with_unknown_r_falls_back_to_section(self):
        # R9 is not a registered bullet; the  7.2 part still resolves.
        out, unresolved = autolink_text("see §7.2 R9", "part-2", make_ctx())
        self.assertIn('href="#s7-2"', out)
        self.assertEqual(unresolved, [])


class TestExternalRfcRefs(unittest.TestCase):
    def test_rfc_adjacent_ref_is_not_linkified_or_gated(self):
        # "RFC 9420 §8.2" -- even though Drystone HAS a §8.2 elsewhere, the RFC
        # adjacency marks this as the RFC's section: leave literal, do not gate.
        out, unresolved = autolink_text("per RFC 9420 §8.2 the tree", "part-2", make_ctx())
        self.assertNotIn("<a ", out)
        self.assertEqual(unresolved, [])

    def test_bcp_adjacent_ref_is_external(self):
        out, unresolved = autolink_text("RFC 8126 (BCP 26), §5.2 quoted", "part-1", make_ctx())
        self.assertNotIn("<a ", out)
        self.assertEqual(unresolved, [])

    def test_external_section_set_ref_is_not_gated(self):
        # §16.4 is in part-2's external set: a bare detached ref is left literal,
        # not counted as a broken Drystone reference.
        out, unresolved = autolink_text("the limits at §16.4 are named", "part-2", make_ctx())
        self.assertNotIn("<a ", out)
        self.assertEqual(unresolved, [])


class TestCodeSpanNotLinkified(unittest.TestCase):
    def test_ref_inside_inline_code_is_not_linkified(self):
        html = "<p>real <code>see §7.6 here</code> and §7.6 real</p>"
        out, unresolved = autolink_html(html, "part-2", make_ctx())
        # exactly one link: the one OUTSIDE the code span
        self.assertEqual(out.count("<a "), 1)
        # the code span content is untouched
        self.assertIn("<code>see §7.6 here</code>", out)

    def test_ref_inside_pre_block_is_not_linkified(self):
        html = "<pre><code>§7.6 and Part 1 §2.5</code></pre>"
        out, unresolved = autolink_html(html, "part-2", make_ctx())
        self.assertNotIn("<a ", out)

    def test_existing_anchor_not_double_linked(self):
        html = '<p><a href="x">§7.6</a></p>'
        out, unresolved = autolink_html(html, "part-2", make_ctx())
        self.assertEqual(out.count("<a "), 1)


class TestRepoPathCitation(unittest.TestCase):
    def test_published_path_becomes_link(self):
        out, unresolved = autolink_text(
            "see alpha/thinking/reconciliation-horizon.md for more", "part-2", make_ctx())
        self.assertIn('href="reconciliation-horizon.html"', out)
        self.assertEqual(unresolved, [])

    def test_unpublished_path_left_literal(self):
        text = "see alpha/experiments/local_storage_projection/X3-AUTOMATED-SWEEP.md"
        out, unresolved = autolink_text(text, "part-2", make_ctx())
        self.assertNotIn("<a ", out)
        self.assertIn("X3-AUTOMATED-SWEEP.md", out)
        # an unpublished path is intentionally-not-a-link, NOT a broken ref
        self.assertEqual(unresolved, [])


if __name__ == "__main__":
    unittest.main()
