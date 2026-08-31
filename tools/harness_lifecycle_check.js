#!/usr/bin/env node
"use strict";

// Adversarial acceptance tests for active schema 4. Historical schema-3
// behavior remains covered by harness_control_check.js.

const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const repository = path.resolve(__dirname, "..");
const sandbox = fs.mkdtempSync(path.join(os.tmpdir(), "ic-lifecycle-check-"));
const hooks = path.join(sandbox, ".claude", "hooks");
const successor = path.join(sandbox, "formal-successor");
fs.mkdirSync(hooks, { recursive: true });
fs.mkdirSync(successor, { recursive: true });
fs.cpSync(path.join(repository, ".claude", "hooks"), hooks, { recursive: true });
for (const name of [
  "Questions.txt", "ENGINEERING_QUESTION_PROGRAMS.json", "PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS.md",
  "PREFORMAL_SEARCH_ASYMMETRY.md", "SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md",
  "QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md", "RESIDUAL_OBLIGATIONS.json",
]) fs.copyFileSync(path.join(repository, "formal-successor", name), path.join(successor, name));
fs.writeFileSync(path.join(sandbox, "IMPLEMENTATION_FRONTIER.md"), [
  "<!-- LIVE_FRONTIER_BEGIN -->", "id: FORMAL-B-TEST", "plan_phase: B", "goal: test",
  "protected_difference: lifecycle", "discriminator: checks", "horizon: fixture",
  "relevant_decisions: none", "relevant_failures: none", "if_pass: FORMAL-B-NEXT",
  "if_fail: repair", "<!-- LIVE_FRONTIER_END -->", "",
].join("\n"));

const appendProgram = path.join(hooks, "ic-append.js");
const manifestPath = path.join(successor, "ENGINEERING_QUESTION_PROGRAMS.json");
const sourcePath = path.join(successor, "Questions.txt");
// These fixtures retain the predecessor schema-4 evidence policy, including its
// old label-only fold examples. Active evidence-policy-1 admission and migration
// are separately challenged by harness_fold_evidence_check.js, not weakened here.
const predecessorManifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
delete predecessorManifest.active_lifecycle.fold_evidence;
fs.writeFileSync(manifestPath, `${JSON.stringify(predecessorManifest, null, 2)}\n`);
const digest = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
const forms = new Map(manifest.preformal_harness.compiled_questions.map((entry) => [entry.id, entry]));
const families = new Map(manifest.preformal_harness.program_families.map((entry) => [entry.id, entry]));

let scenarioCount = 0;
function scenario(name) {
  scenarioCount += 1;
  const trace = path.join(sandbox, `${String(scenarioCount).padStart(2, "0")}-${name}.jsonl`);
  const fuel = `${trace}.fuel`;
  fs.writeFileSync(trace, "");
  fs.writeFileSync(fuel, "24");
  const state = { trace, fuel };
  append(state, {
    kind: "policy", question_program_schema: "4", source_digest: digest(sourcePath),
    program_manifest_digest: digest(manifestPath),
  });
  return state;
}

function runAppend(state, record) {
  return spawnSync(process.execPath, [appendProgram, "append", state.trace, state.fuel], {
    cwd: sandbox, input: `${JSON.stringify(record)}\n`, encoding: "utf8", windowsHide: true,
  });
}

function append(state, record) {
  const result = runAppend(state, record);
  assert.equal(result.status, 0, `${record.kind} failed: ${result.stdout}${result.stderr}`);
  return result;
}

function reject(state, record, pattern) {
  const result = runAppend(state, record);
  assert.notEqual(result.status, 0, `${record.kind} unexpectedly succeeded`);
  if (pattern) assert.match(`${result.stdout}${result.stderr}`, pattern);
}

function member(occurrence, questionForm = "CQ-FRAME-FIELD", pathId = "path-a", overrides = {}) {
  const form = forms.get(questionForm);
  return {
    occurrence,
    question_form: questionForm,
    rendering: `RENDER-${questionForm}`,
    prompt: form.prompt,
    source_lines: form.source_lines,
    generator_ids: manifest.active_lifecycle.generator_registry
      .filter((entry) => entry.question_forms.includes(questionForm)).map((entry) => entry.id),
    path: pathId,
    disposition: "Productive",
    executable: true,
    dependencies: [],
    ...overrides,
  };
}

function field(fieldId, members, overrides = {}) {
  return {
    kind: "field", field_id: fieldId, members: JSON.stringify(members), basis: "fixture ancestry",
    coverage: "isolated lifecycle fixture", regenerated_from: "bootstrap",
    dispositions: "{}", removal_evidence: "{}", ...overrides,
  };
}

function ask(fieldId, occurrence, questionForm = "CQ-FRAME-FIELD", mode = "Pure", pathId = "path-a") {
  const form = forms.get(questionForm);
  const family = families.get(form.family);
  const generators = manifest.active_lifecycle.generator_registry
    .filter((entry) => entry.question_forms.includes(questionForm)).map((entry) => entry.id);
  return {
    kind: "ask", fp: `fp-${fieldId}-${occurrence}-${mode}`, q: form.prompt, mode, occurrence,
    field_id: fieldId, question_form: questionForm, rendering: `RENDER-${questionForm}`,
    source_lines: form.source_lines.join(","), generator_ids: generators.join(","),
    reciprocal_relations: family.reciprocal_challenges.join(",") || "none", path: pathId,
    bindings: "fixture", horizon: "fixture", coverage: "fixture", authority: "test authority",
    evidence: "field membership", dependencies: "none", source_digest: digest(sourcePath),
    program_manifest_digest: digest(manifestPath),
  };
}

