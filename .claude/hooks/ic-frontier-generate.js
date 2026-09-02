#!/usr/bin/env node
"use strict";

// Generate IMPLEMENTATION_FRONTIER.md from the derived construction obligation
// field. The frontier is an OUTPUT of the construction field, never an input
// telling the field what matters. Editing the Markdown by hand changes no live
// obligation; `check` reports the drift and `write` restores the projection.

const fs = require("node:fs");
const path = require("node:path");
const obligationIndex = require("./ic-obligation-index.js");

const BEGIN = "<!-- LIVE_FRONTIER_BEGIN -->";
const END = "<!-- LIVE_FRONTIER_END -->";

// Exactly the keys ic-frontier.js validates, in its required order.
const BLOCK_KEYS = [
  "id", "plan_phase", "goal", "protected_difference", "discriminator",
  "horizon", "relevant_decisions", "relevant_failures", "if_pass", "if_fail",
];

function fail(message) {
  throw new Error(message);
}

// The block is one line per key, so every derived value must be collapsed to a
// single line. Truncation would silently weaken a protected difference, so this
// only normalizes whitespace.
function oneLine(value, fallback) {
  const text = String(value ?? "").replace(/\s+/gu, " ").trim();
  return text.length > 0 ? text : fallback;
}

function list(values, fallback) {
  const entries = (values ?? []).filter((entry) => typeof entry === "string" && entry.trim());
  return entries.length > 0 ? entries.join(", ") : fallback;
}

function frontierFields(selected) {
  if (!selected) selected = { id: "FORMAL-CONSTRUCTION-NO-EXECUTABLE", gate: "C",
    statement: "No executable obligation is currently available. Retain all live requirements and typed operational blockers; this is not closure.",
    protected_consequence: "Live differs from executable; empty execution is not semantic closure.",
    breakers: ["An exhausted backend or missing question must not delete a live obligation."], horizon: "operational availability only" };
  return {
    id: selected.id,
    plan_phase: selected.gate ?? "C",
    goal: oneLine(selected.statement, "unstated"),
    protected_difference: oneLine(selected.protected_consequence, "unstated"),
    discriminator: oneLine(selected.breakers?.[0], "unstated"),
    horizon: oneLine(selected.horizon, "unstated"),
    relevant_decisions: list(selected.relevant_decisions, "none"),
    relevant_failures: list(selected.relevant_failures, "none"),
    if_pass: oneLine(selected.if_pass, "discharge the obligation and let the rebuilt field determine the next live obligation"),
    if_fail: oneLine(selected.if_fail, "retain the exact typed blocker and rebuild the field without advancing the target status"),
  };
}

function render(root) {
  const { index, selected } = obligationIndex.build(root);
  const fields = frontierFields(selected);
  const alternatives = index.selection.ranked.slice(1, 6);
  const gaps = index.representation_gaps.slice(0, 5);

  return `# Formal Successor Frontier

<!-- GENERATED FILE. Do not edit by hand. -->

This file is a generated projection of the derived construction obligation field.
It is not upstream authority: the live obligation is derived from checked formal
structure, protected predecessor capabilities, dependency relations, and open
correspondence obligations. Regenerate with:

    node .claude/hooks/ic-frontier-generate.js write .

## Derivation

    protected construction surface
      -> required obligation closure
      -> reference live field
      -> executable field
      -> selected occurrence
      -> this projection

required: ${index.counts.required}  live: ${index.counts.live}  executable: ${index.counts.executable}  terminal: ${index.counts.terminal}

## Strongest live obligation

${fields.goal}

${BEGIN}
${BLOCK_KEYS.map((key) => `${key}: ${fields[key]}`).join("\n")}
${END}

## Selection witness

Selection used explicit relations only: ${index.selection.relations.join(", ")}.
A stable identity tie-break may allocate execution; it asserts no semantic optimality.
Every unchosen live occurrence remains represented.

Next-ranked live executable obligations, retained and unchosen:

${alternatives.length > 0 ? alternatives.map((id) => `- ${id}`).join("\n") : "- none"}

## Retained nonterminal accounting

Representation gaps (expressible-language failures, not semantic absence):

${gaps.length > 0 ? gaps.map((gap) => `- ${gap.obligation}: ${gap.reason}`).join("\n") : "- none"}

## Non-collapse at this boundary

- Live is not executable, and no generated question is not no live obligation.
- ResourceBounded is not semantic closure.
- A generator gap means the obligation is not currently generated, not that it is false, impossible, or closed.
- Model output is not proof; harness state is not semantic authority.
- The explicit theorem registry is a seed and audit surface, not the complete theorem universe.

## Exit

On pass, apply the obligation's declared pass continuation, rebuild the obligation
field, and let the changed field determine the next live obligation. On failure,
retain the exact breaker or missing structure, rebuild the field, and continue from
the resulting live residual.
`;
}

function check(root) {
  const expected = render(root);
  const target = path.join(root, "IMPLEMENTATION_FRONTIER.md");
  const actual = fs.existsSync(target) ? fs.readFileSync(target, "utf8") : "";
  return { drifted: actual.replaceAll("\r\n", "\n") !== expected, expected, target };
}

module.exports = { render, check, frontierFields, BLOCK_KEYS };

if (require.main === module) {
  try {
    const [command, suppliedRoot] = process.argv.slice(2);
    const root = path.resolve(suppliedRoot ?? path.resolve(__dirname, "../.."));
    if (command === "render") process.stdout.write(render(root));
    else if (command === "write") {
      const { expected, target } = check(root);
      fs.writeFileSync(target, expected);
      process.stdout.write(`regenerated ${path.basename(target)}\n`);
    } else if (command === "check") {
      const { drifted } = check(root);
      if (drifted) fail("IMPLEMENTATION_FRONTIER.md drifted from its derived projection; regenerate it");
      process.stdout.write("frontier projection matches the derived obligation field\n");
    } else fail("usage: ic-frontier-generate.js render|write|check ROOT");
  } catch (error) {
    process.stderr.write(`ic-frontier-generate: ${error.message}\n`);
    process.exitCode = 1;
  }
}
