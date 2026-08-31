#!/usr/bin/env node
"use strict";

// Independent finite report fixtures exercise real append, replay, projection and
// reopening. They certify this engineering scope, not LLM semantic equivalence.
const assert = require("node:assert/strict");
const fs = require("node:fs"), path = require("node:path"), os = require("node:os");
const cp = require("node:child_process"), crypto = require("node:crypto");
const repo = path.resolve(__dirname, "..");
const root = fs.mkdtempSync(path.join(os.tmpdir(), "ic-fold-evidence-"));
fs.cpSync(path.join(repo, ".claude/hooks"), path.join(root, ".claude/hooks"), { recursive: true });
fs.mkdirSync(path.join(root, ".claude/trace/raw"), { recursive: true });
fs.mkdirSync(path.join(root, "formal-successor"));
for (const name of ["Questions.txt", "ENGINEERING_QUESTION_PROGRAMS.json", "PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS.md",
  "PREFORMAL_SEARCH_ASYMMETRY.md", "SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md", "QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md", "RESIDUAL_OBLIGATIONS.json"]) {
  fs.copyFileSync(path.join(repo, "formal-successor", name), path.join(root, "formal-successor", name));
}
fs.copyFileSync(path.join(repo, "IMPLEMENTATION_FRONTIER.md"), path.join(root, "IMPLEMENTATION_FRONTIER.md"));
const manifestPath = path.join(root, "formal-successor/ENGINEERING_QUESTION_PROGRAMS.json");
const manifest = JSON.parse(fs.readFileSync(manifestPath));
assert.equal(manifest.active_lifecycle.fold_evidence.schema, 2);
const hash = (b) => crypto.createHash("sha256").update(b).digest("hex");
const digest = (p) => hash(fs.readFileSync(p));
const appendPath = path.join(root, ".claude/hooks/ic-append.js");
const evidence = require(path.join(root, ".claude/hooks/ic-fold-evidence.js"));
const trace = path.join(root, ".claude/trace/fold.jsonl"), fuel = path.join(root, ".claude/trace/.fuel");
fs.writeFileSync(trace, ""); fs.writeFileSync(fuel, "30"); fs.writeFileSync(path.join(root, ".claude/trace/.state"), "fold.jsonl");
const read = () => fs.readFileSync(trace, "utf8").trim().split("\n").filter(Boolean).map(JSON.parse);
const exec = (args, input) => cp.spawnSync(process.execPath, [appendPath, ...args], { cwd: root, input, encoding: "utf8", windowsHide: true });
function append(r) { const x = exec(["append", trace, fuel], JSON.stringify(r) + "\n"); assert.equal(x.status, 0, x.stderr); }
function reject(r, pattern) { const before = fs.readFileSync(trace); const x = exec(["append", trace, fuel], JSON.stringify(r) + "\n"); assert.notEqual(x.status, 0); assert.match(x.stderr, pattern); assert.deepEqual(fs.readFileSync(trace), before); }
const state = () => { const x = exec(["state", trace]); assert.equal(x.status, 0, x.stderr); return JSON.parse(x.stdout); };
const policy = { kind: "policy", question_program_schema: "4", source_digest: digest(path.join(root, "formal-successor/Questions.txt")), program_manifest_digest: digest(manifestPath) };
append(policy); assert.equal(state().fold_evidence_schema, "2");
const form = manifest.preformal_harness.compiled_questions.find((f) => f.id === "CQ-ACTUAL-DISCHARGE");
const family = manifest.preformal_harness.program_families.find((f) => f.id === form.family);
function member(id, p = id) { return { occurrence: id, question_form: form.id, rendering: `RENDER-${form.id}`, prompt: form.prompt,
  source_lines: form.source_lines, generator_ids: manifest.active_lifecycle.generator_registry.filter((g) => g.question_forms.includes(form.id)).map((g) => g.id),
  path: p, disposition: "Required", executable: true, dependencies: [] }; }
