#!/usr/bin/env node
"use strict";

// Repository-wide control-topology checks. Mathematical artifacts retain their
// own source-regeneration, countermodel, Lean, and conformance checkers.

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const root = path.resolve(__dirname, "..");

require("./inquiry_spine_check.js");
require("./integrated_theorem_obligations_check.js");
require("./regenerative_spine_check.js");
require("./construction_obligation_check.js");

const read = (relative) => fs.readFileSync(path.join(root, ...relative.split("/")), "utf8");
const spec = read("formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md");
const agents = read("AGENTS.md");
const frontier = read("IMPLEMENTATION_FRONTIER.md");
const active = JSON.parse(read("formal-successor/ACTIVE_INPUTS.json"));

assert.match(spec, /# 104\. One cohesive inquiry spine within every phase/);
assert.match(spec, /D_\{i,t\}=C_\{q_t\}\\circ P_\{i:t\}/);
assert.match(spec, /There is no active\s+residual-shape method dispatcher/);
assert.match(agents, /sole model-facing recurrence/);
assert.equal(active.authority_path.length, 3);
assert.equal(active.derived_machine_contract.role, "rebuildable_implementation_contract_not_independent_authority");
assert.equal(active.derived_construction_memory.role,
  "rebuildable_regenerative_dependency_projection_not_semantic_authority");
assert.deepEqual(active.derived_planning_registries.map(({ role }) => role),
  ["gate_indexed_candidate_theorem_obligations_not_semantic_authority"]);
assert.equal((frontier.match(/<!-- LIVE_FRONTIER_BEGIN -->/g) ?? []).length, 1);
assert.equal((frontier.match(/<!-- LIVE_FRONTIER_END -->/g) ?? []).length, 1);
const frontierIds = [...frontier.matchAll(/^id: ([A-Z0-9-]+)$/gmu)].map((match) => match[1]);
assert.equal(frontierIds.length, 1);
assert.match(frontierIds[0], /^FORMAL-[A-Z0-9-]+$/u);
assert.equal(JSON.parse(read("formal-successor/reports/latest.json")).frontier, frontierIds[0]);

// The frontier is generated. It must declare itself generated and must not have
// drifted from the derived construction obligation field.
const frontierGenerate = require(path.join(root, ".claude/hooks/ic-frontier-generate.js"));
assert.match(frontier, /GENERATED FILE\. Do not edit by hand\./u,
  "IMPLEMENTATION_FRONTIER.md does not declare itself a generated projection");
assert.equal(frontierGenerate.check(root).drifted, false,
  "IMPLEMENTATION_FRONTIER.md drifted from the derived obligation field; regenerate it");

// The autonomy contract must not restore the frontier as upstream construction truth.
assert.doesNotMatch(agents, /IMPLEMENTATION_FRONTIER\.md\s*\n\s*->\s*the single current project-level open position/u,
  "AGENTS.md still declares the frontier an upstream authority");
assert.match(agents, /derived construction obligation field/u,
  "AGENTS.md does not derive ordinary next work from the generated live field");

const runtimeFiles = fs.readdirSync(path.join(root, ".claude/hooks"))
  .map((name) => path.join(root, ".claude/hooks", name)).filter((file) => fs.statSync(file).isFile());
for (const file of runtimeFiles) {
  const text = fs.readFileSync(file, "utf8");
  assert.doesNotMatch(text, /ENGINEERING_QUESTION_PROGRAMS\.json|PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS\.md|QUESTION_RHYTHM\.md/,
    `${path.basename(file)} references a retired controller`);
}

console.log("formal successor control checks passed (single authority path and inquiry spine)");
