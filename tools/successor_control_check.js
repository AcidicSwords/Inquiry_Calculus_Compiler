#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const root = path.resolve(__dirname, "..");
const errors = [];
const baseline = "4a18e2e308f359a64f19b7d056652f19fd9aaeae";

function rel(name) {
  return path.join(root, ...name.split("/"));
}

function read(name) {
  try {
    return fs.readFileSync(rel(name), "utf8");
  } catch (error) {
    errors.push(`${name}: ${error.message}`);
    return "";
  }
}

function digest(name) {
  try {
    return crypto.createHash("sha256").update(fs.readFileSync(rel(name))).digest("hex");
  } catch (error) {
    errors.push(`${name}: ${error.message}`);
    return "";
  }
}

function requireFile(name) {
  if (!fs.statSync(rel(name), { throwIfNoEntry: false })?.isFile()) {
    errors.push(`required successor file is missing: ${name}`);
  }
}

function requireContains(name, fragments) {
  const text = read(name);
  for (const fragment of fragments) {
    if (!text.includes(fragment)) errors.push(`${name}: missing ${JSON.stringify(fragment)}`);
  }
}

const required = [
  "AGENTS.md",
  "IMPLEMENTATION_FRONTIER.md",
  "formal/lean-toolchain",
  "formal/lakefile.toml",
  "formal/lake-manifest.json",
  "formal/InquiryCalculus.lean",
  "formal/InquiryCalculus/Meta/Ambient.lean",
  "formal/Spec.lean",
  "formal/Spec/InquiryCalculus.lean",
  "formal-successor/ACTIVE_INPUTS.json",
  "formal-successor/AUTONOMOUS_ITERATION.md",
  "formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md",
  "formal-successor/Questions.txt",
  "formal-successor/PREDECESSOR_BASELINE.md",
  "formal-successor/CONFORMANCE_STATUS.md",
  "formal-successor/DECISIONS.jsonl",
  "formal-successor/ENGINEERING_QUESTION_PROGRAMS.json",
  "formal-successor/FAILURES.jsonl",
  "formal-successor/reports/latest.json",
  ".claude/hooks/ic-question-program.js",
  ".gitattributes",
];
for (const name of required) requireFile(name);

requireContains("AGENTS.md", [
  "FORMAL-SUCCESSOR BRANCH CONTRACT",
  "formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md",
  "formal-successor/CONFORMANCE_STATUS.md",
  "Until Formal Gate F",
  "Operational question programs from `Questions.txt`",
  "RECIPROCAL WHY PAIR",
  "first policy record",
  "declared `QP-CODING-*` source lines",
  "two-orientation `QP-WHY-*` pairs",
]);

requireContains(".claude/hooks/ic-question-program.js", [
  "QP-CODING-",
  "QP-WHY-",
  "question-program record is detached from the trace policy",
  "reciprocal_pairs=none",
]);

requireContains(".gitattributes", ["formal-successor/Questions.txt -text -whitespace"]);

requireContains("formal-successor/AUTONOMOUS_ITERATION.md", [
  "## Persistent objective",
  "## Resume coordinate",
  "## One finite ratchet",
  "After each actual return",
  "The harness rejects a residual while any raw return lacks a subsequent",
  "## Phase progression",
  "## Autonomous safety boundary",
]);

const frontier = read("IMPLEMENTATION_FRONTIER.md");
if ((frontier.match(/<!-- LIVE_FRONTIER_BEGIN -->/gu) ?? []).length !== 1 ||
    (frontier.match(/<!-- LIVE_FRONTIER_END -->/gu) ?? []).length !== 1) {
  errors.push("IMPLEMENTATION_FRONTIER.md must contain exactly one live block");
}
if (!frontier.includes("id: FORMAL-") || frontier.includes("id: QASK-MIXED-RESOLUTION-007")) {
  errors.push("root frontier is not routed exclusively to the formal successor");
}
for (const key of [
  "id", "plan_phase", "goal", "protected_difference", "discriminator", "horizon",
  "relevant_decisions", "relevant_failures", "if_pass", "if_fail",
]) {
  const count = frontier.match(new RegExp(`^${key}:`, "gmu"))?.length ?? 0;
  if (count !== 1) errors.push(`live frontier key ${key} must occur exactly once (found ${count})`);
}

