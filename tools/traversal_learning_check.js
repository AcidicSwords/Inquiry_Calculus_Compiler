#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.TraversalLearning";
const modulePath = "formal/InquiryCalculus/Legacy/V20/TraversalLearning.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const expectedSources = new Map([
  ["PRED-TEX-DISPLAY-725E3A5295DD5AEF", [4311, 4313]],
  ["PRED-TEX-DISPLAY-97AED5957A642D80", [4338, 4344]],
  ["PRED-TEX-DISPLAY-D7B3C70E3DF2EEDD", [4321, 4333]],
  ["PRED-TEX-PROSE-0FFB690FAFFB5A95", [4335, 4335]],
  ["PRED-TEX-PROSE-489B86E311A4C908", [4310, 4310]],
  ["PRED-TEX-PROSE-5759F2A7B76C4718", [4314, 4314]],
  ["PRED-TEX-PROSE-C4A7970D464E4B65", [4337, 4337]],
  ["PRED-TEX-PROSE-D18829E925EFBB30", [4320, 4320]],
  ["PRED-TEX-PROSE-F658A955921C3976", [4318, 4318]],
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

function finiteContrast() {
  const generated = (candidate) => candidate.oldLeft === candidate.oldRight &&
    candidate.freshLeft !== candidate.freshRight && candidate.protectedLeft !== candidate.protectedRight;
  const admitted = (candidate) => generated(candidate) && candidate.laterBenefit;
  const positive = { oldLeft: false, oldRight: false, freshLeft: false, freshRight: true,
    protectedLeft: false, protectedRight: true, laterBenefit: true };
  assert.equal(generated(positive), true);
  assert.equal(admitted(positive), true);
  assert.equal(generated({ ...positive, laterBenefit: false }), true);
  assert.equal(admitted({ ...positive, laterBenefit: false }), false);
  assert.equal(generated({ ...positive, oldRight: true }), false);
  assert.equal(generated({ ...positive, freshRight: false }), false);
  assert.equal(generated({ ...positive, protectedRight: false }), false);
}

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const read = (relative) => fs.readFileSync(path.join(root, relative));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const doc = read("formal-successor/PHASE_B_TRAVERSAL_LEARNING.md").toString();
  verifySources(tex, classification);
  finiteContrast();
  for (const token of ["PathFoldAdmission", "MethodPromotable", "ProtectedNonredundancy",
    "oldBasisAgrees", "candidateSeparates", "protectedConsequenceSeparates",
    "ProbeGenerated", "ProbeAdmission", "independentBenefit"] ) {
    assert.match(lean, new RegExp(`\\b${token}\\b`, "u"));
  }
  assert.match(doc, /nine classified `LegacyObligation` records at lines 4310–4344/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.TraversalLearning\r?$/mu);
  console.log(`PASS exact traversal-learning sources and finite route separation; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-traversal-learning-"));
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
  const audits = ["pathPromotionAdmitsFold", "pathRecurrenceAloneRejected", "overcompleteGenerated",
    "contractedAdmitted", "oldAgreementRequired", "freshSeparationRequired",
    "protectedSeparationRequired", "generationRemainsInert", "independentLaterUseRequired"];
  const output = probe("contracts", `open ${moduleName}\nopen Countermodel\n` + audits.map((name) =>
    `#print axioms ${moduleName}.Countermodel.${name}`).join("\n"));
  for (const name of audits) assert.match(output,
    new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.Countermodel\\.${name}' does not depend on any axioms`));
  const ablations = [
    ["promotion", "promotion : MethodPromotable context method path", "promotion : True"],
    ["old-agreement", "oldBasisAgrees : ∀ probe, context.oldProbe probe →", "oldBasisAgrees : ∀ probe, context.oldProbe probe → True ∧"],
    ["fresh-separation", "context.candidateObserve candidate left ≠ context.candidateObserve candidate right", "True"],
    ["protected-separation", "context.protectedConsequence candidate left ≠ context.protectedConsequence candidate right", "True"],
    ["independent-use", "independentBenefit : context.independentlyDemonstratesBenefit laterUse candidate", "independentBenefit : True"],
  ];
  for (const [name, before, after] of ablations) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS two typed learning routes; ${ablations.length} rejected source ablations; ${audits.length} axiom-free proof audits`);
}

main();