function answer(askOccurrence, answerOccurrence, overrides = {}) {
  return {
    kind: "answer", occurrence: answerOccurrence, ask_occurrence: askOccurrence,
    answer: "supported fixture answer", resolution_class: "Supported", status: "supported",
    polarity: "Positive", residual: "none", evidence: "fixture derivation", coverage: "fixture",
    authority: "test authority", ...overrides,
  };
}

function reify(answerOccurrence, overrides = {}) {
  return {
    kind: "reify", answer_occurrence: answerOccurrence, status: "supported",
    products: JSON.stringify([{ id: `P-${answerOccurrence}`, kind: "candidate_relation", status: "supported",
      provenance: answerOccurrence, dependencies: [], coverage: "fixture", applicability: "fixture", horizon: "fixture" }]),
    new_questions: "none", coverage: "fixture", ...overrides,
  };
}

function checkpointResume(fieldId, checkpoint, overrides = {}) {
  return {
    kind: "note", event: "checkpoint_resume", field_id: fieldId, checkpoint: String(checkpoint),
    fuel_grant: "24", authority: "user fixture", reason: "continue persistent ratchet",
    remaining_open: "live productive occurrence", text: "checkpoint continuation fixture", ...overrides,
  };
}

function basic(name, members = [member("QO-1")]) {
  const state = scenario(name);
  append(state, field("FIELD-1", members));
  return state;
}

// Positive Pure and Generate flows, with Ask as the only fuel-consuming bookkeeping event.
{
  const state = basic("pure-flow", [member("QO-1"), member("QO-2", "CQ-OPEN-POSITION", "path-b")]);
  append(state, ask("FIELD-1", "QO-1"));
  assert.equal(fs.readFileSync(state.fuel, "utf8"), "23");
  append(state, answer("QO-1", "ANS-1"));
  append(state, reify("ANS-1"));
  append(state, field("FIELD-2", [member("QO-2", "CQ-OPEN-POSITION", "path-b")], {
    regenerated_from: "ANS-1", dispositions: JSON.stringify({ "QO-1": "Answered" }),
    removal_evidence: JSON.stringify({ "QO-1": "ANS-1" }),
  }));
  append(state, { kind: "checkpoint", field_id: "FIELD-2", established: "pure flow",
    remains_open: "QO-2", fold_changes: "none", reopen_changes: "none", coverage: "fixture" });
  assert.equal(fs.readFileSync(state.fuel, "utf8"), "23");
  reject(state, { kind: "stop", state: "Satisfied", warrant: "checkpoint" }, /closure|checkpoint/iu);
}

// Positive effectful flow preserves Seal -> Raw -> Interpret -> Check.
{
  const state = basic("probe-flow");
  append(state, ask("FIELD-1", "QO-1", "CQ-FRAME-FIELD", "Probe"));
  append(state, { kind: "seal", ask_occurrence: "QO-1", should_change: "observable", invariants: "raw immutable",
    discriminator: "fixture", wrong_impl: "fabricated return", coverage: "fixture" });
  append(state, { kind: "raw", ask_occurrence: "QO-1", cmd: "fixture", digest: "a".repeat(64),
    raw_ref: ".claude/trace/raw/fixture", sensitive: "false" });
  append(state, { kind: "interpret", ask_occurrence: "QO-1", raw_digest: "a".repeat(64),
    interpretation: "fixture returned", provenance: "raw digest" });
  append(state, { kind: "check", ask_occurrence: "QO-1", verdict: "matches", coverage: "fixture", evidence: "independent checker" });
  append(state, answer("QO-1", "ANS-PROBE", { status: "checked" }));
  append(state, reify("ANS-PROBE", { status: "checked", products: JSON.stringify([{ id: "P-PROBE", kind: "supported_relation",
    status: "checked", provenance: "ANS-PROBE", dependencies: [], coverage: "fixture", applicability: "fixture", horizon: "fixture" }]) }));
  append(state, field("FIELD-2", [member("QO-2", "CQ-RESIDUAL", "path-next", { disposition: "Blocked", executable: false })], {
    regenerated_from: "ANS-PROBE", dispositions: JSON.stringify({ "QO-1": "Answered" }),
    removal_evidence: JSON.stringify({ "QO-1": "ANS-PROBE" }),
  }));
  append(state, { kind: "closure", field_id: "FIELD-2", scope: "fixture", warrant: "independent lifecycle checks",
    adversarial_question: "QO-1", adversarial_answer: "ANS-PROBE", coverage: "fixture" });
  reject(state, { kind: "stop", state: "Satisfied", warrant: "independent lifecycle checks" }, /unresolved field/iu);
  append(state, { kind: "stop", state: "Unknown", warrant: "independent lifecycle checks; QO-2 remains blocked" });
}

