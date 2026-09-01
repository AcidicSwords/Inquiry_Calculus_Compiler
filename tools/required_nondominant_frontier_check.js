#!/usr/bin/env node
"use strict";
const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.RequiredNondominantFrontier";
const modulePath = "formal/InquiryCalculus/Legacy/V20/RequiredNondominantFrontier.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-PROSE-4E8614813C797872", [4981, 4982, "Ambiguous"]],
  ["PRED-TEX-DISPLAY-9232F23AE99FFE1C", [4983, 4987, "Ambiguous"]],
  ["PRED-TEX-DISPLAY-932AF6B9B331E648", [4988, 4996, "Ambiguous"]],
  ["PRED-TEX-PROSE-3ED5ED395AEBA8BC", [4997, 5001, "Unproved"]]
]);
const digest = value => crypto.createHash("sha256").update(value).digest("hex");
const read = relative => fs.readFileSync(path.join(root, relative));

function main() {
  assert.ok(process.argv.slice(2).every(argument => argument === "--compile"));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const document = read("formal-successor/PHASE_B_REQUIRED_NONDOMINANT_FRONTIER.md").toString();
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  assert.equal(digest(tex), texDigest);
  for (const [id, [start, end, status]] of sources) {
    const matches = classification.records.filter(record => record.source_id === id);
    assert.equal(matches.length, 1, id);
    const record = matches[0];
    const excerpt = lines.slice(start - 1, end).map(line => line.trimEnd()).join("\n").trim();
    assert.equal(record.disposition, "LegacyObligation", id);
    assert.equal(record.legacy_obligation.status, status, id);
    assert.deepEqual([record.source.start_line, record.source.end_line], [start, end], id);
    assert.equal(record.source.sha256, digest(excerpt), id);
    assert.equal(record.source_excerpt_sha256, digest(excerpt), id);
  }
  for (const declaration of [
    "FrontierContext", "required", "strictlyDominatedBy", "discharges", "RequiredSet", "Nondominated",
    "RequiredNondominant", "NoPreorderFrontier", "RequiredRemovalLicensed", "requiredIsRetained",
    "requiredIsOrdinarilyDominated", "requiredSurvivesOrdinaryDominance", "optionalIsDominatedByRequired",
    "optionalIsNotRequiredNondominant", "typedSubstitutesLicenseRequiredRemoval",
    "alphaSubstituteDoesNotDischargeBeta", "missingPreorderRetainsAll"
  ]) assert.match(lean, new RegExp(`\\b${declaration}\\b`));
  assert.match(document, /four exact `LegacyObligation` records/u);
  assert.match(document, /three `Ambiguous` and one `Unproved`/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(), /^import InquiryCalculus\.Legacy\.V20\.RequiredNondominantFrontier\r?$/mu);
  console.log(`PASS exact required-nondominant-frontier sources and typed-substitution contrasts; module sha256 ${digest(lean)}`);
  if (!process.argv.includes("--compile")) return;
  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "ic-required-nondominant-frontier-"));
  const run = arguments_ => childProcess.spawnSync("lake", arguments_, { cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true });
  const build = run(["build", moduleName, "--wfail"]);
  assert.equal(build.status, 0, build.stdout + build.stderr);
  const probe = (name, body, reject = false, ownModule = false) => {
    const file = path.join(temporary, `${name}.lean`);
    fs.writeFileSync(file, `${ownModule ? "" : `import ${moduleName}\n`}${body}`);
    const result = run(["env", "lean", file]);
    const output = result.stdout + result.stderr;
    if (reject) {
      assert.notEqual(result.status, 0, `accepted ${name}`);
      assert.match(output, /error(?:\([^)]*\))?:/u);
    } else assert.equal(result.status, 0, output);
    return output;
  };
  const proofs = [
    "requiredIsRetained", "Countermodel.requiredIsOrdinarilyDominated",
    "Countermodel.requiredSurvivesOrdinaryDominance", "Countermodel.optionalIsDominatedByRequired",
    "Countermodel.optionalIsNotRequiredNondominant", "Countermodel.typedSubstitutesLicenseRequiredRemoval",
    "Countermodel.alphaSubstituteDoesNotDischargeBeta", "Countermodel.missingPreorderRetainsAll"
  ];
  const audit = probe("axioms", proofs.map(proof => `#print axioms ${moduleName}.${proof}`).join("\n"));
  for (const proof of proofs) assert.match(audit, /does not depend on any axioms/u);
  for (const [name, before, after] of [
    ["required", "  required : Occurrence → Dependency → Prop", "  required : Occurrence → Dependency → True"],
    ["dominance", "  strictlyDominatedBy : Occurrence → Occurrence → Prop", "  strictlyDominatedBy : Occurrence → Occurrence → True"],
    ["discharge", "  discharges : Occurrence → Dependency → Prop", "  discharges : Occurrence → Dependency → True"],
    ["required-set", "def RequiredSet", "def RequiredSetRemoved"],
    ["nondominated", "def Nondominated", "def NondominatedRemoved"],
    ["union", "def RequiredNondominant", "def RequiredNondominantRemoved"],
    ["fallback", "def NoPreorderFrontier", "def NoPreorderFrontierRemoved"],
    ["licence", "def RequiredRemovalLicensed", "def RequiredRemovalLicensedRemoved"]
  ]) {
    const altered = lean.replace(before, after);
    assert.notEqual(altered, lean, name);
    probe(`drop-${name}`, altered, true, true);
  }
  console.log(`PASS eight required-nondominant-frontier ablations and ${proofs.length} axiom-free proof audits`);
}

main();