const a = member("Q-A", "path-a"), b = member("Q-B", "path-b");
let fieldId = "FIELD-0", fieldMembers = [a, b];
function field(members, from = "bootstrap", retirement = {}) {
  fieldId = `FIELD-${read().length}`; fieldMembers = members;
  append({ kind: "field", field_id: fieldId, members: JSON.stringify(members), basis: "fixture: finite declared observations and exact ancestry",
    coverage: "fixture", regenerated_from: from, dispositions: JSON.stringify(Object.fromEntries(Object.keys(retirement).map((id) => [id, "Answered"]))), removal_evidence: JSON.stringify(retirement) });
}
field(fieldMembers);
function reportBytes(report) {
  const bytes = Buffer.from(JSON.stringify(report)); const d = hash(bytes);
  fs.writeFileSync(path.join(root, ".claude/trace/raw", d), bytes); return d;
}
function probe(id, reports, products, finishField = true) {
  const q = member(id); field([...fieldMembers, q], "fixture-extension");
  append({ ...policy, kind: "ask", occurrence: id, fp: id, q: q.prompt, mode: "Probe", field_id: fieldId,
    question_form: q.question_form, rendering: q.rendering, path: q.path, source_lines: q.source_lines.join(","), generator_ids: q.generator_ids.join(","),
    reciprocal_relations: family.reciprocal_challenges.join(","), bindings: "fixture", horizon: "fixture", coverage: "fixture", authority: "independent fixture", evidence: "explicit finite observation contract", dependencies: "none" });
  append({ kind: "seal", ask_occurrence: id, should_change: "report resolves exact target", invariants: "source occurrence preserved", discriminator: "independent finite fixture", wrong_impl: "forged observations", coverage: "fixture" });
  const digests = reports.map(reportBytes);
  for (const d of digests) {
    append({ kind: "raw", ask_occurrence: id, cmd: "independent finite fixture report", digest: d, raw_ref: `.claude/trace/raw/${d}`, sensitive: "false" });
    append({ kind: "interpret", ask_occurrence: id, raw_digest: d, interpretation: "declared observations/reconstruction", provenance: d });
  }
  const ev = digests.join(";");
  append({ kind: "check", ask_occurrence: id, verdict: "scoped fixture checked", coverage: "fixture", evidence: ev });
  append({ kind: "answer", occurrence: `ANS-${id}`, ask_occurrence: id, answer: "scoped reports", status: "checked", resolution_class: "Supported", polarity: "Positive", residual: "semantic adequacy outside fixture", evidence: ev, coverage: "fixture", authority: "independent fixture" });
  append({ kind: "reify", answer_occurrence: `ANS-${id}`, status: "checked", products: JSON.stringify(products(digests)), new_questions: "retain unchosen questions", coverage: "fixture" });
  if (finishField) field(fieldMembers.filter((m) => m.occurrence !== id), `ANS-${id}`, { [id]: `ANS-${id}` });
}
const product = (id, dependencies = []) => ({ id, kind: "checked_fixture_evidence", status: "checked", dependencies, provenance: "actual fixture Raw", coverage: "fixture", applicability: "fixture", horizon: "fixture" });
const execution = { schema: 1, language: "question_identity_projection", field: "question_form" };
const protectionReport = { schema: 2, kind: "protected_continuation", claim: { targets: [a.occurrence, b.occurrence], execution },
  observations: [a, b].map((m) => ({ member: m.occurrence, status: "Supported", value: m.question_form })) };
probe("Q-PROTECTION", [protectionReport], ([d]) => [
  product("BASE-SUPPORT"),
  { ...product("C-1", ["BASE-SUPPORT"]), inquiry_protection: { schema: 2, targets: [a.occurrence, b.occurrence], execution, raw_digest: d } },
]);
const claim = (relation) => ({ schema: 2, relation, members: [a.occurrence, b.occurrence], representative: a.occurrence,
  member_identities: { [a.occurrence]: evidence.identityDigest(a), [b.occurrence]: evidence.identityDigest(b) },
  continuations: ["C-1"], horizon: "fixture", coverage: "fixture" });
const eqReport = { schema: 2, kind: "fold_check", claim: claim("protected_equivalence"),
  observations: [a, b].map((m) => ({ member: m.occurrence, continuation: "C-1", status: "Supported", value: m.question_form })) };
const regenerationReport = { schema: 2, kind: "fold_check", claim: claim("regeneration"),
  regenerated_members: { [a.occurrence]: evidence.identity(a), [b.occurrence]: evidence.identity(b) } };