// Exhausted Ask fuel may renew exactly once from a clean checkpoint with a live
// executable field and current user-authorized harness control. This continues
// the task without manufacturing closure or changing semantic priority.
{
  const state = basic("checkpoint-fuel-continuation");
  append(state, { kind: "control", authority: "user fixture", residual: "persistent field",
    predecessor: "prior ratchet", scope: "harness" });
  append(state, { kind: "checkpoint", field_id: "FIELD-1", established: "clean local ratchet",
    remains_open: "QO-1", fold_changes: "none", reopen_changes: "none", coverage: "fixture" });
  const checkpoint = snapshot(state).last_checkpoint;
  fs.writeFileSync(state.fuel, "0");
  append(state, checkpointResume("FIELD-1", checkpoint));
  assert.equal(fs.readFileSync(state.fuel, "utf8"), "24");
  reject(state, checkpointResume("FIELD-1", checkpoint), /repeats checkpoint fuel renewal/iu);
  append(state, ask("FIELD-1", "QO-1"));
  assert.equal(fs.readFileSync(state.fuel, "utf8"), "23");
}
{
  const state = basic("checkpoint-continuation-needs-control");
  append(state, { kind: "checkpoint", field_id: "FIELD-1", established: "clean",
    remains_open: "QO-1", fold_changes: "none", reopen_changes: "none", coverage: "fixture" });
  fs.writeFileSync(state.fuel, "0");
  reject(state, checkpointResume("FIELD-1", snapshot(state).last_checkpoint), /user-authorized harness control/iu);
}
{
  const state = basic("checkpoint-continuation-needs-exhaustion");
  append(state, { kind: "control", authority: "user fixture", residual: "persistent field",
    predecessor: "prior ratchet", scope: "harness" });
  append(state, { kind: "checkpoint", field_id: "FIELD-1", established: "clean",
    remains_open: "QO-1", fold_changes: "none", reopen_changes: "none", coverage: "fixture" });
  reject(state, checkpointResume("FIELD-1", snapshot(state).last_checkpoint), /exactly exhausted fuel/iu);
}
{
  const state = basic("checkpoint-continuation-needs-live-question",
    [member("QO-1", "CQ-FRAME-FIELD", "path-a", { disposition: "Unknown", executable: false })]);
  append(state, { kind: "control", authority: "user fixture", residual: "persistent field",
    predecessor: "prior ratchet", scope: "harness" });
  append(state, { kind: "checkpoint", field_id: "FIELD-1", established: "clean",
    remains_open: "Unknown only", fold_changes: "none", reopen_changes: "none", coverage: "fixture" });
  fs.writeFileSync(state.fuel, "0");
  reject(state, checkpointResume("FIELD-1", snapshot(state).last_checkpoint), /no live productive executable question/iu);
}

// Mandatory negative lifecycle and non-collapse cases.
{
  const state = scenario("ask-without-field");
  reject(state, ask("FIELD-X", "QO-1"), /without a current field/iu);
}
{
  const state = basic("answer-without-ask");
  reject(state, answer("QO-1", "ANS-X"), /no matching Ask/iu);
}
{
  const state = basic("ask-before-reify", [member("QO-1"), member("QO-2", "CQ-OPEN-POSITION", "path-b")]);
  append(state, ask("FIELD-1", "QO-1")); append(state, answer("QO-1", "ANS-1"));
  reject(state, ask("FIELD-1", "QO-2", "CQ-OPEN-POSITION", "Pure", "path-b"), /reified|regenerated/iu);
}
{
  const state = basic("ask-before-regeneration", [member("QO-1"), member("QO-2", "CQ-OPEN-POSITION", "path-b")]);
  append(state, ask("FIELD-1", "QO-1")); append(state, answer("QO-1", "ANS-1")); append(state, reify("ANS-1"));
  reject(state, ask("FIELD-1", "QO-2", "CQ-OPEN-POSITION", "Pure", "path-b"), /regenerated/iu);
}
{
  const state = basic("silent-question-loss", [member("QO-1"), member("QO-2", "CQ-OPEN-POSITION", "path-b")]);
  reject(state, field("FIELD-2", [member("QO-1")], { regenerated_from: "manual" }), /silently removes/iu);
}
{
  const state = basic("partial-is-not-complete"); append(state, ask("FIELD-1", "QO-1"));
  reject(state, answer("QO-1", "ANS-1", { resolution_class: "Partial", residual: "none" }), /Partial/iu);
}
{
  const state = basic("unknown-is-not-negative"); append(state, ask("FIELD-1", "QO-1"));
  reject(state, answer("QO-1", "ANS-1", { resolution_class: "Unknown", polarity: "Negative" }), /Unknown/iu);
}
{
  const state = basic("generate-is-not-actual"); append(state, ask("FIELD-1", "QO-1", "CQ-FRAME-FIELD", "Generate"));
  reject(state, { kind: "raw", ask_occurrence: "QO-1", cmd: "invented", digest: "b".repeat(64), raw_ref: "none", sensitive: "false" }, /sealed Probe/iu);
}
{
  const state = basic("generate-product-is-not-actual"); append(state, ask("FIELD-1", "QO-1", "CQ-FRAME-FIELD", "Generate"));
  append(state, answer("QO-1", "ANS-1", { status: "provisional" }));
  reject(state, reify("ANS-1", { status: "provisional", products: JSON.stringify([{ id: "P-ACTUAL", kind: "ActualEvent",
    status: "provisional", provenance: "ANS-1", dependencies: [], coverage: "fixture", applicability: "fixture", horizon: "fixture" }]) }), /ActualEvent/iu);
}
{
  const state = basic("reify-is-not-warrant"); append(state, ask("FIELD-1", "QO-1"));
  append(state, answer("QO-1", "ANS-1", { status: "provisional" }));
  reject(state, reify("ANS-1", { status: "warranted" }), /upgrades/iu);
}
{
  const state = basic("provisional-is-not-standing"); append(state, ask("FIELD-1", "QO-1", "CQ-FRAME-FIELD", "Generate"));
  append(state, answer("QO-1", "ANS-1", { status: "provisional" }));
  reject(state, reify("ANS-1", { status: "provisional", products: JSON.stringify([{ id: "P-STANDING", kind: "candidate_relation",
    status: "Standing", provenance: "ANS-1", dependencies: [], coverage: "fixture", applicability: "fixture", horizon: "fixture" }]) }), /beyond|promotes/iu);
}
{
  const state = basic("path-distinction-preserved", [member("QO-A", "CQ-FRAME-FIELD", "path-a"), member("QO-B", "CQ-FRAME-FIELD", "path-b")]);
  reject(state, field("FIELD-2", [member("QO-A", "CQ-FRAME-FIELD", "path-a")], { regenerated_from: "manual",
    dispositions: JSON.stringify({ "QO-B": "Redundant" }), removal_evidence: JSON.stringify({ "QO-B": "same signature" }) }), /deduplicates/iu);
}
{
  const state = basic("effect-order-proof");
  reject(state, { kind: "route", source_occurrence: "QO-1", answer: "a", successor_occurrence: "QO-2",
    provenance: "fixture", order_exchange: "true" }, /effect_proof/iu);
}
{
  const state = basic("fold-needs-evidence", [member("QO-A"), member("QO-B", "CQ-FRAME-FIELD", "path-b")]);
  reject(state, { kind: "fold", fold_id: "F-1", members: JSON.stringify(["QO-A", "QO-B"]), representative: "QO-A",
    protected_equivalence_evidence: "none", regeneration: "none", reopen_condition: "new discriminator",
    horizon: "fixture", coverage: "fixture" }, /independent evidence/iu);
}
{
  const state = basic("fold-reopen", [member("QO-A"), member("QO-B", "CQ-FRAME-FIELD", "path-b")]);
  append(state, { kind: "fold", fold_id: "F-1", members: JSON.stringify(["QO-A", "QO-B"]), representative: "QO-A",
    protected_equivalence_evidence: "checker:protected equality", regeneration: "generator:both occurrences",
    reopen_condition: "new discriminator", horizon: "fixture", coverage: "fixture" });
  append(state, field("FIELD-2", [member("QO-A")], { regenerated_from: "F-1" }));
  reject(state, field("FIELD-3", [member("QO-A"), member("QO-B", "CQ-FRAME-FIELD", "path-b")], { regenerated_from: "new-discriminator" }), /without a reopen/iu);
  append(state, { kind: "reopen", fold_id: "F-1", restored_members: JSON.stringify(["QO-B"]),
    discriminator: "path-sensitive checker", evidence: "checker:new partition" });
  append(state, field("FIELD-3", [member("QO-A"), member("QO-B", "CQ-FRAME-FIELD", "path-b")], { regenerated_from: "F-1-reopen" }));
}