let inputs;
try {
  inputs = JSON.parse(read("formal-successor/ACTIVE_INPUTS.json"));
} catch (error) {
  errors.push(`formal-successor/ACTIVE_INPUTS.json: ${error.message}`);
}
const expectedInputs = new Map([
  ["Inquiry_Calculus_v2_0.tex", "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89"],
  ["formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md", "c62ac86b3f551d03ce687e28f0870f53af19a22d70f41fe4a468e707d4da540e"],
  ["formal-successor/Questions.txt", "5a0dbb45bd1e9ff838a0396f6a1f17ba23cfa726889d28ab9b809b0c9b9cd019"],
]);
if (inputs) {
  if (inputs.branch !== "codex/formal-successor" || inputs.predecessor_commit !== baseline) {
    errors.push("ACTIVE_INPUTS branch or predecessor coordinate changed without a control migration");
  }
  const declared = new Map((inputs.inputs ?? []).map((item) => [item.path, item.sha256]));
  for (const [name, expected] of expectedInputs) {
    if (declared.get(name) !== expected) errors.push(`${name}: declared digest is not the accepted input digest`);
    const actual = digest(name);
    if (actual !== expected) errors.push(`${name}: expected SHA-256 ${expected}, got ${actual}`);
  }
  if (inputs.adopted_process_input?.source_sha256 !==
      "7f316218d7a9aa9ba17461445b575034d45365020c785e4f5de9f029b94c8f89" ||
      inputs.adopted_process_input?.role !== "engineering_control_proposal_not_semantic_authority") {
    errors.push("the proposed engineering clock is not classified as non-semantic process input");
  }
}

let questionPrograms;
try {
  questionPrograms = JSON.parse(read("formal-successor/ENGINEERING_QUESTION_PROGRAMS.json"));
} catch (error) {
  errors.push(`formal-successor/ENGINEERING_QUESTION_PROGRAMS.json: ${error.message}`);
}
if (questionPrograms) {
  const questions = read("formal-successor/Questions.txt").split(/\r?\n/u);
  const line = (number) => questions[number - 1] ?? "";
  if (questionPrograms.source_sha256 !== expectedInputs.get("formal-successor/Questions.txt")) {
    errors.push("engineering question programs are not bound to the accepted corpus digest");
  }
  for (const section of [questionPrograms.sections?.coding, questionPrograms.sections?.reciprocal_why]) {
    if (!section || line(section.heading_line) !== section.heading) {
      errors.push(`question-program section heading does not match Questions.txt: ${section?.heading ?? "missing"}`);
    }
    if (!line(section.first_question_line).endsWith("?") ||
        !line(section.last_question_line).endsWith("?")) {
      errors.push(`question-program section bounds do not address questions: ${section?.heading ?? "missing"}`);
    }
  }
  const reciprocalEnd = questionPrograms.sections?.reciprocal_why?.last_question_line ?? 0;
  if (questions.slice(reciprocalEnd).some((entry) => entry.trim() !== "")) {
    errors.push("reciprocal-why section bound does not reach the final corpus question");
  }
  const requiredPrograms = new Set([
    "QP-CODING-FRAME",
    "QP-CODING-TRACE-COMPOSE",
    "QP-CODING-BOUNDARY",
    "QP-CODING-PROPAGATE-RATCHET",
    "QP-WHY-RETURN-CONTRAST",
    "QP-WHY-RECIPROCAL-NECESSITY",
    "QP-WHY-CONTRACT-REOPEN",
    "QP-WHY-EVIDENCE",
    "QP-WHY-FAILURE-REPAIR",
    "QP-WHY-QUESTION-RATCHET",
  ]);
  for (const program of questionPrograms.programs ?? []) {
    requiredPrograms.delete(program.id);
    const references = [
      ...(program.sequence ?? []),
      ...(program.paired_sequence ?? []).flat(),
    ];
    if (references.length === 0) errors.push(`${program.id}: question program has no source questions`);
    for (const sourceLine of references) {
      const question = line(sourceLine);
      if (!question.endsWith("?")) {
        errors.push(`${program.id}: source line ${sourceLine} is not a question in Questions.txt`);
      }
    }
    for (const pair of program.paired_sequence ?? []) {
      if (!Array.isArray(pair) || pair.length !== 2) {
        errors.push(`${program.id}: every reciprocal/boundary pair must have exactly two orientations`);
      }
    }
  }
  if (requiredPrograms.size > 0) {
    errors.push(`missing required engineering question programs: ${[...requiredPrograms].join(", ")}`);
  }
  const order = questionPrograms.composition?.order ?? [];
  for (const stage of [
    "QP-CODING-FRAME", "actual_return", "QP-WHY-RETURN-CONTRAST",
    "QP-WHY-RECIPROCAL-NECESSITY", "QP-CODING-PROPAGATE-RATCHET",
    "recheck_reprove_ablate", "QP-WHY-QUESTION-RATCHET", "next_live_residual",
  ]) {
    if (!order.includes(stage)) errors.push(`composed ratchet is missing stage ${stage}`);
  }
  if (!/inapplicable reciprocal directions remain explicit/u.test(
    questionPrograms.composition?.continuation_rule ?? "",
  )) {
    errors.push("composed ratchet must preserve typed reciprocal inapplicability");
  }
}