probe("Q-EVIDENCE", [eqReport, regenerationReport], ([eq, regen]) => [
  { ...product("E-EQ", ["C-1"]), fold_evidence: { ...eqReport.claim, raw_digest: eq } },
  { ...product("E-REGEN", ["C-1"]), fold_evidence: { ...regenerationReport.claim, raw_digest: regen } },
]);
const fold = { kind: "fold", fold_id: "F-1", members: JSON.stringify([a.occurrence, b.occurrence]), representative: a.occurrence,
  protected_equivalence_evidence: "E-EQ", regeneration: "E-REGEN", protected_continuations: '["C-1"]', reopen_condition: "new protected continuation or invalidated support", horizon: "fixture", coverage: "fixture" };
for (const mutation of [
  { protected_equivalence_evidence: "checker:NEVER-EXISTED" }, { regeneration: "generator:NEVER-EXISTED" },
  { horizon: "unrelated" }, { coverage: "unrelated" }, { representative: b.occurrence },
  { protected_continuations: '[]' }, { protected_continuations: '["made-up"]' }, { regeneration: "E-EQ" },
]) reject({ ...fold, ...mutation }, /fold evidence/iu);
const beforeFold = fs.readFileSync(trace, "utf8");
// Mutate isolated, otherwise valid histories/reports. Rehashing a changed report
// cannot conceal lost observation coverage or changed reconstruction semantics.
let rejected = 8;
function mutated(name, change, pattern = /fold evidence|ENOENT/u) {
  const records = beforeFold.trim().split("\n").map(JSON.parse); change(records);
  const file = path.join(root, `.claude/trace/mutant-${name}.jsonl`); fs.writeFileSync(file, records.map(JSON.stringify).join("\n") + "\n");
  const r = exec(["validate", file]); assert.notEqual(r.status, 0, `escaped ${name}`); assert.match(r.stderr, pattern); rejected++;
}
function mutateReport(records, index, change) {
  const raws = records.filter((r) => r.kind === "raw"); const raw = raws[index]; const old = raw.digest;
  const report = JSON.parse(fs.readFileSync(path.join(root, raw.raw_ref))); change(report); const replacement = reportBytes(report);
  for (const r of records) for (const key of Object.keys(r)) if (typeof r[key] === "string") r[key] = r[key].replaceAll(old, replacement);
}
mutated("lost-cell", (rs) => mutateReport(rs, 1, (r) => r.observations.pop()));
mutated("duplicate-cell", (rs) => mutateReport(rs, 1, (r) => r.observations[1] = r.observations[0]));
mutated("unknown-cell", (rs) => mutateReport(rs, 1, (r) => r.observations[1].status = "Unknown"));
mutated("different-result", (rs) => mutateReport(rs, 1, (r) => r.observations[1].value = "different"));
mutated("lost-path", (rs) => mutateReport(rs, 2, (r) => r.regenerated_members["Q-B"].path = "path-a"));
mutated("missing-member", (rs) => mutateReport(rs, 2, (r) => delete r.regenerated_members["Q-B"]));
mutated("foreign-raw", (rs) => { rs.find((r) => r.kind === "raw" && r.ask_occurrence === "Q-EVIDENCE").ask_occurrence = "Q-PROTECTION"; }, /matching|fold evidence/u);
mutated("unrelated-check", (rs) => { rs.find((r) => r.kind === "check" && r.ask_occurrence === "Q-EVIDENCE").evidence = "some other checker"; });
mutated("partial-answer", (rs) => { rs.find((r) => r.kind === "answer" && r.ask_occurrence === "Q-EVIDENCE").resolution_class = "Partial"; });
mutated("empty-answer", (rs) => { rs.find((r) => r.kind === "answer" && r.ask_occurrence === "Q-EVIDENCE").resolution_class = "ExactEmpty"; });
mutated("negative-answer", (rs) => { rs.find((r) => r.kind === "answer" && r.ask_occurrence === "Q-EVIDENCE").polarity = "Negative"; });
mutated("absent-raw", (rs) => { rs.find((r) => r.kind === "raw").raw_ref = ".claude/trace/raw/elsewhere"; });
mutated("wrong-member-identity", (rs) => { const r = rs.findLast((r) => r.kind === "reify"); const p = JSON.parse(r.products); p[0].fold_evidence.member_identities["Q-B"] = "0".repeat(64); r.products = JSON.stringify(p); });
mutated("false-execution-report", (rs) => {
  mutateReport(rs, 0, (r) => { r.claim.execution.field = "path"; });
  const reify = rs.find((r) => r.kind === "reify" && r.answer_occurrence === "ANS-Q-PROTECTION");
  const products = JSON.parse(reify.products); products.find((p) => p.id === "C-1").inquiry_protection.execution.field = "path";
  reify.products = JSON.stringify(products);
}, /first-order execution/u);
mutated("missing-execution-program", (rs) => {
  mutateReport(rs, 0, (r) => { delete r.claim.execution; });
  const reify = rs.find((r) => r.kind === "reify" && r.answer_occurrence === "ANS-Q-PROTECTION");
  const products = JSON.parse(reify.products); delete products.find((p) => p.id === "C-1").inquiry_protection.execution;
  reify.products = JSON.stringify(products);
}, /requires exactly/u);
mutated("unsupported-execution-program", (rs) => {
  mutateReport(rs, 0, (r) => { r.claim.execution.field = "disposition"; });
  const reify = rs.find((r) => r.kind === "reify" && r.answer_occurrence === "ANS-Q-PROTECTION");
  const products = JSON.parse(reify.products); products.find((p) => p.id === "C-1").inquiry_protection.execution.field = "disposition";
  reify.products = JSON.stringify(products);
}, /non-identity field/u);
mutated("partial-continuation-applicability", (rs) => {
  mutateReport(rs, 0, (r) => { r.claim.targets.pop(); r.observations.pop(); });
  const reify = rs.find((r) => r.kind === "reify" && r.answer_occurrence === "ANS-Q-PROTECTION");
  const products = JSON.parse(reify.products); products.find((p) => p.id === "C-1").inquiry_protection.targets.pop();
  reify.products = JSON.stringify(products);
}, /not applicable to every fold member/u);
append(fold); field([a], "F-1"); assert.equal(state().folds[0].evidence_schema, 2);
const foldedTrace = fs.readFileSync(trace, "utf8");
assert.ok(state().folds[0].support.includes("BASE-SUPPORT"));
append({ kind: "invalidate", product_ids: '["BASE-SUPPORT","C-1","E-EQ","E-REGEN"]', cause: "independent transitive support withdrawn", evidence: "fixture return" });
assert.equal(state().folds[0].reopen_required, true);
reject({ kind: "field", field_id: "FORBIDDEN", members: JSON.stringify([a]), basis: "ignore support", coverage: "fixture", regenerated_from: "invalidate", dispositions: "{}", removal_evidence: "{}" }, /must reopen/u);
append({ kind: "reopen", fold_id: "F-1", restored_members: '["Q-B"]', discriminator: "withdrawn support", evidence: "fixture" });
field([a, b], "reopened-support"); assert.equal(state().open, false);
// A separate continuation branch starts from the same immutable fixture prefix.
fs.writeFileSync(trace, foldedTrace); fieldMembers = [a]; fieldId = read().findLast((r) => r.kind === "field").field_id;
const newExecution = { schema: 1, language: "question_identity_projection", field: "path" };
probe("Q-NEW-CONTINUATION", [{ schema: 2, kind: "protected_continuation", claim: { targets: [b.occurrence], execution: newExecution },
  observations: [{ member: b.occurrence, status: "Supported", value: b.path }] }], ([d]) => [
  { ...product("C-2"), inquiry_protection: { schema: 2, targets: [b.occurrence], execution: newExecution, raw_digest: d } },
], false);
assert.equal(state().folds[0].reopen_required, true);
reject({ kind: "field", field_id: "FORBIDDEN-2", members: JSON.stringify([a]), basis: "ignore continuation", coverage: "fixture", regenerated_from: "ANS-Q-NEW-CONTINUATION", dispositions: "{}", removal_evidence: "{}" }, /must reopen/u);
const surface = cp.spawnSync(process.execPath, [path.join(root, ".claude/hooks/ic-relational-surface.js"), "json", root], { encoding: "utf8", windowsHide: true });
assert.equal(surface.status, 0, surface.stderr); assert.equal(JSON.parse(surface.stdout).folds[0].reopen_required, true);
append({ kind: "reopen", fold_id: "F-1", restored_members: '["Q-B"]', discriminator: "new protected C-2", evidence: "checked continuation report" });
field([a, b], "ANS-Q-NEW-CONTINUATION", { "Q-NEW-CONTINUATION": "ANS-Q-NEW-CONTINUATION" });
assert.equal(state().surface_dirty, false); assert.equal(state().folds[0].state, "reopened");
const rawFile = path.join(root, ".claude/trace/raw", reportBytes(eqReport)); const saved = fs.readFileSync(rawFile);
fs.writeFileSync(rawFile, "tampered"); const broken = exec(["state", trace]); assert.notEqual(broken.status, 0); assert.match(broken.stderr, /Raw bytes/u); fs.writeFileSync(rawFile, saved);
const newPolicy = { ...policy, kind: "policy_transition", fold_evidence_schema: "0", predecessor_source_digest: policy.source_digest,
  predecessor_program_manifest_digest: policy.program_manifest_digest, authority: "explicit user", reason: "attempted downgrade" };