// Active manifest rejects semantic scheduling or scoring additions.
{
  const original = fs.readFileSync(manifestPath, "utf8");
  const candidate = JSON.parse(original); candidate.active_lifecycle.method_dispatch = { Generic: ["oracle"] };
  fs.writeFileSync(manifestPath, `${JSON.stringify(candidate, null, 2)}\n`);
  const result = spawnSync(process.execPath, [path.join(hooks, "ic-question-program.js"), "summary", sandbox], { encoding: "utf8", windowsHide: true });
  assert.notEqual(result.status, 0); assert.match(result.stderr, /fixed method dispatch/iu);
  candidate.active_lifecycle.method_dispatch = undefined; candidate.active_lifecycle.universal_question_score = "score";
  fs.writeFileSync(manifestPath, `${JSON.stringify(candidate, null, 2)}\n`);
  const scored = spawnSync(process.execPath, [path.join(hooks, "ic-question-program.js"), "summary", sandbox], { encoding: "utf8", windowsHide: true });
  assert.notEqual(scored.status, 0); assert.match(scored.stderr, /fixed method dispatch|scheduling/iu);
  fs.writeFileSync(manifestPath, original);
}

// A schema migration cannot erase predecessor controls without controlled ancestry.
{
  const trace = path.join(sandbox, "migration-without-control.jsonl"); const fuel = `${trace}.fuel`;
  fs.writeFileSync(trace, ""); fs.writeFileSync(fuel, "24");
  const state = { trace, fuel };
  append(state, { kind: "policy", question_program_schema: "3", source_digest: digest(sourcePath), program_manifest_digest: digest(manifestPath) });
  reject(state, { kind: "policy_transition", question_program_schema: "4", source_digest: digest(sourcePath),
    program_manifest_digest: digest(manifestPath), predecessor_source_digest: digest(sourcePath),
    predecessor_program_manifest_digest: digest(manifestPath), authority: "generated", reason: "replace protections" }, /controlled|cross-schema/iu);
}

