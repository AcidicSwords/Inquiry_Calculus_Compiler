#!/usr/bin/env node
"use strict";
const assert = require("node:assert/strict"), path = require("node:path"), fs = require("node:fs"), os = require("node:os"), cp = require("node:child_process"), crypto = require("node:crypto");
const manifest = require(path.resolve(__dirname, "../formal-successor/ENGINEERING_QUESTION_PROGRAMS.json"));
const generator = require(path.resolve(__dirname, "../.claude/hooks/ic-recursive-generator.js"));

const surface = {
  relations: [
    { id: "R-AB", source: "A", target: "B", path: "p/ab", dependencies: ["P-A"], reverse_id: "R-BA" },
    { id: "R-BA", source: "B", target: "A", path: "p/ba", dependencies: ["P-B"], reverse_id: "R-AB" },
    { id: "R-BC", source: "B", target: "C", path: "p/bc", dependencies: ["P-C"] },
  ],
  discriminators: [{ id: "D-C", domain: "C", path: "d/c", dependencies: ["E-C"] }],
  questions: [
    { occurrence: "Q-1", path: "q/1", context: "CTX", exchangeable: true, dependencies: ["R-AB"] },
    { occurrence: "Q-2", path: "q/2", context: "CTX", exchangeable: true, dependencies: ["R-BC"] },
  ],
};
const generated = generator.generate(surface, manifest);
const families = new Set(generated.map((question) => question.derivation.family));
assert.deepEqual(families, new Set(["DIRECT", "RECIPROCAL", "COMPOSE", "TRANSPORT", "QUESTION_SUBJECT", "PERMUTE", "REGENERATE"]));
assert.ok(generated.every((question) => question.disposition === "Unknown" && question.executable === false));
assert.equal(new Set(generated.map((question) => question.occurrence)).size, generated.length);

const withoutReverse = structuredClone(surface); delete withoutReverse.relations[0].reverse_id; delete withoutReverse.relations[1].reverse_id;
const noReverse = generator.generate(withoutReverse, manifest);
assert.equal(noReverse.some((question) => question.derivation.family === "RECIPROCAL"), false);
assert.equal(noReverse.filter((question) => question.derivation.family === "DIRECT").length, 3);
const withoutBridge = structuredClone(surface); withoutBridge.relations[2].source = "X";
assert.ok(generator.generate(withoutBridge, manifest).filter((question) => question.derivation.family === "COMPOSE").length < generated.filter((question) => question.derivation.family === "COMPOSE").length);
const withoutDiscriminator = structuredClone(surface); withoutDiscriminator.discriminators = [];
assert.equal(generator.generate(withoutDiscriminator, manifest).some((question) => question.derivation.family === "TRANSPORT"), false);
const withoutExchange = structuredClone(surface); withoutExchange.questions[1].exchangeable = false;
assert.equal(generator.generate(withoutExchange, manifest).some((question) => question.derivation.family === "PERMUTE"), false);
const otherPath = structuredClone(surface); otherPath.relations[0].path = "other/ab";
assert.notEqual(generator.generate(otherPath, manifest).find((q) => q.derivation.family === "DIRECT" && q.derivation.inputs[0] === "R-AB").occurrence,
  generated.find((q) => q.derivation.family === "DIRECT" && q.derivation.inputs[0] === "R-AB").occurrence);
assert.throws(() => generator.generate({ ...surface, relations: [...surface.relations, surface.relations[0]] }, manifest), /duplicate relation/u);
assert.throws(() => generator.generate({ ...surface, relations: [{ ...surface.relations[0], reverse_id: "MISSING" }] }, manifest), /undeclared reverse/u);

// Cross the integration boundary: a reified finite surface must regenerate all
// of its derived occurrences through the actual append gate and projection.
const root = path.resolve(__dirname, "..");
const sandbox = fs.mkdtempSync(path.join(os.tmpdir(), "ic-recursive-generator-"));
const trace = path.join(sandbox, "trace.jsonl"), fuel = `${trace}.fuel`;
fs.writeFileSync(trace, ""); fs.writeFileSync(fuel, "12");
const hash = (name) => crypto.createHash("sha256").update(fs.readFileSync(path.join(root, "formal-successor", name))).digest("hex");
function append(record, rejected) {
  const before = fs.readFileSync(trace);
  const result = cp.spawnSync(process.execPath, [path.join(root, ".claude/hooks/ic-append.js"), "append", trace, fuel],
    { cwd: root, input: `${JSON.stringify(record)}\n`, encoding: "utf8", windowsHide: true });
  if (rejected) {
    assert.notEqual(result.status, 0); assert.match(result.stderr, rejected); assert.deepEqual(fs.readFileSync(trace), before);
  } else assert.equal(result.status, 0, `${record.kind}: ${result.stdout}${result.stderr}`);
}
const form = manifest.preformal_harness.compiled_questions.find((entry) => entry.id === "CQ-OPEN-POSITION");
const generatorIds = manifest.active_lifecycle.generator_registry.filter((entry) => entry.question_forms.includes(form.id)).map((entry) => entry.id);
const reciprocalRelations = manifest.preformal_harness.program_families.find((entry) => entry.id === form.family).reciprocal_challenges;
const baseQuestion = (occurrence, pathName) => ({ occurrence, question_form: form.id, rendering: `RENDER-${form.id}`, prompt: form.prompt,
  source_lines: form.source_lines, generator_ids: generatorIds, path: pathName, dependencies: [], disposition: "Required", executable: true });
