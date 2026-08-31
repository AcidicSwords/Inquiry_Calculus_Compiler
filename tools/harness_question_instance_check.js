#!/usr/bin/env node
"use strict";

// Independently specified role-binding fixtures; process typing, not semantic
// truth of a supplied relation. Integration goes through the real append gate.
const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const instance = require("../.claude/hooks/ic-question-instance.js");
const { validateFieldRecord } = require("../.claude/hooks/ic-question-program.js");
const root = path.resolve(__dirname, "..");
const manifest = require("../formal-successor/ENGINEERING_QUESTION_PROGRAMS.json");
const form = manifest.preformal_harness.compiled_questions.find((entry) => entry.id === "CQ-OPEN-POSITION");
const product = (id, extra = {}) => ({ id, kind: "candidate_relation", status: "provisional", provenance: "fixture Generate answer",
  dependencies: [], horizon: "parallel terms in a supplied horizon", coverage: "declared carriers only",
  applicability: "predecessor protected-equivalence frame", ...extra });
const products = [
  product("F", { inquiry_carrier: "Term" }), product("G", { inquiry_carrier: "Term" }),
  product("H", { inquiry_carrier: "Horizon" }),
  product("SEP", { inquiry_relation: { label: "context separates terms within horizon", roles: [
    { name: "left", carrier: "Term" }, { name: "right", carrier: "Term" },
    { name: "horizon", carrier: "Horizon" }, { name: "context", carrier: "Context" },
  ] } }),
];
const ref = (id) => ({ kind: "product", id });
const seed = product("SEED", { dependencies: ["SEP", "F", "G", "H"], inquiry_seed: {
  question_form: form.id, relation_product: "SEP", bindings: { left: ref("F"), right: ref("G"), horizon: ref("H") },
  open_roles: ["context"], path: "formal/conditional-equivalence/separator",
} });
products.push(seed);
const context = { products: new Map(products.map((p) => [p.id, p])), questions: new Map(), invalidated: new Set() };
const member = instance.materialize(seed, context, manifest);
assert.deepEqual(member.relational_instance.open_roles, ["context"]);
assert.deepEqual(member.relational_instance.bindings, { left: ref("F"), right: ref("G"), horizon: ref("H") });
assert.match(member.prompt, /Open jointly \("context":"Context"\)/u);
assert.match(member.prompt, /"left":"Term"=product:"F"/u);
assert.match(member.prompt, /unknown, not asserted witnesses/u);
assert.equal(member.executable, false);
assert.equal(member.disposition, "Unknown");
assert.deepEqual(instance.materialize(structuredClone(seed), context, manifest), member);
instance.validateMember(member, context);
let negative = 0;
function rejectSeed(mutate, pattern) {
  const wrong = structuredClone(seed); mutate(wrong); negative++;
  assert.throws(() => instance.materialize(wrong, context, manifest), pattern);
}
rejectSeed((p) => p.inquiry_seed.bindings.left = ref("H"), /carrier mismatch/u);
rejectSeed((p) => p.inquiry_seed.bindings.left = ref("MISSING"), /dangling/u);
rejectSeed((p) => p.inquiry_seed.bindings.left = { kind: "question", id: "MISSING" }, /dangling/u);
rejectSeed((p) => p.inquiry_seed.open_roles = ["missing"], /undeclared/u);
rejectSeed((p) => p.inquiry_seed.open_roles = ["left", "context"], /capture/u);
rejectSeed((p) => delete p.inquiry_seed.bindings.left, /bound or open/u);
rejectSeed((p) => p.inquiry_seed.open_roles = ["context", "context"], /distinct/u);
rejectSeed((p) => p.inquiry_seed.open_roles = [], /nonempty/u);
rejectSeed((p) => p.dependencies = ["SEP"], /dependencies omit/u);
rejectSeed((p) => p.inquiry_seed.bindings.left.kind = "literal", /unknown reference/u);
rejectSeed((p) => p.inquiry_seed.quantifier = "exists", /exact fields/u);
rejectSeed((p) => p.inquiry_seed.question_form = "UNDECLARED", /undeclared question/u);
rejectSeed((p) => p.inquiry_seed.relation_product = "F", /missing relation/u);
rejectSeed((p) => { delete p.inquiry_seed.bindings.left; p.inquiry_seed.open_roles = ["context", "left"]; }, /role order/u);
const changedPath = structuredClone(seed); changedPath.inquiry_seed.path += "/another-occurrence";
assert.notEqual(instance.materialize(changedPath, context, manifest).occurrence, member.occurrence);
const distinctSeed = structuredClone(seed); distinctSeed.id = "SEED-SAME-RELATION-OTHER-ANCESTRY";
assert.notEqual(instance.materialize(distinctSeed, context, manifest).occurrence, member.occurrence);
const reversed = structuredClone(seed); reversed.inquiry_seed.bindings.left = ref("G"); reversed.inquiry_seed.bindings.right = ref("F");
assert.notEqual(instance.materialize(reversed, context, manifest).rendering, member.rendering);
const revisedCorpus = structuredClone(manifest);
revisedCorpus.preformal_harness.compiled_questions.find((entry) => entry.id === form.id).prompt += " Different rendering.";
assert.notEqual(instance.materialize(seed, context, revisedCorpus).occurrence, member.occurrence, "template changes must not reuse the occurrence");
const badMember = structuredClone(member); badMember.relational_instance.bindings.left = ref("G");
assert.throws(() => instance.validateMember(badMember, context), /meaning differs/u); negative++;
assert.throws(() => instance.validateMember({ ...member, executable: true }, { ...context, invalidated: new Set(["F"]) }), /invalidated/u); negative++;

