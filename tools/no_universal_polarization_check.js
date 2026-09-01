#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.NoUniversalPolarization";
const modulePath = "formal/InquiryCalculus/Legacy/V20/NoUniversalPolarization.lean";
const sourceId = "PRED-TEX-DECL-LAW-NO-UNIVERSAL-POLARIZATION";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const read = (name) => fs.readFileSync(path.join(root, name));
const digest = (value) => crypto.createHash("sha256").update(value).digest("hex");

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const document = read("formal-successor/PHASE_B_NO_UNIVERSAL_POLARIZATION.md").toString();
  assert.equal(digest(tex), texDigest);
  const matches = classification.records.filter((record) => record.source_id === sourceId);
  assert.equal(matches.length, 1);
  const record = matches[0];
  const excerpt = lines.slice(5102, 5108).map((line) => line.trimEnd()).join("\n").trim();
  assert.equal(record.disposition, "LegacyObligation");
  assert.equal(record.legacy_obligation.status, "Unproved");
  assert.deepEqual([record.source.start_line, record.source.end_line], [5103, 5108]);
  assert.equal(record.source.sha256, digest(excerpt));
  assert.equal(record.source_excerpt_sha256, digest(excerpt));
  for (const declaration of [
    "PolarizationKind", "logicalComplement", "counterexample", "alternativeDesign", "separator",
    "contextualPositiveNegation", "AdmittedSemantic", "negationUse", "exterior", "departureWitness",
    "standingBreaker", "BindingPolarization", "PolarizedCandidate", "AdmissionBoundary", "MayTransport",
    "NoUniversalPolarizationObligation", "fiveKindsArePairwiseDistinct",
    "separatedCannotTransportCounterexampleToComplement", "separatedCannotTransportAlternativeToSeparator",
    "bridgedCanTransportCounterexampleToSeparator", "generatedCandidateHasNoAutomaticAdmission",
    "generatedCandidateDoesNotMakeNegationUse", "generatedCandidateDoesNotMakeExterior",
    "generatedCandidateDoesNotMakeDepartureWitness", "generatedCandidateDoesNotMakeStandingBreaker",
    "explicitAdmissionIsIndependent"
  ]) assert.match(lean, new RegExp(`\\b${declaration}\\b`));
  assert.match(document, /source law remains unproved/iu);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.NoUniversalPolarization\r?$/mu);
  console.log(`PASS exact no-universal-polarization source and independent relation/admission contrasts; module sha256 ${digest(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "ic-no-universal-polarization-"));
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
    "Countermodel.fiveKindsArePairwiseDistinct",
    "Countermodel.separatedCannotTransportCounterexampleToComplement",
    "Countermodel.separatedCannotTransportAlternativeToSeparator",
    "Countermodel.bridgedCanTransportCounterexampleToSeparator",
    "Countermodel.generatedCandidateHasNoAutomaticAdmission",
    "Countermodel.generatedCandidateDoesNotMakeNegationUse",
    "Countermodel.generatedCandidateDoesNotMakeExterior",
    "Countermodel.generatedCandidateDoesNotMakeDepartureWitness",
    "Countermodel.generatedCandidateDoesNotMakeStandingBreaker",
    "Countermodel.explicitAdmissionIsIndependent"
  ];
  const audit = probe("axioms", proofs.map((proof) => `#print axioms ${moduleName}.${proof}`).join("\n"));
  for (const proof of proofs) assert.match(audit, /does not depend on any axioms/u);
  for (const [name, before, after] of [
    ["available", "  available : Binding → PolarizationKind → Prop", "  available : True"],
    ["corresponds", "  corresponds : Binding → PolarizationKind → PolarizationKind → Prop", "  corresponds : True"],
    ["candidate-kind", "  kind : PolarizationKind", "  kind : True"],
    ["generated", "  generated : Prop", "  generated : True"],
    ["admitted", "  admitted : PolarizedCandidate Binding → AdmittedSemantic → Prop", "  admitted : True"],
    ["transport", "def MayTransport", "def MayTransportRemoved"]
  ]) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS six no-universal-polarization ablations and ${proofs.length} axiom-free proof audits`);
}

main();
