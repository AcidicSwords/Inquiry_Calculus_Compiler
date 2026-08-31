#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const cp = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const crypto = require("node:crypto");
const root = path.resolve(__dirname, "..");
const appendProgram = path.join(root, ".claude/hooks/ic-append.js");
const contractBytes = fs.readFileSync(path.join(root, "formal-successor/INQUIRY_SPINE_CONTRACT.json"));
const corpusBytes = fs.readFileSync(path.join(root, "formal-successor/Questions.txt"));
const sha = (bytes) => crypto.createHash("sha256").update(bytes).digest("hex");

function fixture() {
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-spine-lifecycle-"));
  const trace = path.join(directory, "trace.jsonl");
  const fuel = path.join(directory, "fuel");
  fs.writeFileSync(trace, ""); fs.writeFileSync(fuel, "24");
  return { directory, trace, fuel };
}
function append(target, record, expect = true) {
  const result = cp.spawnSync(process.execPath, [appendProgram, "append", target.trace, target.fuel], {
    cwd: root, input: `${JSON.stringify(record)}\n`, encoding: "utf8", windowsHide: true,
  });
  if (expect && result.status !== 0) throw new Error(result.stderr || result.stdout);
  if (!expect && result.status === 0) throw new Error(`unexpected admission: ${record.kind}`);
  return result;
}
function state(target) {
  return JSON.parse(cp.execFileSync(process.execPath, [appendProgram, "state", target.trace], { cwd: root, encoding: "utf8" }));
}
const policy = {
  kind: "policy", question_program_schema: "5", source_digest: sha(corpusBytes),
  program_manifest_digest: sha(contractBytes),
};
function member(id, disposition = "Required") {
  return {
    occurrence: id, question_form: "CQ-FRAME-FIELD", rendering: "RENDER-CQ-FRAME-FIELD",
    prompt: "What is here and how is it related?", source_lines: [1033, 1051, 1073, 1075],
    generator_ids: ["GEN-RELATE"], context: `context:${id}`, path: `fixture/${id}`,
    disposition, executable: true, dependencies: [],
  };
}
function field(id, members, regeneratedFrom = "bootstrap", dispositions = { none: "retained" }, removal = { none: "retained" }) {
  return { kind: "field", field_id: id, members: JSON.stringify(members), basis: "fixture",
    coverage: "fixture", regenerated_from: regeneratedFrom,
    dispositions: JSON.stringify(dispositions), removal_evidence: JSON.stringify(removal) };
}
function ask(question, mode, fieldId) {
  return {
    kind: "ask", q: question.prompt, mode, occurrence: question.occurrence, field_id: fieldId,
    question_form: question.question_form, rendering: question.rendering,
    source_lines: question.source_lines.join(","), generator_ids: question.generator_ids.join(","),
    reciprocal_relations: "none", context: question.context, path: question.path, bindings: "none",
    horizon: "fixture", coverage: "fixture", authority: "fixture", evidence: "fixture",
    dependencies: "none", source_digest: sha(corpusBytes), program_manifest_digest: sha(contractBytes), fp: `fp-${question.occurrence}`,
  };
}
function answer(id, question, status = "provisional") {
  return { kind: "answer", occurrence: id, ask_occurrence: question.occurrence, answer: "fixture return",
    resolution_class: "Supported", status, polarity: "Positive", residual: "none",
    evidence: "fixture evidence", coverage: "fixture", authority: "fixture" };
}
function reify(answerId, status = "provisional") {
  return { kind: "reify", answer_occurrence: answerId, status,
    products: JSON.stringify([{ id: `PRODUCT-${answerId}`, kind: "GeneratedRelation", status,
      provenance: answerId, coverage: "fixture", applicability: "fixture", horizon: "fixture", dependencies: [] }]),
    new_questions: "none", coverage: "fixture" };
}

// Pure/Generate lifecycle, authority non-upgrade, no silent disappearance.
{
  const f = fixture(), q = member("Q-PURE");
  append(f, policy); append(f, field("FIELD-1", [q]));
  append(f, answer("A-EARLY", q), false);
  append(f, ask(q, "Generate", "FIELD-1"));
  append(f, { ...answer("A-UPGRADE", q, "checked") }, false);
  append(f, answer("A-1", q));
  append(f, field("FIELD-DIRTY", [], "A-1", { "Q-PURE": "Answered" }, { "Q-PURE": "A-1" }), false);
  append(f, reify("A-1"));
  append(f, field("FIELD-SILENT", [], "A-1"), false);
  append(f, field("FIELD-2", [], "A-1", { "Q-PURE": "Answered" }, { "Q-PURE": "A-1" }));
  assert.equal(state(f).open, false);
  fs.rmSync(f.directory, { recursive: true, force: true });
}

// Effectful lifecycle cannot skip Seal, Raw, Interpretation, or Check.
{
  const f = fixture(), q = member("Q-PROBE");
  append(f, policy); append(f, field("FIELD-P1", [q])); append(f, ask(q, "Probe", "FIELD-P1"));
  const raw = { kind: "raw", ask_occurrence: q.occurrence, cmd: "fixture", digest: "a".repeat(64), raw_ref: "digest-only:sensitive", sensitive: "true" };
  append(f, raw, false);
  append(f, { kind: "seal", ask_occurrence: q.occurrence, should_change: "yes", invariants: "kept", discriminator: "fixture", wrong_impl: "skip evidence", coverage: "fixture" });
  append(f, answer("A-NO-RAW", q, "checked"), false);
  append(f, raw);
  append(f, { kind: "check", ask_occurrence: q.occurrence, verdict: "pass", coverage: "fixture", evidence: "fixture" }, false);
  append(f, { kind: "interpret", ask_occurrence: q.occurrence, raw_digest: raw.digest, interpretation: "fixture", provenance: "fixture" });
  append(f, answer("A-NO-CHECK", q, "checked"), false);
  append(f, { kind: "check", ask_occurrence: q.occurrence, verdict: "pass", coverage: "fixture", evidence: "fixture" });
  append(f, answer("A-PROBE", q, "checked")); append(f, reify("A-PROBE", "checked"));
  append(f, field("FIELD-P2", [], "A-PROBE", { "Q-PROBE": "Answered" }, { "Q-PROBE": "A-PROBE" }));
  assert.equal(state(f).open, false);
  fs.rmSync(f.directory, { recursive: true, force: true });
}

// A fold cannot be admitted from similarity or missing independent evidence.
{
  const f = fixture(), left = member("Q-L"), right = member("Q-R");
  append(f, policy); append(f, field("FIELD-F1", [left, right]));
  append(f, { kind: "fold", fold_id: "F", members: JSON.stringify([left.occurrence, right.occurrence]),
    representative: left.occurrence, protected_equivalence_evidence: "missing-equivalence",
    regeneration: "missing-regeneration", protected_continuations: JSON.stringify(["missing-continuation"]),
    reopen_condition: "new carried discriminator", horizon: "fixture", coverage: "fixture" }, false);
  fs.rmSync(f.directory, { recursive: true, force: true });
}

console.log("inquiry spine lifecycle checks passed (Ask/Answer/Reify, effectful evidence, authority, retirement, fold admission)");