const field = (id, members, extra = {}) => ({ kind: "field", field_id: id, members: JSON.stringify(members), basis: "fixture generation",
  coverage: "declared constructor", regenerated_from: "bootstrap", dispositions: "{}", removal_evidence: "{}", ...extra });
// This positive contrast failed before integration: a fixed generic prompt list
// rejects a well-formed open-role question even though its corpus form is known.
validateFieldRecord(field("CONTRAST", [member]), root);

const sandbox = fs.mkdtempSync(path.join(os.tmpdir(), "ic-instance-check-"));
fs.cpSync(path.join(root, ".claude/hooks"), path.join(sandbox, ".claude/hooks"), { recursive: true });
fs.mkdirSync(path.join(sandbox, "formal-successor"), { recursive: true });
for (const name of ["Questions.txt", "ENGINEERING_QUESTION_PROGRAMS.json", "PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS.md",
  "PREFORMAL_SEARCH_ASYMMETRY.md", "SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md", "QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md",
  "RESIDUAL_OBLIGATIONS.json"]) fs.copyFileSync(path.join(root, "formal-successor", name), path.join(sandbox, "formal-successor", name));
fs.copyFileSync(path.join(root, "IMPLEMENTATION_FRONTIER.md"), path.join(sandbox, "IMPLEMENTATION_FRONTIER.md"));
fs.mkdirSync(path.join(sandbox, ".claude/trace"), { recursive: true });
const trace = path.join(sandbox, ".claude/trace/instances.jsonl"), fuel = `${trace}.fuel`;
fs.writeFileSync(trace, ""); fs.writeFileSync(fuel, "12"); fs.writeFileSync(path.join(sandbox, ".claude/trace/.state"), "instances.jsonl\n");
const hash = (name) => crypto.createHash("sha256").update(fs.readFileSync(path.join(sandbox, "formal-successor", name))).digest("hex");
function append(record, rejectPattern) {
  const before = fs.readFileSync(trace);
  const out = cp.spawnSync(process.execPath, [path.join(sandbox, ".claude/hooks/ic-append.js"), "append", trace, fuel],
    { cwd: sandbox, input: `${JSON.stringify(record)}\n`, encoding: "utf8", windowsHide: true });
  if (rejectPattern) {
    negative++; assert.notEqual(out.status, 0); assert.match(out.stderr, rejectPattern);
    assert.deepEqual(fs.readFileSync(trace), before, "rejected append changed ancestry");
  } else assert.equal(out.status, 0, `${record.kind}: ${out.stdout}${out.stderr}`);
}
function ask(id, q, extra = {}) {
  const family = manifest.preformal_harness.program_families.find((entry) => entry.id === form.family);
  return { kind: "ask", occurrence: q.occurrence, field_id: id, q: q.prompt, question_form: q.question_form, rendering: q.rendering,
    source_lines: q.source_lines.join(","), generator_ids: q.generator_ids.join(","), reciprocal_relations: family.reciprocal_challenges.join(",") || "none",
    path: q.path, bindings: q.relational_instance ? instance.canonical(q.relational_instance.bindings) : "fixture",
    dependencies: q.dependencies.join(",") || "none", horizon: q.relational_instance?.horizon ?? "fixture",
    coverage: q.relational_instance?.coverage ?? "fixture", authority: "fixture", evidence: "declarations only", mode: "Generate",
    source_digest: hash("Questions.txt"), program_manifest_digest: hash("ENGINEERING_QUESTION_PROGRAMS.json"), fp: q.occurrence, ...extra };
}
function answerReify(q, id, ps) {
  append({ kind: "answer", occurrence: id, ask_occurrence: q.occurrence, answer: "candidate relations, not actuality", resolution_class: "Supported",
    status: "provisional", polarity: "Positive", residual: "semantic grounding", evidence: "fixture generation", coverage: "fixture", authority: "fixture" });
  append({ kind: "reify", answer_occurrence: id, status: "provisional", products: JSON.stringify(ps), new_questions: "seed-derived", coverage: "fixture" });
}
append({ kind: "policy", question_program_schema: "4", source_digest: hash("Questions.txt"), program_manifest_digest: hash("ENGINEERING_QUESTION_PROGRAMS.json") });
const bootstrap = { occurrence: "BOOT", question_form: form.id, rendering: `RENDER-${form.id}`, prompt: form.prompt,
  source_lines: form.source_lines, generator_ids: member.generator_ids, path: "bootstrap", disposition: "Required", executable: true, dependencies: [] };
