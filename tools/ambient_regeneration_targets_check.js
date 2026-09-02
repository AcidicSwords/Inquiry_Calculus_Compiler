#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const memory = JSON.parse(fs.readFileSync(path.join(root, "formal-successor/REGENERATIVE_SPINE.json")));
const targets = memory.protected_predecessor_capabilities;
const expectedIds = [
  "typed-relational-surface", "partial-binding-question-fibers",
  "supported-answer-source-succession", "protection-discrimination-determination",
  "compression-recovery-reopening", "question-productivity-frontier",
  "question-route-occurrence-regeneration", "canonical-language-roots-and-static-pairs"
];
const capabilities = new Set(["ctxFam", "regPred", "indPlus", "bindingPresentation"]);

assert.deepEqual(targets.map((entry) => entry.id), expectedIds);
assert.equal(new Set(targets.map((entry) => entry.successor_target)).size, targets.length);
assert.deepEqual(targets.map((entry) => entry.dependency_order), [1, 2, 3, 4, 5, 6, 7, 8]);
for (const entry of targets) {
  assert.equal(entry.regeneration_status, "OPEN_NO_SUCCESSOR_CONSTRUCTION_OR_CORRESPONDENCE");
  assert.ok(entry.artifacts.length > 0, entry.id);
  assert.ok(entry.candidate_ambient_requirements.length > 0, entry.id);
  assert.ok(entry.candidate_ambient_requirements.every((capability) => capabilities.has(capability)), entry.id);
  for (const artifact of entry.artifacts) assert.ok(fs.existsSync(path.join(root, artifact)), `${entry.id}: ${artifact}`);
}
assert.equal(targets[0].successor_target, "FORMAL-C-TYPED-RELATIONAL-SURFACE");
assert.ok(targets.every((entry) => !/CHECKED|PROVED|REGENERATED/u.test(entry.regeneration_status)));

console.log("PASS total eight-class regeneration target projection with exact artifacts, candidate ambient dependencies, and no forged correspondence");
