// EXP-LEX-01 official-tooling gate.
//
// The *validator of record* for lexicon.community schemas is the official
// @atproto/lexicon package, not our Rust mirror. This gate loads every vendored
// + candidate lexicon and asserts the corpus behaves as claimed UNDER THAT
// TOOLING: golden records validate; adversarial records fail; knownValues is
// open; enum is closed. Where official behavior differs from our strict Rust
// validator, it is reported as a finding (unknown-field handling — A-6).
//
// Run: node scripts/gate.mjs   (exit 0 = gate green)

import { readFileSync, readdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { Lexicons } from "@atproto/lexicon";

const here = dirname(fileURLToPath(import.meta.url));
const root = join(here, "..");
const fx = join(root, "fixtures");

const lex = new Lexicons();

// A minimal com.atproto.repo.strongRef (referenced by rsvp + our staple) — the
// core lexicon @atproto/lexicon does not bundle standalone.
lex.add({
  lexicon: 1,
  id: "com.atproto.repo.strongRef",
  defs: {
    main: {
      type: "object",
      required: ["uri", "cid"],
      properties: { uri: { type: "string", format: "at-uri" }, cid: { type: "string", format: "cid" } },
    },
  },
});

for (const f of readdirSync(join(fx, "lexicons"))) {
  if (!f.endsWith(".json")) continue;
  lex.add(JSON.parse(readFileSync(join(fx, "lexicons", f), "utf8")));
}

const load = (p) => JSON.parse(readFileSync(join(fx, p), "utf8"));
const nsidOf = (r) => r["$type"];

let fail = 0;
const line = (ok, msg) => {
  console.log(`${ok ? "PASS" : "FAIL"}  ${msg}`);
  if (!ok) fail++;
};

function validates(rel, nsidOverride) {
  const rec = load(rel);
  const nsid = nsidOverride || nsidOf(rec);
  try {
    lex.assertValidRecord(nsid, rec);
    return { ok: true };
  } catch (e) {
    return { ok: false, err: String(e.message || e) };
  }
}

console.log("# EXP-LEX-01 — @atproto/lexicon official-tooling gate\n");

// Golden — must validate.
for (const g of ["golden/event_valid.json", "golden/rsvp_valid.json"]) {
  const r = validates(g);
  line(r.ok, `golden validates: ${g}${r.ok ? "" : "  <-- " + r.err}`);
}

// knownValues is OPEN — a novel status still validates.
{
  const r = validates("golden/rsvp_novel_status_open.json");
  line(r.ok, `knownValues OPEN: novel rsvp status still validates${r.ok ? "" : "  <-- " + r.err}`);
}

// Adversarial that official tooling MUST reject.
const mustReject = [
  ["adversarial/event_missing_name.json", null, "missing required `name`"],
  ["adversarial/event_name_wrong_type.json", null, "wrong type (name: integer)"],
  ["adversarial/rsvp_subject_missing_cid.json", null, "strongRef missing cid"],
  ["adversarial/staple_bad_alg.json", "community.lexicon.attest.inclusionStaple", "closed enum rejects RS256"],
];
for (const [rel, nsid, why] of mustReject) {
  const r = validates(rel, nsid);
  line(!r.ok, `adversarial rejected (${why})`);
}

// Divergence finding (A-6): unknown-field handling. Report official behavior.
{
  const r = validates("adversarial/rsvp_unknown_field.json");
  const officialRejects = !r.ok;
  console.log(
    `NOTE  A-6 unknown-field: @atproto/lexicon ${officialRejects ? "REJECTS" : "IGNORES"} the unknown field ` +
      `(our strict Rust validator rejects). ${officialRejects ? "" : "Divergence to raise in-thread."}`
  );
}

console.log(`\n${fail === 0 ? "GATE GREEN" : `GATE RED (${fail} failures)`}`);
process.exit(fail === 0 ? 0 : 1);