append(field("F1", [bootstrap])); append(ask("F1", bootstrap)); answerReify(bootstrap, "A1", products);
const ready = { ...member, disposition: "Required", executable: true };
const f2 = field("F2", [ready], { regenerated_from: "A1", dispositions: '{"BOOT":"Answered"}', removal_evidence: '{"BOOT":"A1"}' });
append({ ...f2, members: "[]" }, /fails to materialize/u);
const missing = structuredClone(ready); missing.relational_instance.seed_product = "MISSING";
append({ ...f2, members: JSON.stringify([missing]) }, /missing reified inquiry seed|identity/u);
const forged = structuredClone(ready);
forged.relational_instance.bindings.left = ref("G");
forged.prompt = instance.render(form.prompt, forged.relational_instance);
forged.rendering = `RI-${instance.renderingIdentity(forged)}`;
forged.occurrence = `QI-${instance.digest([forged.rendering.slice(3), forged.path])}`;
append({ ...f2, members: JSON.stringify([forged]) }, /meaning differs/u);
append(f2);
append(ask("F2", ready, { bindings: "{}" }), /bindings/u);
append(ask("F2", ready, { dependencies: "none" }), /dependencies/u);
append(ask("F2", ready, { horizon: "unrestricted" }), /horizon/u);
append(ask("F2", ready));
const qRelation = product("QR", { inquiry_relation: { label: "downstream discriminator of a question", roles: [
  { name: "subject", carrier: "Question" }, { name: "discriminator", carrier: "Relation" },
] } });
const qSeed = product("QS", { dependencies: ["QR", ...member.dependencies], inquiry_seed: {
  question_form: form.id, relation_product: "QR", bindings: { subject: { kind: "question", id: member.occurrence } },
  open_roles: ["discriminator"], path: "question-of-question/downstream",
} });
answerReify(ready, "A2", [qRelation, qSeed]);
context.products.set(qRelation.id, qRelation); context.products.set(qSeed.id, qSeed); context.questions.set(member.occurrence, member);
const recursive = instance.materialize(qSeed, context, manifest);
assert.equal(recursive.relational_instance.bindings.subject.id, member.occurrence);
assert.equal(recursive.question_form, form.id, "ordinary question; no MetaQuestion type");
append(field("F3", [recursive], { regenerated_from: "A2", dispositions: JSON.stringify({ [member.occurrence]: "Answered" }),
  removal_evidence: JSON.stringify({ [member.occurrence]: "A2" }) }));
append(field("F4", [{ ...recursive, path: "silently-moved" }], { regenerated_from: "A2" }), /path|identity/u);
const result = cp.spawnSync(process.execPath, [path.join(sandbox, ".claude/hooks/ic-append.js"), "validate", trace], { encoding: "utf8", windowsHide: true });
assert.equal(result.status, 0, result.stderr);
const projected = JSON.parse(cp.execFileSync(process.execPath, [path.join(sandbox, ".claude/hooks/ic-relational-surface.js"), "json", sandbox], { encoding: "utf8", windowsHide: true }));
assert.ok(projected.generated_questions.some((q) => q.occurrence === recursive.occurrence));
assert.equal(projected.generated_questions.some((q) => q.occurrence === member.occurrence), false, "answered seed must not return as unanswered");
assert.equal(projected.products.find((p) => p.id === "QS").status, "provisional");
append({ kind: "invalidate", product_ids: '["F","SEED","QS"]', cause: "fixture carrier evidence withdrawn", evidence: "independent fixture perturbation" });
append(field("F-INVALID", [{ ...recursive, executable: true }], { regenerated_from: "A2" }), /invalidated/u);
append(field("F-BLOCKED", [{ ...recursive, disposition: "Blocked" }], { regenerated_from: "A2" }));
const invalidProjection = JSON.parse(cp.execFileSync(process.execPath, [path.join(sandbox, ".claude/hooks/ic-relational-surface.js"), "json", sandbox], { encoding: "utf8", windowsHide: true }));
assert.equal(invalidProjection.generated_questions.find((q) => q.occurrence === recursive.occurrence).disposition, "Blocked");
console.log(`PASS bounded relational question instances: ${negative} negative contrasts plus recursive append/replay/projection; no semantic truth claim`);
