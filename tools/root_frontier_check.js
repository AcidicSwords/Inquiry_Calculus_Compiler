#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.RootFrontier";
const modulePath = "formal/InquiryCalculus/Legacy/V20/RootFrontier.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-PROSE-69B0535AAA958C2D", { lines: [5163, 5164], status: "Ambiguous" }],
  ["PRED-TEX-DISPLAY-E07201FB53CF17DC", { lines: [5165, 5179], status: "Ambiguous" }],
  ["PRED-TEX-PROSE-CD9D7AE878A04C10", { lines: [5180, 5183], status: "Unproved" }]
]);
const read = (name) => fs.readFileSync(path.join(root, name));
const digest = (value) => crypto.createHash("sha256").update(value).digest("hex");

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const document = read("formal-successor/PHASE_B_ROOT_FRONTIER.md").toString();
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
    "RootFrontierContext", "Eligible", "RootFrontier", "ExactEligibleField",
    "rootFrontierIsEligible", "missingPreorderPreservesRootField", "RootFrontierObligation",
    "FiniteProfile", "eligibleFlag", "rootField", "dominatedFlag", "nondominatedFlag",
    "rootFrontierFlag", "eligibleFieldIsExact", "productiveIsRootFrontier",
    "requiredIsOrdinarilyDominated", "requiredIsRootFrontier",
    "dominatedOptionalIsNotRootFrontier", "nonRootIsNotEligible",
    "rootProducedDoesNotImplyAdmission", "nonformableIsNotEligible",
    "inapplicableIsNotEligible", "nonexecutableIsNotEligible", "idleIsNotEligible",
    "missingPreorderRetainsEveryEligibleCandidate"
  ]) assert.match(lean, new RegExp(`\\b${declaration}\\b`));
  assert.match(document, /not as another\s+schema/iu);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.RootFrontier\r?$/mu);
  console.log(`PASS exact root-frontier sources and compositional finite contrasts; module sha256 ${digest(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "ic-root-frontier-"));
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
    "rootFrontierIsEligible", "missingPreorderPreservesRootField",
    "Countermodel.eligibleFieldIsExact", "Countermodel.productiveIsRootFrontier",
    "Countermodel.requiredIsOrdinarilyDominated", "Countermodel.requiredIsRootFrontier",
    "Countermodel.dominatedOptionalIsNotRootFrontier", "Countermodel.nonRootIsNotEligible",
    "Countermodel.rootProducedDoesNotImplyAdmission", "Countermodel.nonformableIsNotEligible",
    "Countermodel.inapplicableIsNotEligible", "Countermodel.nonexecutableIsNotEligible",
    "Countermodel.idleIsNotEligible", "Countermodel.missingPreorderRetainsEveryEligibleCandidate"
  ];
  for (const [index, proof] of proofs.entries()) {
    const audit = probe(`axiom-${index}`, `#print axioms ${moduleName}.${proof}`);
    assert.match(audit, /does not depend on any axioms/u, proof);
  }
  for (const [name, before, after] of [
    ["root-produced", "  rootProduced : Occurrence → Prop", "  rootProduced : True"],
    ["admitted", "  admitted : Occurrence → Prop", "  admitted : True"],
    ["membership", "  membership : MembershipContext Occurrence Dependency", "  membership : True"],
    ["root-field", "  rootField : List Occurrence", "  rootField : True"],
    ["finite-root", "  rootProduced : Bool", "  rootProduced : True"],
    ["finite-admitted", "  admitted : Bool", "  admitted : True"],
    ["formable", "  formable : Bool", "  formable : True"],
    ["applicable", "  applicable : Bool", "  applicable : True"],
    ["executable", "  executable : Bool", "  executable : True"],
    ["productive", "  productive : Bool", "  productive : True"],
    ["required", "  required : Bool", "  required : True"],
    ["eligible", "def eligibleFlag (occurrence : Occurrence) : Bool :=", "def eligibleFlag (occurrence : True) : Bool :="],
    ["dominance", "def dominatedFlag (occurrence other : Occurrence) : Bool :=", "def dominatedFlag (occurrence other : True) : Bool :="],
    ["nondominance", "def nondominatedFlag (occurrence : Occurrence) : Bool :=", "def nondominatedFlag (occurrence : True) : Bool :="],
    ["frontier", "def rootFrontierFlag (occurrence : Occurrence) : Bool :=", "def rootFrontierFlag (occurrence : True) : Bool :="]
  ]) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS fifteen root-frontier ablations and ${proofs.length} axiom-free proof audits`);
}

main();
