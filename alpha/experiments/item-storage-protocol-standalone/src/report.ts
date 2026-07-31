// Generates RUN_REPORT.md from a completed run. The report reads as the Part 4
// narrative: the plain-language sentence from each experiment in order, the E5
// measured-vs-predicted table, the E11 years-to-extinguish table, and the
// per-period statement summaries.

import type { World } from "./world.ts";
import type { ReportTable } from "./types.ts";
import { tally } from "./suite.ts";

function renderTable(t: ReportTable): string {
  const head = `| ${t.headers.join(" | ")} |`;
  const sep = `| ${t.headers.map(() => "---").join(" | ")} |`;
  const rows = t.rows.map((r) => `| ${r.join(" | ")} |`);
  return [`**${t.title}**`, "", head, sep, ...rows, ""].join("\n");
}

export function generateReport(world: World): string {
  const { total, failed } = tally(world);
  const lines: string[] = [];

  lines.push("# Item Storage Protocol — RUN REPORT");
  lines.push("");
  lines.push(
    "Deterministic run of the twelve-experiment suite (E0-E11). Every figure below " +
      "is reproducible byte-for-byte from the master seed; nothing here depends on " +
      "wall-clock time or unseeded randomness.",
  );
  lines.push("");
  lines.push(`- Master seed: \`${world.masterSeed}\``);
  lines.push(`- Assertions: **${total - failed}/${total} passed**` + (failed ? ` (${failed} FAILED)` : ""));
  lines.push(`- Statements in the chain: ${world.statements.length}`);
  lines.push(`- Simulated span: ${world.clock.now()} days`);
  lines.push(`- Co-op grace account absorbed: ${world.graceAccountCents} cents`);
  lines.push("");

  // The narrative thread: the plain sentences in order.
  lines.push("## The story, in one line each");
  lines.push("");
  for (const r of world.results) {
    lines.push(`- **${r.id} ${r.title}** — ${r.plainSentence}`);
  }
  lines.push("");

  // Per-experiment detail.
  lines.push("## Experiments");
  lines.push("");
  for (const r of world.results) {
    const passed = r.assertions.filter((a) => a.ok).length;
    lines.push(`### ${r.id}. ${r.title}`);
    lines.push("");
    lines.push(`_${r.plainSentence}_`);
    lines.push("");
    lines.push(`Assertions: ${passed}/${r.assertions.length} passed.`);
    lines.push("");
    for (const a of r.assertions) {
      lines.push(`- ${a.ok ? "PASS" : "FAIL"} — ${a.name}${a.ok ? "" : ` (${a.detail ?? ""})`}`);
    }
    lines.push("");
    for (const note of r.notes) lines.push(`> ${note}`);
    if (r.notes.length) lines.push("");
    for (const t of r.tables) {
      lines.push(renderTable(t));
    }
  }

  // Per-period statement summaries.
  lines.push("## Statement chain (per-period summary, cents)");
  lines.push("");
  lines.push("| period | days | tier | rent | postage | audit | fees | grace | total |");
  lines.push("| --- | --- | --- | --- | --- | --- | --- | --- | --- |");
  for (const s of world.statements) {
    lines.push(
      `| ${s.period} | ${s.periodStartDay}-${s.periodEndDay} | ${s.auditTier} | ${s.rentCents} | ` +
        `${s.postageCents} | ${s.auditCents} | ${s.feesCents} | ${s.graceCents} | ${s.totalCents} |`,
    );
  }
  lines.push("");

  lines.push("## Reproduce");
  lines.push("");
  lines.push("```");
  lines.push("node src/run.ts      # runs the suite, writes ledgers/ and this report");
  lines.push("node src/verify.ts   # re-verifies every ledger entry's signature and chain");
  lines.push("node --test          # the same assertions under the Node test runner");
  lines.push("```");
  lines.push("");

  return lines.join("\n");
}