const toolchain = read("formal/lean-toolchain").trim();
if (toolchain !== "leanprover/lean4:v4.33.1") errors.push(`unexpected Lean toolchain: ${toolchain}`);
requireContains("formal/lakefile.toml", [
  'rev = "0df444a360eaa60ab8c11dca51a86af692955474"',
  'rev = "3bdedf29bada13d8103e6c979001c51dcee210c8"',
  'warningAsError = true',
]);
const manifest = read("formal/lake-manifest.json");
for (const revision of [
  '"rev": "0df444a360eaa60ab8c11dca51a86af692955474"',
  '"rev": "3bdedf29bada13d8103e6c979001c51dcee210c8"',
]) {
  if (!manifest.includes(revision)) errors.push(`formal/lake-manifest.json: missing locked ${revision}`);
}
if (/rev\s*=\s*"(?:main|master|nightly)"/iu.test(read("formal/lakefile.toml"))) {
  errors.push("formal dependencies must not track a moving revision");
}

for (const ledger of ["formal-successor/DECISIONS.jsonl", "formal-successor/FAILURES.jsonl"]) {
  const ids = new Set();
  for (const [index, line] of read(ledger).split(/\r?\n/u).entries()) {
    if (!line.trim()) continue;
    try {
      const record = JSON.parse(line);
      assert.equal(typeof record.id, "string");
      if (ids.has(record.id)) errors.push(`${ledger}:${index + 1}: duplicate id ${record.id}`);
      ids.add(record.id);
    } catch (error) {
      errors.push(`${ledger}:${index + 1}: invalid JSONL (${error.message})`);
    }
  }
}

const conformance = read("formal-successor/CONFORMANCE_STATUS.md");
const gateFPassed = /^\| FORMAL-GATE-F \| PASS \|/mu.test(conformance);
if (!gateFPassed) {
  const protectedPaths = [
    "Inquiry_Calculus_v2_0.tex",
    "Inquiry_Calculus_v2_0_Comprehensive_Implementation_Plan.md",
    "Cargo.toml", "Cargo.lock", "rust-toolchain.toml",
    "crates", "fixtures", "migrations",
    "CONFORMANCE_STATUS.md", "DECISIONS.jsonl", "FAILURES.jsonl",
  ];
  const result = spawnSync(
    "git",
    ["diff", "--name-only", baseline, "--", ...protectedPaths],
    { cwd: root, encoding: "utf8", windowsHide: true },
  );
  if (result.status !== 0) {
    errors.push(`could not compare frozen predecessor to ${baseline}: ${result.stderr.trim()}`);
  } else if (result.stdout.trim()) {
    errors.push(`pre-Gate-F predecessor surfaces changed:\n${result.stdout.trim()}`);
  }
}

function leanSources(directory) {
  const found = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    if (entry.name === ".lake") continue;
    const candidate = path.join(directory, entry.name);
    if (entry.isDirectory()) found.push(...leanSources(candidate));
    else if (entry.isFile() && entry.name.endsWith(".lean")) found.push(candidate);
  }
  return found;
}
for (const absolute of leanSources(rel("formal"))) {
  const name = path.relative(root, absolute).split(path.sep).join("/");
  const source = read(name);
  if (/\bsorry\b/u.test(source)) errors.push(`${name}: contains sorry`);
  if (/^\s*axiom\s+/mu.test(source)) errors.push(`${name}: contains a custom axiom declaration`);
}

requireContains(".github/workflows/ci.yml", [
  "codex/formal-successor",
  "tools/successor_control_check.js",
  "node --check .claude/hooks/ic-question-program.js",
  "lake-package-directory: formal",
  "leanchecker: true",
  "axiom-audit: true",
]);

if (errors.length > 0) {
  process.stderr.write(`formal successor control check failed:\n- ${errors.join("\n- ")}\n`);
  process.exit(1);
}

process.stdout.write("formal successor control checks passed\n");