reject(newPolicy, /fold evidence policy/u);
const bash = process.platform === "win32" ? path.join(process.env.ProgramFiles, "Git/bin/bash.exe") : "bash";
function hook(name, payload) {
  const result = cp.spawnSync(bash, [path.join(root, ".claude/hooks", name)], { cwd: root, input: JSON.stringify(payload), encoding: "utf8", windowsHide: true });
  assert.equal(result.status, 0, result.stderr); return result.stdout;
}
for (const command of ["field", "ask", "interpret", "answer", "reify", "invalidate", "fold", "reopen", "checkpoint", "closure", "state", "mutation-open"]) {
  assert.equal(hook("ic-guard", { tool_name: "Bash", tool_input: { command: `.claude/hooks/ic-trace ${command}` } }), "", `legitimate ${command} cannot reach its validator`);
}
for (const command of [".claude/hooks/ic-trace reopen; echo malicious", "node .claude/hooks/ic-append.js append trace", "echo rewrite > .claude/trace/fold.jsonl"]) {
  assert.match(hook("ic-guard", { tool_name: "Bash", tool_input: { command } }), /permissionDecision":"deny/u);
}
assert.match(hook("ic-stop", { stop_hook_active: true }), /"decision":"block"/u);

const currentManifestBytes = fs.readFileSync(manifestPath);
// A complete schema-1 report-bound fold remains historically replayable, but a
// controlled 1 -> 2 transition must reopen it rather than infer execution
// correspondence that its occurrence-time policy never checked.
const schema1Manifest = JSON.parse(currentManifestBytes);
schema1Manifest.active_lifecycle.fold_evidence = { schema: 1,
  boundary: "Exact checked Raw reports, member identities, declared protected continuations and regeneration; process evidence, not semantic self-warrant",
  reopening: schema1Manifest.active_lifecycle.fold_evidence.reopening };
