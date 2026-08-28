#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const path = require("node:path");
const policy = require(path.resolve(__dirname, "..", ".claude", "hooks", "ic-construction-policy.js"));

const dispositions = [
  policy.classifyQuestion({ answer: "a" }).disposition,
  policy.classifyQuestion({ productive: true }).disposition,
  policy.classifyQuestion({ required: true }).disposition,
  policy.classifyQuestion({ redundant_via: "q1" }).disposition,
  policy.classifyQuestion({ inapplicable_reason: "wrong type" }).disposition,
  policy.classifyQuestion({ blocked_reason: "tool gap" }).disposition,
  policy.classifyQuestion({ coverage: "partial" }).disposition,
];
assert.deepEqual(dispositions, [
  "Answered", "Productive", "Required", "Redundant", "Inapplicable", "Blocked", "Unknown",
]);

for (const term of ["expected", "smallest", "largest", "strongest", "best", "cheapest"]) {
  assert.equal(policy.comparatorApplicability(term, {}).disposition, "Inapplicable");
}
assert.equal(
  policy.comparatorApplicability("expected", { probability_model: "mu" }).disposition,
  "Productive",
);

const conditionA = policy.normalizeConditionKey({
  schema: "Forces", bound_roles: "W_phi", scope: "phaseA", applicability: "formal",
  grain: "protected", orientation: "forward",
});
const conditionB = policy.normalizeConditionKey({
  schema: "Forces", bound_roles: "W_phi", scope: "phaseB", applicability: "formal",
  grain: "protected", orientation: "forward",
});
assert.notEqual(conditionA, conditionB, "scope-specific conditions must not merge by wording");

const incidence = policy.buildReverseIncidence([
  { id: "R1", boundary_features: [{ kind: "Condition", key: conditionA }] },
  { id: "R2", boundary_features: [{ kind: "Condition", key: conditionA }] },
  { id: "R3", boundary_features: [{ kind: "Condition", key: conditionB }] },
]);
assert.deepEqual([...incidence.get(`Condition:${conditionA}`)], ["R1", "R2"]);
assert.deepEqual([...incidence.get(`Condition:${conditionB}`)], ["R3"]);

const expectedDispatch = {
  OrderedBoundary: ["Bisection", "GeneralizedBinarySearch"],
  DecomposableBreaker: ["DeltaDebugging"],
  ConjunctiveConflict: ["QuickXplain", "MUS"],
  CompetingDiagnoses: ["ModelBasedDiagnosis", "SequentialDiscrimination"],
  CoarseAbstraction: ["CEGAR"],
  Synthesis: ["CEGIS"],
  FinitePartition: ["PartitionRefinement"],
  UnknownAutomaton: ["ActiveAutomataLearning"],
  SharedConditionBasis: ["AttributeExploration"],
  MultiContext: ["AssumptionContextManagement"],
  Generic: ["NondominatedApplicableMethods"],
};
for (const [shape, methods] of Object.entries(expectedDispatch)) {
  assert.deepEqual(policy.chooseMethodFrontier(shape), methods);
}

assert.equal(policy.resolveField({
  coverage_complete: false, coverage: "sample", classes: [], certificate: null,
}).resolution, "Unknown");
assert.equal(policy.resolveField({
  coverage_complete: true, coverage: "finite-complete", classes: [], certificate: "proof-empty",
}).resolution, "ExactEmpty");
assert.equal(policy.resolveField({
  coverage_complete: true, coverage: "finite-complete", classes: ["a", "b"],
}).resolution, "Plural");

const frontier = policy.selectQuestionFrontier([
  { id: "dominated", worst: 8, leverage: 1, cost: 8, risk: 2, authority_debt: 2, coverage_gain: 1 },
  { id: "wide", worst: 3, leverage: 5, cost: 4, risk: 1, authority_debt: 0, coverage_gain: 5 },
  { id: "cheap", worst: 5, leverage: 2, cost: 1, risk: 0, authority_debt: 0, coverage_gain: 2 },
]);
assert.deepEqual(frontier.map((question) => question.id), ["wide", "cheap"]);

process.stdout.write("question-bank exploration algorithm checks passed\n");
