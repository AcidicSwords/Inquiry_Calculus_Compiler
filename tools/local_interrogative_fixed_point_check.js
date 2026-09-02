#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.LocalInterrogativeFixedPoint";
const modulePath = "formal/InquiryCalculus/Legacy/V20/LocalInterrogativeFixedPoint.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-DECL-DEF-LOCAL-IFP", { lines: [5185, 5191], disposition: "FormalDefinition" }],
  ["PRED-TEX-DECL-LAW-LOCAL-IFP-REOPENS", {
    lines: [5193, 5197], disposition: "LegacyObligation", status: "Unproved"
  }]
]);
const read = (name) => fs.readFileSync(path.join(root, name));
const digest = (value) => crypto.createHash("sha256").update(value).digest("hex");

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const document = read("formal-successor/PHASE_B_LOCAL_INTERROGATIVE_FIXED_POINT.md").toString();
  assert.equal(digest(tex), texDigest);
  for (const [sourceId, expected] of sources) {
    const matches = classification.records.filter((record) => record.source_id === sourceId);
    assert.equal(matches.length, 1, sourceId);
    const record = matches[0];
    const excerpt = lines.slice(expected.lines[0] - 1, expected.lines[1])
      .map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(record.disposition, expected.disposition);
    if (expected.status) assert.equal(record.legacy_obligation.status, expected.status);
    assert.deepEqual([record.source.start_line, record.source.end_line], expected.lines);
    assert.equal(record.source.sha256, digest(excerpt));
    assert.equal(record.source_excerpt_sha256, digest(excerpt));
  }
  for (const declaration of [
    "ExitEvidence", "AllowedExit", "LocalContext", "CarriesRequiredDischarge",
    "OpenResidual", "LocalIFP", "localIFPHasExplicitCoverage",
    "localIFPClassifiesEveryRelevantOccurrence", "localIFPHasNoOpenResidual",
    "EnlargementKind", "ReopeningWitness", "Reopens", "sameContextDoesNotReopen",
    "positiveOpenResidualReopens", "LocalIFPObligation", "ExitKind", "relevant", "allowed",
    "openResidual", "finiteField", "localIFPFlag", "reopeningFlag",
    "everyAllowedExitIsRepresented", "coveredCompleteFieldIsLocallyClosed",
    "missingCoverageIsNotClosed", "productiveResidualPreventsClosure",
    "requiredResidualPreventsClosure", "extensionDependentExitRemainsPresent",
    "blockedExitRemainsPresent", "resourceExitRemainsPresent", "newlyLiveOccurrenceReopens",
    "contextLabelWithoutNewOpenOccurrenceDoesNotReopen"
  ]) assert.match(lean, new RegExp(`\\b${declaration}\\b`));
  assert.match(document, /not as another controller/iu);
  assert.match(document, /same context cannot reopen itself/iu);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.LocalInterrogativeFixedPoint\r?$/mu);
  console.log(`PASS exact local-IFP sources, seven exits, open-residual boundary, and positive reopening; module sha256 ${digest(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "ic-local-ifp-"));
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
    "localIFPHasExplicitCoverage", "localIFPClassifiesEveryRelevantOccurrence",
    "localIFPHasNoOpenResidual", "sameContextDoesNotReopen", "positiveOpenResidualReopens",
    "Countermodel.everyAllowedExitIsRepresented",
    "Countermodel.coveredCompleteFieldIsLocallyClosed", "Countermodel.missingCoverageIsNotClosed",
    "Countermodel.productiveResidualPreventsClosure", "Countermodel.requiredResidualPreventsClosure",
    "Countermodel.extensionDependentExitRemainsPresent", "Countermodel.blockedExitRemainsPresent",
    "Countermodel.resourceExitRemainsPresent", "Countermodel.newlyLiveOccurrenceReopens",
    "Countermodel.contextLabelWithoutNewOpenOccurrenceDoesNotReopen"
  ];
  for (const [index, proof] of proofs.entries()) {
    const audit = probe(`axiom-${index}`, `#print axioms ${moduleName}.${proof}`);
    assert.match(audit, /does not depend on any axioms/u, proof);
  }
  for (const [name, before, after] of [
    ["determined", "  determinedByRetainedProfile : Occurrence → Prop", "  determinedByRetainedProfile : True"],
    ["redundant", "  factorablyRedundant : Occurrence → Prop", "  factorablyRedundant : True"],
    ["inapplicable", "  inapplicable : Occurrence → Prop", "  inapplicable : True"],
    ["equivalent", "  protectedContinuationEquivalentWithoutRequired : Occurrence → Prop", "  protectedContinuationEquivalentWithoutRequired : True"],
    ["blocked", "  explicitlyBlocked : Occurrence → Prop", "  explicitlyBlocked : True"],
    ["resource", "  resourceBounded : Occurrence → Prop", "  resourceBounded : True"],
    ["extension", "  representedExtensionDependent : Occurrence → Prop", "  representedExtensionDependent : True"],
    ["coverage", "  explicitCoverage : Prop", "  explicitCoverage : True"],
    ["relevant", "  relevantRootOccurrence : Occurrence → Prop", "  relevantRootOccurrence : True"],
    ["exit-carrier", "  exits : ExitEvidence Occurrence", "  exits : True"],
    ["root", "  root : RootFrontierContext Occurrence Dependency", "  root : True"],
    ["required", "def CarriesRequiredDischarge {Occurrence : Type u} {Dependency : Type v}", "def CarriesRequiredDischarge {Occurrence : Type u} {Dependency : True}"],
    ["open", "def OpenResidual {Occurrence : Type u} {Dependency : Type v}", "def OpenResidual {Occurrence : Type u} {Dependency : True}"],
    ["closure", "def LocalIFP {Occurrence : Type u} {Dependency : Type v}", "def LocalIFP {Occurrence : Type u} {Dependency : True}"],
    ["reopening", "def Reopens {Occurrence : Type u} {Dependency : Type v}", "def Reopens {Occurrence : Type u} {Dependency : True}"]
  ]) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS fifteen local-IFP ablations and ${proofs.length} axiom-free proof audits`);
}

main();