// A completed answer can retire the final occurrence: an empty field is not an
// error. Incompleteness, by contrast, stays represented even after reification.
{
  const state = basic("complete-answer-empty-field");
  append(state, ask("FIELD-1", "QO-1"));
  append(state, answer("QO-1", "ANS-1"));
  append(state, reify("ANS-1"));
  append(state, field("FIELD-2", [], { regenerated_from: "ANS-1",
    dispositions: JSON.stringify({ "QO-1": "Answered" }), removal_evidence: JSON.stringify({ "QO-1": "ANS-1" }) }));
  append(state, { kind: "checkpoint", field_id: "FIELD-2", established: "complete fixture answer retired",
    remains_open: "task closure is separately warranted", fold_changes: "none", reopen_changes: "none", coverage: "fixture" });
}
{
  const state = basic("partial-answer-preserved");
  append(state, ask("FIELD-1", "QO-1"));
  append(state, answer("QO-1", "ANS-PART", { resolution_class: "Partial", residual: "unanswered-component" }));
  append(state, reify("ANS-PART"));
  append(state, field("FIELD-2", [member("QO-1", "CQ-FRAME-FIELD", "path-a", { disposition: "Required" })], {
    regenerated_from: "ANS-PART" }));
}
{
  const state = basic("inapplicability-not-self-warrant");
  reject(state, field("FIELD-2", [], { regenerated_from: "manual",
    dispositions: JSON.stringify({ "QO-1": "Inapplicable" }), removal_evidence: JSON.stringify({ "QO-1": "typed:assertion" }) }),
  /unsupported retirement disposition/iu);
}
{
  const state = basic("fold-cannot-drop-representative", [member("QO-A"), member("QO-B", "CQ-FRAME-FIELD", "path-b")]);
  append(state, { kind: "fold", fold_id: "F-1", members: JSON.stringify(["QO-A", "QO-B"]), representative: "QO-A",
    protected_equivalence_evidence: "checker:protected equality", regeneration: "generator:both occurrences",
    reopen_condition: "new discriminator", horizon: "fixture", coverage: "fixture" });
  reject(state, field("FIELD-2", [], { regenerated_from: "F-1" }), /silently removes/iu);
}

// Evidence must resolve ancestry, not merely look like a nonempty reference.
// Run every contrast independently so a single early escape cannot hide siblings.
const retirementEscapes = [];
const retirementCases = [
  ["fabricated-answer-retirement", (state) => {
    reject(state, field("FIELD-2", [member("QO-2", "CQ-OPEN-POSITION", "path-b")], {
      regenerated_from: "manual", dispositions: JSON.stringify({ "QO-1": "Answered" }),
      removal_evidence: JSON.stringify({ "QO-1": "ANS-NEVER-OCCURRED" }),
    }), /matching|Answer|evidence/iu);
  }],
  ["unrecognized-retirement-disposition", (state) => {
    reject(state, field("FIELD-2", [member("QO-2", "CQ-OPEN-POSITION", "path-b")], {
      regenerated_from: "manual", dispositions: JSON.stringify({ "QO-1": "Uninteresting" }),
      removal_evidence: JSON.stringify({ "QO-1": "policy:ignore" }),
    }), /disposition|retirement/iu);
  }],
  ["same-occurrence-different-path", (state) => {
    reject(state, field("FIELD-2", [member("QO-1", "CQ-FRAME-FIELD", "alien-path"),
      member("QO-2", "CQ-OPEN-POSITION", "path-b")], { regenerated_from: "manual" }), /identity|path/iu);
  }],
  ["partial-answer-retired-as-complete", (state) => {
    append(state, ask("FIELD-1", "QO-1"));
    append(state, answer("QO-1", "ANS-PART", { resolution_class: "Partial", residual: "unanswered-component" }));
    append(state, reify("ANS-PART"));
    reject(state, field("FIELD-2", [member("QO-2", "CQ-OPEN-POSITION", "path-b")], {
      regenerated_from: "ANS-PART", dispositions: JSON.stringify({ "QO-1": "Answered" }),
      removal_evidence: JSON.stringify({ "QO-1": "ANS-PART" }),
    }), /Partial|complete|unresolved/iu);
  }],
  ["foreign-answer-retirement", (state) => {
    append(state, ask("FIELD-1", "QO-1")); append(state, answer("QO-1", "ANS-1")); append(state, reify("ANS-1"));
    reject(state, field("FIELD-2", [member("QO-1")], { regenerated_from: "ANS-1",
      dispositions: JSON.stringify({ "QO-2": "Answered" }), removal_evidence: JSON.stringify({ "QO-2": "ANS-1" }),
    }), /matching|Answer|evidence/iu);
  }],
  ["generate-answer-authority-laundering", (state) => {
    append(state, ask("FIELD-1", "QO-1", "CQ-FRAME-FIELD", "Generate"));
    reject(state, answer("QO-1", "ANS-1", { status: "warranted" }), /Generate|authority|provisional/iu);
  }],
  ["field-id-reuse", (state) => {
    reject(state, field("FIELD-1", [member("QO-1"), member("QO-2", "CQ-OPEN-POSITION", "path-b")]), /field.*id|field.*identity/iu);
  }],
  ["ask-occurrence-reuse", (state) => {
    append(state, ask("FIELD-1", "QO-1")); append(state, answer("QO-1", "ANS-1")); append(state, reify("ANS-1"));
    append(state, field("FIELD-2", [member("QO-1"), member("QO-2", "CQ-OPEN-POSITION", "path-b")], { regenerated_from: "ANS-1" }));
    reject(state, ask("FIELD-2", "QO-1"), /occurrence|already/iu);
  }],
];
for (const [name, check] of retirementCases) {
  try {
    const state = basic(name, [member("QO-1"), member("QO-2", "CQ-OPEN-POSITION", "path-b")]);
    check(state);
    process.stdout.write(`retirement boundary rejected: ${name}\n`);
  } catch (error) {
    retirementEscapes.push(`${name}: ${error.message}`);
  }
}
assert.deepEqual(retirementEscapes, [], `lifecycle evidence/identity escapes:\n${retirementEscapes.join("\n")}`);

