#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.QuestionPatternLearning";
const modulePath = "formal/InquiryCalculus/Legacy/V20/QuestionPatternLearning.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const expectedSources = new Map([
  ["PRED-TEX-DISPLAY-01537FE3296EDE8F", [4365, 4371]],
  ["PRED-TEX-DISPLAY-190331AEC1877D72", [4360, 4363]],
  ["PRED-TEX-DISPLAY-94591823028CA53D", [4373, 4387]],
  ["PRED-TEX-DISPLAY-AFEBF164021F6209", [4350, 4358]],
  ["PRED-TEX-PROSE-633787EABC7A96C6", [4388, 4393]],
  ["PRED-TEX-PROSE-6A2E83065C76D829", [4364, 4364]],
  ["PRED-TEX-PROSE-6C353AEF6754E514", [4348, 4349]],
  ["PRED-TEX-PROSE-8DC6A6D142B457B9-04", [4372, 4372]],
  ["PRED-TEX-PROSE-B1E71B52F849E16C", [4395, 4395]],
  ["PRED-TEX-PROSE-EF4E8F790A8B8D1E-02", [4359, 4359]],
]);
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");

function verifySources(tex, classification) {
  assert.equal(hash(tex), texDigest);
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  for (const [sourceId, [start, end]] of expectedSources) {
    const records = classification.records.filter((record) => record.source_id === sourceId);
    assert.equal(records.length, 1, sourceId);
    const record = records[0];
    assert.equal(record.disposition, "LegacyObligation", sourceId);
    assert.equal(record.source.start_line, start, sourceId);
    assert.equal(record.source.end_line, end, sourceId);
    assert.equal(record.source.revision, `sha256:${texDigest}`, sourceId);
    const excerpt = lines.slice(start - 1, end).map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(record.source.sha256, hash(excerpt), sourceId);
    assert.equal(record.source_excerpt_sha256, hash(excerpt), sourceId);
  }
}

function finiteCollision() {
  const left = { ask: "a", events: ["e1"], projection: ["open", "probe", "open"],
    stateProjection: ["s", "open", "b", "probe", "s2", "open"], continuation: "k1", next: "n1" };
  const right = { ask: "a2", events: ["e2"], projection: [...left.projection],
    stateProjection: [...left.stateProjection], continuation: "k2", next: "n2" };
  assert.deepEqual(left.projection, right.projection);
  assert.deepEqual(left.stateProjection, right.stateProjection);
  assert.notDeepEqual(left, right);
  const lookup = [left, right].filter((candidate) =>
    JSON.stringify(candidate.projection) === JSON.stringify(left.projection));
  assert.equal(lookup.length, 2);
  assert.equal(lookup.find((candidate) => candidate.ask === "a2"), right);
}

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const read = (relative) => fs.readFileSync(path.join(root, relative));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const doc = read("formal-successor/PHASE_B_QUESTION_PATTERN_LEARNING.md").toString();
  verifySources(tex, classification);
  finiteCollision();
  for (const token of ["OccurrenceChain", "operatorProjection", "stateProjection", "ProjectionLookup",
    "AggregationLicensed", "protectedEquivalent", "LearnedPolicy", "OccurrenceSpecificSelection",
    "applicability", "lookupCandidate", "recovery", "exactChain", "selected"] ) {
    assert.match(lean, new RegExp(`\\b${token}\\b`, "u"));
  }
  assert.match(doc, /ten exact `LegacyObligation` records at lines 4348–4395/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.QuestionPatternLearning\r?$/mu);
  console.log(`PASS exact question-pattern sources and projection collision; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-question-pattern-"));
  const run = (args) => cp.spawnSync("lake", args,
    { cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true });
  function probe(name, body, reject = false, ownSource = false) {
    const file = path.join(directory, `${name}.lean`);
    fs.writeFileSync(file, (ownSource ? "" : `import ${moduleName}\n`) + body);
    const result = run(["env", "lean", file]);
    const output = result.stdout + result.stderr;
    if (reject) {
      assert.notEqual(result.status, 0, `accepted ${name}`);
      assert.match(output, /error:/u);
    } else assert.equal(result.status, 0, output);
    return output;
  }
  const build = run(["build", moduleName, "--wfail"]);
  assert.equal(build.status, 0, build.stdout + build.stderr);
  const audits = ["operatorViewsCollide", "stateViewsCollide", "occurrencesRemainDistinct",
    "projectionLookupRetainsLeft", "projectionLookupRetainsRight", "explicitLicenseAdmits",
    "projectionEqualityDoesNotLicense", "aggregationApplicabilityRequired",
    "recoveredApplicableSelection", "projectionOnlyCannotSelect", "applicabilityRequired",
    "exactOccurrenceRecoveryRequired", "selectedContinuationRequired"];
  const output = probe("contracts", `open ${moduleName}\nopen Countermodel\n` + audits.map((name) =>
    `#print axioms ${moduleName}.Countermodel.${name}`).join("\n"));
  for (const name of audits) assert.match(output,
    new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.Countermodel\\.${name}' does not depend on any axioms`));
  const ablations = [
    ["occurrence-ask", "askRef : AskRef", "askRef : Unit"],
    ["aggregation-applicability", "context.applicable left ∧ context.applicable right ∧ context.protectedEquivalent left right", "context.protectedEquivalent left right"],
    ["protected-license", "context.protectedEquivalent left right", "True"],
    ["policy-applicability", "applicability : policy.applicable target state", "applicability : True"],
    ["exact-recovery", "exactChain : recovered = target", "exactChain : True"],
    ["recovery", "recovery : policy.recover target recovered", "recovery : True"],
    ["selection", "selected : policy.selects recovered continuation", "selected : True"],
  ];
  for (const [name, before, after] of ablations) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS occurrence-primary learning; ${ablations.length} rejected source ablations; ${audits.length} axiom-free proof audits`);
}

main();
