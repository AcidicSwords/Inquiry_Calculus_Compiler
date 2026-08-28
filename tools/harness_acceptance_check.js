#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const path = require("node:path");
const policy = require(path.resolve(__dirname, "..", ".claude", "hooks", "ic-construction-policy.js"));

const results = [];
function test(name, body) {
  body();
  results.push(name);
}

test("1 wide breaker search", () => {
  const selected = policy.selectWideContrast([
    { id: "local-1", admissible: true, decisive: true, eliminated_region: 1, cost: 1 },
    { id: "extreme", admissible: true, decisive: true, eliminated_region: 64, cost: 4 },
    { id: "unsafe", admissible: false, decisive: true, eliminated_region: 1000, cost: 1 },
  ]);
  assert.equal(selected.candidate.id, "extreme");
});

test("2 narrow ratchet", () => {
  const retained = policy.narrowRatchet([
    { id: "r1", independent_check: true },
    { id: "r2", independent_check: false },
    { id: "r3", independent_check: true },
  ], ["r1"]);
  assert.deepEqual(retained.map((relation) => relation.id), ["r1"]);
});

const residuals = [
  { id: "R-A", state: "active", condition_ids: ["C-SHARED"] },
  { id: "R-B", state: "latent", condition_ids: ["C-SHARED", "C-B"] },
  { id: "R-C", state: "blocked", condition_ids: ["C-C"] },
];

test("3 residual persistence", () => {
  const updated = policy.closeLocalResidual(residuals, "R-A", "sample-a-only");
  assert.equal(updated.find((residual) => residual.id === "R-A").state, "locally_closed");
  assert.equal(updated.find((residual) => residual.id === "R-B").state, "latent");
  assert.equal(updated.find((residual) => residual.id === "R-C").state, "blocked");
});

test("4 shared-condition propagation", () => {
  assert.deepEqual(
    policy.residualsSharingCoordinate(residuals, "condition_ids", "C-SHARED"),
    ["R-A", "R-B"],
  );
});

test("5 contradiction localization", () => {
  const localized = policy.localizeContradiction(
    { claim: "phi", polarity: "positive", applicability: "linux", premise: ["p"], role: "x", path: "a", representation: "r" },
    { claim: "phi", polarity: "negative", applicability: "windows", premise: ["p"], role: "x", path: "a", representation: "r" },
  );
  assert.deepEqual(localized, { status: "separator_required", dimensions: ["applicability"] });
});

test("6 failed-fold reopening", () => {
  const reopened = policy.revisitFold(
    { id: "F-1", left: "a", right: "b", state: "folded" },
    { id: "D-NEW", partition: [["a"], ["b", "c"]] },
  );
  assert.equal(reopened.state, "reopened");
  assert.equal(reopened.breaker, "D-NEW");
});

const questionBase = {
  underlying_relation: "Separates",
  partial_binding: "fixture",
  exposed_ports: ["left", "right"],
  scope: "acceptance-test",
  grain: "protected-class",
  discharge_obligation: "Check",
  path: "probe->check",
  mode: "Check",
  continuation: "k1",
  binding: "b1",
  horizon: "h1",
  coverage: "c1",
};

test("7 question redundancy", () => {
  const retained = policy.deduplicateQuestions([
    { ...questionBase, wording: "Is A separate?", partition: [["a"], ["b"]] },
    { ...questionBase, wording: "Can B merge with A?", partition: [["b"], ["a"]] },
  ]);
  assert.equal(retained.length, 1);
});

test("8 question non-redundancy", () => {
  const retained = policy.deduplicateQuestions([
    { ...questionBase, wording: "Does it return?", partition: [["a"], ["b"]] },
    { ...questionBase, wording: "Does it return here?", partition: [["a"], ["b"]], mode: "Probe" },
    { ...questionBase, wording: "Does it return later?", partition: [["a", "b"]], continuation: "k2" },
  ]);
  assert.equal(retained.length, 3);
});

test("9 joint variation", () => {
  const breaker = policy.findJointBreaker(["x", "y", "z"], (combination) =>
    combination.includes("x") && combination.includes("y") ? "break" : "hold",
  );
  assert.deepEqual(breaker.coordinates, ["x", "y"]);
});

test("10 anti-premature closure", () => {
  assert.equal(
    policy.closureFromSearch({ breakerFound: false, coverageComplete: false }),
    "Unknown",
  );
});

test("11 method factoring", () => {
  const method = policy.factorMethod([
    { id: "o1", status: "supported", typed_path: ["Q4", "Q9", "Q5"], failure_exits: ["no-crossing"] },
    { id: "o2", status: "supported", typed_path: ["Q4", "Q9", "Q5"], failure_exits: ["mode-blocked"] },
  ]);
  assert.equal(method.status, "candidate_method_not_warranted");
  assert.deepEqual(method.expansion, ["o1", "o2"]);
  assert.deepEqual(method.failure_exits, ["no-crossing", "mode-blocked"]);
});

test("12 harness non-self-warrant", () => {
  assert.equal(policy.acceptanceAuthority({
    baselineDigest: "a",
    candidateDigest: "b",
    authority: "candidate_successor",
  }), "reject_self_warrant");
  assert.equal(policy.acceptanceAuthority({
    baselineDigest: "a",
    candidateDigest: "b",
    authority: "explicit_user_control_migration",
  }), "authorized_change");
});

process.stdout.write(`successor harness acceptance checks passed (${results.length}/12)\n`);
