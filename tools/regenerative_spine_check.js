#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const read = (name) => fs.readFileSync(path.join(root, ...name.split("/")), "utf8");
const spine = JSON.parse(read("formal-successor/REGENERATIVE_SPINE.json"));
const registry = JSON.parse(read("formal-successor/INTEGRATED_THEOREM_OBLIGATIONS.json"));
const frontier = read("IMPLEMENTATION_FRONTIER.md");

assert.equal(spine.schema, 1);
assert.equal(spine.status, "rebuildable_regenerative_dependency_projection_not_semantic_authority");
assert.equal(spine.candidate_source.sha256,
  "ac529ef95b8bdbbcf6c83c8bc7a74545970bc0754ad4b8ebcde3946fe1733c83");
const frontierIds = [...frontier.matchAll(/^id: ([A-Z0-9-]+)$/gmu)].map((match) => match[1]);
assert.deepEqual(frontierIds, [spine.current_strongest_formal_frontier.id]);

const obligationIds = new Set(registry.obligations.map(({ id }) => id));
const allowed = new Set(["proved", "derivable", "binding-conditional", "broken", "inapplicable", "unresolved"]);
assert.equal(spine.candidate_inquiries.length, 7);
for (const inquiry of spine.candidate_inquiries) {
  for (const field of [
    "id", "touches", "current_meaning", "proposed_relation", "making_question", "types_and_hypotheses",
    "derivation_attempt", "missing_structure", "breaker", "disposition", "propagate_to",
    "predecessor_preservation", "primitive_elimination", "theorem_obligations"
  ]) assert.ok(Object.hasOwn(inquiry, field), `${inquiry.id} lacks ${field}`);
  assert.ok(allowed.has(inquiry.disposition), `${inquiry.id} has invalid disposition`);
  assert.ok(inquiry.making_question.endsWith("?"), `${inquiry.id} does not expose an ordinary question`);
  for (const id of inquiry.theorem_obligations) assert.ok(obligationIds.has(id), `${inquiry.id} references ${id}`);
}
for (const capability of spine.protected_predecessor_capabilities) {
  assert.ok(capability.artifacts.length > 0, `${capability.id} has no artifact`);
  for (const artifact of capability.artifacts) assert.ok(fs.existsSync(path.join(root, artifact)), artifact);
}
for (const id of spine.unresolved_typed_obligations) assert.ok(obligationIds.has(id), id);
assert.equal(new Set(spine.unresolved_typed_obligations).size, spine.unresolved_typed_obligations.length);
assert.ok(spine.noncollapse_laws.some((law) => law.includes("selection != semantic admissibility")));
assert.ok(spine.noncollapse_laws.some((law) => law.includes("semantic maximal quotient")));
assert.deepEqual(spine.reconstruction_questions, [
  "Could the current successor be reconstructed from v2.0 plus this spine?",
  "Can anything in the spine now be removed without losing protected capability?"
]);
console.log(`regenerative spine checks passed (${spine.candidate_inquiries.length} integrated inquiries; ${spine.unresolved_typed_obligations.length} live theorem obligations)`);