fs.writeFileSync(manifestPath, JSON.stringify(schema1Manifest));
const schema1Policy = { ...policy, program_manifest_digest: digest(manifestPath) };
const schema1Trace = path.join(root, ".claude/trace/schema1-migration.jsonl"); fs.writeFileSync(schema1Trace, "");
function schema1Append(r) { const x = exec(["append", schema1Trace, fuel], JSON.stringify(r) + "\n"); assert.equal(x.status, 0, x.stderr); }
schema1Append(schema1Policy);
let historicalField = "H-0";
schema1Append({ kind: "field", field_id: historicalField, members: JSON.stringify([a, b]), basis: "schema-1 fixture", coverage: "fixture",
  regenerated_from: "bootstrap", dispositions: "{}", removal_evidence: "{}" });
function schema1Probe(id, reports, products) {
  const q = member(id); historicalField = `H-${id}`;
  schema1Append({ kind: "field", field_id: historicalField, members: JSON.stringify([a, b, q]), basis: "schema-1 probe", coverage: "fixture",
    regenerated_from: "fixture-extension", dispositions: "{}", removal_evidence: "{}" });
  schema1Append({ ...schema1Policy, kind: "ask", occurrence: id, fp: `historical-${id}`, q: q.prompt, mode: "Probe", field_id: historicalField,
    question_form: q.question_form, rendering: q.rendering, path: q.path, source_lines: q.source_lines.join(","), generator_ids: q.generator_ids.join(","),
    reciprocal_relations: family.reciprocal_challenges.join(","), bindings: "fixture", horizon: "fixture", coverage: "fixture", authority: "independent fixture", evidence: "schema-1 report", dependencies: "none" });
  schema1Append({ kind: "seal", ask_occurrence: id, should_change: "schema-1 report", invariants: "ancestry", discriminator: "finite fixture", wrong_impl: "foreign report", coverage: "fixture" });
  const digests = reports.map(reportBytes), evidenceList = digests.join(";");
  for (const d of digests) {
    schema1Append({ kind: "raw", ask_occurrence: id, cmd: "schema-1 fixture", digest: d, raw_ref: `.claude/trace/raw/${d}`, sensitive: "false" });
    schema1Append({ kind: "interpret", ask_occurrence: id, raw_digest: d, interpretation: "schema-1 report", provenance: d });
  }
  schema1Append({ kind: "check", ask_occurrence: id, verdict: "schema-1 checked", coverage: "fixture", evidence: evidenceList });
  schema1Append({ kind: "answer", occurrence: `ANS-${id}`, ask_occurrence: id, answer: "schema-1 report", status: "checked", resolution_class: "Supported", polarity: "Positive",
    residual: "execution correspondence absent", evidence: evidenceList, coverage: "fixture", authority: "independent fixture" });
  schema1Append({ kind: "reify", answer_occurrence: `ANS-${id}`, status: "checked", products: JSON.stringify(products(digests)), new_questions: "preserved", coverage: "fixture" });
  historicalField = `H-${id}-DONE`;
  schema1Append({ kind: "field", field_id: historicalField, members: JSON.stringify([a, b]), basis: "schema-1 result", coverage: "fixture",
    regenerated_from: `ANS-${id}`, dispositions: JSON.stringify({ [id]: "Answered" }), removal_evidence: JSON.stringify({ [id]: `ANS-${id}` }) });
}
schema1Probe("H-PROTECTION", [{ schema: 1, kind: "protected_continuation", targets: [a.occurrence, b.occurrence] }], ([d]) => [
  { ...product("HC-1"), inquiry_protection: { schema: 1, targets: [a.occurrence, b.occurrence], raw_digest: d } },
]);
const historicalClaim = (relation) => ({ schema: 1, relation, members: [a.occurrence, b.occurrence], representative: a.occurrence,
  member_identities: { [a.occurrence]: evidence.identityDigest(a), [b.occurrence]: evidence.identityDigest(b) }, continuations: ["HC-1"], horizon: "fixture", coverage: "fixture" });
