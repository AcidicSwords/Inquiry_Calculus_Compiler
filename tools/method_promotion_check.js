#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs"), path = require("node:path"), os = require("node:os");
const cp = require("node:child_process"), crypto = require("node:crypto");
const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.MethodPromotion";
const modulePath = "formal/InquiryCalculus/Legacy/V20/MethodPromotion.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");

function sourceRecord(tex, classification) {
  assert.equal(hash(tex), texDigest);
  const id = "PRED-TEX-DECL-5F05913A51977D2E", records = classification.records.filter((record) => record.source_id === id);
  assert.equal(records.length, 1);
  const record = records[0], lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  const excerpt = lines.slice(4275, 4291).map((line) => line.trimEnd()).join("\n").trim();
  assert.equal(record.source.start_line, 4276); assert.equal(record.source.end_line, 4291);
  assert.equal(record.disposition, "FormalDefinition");
  assert.equal(record.source.revision, `sha256:${texDigest}`);
  assert.equal(record.source.sha256, hash(excerpt)); assert.equal(record.source_excerpt_sha256, hash(excerpt));
  return record;
}

function finiteModel() {
  const promotable = (candidate) => (candidate.parallel || candidate.bridge) && candidate.equivalent &&
    candidate.descent && (candidate.gain || candidate.purpose) && candidate.recovery.some(Boolean) &&
    candidate.unlock.some(Boolean) && !candidate.futureWarrant;
  const overcomplete = { parallel: true, bridge: true, equivalent: true, descent: true, gain: true,
    purpose: true, recovery: [true, true], unlock: [true, true], futureWarrant: false };
  const contracted = { ...overcomplete, bridge: false, purpose: false, recovery: [true, false], unlock: [true, false] };
  assert.equal(promotable(overcomplete), true); assert.equal(promotable(contracted), true);
  const breakers = [
    { ...contracted, parallel: false, bridge: false }, { ...contracted, equivalent: false },
    { ...contracted, descent: false }, { ...contracted, gain: false, purpose: false },
    { ...contracted, recovery: [false, false] }, { ...contracted, unlock: [false, false] },
    { ...contracted, futureWarrant: true },
  ];
  assert.ok(breakers.every((candidate) => !promotable(candidate)));
}

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const read = (relative) => fs.readFileSync(path.join(root, relative));
  const tex = read("Inquiry_Calculus_v2_0.tex"), classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const source = sourceRecord(tex, classification), lean = read(modulePath).toString();
  const doc = read("formal-successor/PHASE_B_METHOD_PROMOTION.md").toString();
  assert.match(doc, /PRED-TEX-DECL-5F05913A51977D2E` \| 4276–4291 \| `FormalDefinition/u);
  const changedClassification = structuredClone(classification);
  changedClassification.records.find((record) => record.source_id === source.source_id).disposition = "LegacyObligation";
  assert.throws(() => sourceRecord(tex, changedClassification));
  assert.throws(() => sourceRecord(Buffer.concat([tex, Buffer.from("\n")]), classification));
  finiteModel();
  for (const token of ["parallelOrBridged", "equivalentOnApplicability", "requiredContinuationsDescend",
    "gainOrNecessaryPurpose", "recoverableDefiningPath", "reopeningStored", "utilityDoesNotWarrantFutureOutputs"] ) {
    assert.match(lean, new RegExp(`\\b${token}\\b`, "u"));
  }
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(), /^import InquiryCalculus\.Legacy\.V20\.MethodPromotion\r?$/mu);
  console.log(`PASS exact method-promotion source and seven finite condition breakers; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-method-promotion-"));
  const run = (args) => cp.spawnSync("lake", args, { cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true });
  function probe(name, body, reject = false, ownSource = false) {
    const file = path.join(directory, `${name}.lean`); fs.writeFileSync(file, (ownSource ? "" : `import ${moduleName}\n`) + body);
    const result = run(["env", "lean", file]), output = result.stdout + result.stderr;
    if (reject) { assert.notEqual(result.status, 0, `accepted ${name}`); assert.match(output, /error:/u); }
    else assert.equal(result.status, 0, output);
    return output;
  }
  const build = run(["build", moduleName, "--wfail"]); assert.equal(build.status, 0, build.stdout + build.stderr);
  const audits = ["overcompleteCrosses", "contractedRetainsBoundary", "alignmentRequired", "equivalenceRequired",
    "descentRequired", "gainOrPurposeRequired", "recoveryRequired", "unlockRequired", "futureOutputWarrantRejected"];
  const output = probe("contracts", `open ${moduleName}\nopen Countermodel\n` + audits.map((name) =>
    `#print axioms ${moduleName}.Countermodel.${name}`).join("\n"));
  for (const name of audits) assert.match(output, new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.Countermodel\\.${name}' does not depend on any axioms`));

  const ablations = [
    ["alignment", "context.parallel method path ∨ context.explicitlyBridged method path", "True"],
    ["equivalence", "∀ region, context.applicable method path region → context.protectedEquivalent method path region", "True"],
    ["descent", "∀ continuation, context.requiredContinuation continuation →\n      context.continuationDescends method path continuation", "True"],
    ["gain-purpose", "context.typedOperationalGain method path ∨ context.necessaryImplementationPurpose method path", "True"],
    ["recovery", "∃ evidence, context.definingEvidence method path evidence ∧\n      context.recoversDefiningPath evidence method path", "True"],
    ["unlock", "∃ unlock, context.unlockStored method path unlock", "True"],
    ["future-warrant", "¬ context.utilityConfersFutureOutputWarrant method path", "True"],
  ];
  for (const [name, before, after] of ablations) {
    const changed = lean.replace(before, after); assert.notEqual(changed, lean); probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS typed method promotion; 7 rejected source ablations; ${audits.length} axiom-free proof audits`);
}

main();
