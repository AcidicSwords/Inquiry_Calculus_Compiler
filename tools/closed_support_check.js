#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.ClosedSupport";
const modulePath = "formal/InquiryCalculus/Legacy/V20/ClosedSupport.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-PROSE-24D9352360BD5CAB", [4564, 4564, "Ambiguous"]],
  ["PRED-TEX-DISPLAY-2D72F8019D4A9E58", [4565, 4567, "Ambiguous"]],
  ["PRED-TEX-PROSE-51CC38D401B91ADF", [4568, 4568, "Ambiguous"]],
  ["PRED-TEX-ITEM-BB08A40E6AC23565", [4570, 4570, "Ambiguous"]],
  ["PRED-TEX-ITEM-0E5B1A98C242662F", [4571, 4571, "Ambiguous"]],
  ["PRED-TEX-ITEM-33EEFE3D12FD9390", [4572, 4572, "Ambiguous"]],
  ["PRED-TEX-ITEM-6566C1167B7D015C", [4573, 4573, "Unproved"]],
  ["PRED-TEX-ITEM-25A562902D8941FF", [4574, 4574, "Ambiguous"]]
]);
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");

function verifySources(tex, classification) {
  assert.equal(hash(tex), texDigest);
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  for (const [id, [start, end, status]] of sources) {
    const records = classification.records.filter((record) => record.source_id === id);
    assert.equal(records.length, 1, id);
    const record = records[0];
    const excerpt = lines.slice(start - 1, end).map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(record.disposition, "LegacyObligation", id);
    assert.equal(record.legacy_obligation.status, status, id);
    assert.deepEqual([record.source.start_line, record.source.end_line], [start, end], id);
    assert.equal(record.source.revision, `sha256:${texDigest}`, id);
    assert.equal(record.source.sha256, hash(excerpt), id);
    assert.equal(record.source_excerpt_sha256, hash(excerpt), id);
  }
}

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const read = (relative) => fs.readFileSync(path.join(root, relative));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const doc = read("formal-successor/PHASE_B_CLOSED_SUPPORT.md").toString();
  verifySources(tex, classification);
  for (const token of ["ClosureContext", "requiresStandingPremise", "belongsToStanding",
    "applicable", "scopeHolds", "openDependency", "requiredIndependentCheck",
    "independentCheckSucceeded", "explicitPolicy", "policyInvalidates", "targetStanding",
    "StandingPremisesSatisfied", "ApplicabilityAndScopeHold", "OpenBoundaryEmpty",
    "RequiredChecksSucceeded", "NoExplicitPolicyInvalidates", "IsClosedSupport",
    "completeRouteIsClosed", "emptyBoundaryAloneIsInsufficient",
    "sixAtomicAblationsBreakClosure", "closedRouteDoesNotEstablishTargetStanding",
    "policyAbsenceDiffersFromExplicitNonInvalidation"]) {
    assert.match(lean, new RegExp(`\\b${token}\\b`, "u"));
  }
  assert.match(doc, /eight `LegacyObligation` records at v2\.0 lines 4564–4575/u);
  assert.match(doc, /Seven remain\s+`Ambiguous`/u);
  assert.match(doc, /required-independent-check clause remains `Unproved`/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.doesNotMatch(lean, /closureImpliesStanding|emptyBoundaryImpliesClosure|automaticStanding/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(), /^import InquiryCalculus\.Legacy\.V20\.ClosedSupport\r?$/mu);
  console.log(`PASS exact closed-support sources and six-breaker contrast; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-closed-support-"));
  const run = (args) => cp.spawnSync("lake", args, {
    cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true
  });
  function probe(name, body, reject = false, own = false) {
    const file = path.join(directory, `${name}.lean`);
    fs.writeFileSync(file, (own ? "" : `import ${moduleName}\n`) + body);
    const result = run(["env", "lean", file]);
    const output = result.stdout + result.stderr;
    if (reject) {
      assert.notEqual(result.status, 0, `accepted ${name}`);
      assert.match(output, /error(?:\([^)]*\))?:/u);
    } else {
      assert.equal(result.status, 0, output);
    }
    return output;
  }
  const build = run(["build", moduleName, "--wfail"]);
  assert.equal(build.status, 0, build.stdout + build.stderr);
  const audits = [
    "closedSupportRetainsStandingPremises", "closedSupportRetainsApplicabilityAndScope",
    "closedSupportRetainsEmptyBoundary", "closedSupportRetainsChecks",
    "closedSupportRetainsPolicyBoundary", "Countermodel.completeRouteIsClosed",
    "Countermodel.emptyBoundaryAloneIsInsufficient",
    "Countermodel.sixAtomicAblationsBreakClosure",
    "Countermodel.closedRouteDoesNotEstablishTargetStanding",
    "Countermodel.policyAbsenceDiffersFromExplicitNonInvalidation"
  ];
  const output = probe("contracts", `open ${moduleName}\n` + audits
    .map((name) => `#print axioms ${moduleName}.${name}`).join("\n"));
  for (const name of audits) {
    assert.match(output, new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.${name.replaceAll(".", "\\.")}' does not depend on any axioms`));
  }
  const ablations = [
    ["standing-premise-relation", "requiresStandingPremise : Environment → Premise → Prop", "requiresStandingPremise : True"],
    ["standing-membership", "belongsToStanding : StandingSet → Premise → Prop", "belongsToStanding : True"],
    ["applicability", "applicable : Environment → Candidate → Prop", "applicable : True"],
    ["scope", "scopeHolds : Environment → Candidate → Prop", "scopeHolds : True"],
    ["open-boundary", "openDependency : Environment → Candidate → Dependency → Prop", "openDependency : True"],
    ["required-check", "requiredIndependentCheck : Environment → Candidate → IndependentCheck → Prop", "requiredIndependentCheck : True"],
    ["check-success", "independentCheckSucceeded : IndependentCheck → Prop", "independentCheckSucceeded : True"],
    ["explicit-policy", "explicitPolicy : Environment → InconsistencyPolicy → Prop", "explicitPolicy : True"],
    ["policy-invalidation", "policyInvalidates : InconsistencyPolicy → Environment → Prop", "policyInvalidates : True"],
    ["target-standing", "targetStanding : StandingSet → Candidate → Prop", "targetStanding : True"],
    ["standing-clause", "StandingPremisesSatisfied context standing environment ∧", "True ∧"],
    ["applicability-clause", "context.applicable environment candidate ∧ context.scopeHolds", "True ∧ context.scopeHolds"],
    ["scope-clause", "context.applicable environment candidate ∧ context.scopeHolds", "context.applicable environment candidate ∧ True"],
    ["boundary-clause", "OpenBoundaryEmpty context environment candidate ∧", "True ∧"],
    ["checks-clause", "RequiredChecksSucceeded context environment candidate ∧", "True ∧"],
    ["policy-clause", "NoExplicitPolicyInvalidates context environment", "True"]
  ];
  for (const [name, before, after] of ablations) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS five-clause closed support; ${ablations.length} rejected source ablations; ${audits.length} axiom-free proof audits`);
}

main();