// Exercise the real raw CLI, including GNU's escaped-filename case. The content
// digest must be independent of path spelling, not just a nonempty token.
{
  const state = basic("raw-backslash-path");
  append(state, ask("FIELD-1", "QO-1", "CQ-FRAME-FIELD", "Probe"));
  append(state, { kind: "seal", ask_occurrence: "QO-1", should_change: "canonical digest",
    invariants: "unchanged raw bytes", discriminator: "independent content hash", wrong_impl: "filename-escaped token", coverage: "backslash path" });
  const traceDir = path.join(sandbox, ".claude", "trace");
  fs.mkdirSync(traceDir, { recursive: true });
  fs.copyFileSync(state.trace, path.join(traceDir, "raw-path.jsonl"));
  fs.writeFileSync(path.join(traceDir, ".state"), "raw-path.jsonl");
  fs.writeFileSync(path.join(traceDir, ".fuel"), "23");
  const rawPath = path.join(sandbox, process.platform === "win32" ? "raw-return.txt" : "raw\\return.txt");
  fs.writeFileSync(rawPath, "preserved bytes\n");
  const bash = process.platform === "win32" ? path.join(process.env.ProgramFiles, "Git", "bin", "bash.exe") : "bash";
  const result = spawnSync(bash, [path.join(hooks, "ic-trace"), "raw", "ask_occurrence=QO-1",
    "cmd=fixture", `file=${rawPath}`, "sensitive=false"], { cwd: sandbox, encoding: "utf8", windowsHide: true });
  assert.equal(result.status, 0, `${result.stdout}${result.stderr}`);
  const raw = JSON.parse(fs.readFileSync(path.join(traceDir, "raw-path.jsonl"), "utf8").trim().split(/\r?\n/u).at(-1));
  assert.match(raw.digest, /^[0-9a-f]{64}$/u);
  assert.equal(raw.digest, digest(rawPath));
  assert.equal(digest(path.join(sandbox, raw.raw_ref)), raw.digest);
}
function closure(fieldId = "FIELD-2") {
  return { kind: "closure", field_id: fieldId, scope: "fixture", warrant: "independent:test authority",
    adversarial_question: "QO-1", adversarial_answer: "ANS-1", coverage: "fixture" };
}

function checkedClosure(name) {
  const state = basic(name);
  append(state, ask("FIELD-1", "QO-1", "CQ-FRAME-FIELD", "Check"));
  append(state, { kind: "check", ask_occurrence: "QO-1", verdict: "no admitted breaker in fixture",
    coverage: "fixture", evidence: "independent:test discriminator" });
  append(state, answer("QO-1", "ANS-1", { status: "checked" }));
  append(state, reify("ANS-1"));
  append(state, field("FIELD-2", [], { regenerated_from: "ANS-1",
    dispositions: JSON.stringify({ "QO-1": "Answered" }), removal_evidence: JSON.stringify({ "QO-1": "ANS-1" }) }));
  append(state, closure());
  return state;
}

function foldedPair(name) {
  const state = basic(name, [member("QO-1"), member("QO-2", "CQ-FRAME-FIELD", "path-b")]);
  append(state, { kind: "fold", fold_id: "F-1", members: JSON.stringify(["QO-1", "QO-2"]), representative: "QO-1",
    protected_equivalence_evidence: "checker:fixture", regeneration: "generator:fixture", reopen_condition: "new discriminator",
    horizon: "fixture", coverage: "fixture" });
  append(state, field("FIELD-2", [member("QO-1")], { regenerated_from: "F-1" }));
  return state;
}

const closureEscapes = [];
const closureCases = [
  ["fabricated-final-challenge", () => {
    const state = basic("fabricated-final-challenge", []);
    reject(state, closure("FIELD-1"), /adversarial|matching|Answer/iu);
  }],
  ["stale-closure-after-new-ask", () => {
    const state = checkedClosure("stale-closure-after-new-ask");
    append(state, field("FIELD-3", [member("QO-NEW")], { regenerated_from: "new evidence" }));
    append(state, ask("FIELD-3", "QO-NEW"));
    reject(state, { kind: "stop", state: "Satisfied", warrant: "independent:test authority" }, /unresolved|stale|closure/iu);
  }],
  ["self-warrant-after-closure", () => {
    const state = checkedClosure("self-warrant-after-closure");
    reject(state, { kind: "stop", state: "Satisfied", warrant: "self:green" }, /warrant/iu);
  }],
  ["ask-before-reopened-members-restored", () => {
    const state = foldedPair("ask-before-reopened-members-restored");
    append(state, { kind: "reopen", fold_id: "F-1", restored_members: JSON.stringify(["QO-2"]),
      discriminator: "path-sensitive", evidence: "independent:new distinction" });
    reject(state, ask("FIELD-2", "QO-1"), /reopen|regenerat|restore/iu);
  }],
  ["empty-reopening", () => {
    const state = foldedPair("empty-reopening");
    reject(state, { kind: "reopen", fold_id: "F-1", restored_members: "[]",
      discriminator: "path-sensitive", evidence: "independent:new distinction" }, /restore|reopen|member/iu);
  }],
  ["uncontrolled-schema4-policy-change", () => {
    const state = basic("uncontrolled-schema4-policy-change");
    reject(state, { kind: "policy_transition", question_program_schema: "4", source_digest: digest(sourcePath),
      program_manifest_digest: "f".repeat(64), predecessor_source_digest: digest(sourcePath),
      predecessor_program_manifest_digest: digest(manifestPath), authority: "generated", reason: "bypass old checker" }, /control|policy/iu);
  }],
  ["schema4-downgrade", () => {
    const state = basic("schema4-downgrade");
    reject(state, { kind: "policy_transition", question_program_schema: "3", source_digest: digest(sourcePath),
      program_manifest_digest: digest(manifestPath), predecessor_source_digest: digest(sourcePath),
      predecessor_program_manifest_digest: digest(manifestPath), authority: "user fixture", reason: "remove lifecycle" }, /downgrade|schema|control/iu);
  }],
];
for (const [name, check] of closureCases) {
  try { check(); process.stdout.write(`closure boundary rejected: ${name}\n`); }
  catch (error) { closureEscapes.push(`${name}: ${error.message}`); }
}
assert.deepEqual(closureEscapes, [], `closure/reopening/policy escapes:\n${closureEscapes.join("\n")}`);