const q1 = baseQuestion("Q-1", "q/1"), q2 = baseQuestion("Q-2", "q/2");
const field = (fieldId, members, extra = {}) => ({ kind: "field", field_id: fieldId, members: JSON.stringify(members), basis: "finite represented generator fixture",
  coverage: "bounded first-order generator families", regenerated_from: "bootstrap", dispositions: "{}", removal_evidence: "{}", ...extra });
append({ kind: "policy", question_program_schema: "4", source_digest: hash("Questions.txt"), program_manifest_digest: hash("ENGINEERING_QUESTION_PROGRAMS.json") });
append(field("GF-1", [q1, q2]));
append({ kind: "ask", occurrence: q1.occurrence, field_id: "GF-1", q: q1.prompt, question_form: q1.question_form, rendering: q1.rendering,
  source_lines: q1.source_lines.join(","), generator_ids: q1.generator_ids.join(","), reciprocal_relations: reciprocalRelations.join(","), path: q1.path,
  bindings: "fixture", dependencies: "none", horizon: "fixture", coverage: "bounded", authority: "fixture", evidence: "represented field", mode: "Generate",
  source_digest: hash("Questions.txt"), program_manifest_digest: hash("ENGINEERING_QUESTION_PROGRAMS.json"), fp: q1.occurrence });
append({ kind: "answer", occurrence: "GA-1", ask_occurrence: q1.occurrence, answer: "finite explicit generator surface candidate", resolution_class: "Supported",
  status: "provisional", polarity: "Positive", residual: "semantic completeness remains open", evidence: "fixture generation", coverage: "bounded", authority: "fixture" });
const ordinaryProduct = (id, kind) => ({ id, kind, status: "provisional", provenance: "GA-1 fixture", dependencies: [],
  horizon: "bounded", coverage: "fixture", applicability: "explicit generator input" });
const inputProducts = [ordinaryProduct("R-AB", "candidate_relation"), ordinaryProduct("R-BA", "candidate_relation"),
  ordinaryProduct("R-BC", "candidate_relation"), ordinaryProduct("D-C", "candidate_discriminator")];
const surfaceProduct = { ...ordinaryProduct("GEN-SURFACE", "inquiry_generator_surface"),
  dependencies: ["R-AB", "R-BA", "R-BC", "D-C", "Q-1", "Q-2"], inquiry_generator_surface: surface };
append({ kind: "reify", answer_occurrence: "GA-1", status: "provisional", products: JSON.stringify([...inputProducts, surfaceProduct]),
  new_questions: "all bounded first-order derivations", coverage: "seven declared families" });
const context = { products: new Map([...inputProducts, surfaceProduct].map((product) => [product.id, product])),
  questions: new Map([[q1.occurrence, q1], [q2.occurrence, q2]]), invalidated: new Set() };
const materialized = generator.materialize(surfaceProduct, context, manifest);
generator.validateMember({ ...materialized[0], disposition: "Required", executable: true }, context, manifest);
append(field("GF-2-MISSING", [q2], { regenerated_from: "GA-1", dispositions: '{"Q-1":"Answered"}', removal_evidence: '{"Q-1":"GA-1"}' }), /fails to materialize/u);
append(field("GF-2", [q2, ...materialized], { regenerated_from: "GA-1", dispositions: '{"Q-1":"Answered"}', removal_evidence: '{"Q-1":"GA-1"}' }));
const altered = structuredClone(materialized[0]); altered.derivation.inputs = ["R-BA"];
append(field("GF-3-FORGED", [q2, altered, ...materialized.slice(1)], { regenerated_from: "GA-1" }), /rendering identity|differs from its reified/u);
const validated = cp.spawnSync(process.execPath, [path.join(root, ".claude/hooks/ic-append.js"), "validate", trace], { encoding: "utf8", windowsHide: true });
assert.equal(validated.status, 0, validated.stderr);
console.log(`PASS recursive relational generators: ${generated.length} distinct occurrences across ${families.size} families; 6 incidence/path ablations plus append/replay/field propagation`);