const historicalEq = { schema: 1, kind: "fold_check", claim: historicalClaim("protected_equivalence"), observations: [a, b].map((m) =>
  ({ member: m.occurrence, continuation: "HC-1", status: "Supported", value: m.question_form })) };
const historicalRegen = { schema: 1, kind: "fold_check", claim: historicalClaim("regeneration"), regenerated_members:
  { [a.occurrence]: evidence.identity(a), [b.occurrence]: evidence.identity(b) } };
schema1Probe("H-EVIDENCE", [historicalEq, historicalRegen], ([eq, regen]) => [
  { ...product("HE-EQ", ["HC-1"]), fold_evidence: { ...historicalEq.claim, raw_digest: eq } },
  { ...product("HE-REGEN", ["HC-1"]), fold_evidence: { ...historicalRegen.claim, raw_digest: regen } },
]);
schema1Append({ kind: "fold", fold_id: "HF-1", members: JSON.stringify([a.occurrence, b.occurrence]), representative: a.occurrence,
  protected_equivalence_evidence: "HE-EQ", regeneration: "HE-REGEN", protected_continuations: '["HC-1"]', reopen_condition: "stronger policy", horizon: "fixture", coverage: "fixture" });
historicalField = "H-FOLDED";
schema1Append({ kind: "field", field_id: historicalField, members: JSON.stringify([a]), basis: "schema-1 fold", coverage: "fixture", regenerated_from: "HF-1", dispositions: "{}", removal_evidence: "{}" });
schema1Append({ kind: "control", authority: "explicit user migration", residual: "execution correspondence", predecessor: "schema-1 reports", scope: "harness" });
schema1Append({ ...schema1Policy, kind: "ask", occurrence: a.occurrence, fp: "schema1-migration", q: a.prompt, mode: "Probe", field_id: historicalField,
  question_form: a.question_form, rendering: a.rendering, path: a.path, source_lines: a.source_lines.join(","), generator_ids: a.generator_ids.join(","),
  reciprocal_relations: family.reciprocal_challenges.join(","), bindings: "fixture", horizon: "fixture", coverage: "fixture", authority: "user", evidence: "counterexample", dependencies: "none" });