function hook(state, name, args = [], input = "", extraEnv = {}) {
  const directory = path.join(sandbox, ".claude", "trace");
  fs.mkdirSync(directory, { recursive: true });
  fs.copyFileSync(state.trace, path.join(directory, "hook-state.jsonl"));
  fs.writeFileSync(path.join(directory, ".state"), "hook-state.jsonl");
  fs.writeFileSync(path.join(directory, ".fuel"), fs.readFileSync(state.fuel));
  const bash = process.platform === "win32" ? path.join(process.env.ProgramFiles, "Git", "bin", "bash.exe") : "bash";
  return spawnSync(bash, [path.join(hooks, name), ...args], {
    cwd: sandbox, input, encoding: "utf8", windowsHide: true, env: { ...process.env, ...extraEnv },
  });
}

function snapshot(state) {
  // Use the same validator as append, not fixture-side event counting.
  const raw = spawnSync(process.execPath, [appendProgram, "state", state.trace], { encoding: "utf8", windowsHide: true });
  assert.equal(raw.status, 0, raw.stderr);
  return JSON.parse(raw.stdout);
}

// The public launcher derives the checkpoint and field from validated state;
// callers cannot choose a different history coordinate or fuel amount.
{
  const state = basic("checkpoint-resume-launcher");
  append(state, { kind: "control", authority: "user fixture", residual: "persistent field",
    predecessor: "prior ratchet", scope: "harness" });
  append(state, { kind: "checkpoint", field_id: "FIELD-1", established: "clean",
    remains_open: "QO-1", fold_changes: "none", reopen_changes: "none", coverage: "fixture" });
  fs.writeFileSync(state.fuel, "0");
  const resumed = hook(state, "ic-trace", ["resume", "reason=continue autonomous task"]);
  assert.equal(resumed.status, 0, resumed.stderr);
  const traceDirectory = path.join(sandbox, ".claude", "trace");
  assert.equal(fs.readFileSync(path.join(traceDirectory, ".fuel"), "utf8"), "24");
  const records = fs.readFileSync(path.join(traceDirectory, "hook-state.jsonl"), "utf8")
    .trimEnd().split(/\r?\n/u).map(JSON.parse);
  assert.equal(records.at(-1).kind, "note");
  assert.equal(records.at(-1).event, "checkpoint_resume");
  assert.equal(records.at(-1).checkpoint, String(snapshot(state).last_checkpoint));
}

// A non-successful closure preserves unanswered material, including when the
// admitted final challenge itself is unavailable or resource-bounded.
for (const outcome of ["Unknown", "Blocked", "ResourceBounded"]) {
  const state = basic(`explicit-${outcome}-closure`);
  append(state, ask("FIELD-1", "QO-1"));
  append(state, answer("QO-1", "ANS-1", { resolution_class: outcome, polarity: "None", residual: "unresolved-component" }));
  append(state, reify("ANS-1"));
  append(state, field("FIELD-2", [member("QO-1", "CQ-FRAME-FIELD", "path-a", { disposition: outcome, executable: false })], { regenerated_from: "ANS-1" }));
  reject(state, closure(), /checked adversarial Answer/iu);
  append(state, { ...closure(), state: outcome });
  reject(state, { kind: "stop", state: "Satisfied", warrant: "independent:fixture" }, /closure outcome|unresolved field/iu);
  append(state, { kind: "stop", state: outcome, warrant: "explicit retained unanswered component" });
  assert.equal(snapshot(state).can_initialize, true);
}

