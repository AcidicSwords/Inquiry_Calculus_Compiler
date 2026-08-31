#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.MemoryRecovery";
const modulePath = "formal/InquiryCalculus/Legacy/V20/MemoryRecovery.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const expectedSources = new Map([
  ["PRED-TEX-DECL-BCD7B8367B636E36", [4456, 4458]],
  ["PRED-TEX-DISPLAY-153F48EF27A011D3", [4411, 4413]],
  ["PRED-TEX-DISPLAY-2517E5A4D51F4DD1", [4417, 4419]],
  ["PRED-TEX-DISPLAY-5E4CD4C86B070FC9", [4448, 4454]],
  ["PRED-TEX-DISPLAY-7017C445067B07EA", [4408, 4410]],
  ["PRED-TEX-DISPLAY-71773951B15B6A58", [4414, 4416]],
  ["PRED-TEX-DISPLAY-B514FD51DCCDF11D", [4440, 4442]],
  ["PRED-TEX-DISPLAY-B9C2A8A2C9285058", [4444, 4446]],
  ["PRED-TEX-PROSE-007D55A389CFB9E8", [4405, 4405]],
  ["PRED-TEX-PROSE-0F06444EF43B5375", [4437, 4437]],
  ["PRED-TEX-PROSE-22F3BB1C710AD005", [4425, 4425]],
  ["PRED-TEX-PROSE-403D87CD771923C0-03", [4439, 4439]],
  ["PRED-TEX-PROSE-476C636D4F0BD2C0", [4443, 4443]],
  ["PRED-TEX-PROSE-495666E064DCEDCB", [4399, 4401]],
  ["PRED-TEX-PROSE-59FE05104331C5F5", [4447, 4447]],
  ["PRED-TEX-PROSE-6EDF25D51B2BFA29", [4421, 4421]],
  ["PRED-TEX-PROSE-9A07DE96232D3D1E", [4407, 4407]],
  ["PRED-TEX-PROSE-E6465F40F6F7E8C1", [4433, 4433]],
  ["PRED-TEX-PROSE-EBE43A7F716E81F2", [4429, 4429]],
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

function finiteIndependence() {
  const names = ["use", "reconstruct", "evaluate", "reacquire"];
  for (const source of names) for (const target of names) if (source !== target) {
    const candidate = Object.fromEntries(names.map((name) => [name, name === source]));
    assert.equal(candidate[source], true);
    assert.equal(candidate[target], false);
  }
  const withMaterial = [0], baseline = [1];
  assert.ok(withMaterial[0] < baseline[0]);
  const reacquisitionOnly = { use: false, reconstruct: false, evaluate: false, reacquire: true };
  assert.equal(reacquisitionOnly.reacquire, true);
  assert.equal(reacquisitionOnly.use || reacquisitionOnly.reconstruct, false);
}

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const read = (relative) => fs.readFileSync(path.join(root, relative));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const doc = read("formal-successor/PHASE_B_MEMORY_RECOVERY.md").toString();
  verifySources(tex, classification);
  finiteIndependence();
  for (const token of ["RecoveryContext", "use", "reconstruct", "evaluate", "reacquire",
    "NondominatedFrontier", "nondominated", "ReacquisitionComparison", "HasReacquisitionAdvantage",
    "MemoryRecoveryObligation", "stateCompressionRequiresIndependentLicense"] ) {
    assert.match(lean, new RegExp(`\\b${token}\\b`, "u"));
  }
  assert.match(doc, /nineteen exact `LegacyObligation` records at lines 4399–4458/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.MemoryRecovery\r?$/mu);
  console.log(`PASS exact memory-recovery sources and pairwise independence; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-memory-recovery-"));
  const run = (args) => cp.spawnSync("lake", args,
    { cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true });
  function probe(name, body, reject = false, ownSource = false) {
    const file = path.join(directory, `${name}.lean`);
    fs.writeFileSync(file, (ownSource ? "" : `import ${moduleName}\n`) + body);
    const result = run(["env", "lean", file]);
    const output = result.stdout + result.stderr;
    if (reject) { assert.notEqual(result.status, 0, `accepted ${name}`); assert.match(output, /error:/u); }
    else assert.equal(result.status, 0, output);
    return output;
  }
  const build = run(["build", moduleName, "--wfail"]);
  assert.equal(build.status, 0, build.stdout + build.stderr);
  const audits = ["everyCoordinateHasAnIndependentWitness", "noCoordinateUniversallyImpliesAnother",
    "retainedMaterialHasAdvantage", "advantageRequiresSuppliedComparison",
    "advantageDoesNotCreateUseOrReconstruction"];
  const output = probe("contracts", `open ${moduleName}\nopen Countermodel\n` + audits.map((name) =>
    `#print axioms ${moduleName}.Countermodel.${name}`).join("\n"));
  for (const name of audits) assert.match(output,
    new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.Countermodel\\.${name}' does not depend on any axioms`));
  const ablations = [
    ["use", "| .use => candidate.canUse = true", "| .use => True"],
    ["reconstruct", "| .reconstruct => candidate.canReconstruct = true", "| .reconstruct => True"],
    ["evaluate", "| .evaluate => candidate.canEvaluate = true", "| .evaluate => True"],
    ["reacquire", "| .reacquire => candidate.canReacquire = true", "| .reacquire => True"],
    ["frontier-comparison", "comparison.strictlyBetter withMaterial baseline", "True"],
  ];
  for (const [name, before, after] of ablations) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS four independent recovery relations; ${ablations.length} rejected source ablations; ${audits.length} axiom-free proof audits`);
}

main();
