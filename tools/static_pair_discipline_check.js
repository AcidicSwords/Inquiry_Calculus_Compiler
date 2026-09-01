#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.StaticPairDiscipline";
const modulePath = "formal/InquiryCalculus/Legacy/V20/StaticPairDiscipline.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-PROSE-54953E49A9BA2237", { lines: [5118, 5118], status: "Ambiguous" }],
  ["PRED-TEX-DISPLAY-316B254119AFAC98", { lines: [5119, 5129], status: "Ambiguous" }],
  ["PRED-TEX-PROSE-623519C01195D43C", { lines: [5130, 5136], status: "Unproved" }]
]);
const read = (name) => fs.readFileSync(path.join(root, name));
const digest = (value) => crypto.createHash("sha256").update(value).digest("hex");

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const document = read("formal-successor/PHASE_B_STATIC_PAIR_DISCIPLINE.md").toString();
  assert.equal(digest(tex), texDigest);
  for (const [sourceId, expected] of sources) {
    const matches = classification.records.filter((record) => record.source_id === sourceId);
    assert.equal(matches.length, 1, sourceId);
    const record = matches[0];
    const excerpt = lines.slice(expected.lines[0] - 1, expected.lines[1])
      .map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(record.disposition, "LegacyObligation");
    assert.equal(record.legacy_obligation.status, expected.status);
    assert.deepEqual([record.source.start_line, record.source.end_line], expected.lines);
    assert.equal(record.source.sha256, digest(excerpt));
    assert.equal(record.source_excerpt_sha256, digest(excerpt));
  }
  for (const declaration of [
    "StaticPairKind", "inverse", "converse", "logicalBreakerDual", "bindingSuppliedAdjoint",
    "sameUseReciprocalReturn", "staticPairKinds", "staticPairKindsArePairwiseDistinct",
    "PairPresentation", "BindingSuppliedEquivalence", "LawfulSubstitution",
    "ReciprocalExpectation", "ReciprocalReturnWitness", "LawfulReciprocalReturn",
    "StaticPairObligation", "manyToOne", "relationalConverse", "converseFiberContainsBoth",
    "converseIsNotStrictInverse", "completeBridgeIsLawful", "unsuppliedBridgeIsNotLawful",
    "unpreservedBridgeIsNotLawful", "suppliedBridgeDoesNotCollapseKinds",
    "completeReciprocalIsLawful", "wrongUseIsNotReciprocal",
    "missingAdmissionIsNotReciprocal", "missingDepartureIsNotReciprocal",
    "missingCoverageIsNotReciprocal", "missingFiberIsNotReciprocal",
    "missingProvenanceIsNotReciprocal", "genericBackwardDoesNotSupplyReciprocal"
  ]) assert.match(lean, new RegExp(`\\b${declaration}\\b`));
  assert.match(document, /not successor\s+semantics/iu);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.StaticPairDiscipline\r?$/mu);
  console.log(`PASS exact static-pair sources and finite separation contrasts; module sha256 ${digest(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "ic-static-pair-"));
  const run = (arguments_) => childProcess.spawnSync("lake", arguments_, {
    cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true
  });
  const probe = (name, body, reject = false, ownModule = false) => {
    const filename = path.join(temporary, `${name}.lean`);
    fs.writeFileSync(filename, `${ownModule ? "" : `import ${moduleName}\n`}${body}`);
    const result = run(["env", "lean", filename]);
    const output = result.stdout + result.stderr;
    if (reject) {
      assert.notEqual(result.status, 0, `accepted ${name}`);
      assert.match(output, /error(?:\([^)]*\))?:/u);
    } else assert.equal(result.status, 0, output);
    return output;
  };
  const build = run(["build", moduleName, "--wfail"]);
  assert.equal(build.status, 0, build.stdout + build.stderr);
  const proofs = [
    "staticPairKindsArePairwiseDistinct", "Countermodel.converseFiberContainsBoth",
    "Countermodel.converseIsNotStrictInverse", "Countermodel.completeBridgeIsLawful",
    "Countermodel.unsuppliedBridgeIsNotLawful", "Countermodel.unpreservedBridgeIsNotLawful",
    "Countermodel.suppliedBridgeDoesNotCollapseKinds", "Countermodel.completeReciprocalIsLawful",
    "Countermodel.sevenIsNotFortyTwo", "Countermodel.wrongUseIsNotReciprocal",
    "Countermodel.missingAdmissionIsNotReciprocal",
    "Countermodel.missingDepartureIsNotReciprocal", "Countermodel.missingCoverageIsNotReciprocal",
    "Countermodel.missingFiberIsNotReciprocal", "Countermodel.missingProvenanceIsNotReciprocal",
    "Countermodel.genericBackwardDoesNotSupplyReciprocal"
  ];
  for (const [index, proof] of proofs.entries()) {
    const audit = probe(`axiom-${index}`, `#print axioms ${moduleName}.${proof}`);
    assert.match(audit, /does not depend on any axioms/u, proof);
  }
  for (const [name, before, after] of [
    ["inverse-kind", "  | inverse\n", "  | inverseDropped\n"],
    ["converse-kind", "  | converse\n", "  | converseDropped\n"],
    ["dual-kind", "  | logicalBreakerDual\n", "  | logicalBreakerDualDropped\n"],
    ["adjoint-kind", "  | bindingSuppliedAdjoint\n", "  | bindingSuppliedAdjointDropped\n"],
    ["reciprocal-kind", "  | sameUseReciprocalReturn\n", "  | sameUseReciprocalReturnDropped\n"],
    ["presentation-kind", "  kind : StaticPairKind", "  kind : True"],
    ["presentation-identity", "  relationIdentity : Nat", "  relationIdentity : True"],
    ["bridge-law", "  lawSupplied : Bool", "  lawSupplied : True"],
    ["bridge-preservation", "  protectedContinuationPreserved : Bool", "  protectedContinuationPreserved : True"],
    ["expected-use", "structure ReciprocalExpectation where\n  orientedUse : Nat", "structure ReciprocalExpectation where\n  orientedUse : True"],
    ["witness-use", "structure ReciprocalReturnWitness where\n  orientedUse : Nat", "structure ReciprocalReturnWitness where\n  orientedUse : True"],
    ["admission", "  useAdmitted : Bool", "  useAdmitted : True"],
    ["departure", "  departurePresent : Bool", "  departurePresent : True"],
    ["coverage", "  coveragePresent : Bool", "  coveragePresent : True"],
    ["fiber", "  wholeFiberPresent : Bool", "  wholeFiberPresent : True"],
    ["provenance", "  provenancePresent : Bool", "  provenancePresent : True"]
  ]) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS sixteen static-pair ablations and ${proofs.length} axiom-free proof audits`);
}

main();