// Positive flow and consumer propagation: lifecycle-open != mutation-authorized.
{
  const state = basic("hook-probe-lifecycle", [member("QO-1"), member("QO-2")]);
  append(state, ask("FIELD-1", "QO-1", "CQ-FRAME-FIELD", "Probe"));
  assert.equal(snapshot(state).open, true); assert.equal(snapshot(state).mutation_open, false);
  const edit = JSON.stringify({ tool_name: "Edit", tool_input: { file_path: path.join(sandbox, "ordinary.txt") } });
  assert.match(hook(state, "ic-guard", [], edit).stdout, /permissionDecision":"deny/u);
  append(state, { kind: "seal", ask_occurrence: "QO-1", should_change: "fixture", invariants: "identity",
    discriminator: "fixture", wrong_impl: "stale seal", coverage: "fixture" });
  assert.equal(hook(state, "ic-trace", ["mutation-open"]).status, 0);
  assert.equal(hook(state, "ic-guard", [], edit).stdout, "");
  append(state, { kind: "raw", ask_occurrence: "QO-1", cmd: "fixture", digest: "a".repeat(64), raw_ref: "digest-only:sensitive", sensitive: "true" });
  append(state, { kind: "interpret", ask_occurrence: "QO-1", raw_digest: "a".repeat(64), interpretation: "fixture", provenance: "fixture" });
  append(state, { kind: "check", ask_occurrence: "QO-1", verdict: "fixture", coverage: "fixture", evidence: "independent:fixture" });
  append(state, answer("QO-1", "ANS-1", { status: "checked" }));
  assert.equal(snapshot(state).open, true); assert.equal(snapshot(state).mutation_open, false);
  assert.match(hook(state, "ic-guard", [], edit).stdout, /permissionDecision":"deny/u);
  append(state, reify("ANS-1"));
  append(state, field("FIELD-2", [member("QO-2")], { regenerated_from: "ANS-1",
    dispositions: JSON.stringify({ "QO-1": "Answered" }), removal_evidence: JSON.stringify({ "QO-1": "ANS-1" }) }));
  assert.equal(snapshot(state).open, false); assert.equal(snapshot(state).stop_pending, true);
  assert.equal(hook(state, "ic-trace", ["open"]).status, 1);
  assert.match(hook(state, "ic-stop", [], '{"stop_hook_active":true}').stdout, /"decision":"block"/u);
  assert.notEqual(hook(state, "ic-trace", ["init", "cannot-hide-checkpoint"]).status, 0);
}
{
  const state = checkedClosure("lawful-stop-and-recurrence");
  append(state, { kind: "stop", state: "Satisfied", warrant: "independent:test authority" });
  assert.equal(snapshot(state).can_initialize, true);
  assert.equal(hook(state, "ic-stop", [], '{"stop_hook_active":true}').stdout, "");
  const initialized = hook(state, "ic-trace", ["init", "cannot-downgrade"], "", { IC_TRACE_SCHEMA: "3" });
  assert.equal(initialized.status, 0, initialized.stderr);
  const dir = path.join(sandbox, ".claude", "trace"), name = fs.readFileSync(path.join(dir, ".state"), "utf8");
  const newPolicy = JSON.parse(fs.readFileSync(path.join(dir, name), "utf8").trim());
  assert.equal(newPolicy.question_program_schema, "4");
  assert.equal(newPolicy.predecessor_trace_sha256, digest(state.trace));
  assert.equal(newPolicy.predecessor_policy_schema, "4");
  append(state, field("FIELD-3", [member("QO-NEW")], { regenerated_from: "new evidence" }));
  assert.equal(snapshot(state).stop_pending, true); assert.equal(snapshot(state).can_initialize, false);
  assert.match(hook(state, "ic-stop", [], '{"stop_hook_active":true}').stdout, /"decision":"block"/u);
  assert.notEqual(hook(state, "ic-trace", ["init", "cannot-hide-new-work"]).status, 0);
}
{
  const state = foldedPair("reopening-projection");
  append(state, { kind: "reopen", fold_id: "F-1", restored_members: '["QO-2"]', discriminator: "path-sensitive", evidence: "independent:fixture" });
  assert.deepEqual(snapshot(state).restore_required, ["QO-2"]);
  hook(state, "ic-trace", ["status"]);
  const projected = spawnSync(process.execPath, [path.join(hooks, "ic-relational-surface.js"), "json", sandbox], { encoding: "utf8", windowsHide: true });
  assert.equal(projected.status, 0, projected.stderr);
  assert.equal(JSON.parse(projected.stdout).surface_dirty, true);
  append(state, field("FIELD-3", [member("QO-1"), member("QO-2", "CQ-FRAME-FIELD", "path-b")], { regenerated_from: "F-1-reopen" }));
  assert.equal(snapshot(state).surface_dirty, false);
  append(state, ask("FIELD-3", "QO-2", "CQ-FRAME-FIELD", "Pure", "path-b"));
}
{
  const original = fs.readFileSync(manifestPath, "utf8");
  const state = basic("controlled-schema4-policy-transition");
  append(state, { kind: "control", authority: "user fixture", residual: "policy evolution", predecessor: "original manifest", scope: "harness" });
  append(state, ask("FIELD-1", "QO-1", "CQ-FRAME-FIELD", "Probe"));
  append(state, { kind: "seal", ask_occurrence: "QO-1", should_change: "policy", invariants: "old question ancestry", discriminator: "new pin", wrong_impl: "silent change", coverage: "fixture" });
  const oldDigest = digest(manifestPath);
  try {
    const changed = JSON.parse(original); changed.transition_fixture = "evidence remains policy-bound";
    fs.writeFileSync(manifestPath, JSON.stringify(changed));
    append(state, { kind: "policy_transition", question_program_schema: "4", source_digest: digest(sourcePath),
      program_manifest_digest: digest(manifestPath), predecessor_source_digest: digest(sourcePath),
      predecessor_program_manifest_digest: oldDigest, authority: "user fixture", reason: "controlled compatible manifest extension" });
    assert.equal(snapshot(state).schema, "4");
    append(state, { kind: "raw", ask_occurrence: "QO-1", cmd: "fixture", digest: "a".repeat(64), raw_ref: "digest-only:sensitive", sensitive: "true" });
    reject(state, { kind: "policy_transition", question_program_schema: "4", source_digest: digest(sourcePath),
      program_manifest_digest: digest(manifestPath), predecessor_source_digest: digest(sourcePath),
      predecessor_program_manifest_digest: digest(manifestPath), authority: "user fixture", reason: "after return" }, /pre-return/iu);
  } finally { fs.writeFileSync(manifestPath, original); }
}
process.stdout.write(`active inquiry lifecycle checks passed (${scenarioCount} isolated traces)\n`);