schema1Append({ kind: "seal", ask_occurrence: a.occurrence, should_change: "execution policy", invariants: "history", discriminator: "schema migration", wrong_impl: "grandfather report equality", coverage: "fixture" });
fs.writeFileSync(manifestPath, currentManifestBytes);
schema1Append({ ...policy, kind: "policy_transition", predecessor_source_digest: schema1Policy.source_digest,
  predecessor_program_manifest_digest: schema1Policy.program_manifest_digest, authority: "explicit user", reason: "checked report-to-execution counterexample" });
const schema1Migrated = exec(["state", schema1Trace]); assert.equal(schema1Migrated.status, 0, schema1Migrated.stderr);
assert.equal(JSON.parse(schema1Migrated.stdout).folds[0].reopen_required, true);
assert.deepEqual(JSON.parse(schema1Migrated.stdout).folds[0].reopen_reasons, ["evidence-policy-migration"]);

// Actual issuance under a policy-0 predecessor manifest, followed by a
// controlled transition: old label-only folds also reopen.
const oldManifest = JSON.parse(currentManifestBytes); delete oldManifest.active_lifecycle.fold_evidence;
fs.writeFileSync(manifestPath, JSON.stringify(oldManifest));
const oldPolicy = { ...policy, program_manifest_digest: digest(manifestPath) };
const migrationTrace = path.join(root, ".claude/trace/migration.jsonl"); fs.writeFileSync(migrationTrace, "");
function migrationAppend(r) { const x = exec(["append", migrationTrace, fuel], JSON.stringify(r) + "\n"); assert.equal(x.status, 0, x.stderr); }
migrationAppend(oldPolicy);
migrationAppend({ kind: "field", field_id: "M-0", members: JSON.stringify([a, b]), basis: "predecessor", coverage: "fixture", regenerated_from: "bootstrap", dispositions: "{}", removal_evidence: "{}" });
migrationAppend({ ...fold, protected_equivalence_evidence: "checker:old-label", regeneration: "generator:old-label" });
migrationAppend({ kind: "field", field_id: "M-1", members: JSON.stringify([a]), basis: "old fold", coverage: "fixture", regenerated_from: "F-1", dispositions: "{}", removal_evidence: "{}" });
migrationAppend({ kind: "control", authority: "explicit user migration", residual: "fold evidence", predecessor: "old labels", scope: "harness" });
migrationAppend({ ...oldPolicy, kind: "ask", occurrence: a.occurrence, fp: "migration", q: a.prompt, mode: "Probe", field_id: "M-1", question_form: a.question_form, rendering: a.rendering, path: a.path,
  source_lines: a.source_lines.join(","), generator_ids: a.generator_ids.join(","), reciprocal_relations: family.reciprocal_challenges.join(","), bindings: "fixture", horizon: "fixture", coverage: "fixture", authority: "user", evidence: "predecessor", dependencies: "none" });
migrationAppend({ kind: "seal", ask_occurrence: a.occurrence, should_change: "evidence policy", invariants: "ancestry", discriminator: "migration", wrong_impl: "grandfather unsupported folds", coverage: "fixture" });
fs.writeFileSync(manifestPath, currentManifestBytes);
migrationAppend({ ...policy, kind: "policy_transition", predecessor_source_digest: oldPolicy.source_digest, predecessor_program_manifest_digest: oldPolicy.program_manifest_digest, authority: "explicit user", reason: "independently demonstrated fold evidence gap" });
const migrated = exec(["state", migrationTrace]); assert.equal(migrated.status, 0, migrated.stderr);
assert.equal(JSON.parse(migrated.stdout).folds[0].reopen_required, true);
assert.deepEqual(JSON.parse(migrated.stdout).folds[0].reopen_reasons, ["evidence-policy-migration"]);
console.log(`PASS fold evidence admission: ${rejected} isolated reference/report/execution contrasts, immutable Raw tamper rejection, positive checked fold, support/continuation reopening, schema-0/1 migration, projection and no policy downgrade`);
