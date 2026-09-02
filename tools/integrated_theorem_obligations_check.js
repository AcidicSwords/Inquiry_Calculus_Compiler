#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const registryPath = path.join(root, "formal-successor", "INTEGRATED_THEOREM_OBLIGATIONS.json");
const specPath = path.join(root, "formal-successor", "FORMAL_CALCULUS_CONSTRUCTION_SPEC.md");
const registry = JSON.parse(fs.readFileSync(registryPath, "utf8"));
const spec = fs.readFileSync(specPath, "utf8");

const expected = [
  ["IC-THM-C-000", "C"],
  ["IC-THM-C-001", "C"], ["IC-THM-C-002", "C"], ["IC-THM-C-003", "C"],
  ["IC-THM-C-004", "C"], ["IC-THM-C-005", "C"], ["IC-THM-C-006", "C"],
  ["IC-THM-C-007", "C"], ["IC-THM-C-008", "C"], ["IC-THM-C-009", "C"],
  ["IC-THM-C-010", "C"], ["IC-THM-C-011", "C"], ["IC-THM-C-012", "C"],
  ["IC-THM-D-001", "D"], ["IC-THM-D-002", "D"], ["IC-THM-D-003", "D"],
  ["IC-THM-E-001", "E"], ["IC-THM-E-002", "E"], ["IC-THM-E-003", "E"], ["IC-THM-E-004", "E"],
  ["IC-THM-H-001", "H"], ["IC-THM-H-002", "H"], ["IC-THM-H-003", "H"],
  ["IC-THM-H-004", "H"], ["IC-THM-H-005", "H"], ["IC-THM-H-006", "H"],
  ["IC-THM-H-007", "H"], ["IC-THM-H-008", "H"], ["IC-THM-J-001", "J"],
  ["IC-THM-K-001", "K"], ["IC-THM-K-002", "K"], ["IC-THM-K-003", "K"],
  ["IC-THM-L-001", "L"], ["IC-THM-L-002", "L"], ["IC-THM-L-003", "L"]
];
const gateOrder = new Map("ABCDEFGHIJKLMN".split("").map((gate, index) => [gate, index]));

assert.equal(registry.schema, 1);
assert.equal(registry.status, "planned_candidate_theorem_family_not_successor_semantics");
assert.equal(registry.authority, "formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md#103.1");
assert.equal(registry.source_provenance.sha256,
  "7af86d14d36c63e995cf02a5c2bca04eb1dbc1ee67728a4dede846d6aba7dcb");
assert.equal(registry.additional_source_provenance.sha256,
  "ac529ef95b8bdbbcf6c83c8bc7a74545970bc0754ad4b8ebcde3946fe1733c83");
assert.deepEqual(registry.obligations.map(({ id, gate }) => [id, gate]), expected);
assert.deepEqual(registry.promotion_contract.required_evidence, [
  "typed_formal_statement",
  "dependency_closure",
  "lean_kernel_return_or_checked_countermodel",
  "independent_contract_checker",
  "proof_dependency_audit",
  "conformance_record_with_coverage_and_reopen_condition"
]);

const byId = new Map(registry.obligations.map((obligation) => [obligation.id, obligation]));
assert.equal(byId.size, registry.obligations.length, "theorem obligation IDs must be unique");
for (const obligation of registry.obligations) {
  assert.equal(obligation.status, "PLANNED", `${obligation.id} was promoted without formal evidence`);
  assert.ok(obligation.title && obligation.kind && obligation.statement && obligation.disposition && obligation.decisive_check,
    `${obligation.id} lacks an inspectable theorem contract`);
  assert.match(obligation.id, new RegExp(`^IC-THM-${obligation.gate}-\\d{3}$`));
  assert.match(spec, new RegExp("\\| `" + obligation.id + "` \\|"),
    `${obligation.id} is absent from the governing schedule`);
  for (const dependencyId of obligation.depends_on) {
    const dependency = byId.get(dependencyId);
    assert.ok(dependency, `${obligation.id} has unknown dependency ${dependencyId}`);
    assert.ok(gateOrder.get(dependency.gate) <= gateOrder.get(obligation.gate),
      `${obligation.id} depends on a later-gate theorem ${dependencyId}`);
  }
}

const requiredTitles = [
  "Ambient capability basis and primitive elimination",
  "Contextual occurrence", "Question transport", "Discriminator pullback composition",
  "Co-anchored meet", "Co-anchored versus sequential noncollapse", "Three-way arrangement distinction",
  "Adjacent-order localization", "Path-sensitive answer section", "Forward distinction transport",
  "Reciprocal section is not an inverse", "Pure closure laws", "Bounded fixed-regime stabilization",
  "Historical reopening", "Regenerative irredundancy", "Lawful self-reentry",
  "Condition-solution polarity", "Canonical render-elaborate round trip", "Linear discriminator pullback",
  "Linear contextual pullback composition", "Protected adjacent-order discriminator",
  "Proof-presupposition recursion", "Regenerative successor equivalence",
  "Completion relation and exact question fiber", "Question-use evaluation as relational composition",
  "Typed observation universe", "Represented reciprocal mate composition",
  "Paired represented-path consequence invariant", "Protected determination bridge and horizon refinement",
  "Occurrence productivity as protected continuation discrimination", "Relational factorization-kernel equivalence",
  "Protected exact-fold kernel", "Positive reopening as witnessed factorization failure"
];
for (const title of requiredTitles) {
  assert.ok(registry.obligations.some((obligation) => obligation.title.includes(title)),
    `missing imported theorem family: ${title}`);
}

assert.match(spec, /Planning a claim does not admit\s+it as successor mathematics/);
assert.match(spec, /Every registry entry remains `PLANNED`/);
console.log(`integrated theorem obligation checks passed (${registry.obligations.length} gate-indexed obligations; none promoted)`);
