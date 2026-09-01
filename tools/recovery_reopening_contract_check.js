#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.RecoveryReopeningContract";
const modulePath = "formal/InquiryCalculus/Legacy/V20/RecoveryReopeningContract.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-PROSE-3C87E1A798899993", [4679, 4679, "Unproved"]],
  ["PRED-TEX-ITEM-20BC3A3C0D838176", [4681, 4681, "Ambiguous"]],
  ["PRED-TEX-ITEM-37BDCCEEB7ADFFFE", [4682, 4682, "Ambiguous"]],
  ["PRED-TEX-ITEM-D2FEF7768EAA05C6", [4683, 4683, "Ambiguous"]],
  ["PRED-TEX-ITEM-B7E9E9C6DA879034", [4684, 4684, "Ambiguous"]],
  ["PRED-TEX-ITEM-F41AA9DDB547A6E3", [4685, 4685, "Ambiguous"]],
  ["PRED-TEX-PROSE-45E35C7A7B219457", [4687, 4687, "Ambiguous"]]
]);
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");

function verifySources(tex, classification) {
  assert.equal(hash(tex), texDigest);
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  for (const [id, [start, end, status]] of sources) {
    const records = classification.records.filter((record) => record.source_id === id);
    assert.equal(records.length, 1, id); const record = records[0];
    const excerpt = lines.slice(start - 1, end).map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(record.disposition, "LegacyObligation", id); assert.equal(record.legacy_obligation.status, status, id);
    assert.deepEqual([record.source.start_line, record.source.end_line], [start, end], id);
    assert.equal(record.source.revision, `sha256:${texDigest}`, id); assert.equal(record.source.sha256, hash(excerpt), id);
    assert.equal(record.source_excerpt_sha256, hash(excerpt), id);
    assert.match(record.destination, new RegExp(`/Obligations/recovery-reopening-contract/${id}$`, "u"));
  }
}

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const read = (relative) => fs.readFileSync(path.join(root, relative));
  const tex = read("Inquiry_Calculus_v2_0.tex"), classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString(), doc = read("formal-successor/PHASE_B_RECOVERY_REOPENING_CONTRACT.md").toString();
  verifySources(tex, classification);
  for (const token of ["RecoveryReopeningContract", "provenance", "residualDistinction", "factorizationRoute", "recoveryRoute", "unlockTrigger", "RetainsEnough", "ProtectsEvery", "completeProtectsEvery", "missingProvenanceFails", "missingResidualFails", "missingFactorizationFails", "missingRecoveryFails", "missingUnlockFails"]) assert.match(lean, new RegExp(`\\b${token}\\b`, "u"));
  assert.match(doc, /seven exact `LegacyObligation` records at\s+v2\.0 lines 4679–4687/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(), /^import InquiryCalculus\.Legacy\.V20\.RecoveryReopeningContract\r?$/mu);
  console.log(`PASS exact recovery/reopening sources and coordinate-depletion contrasts; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-recovery-reopening-"));
  const run = (args) => cp.spawnSync("lake", args, { cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true });
  const probe = (name, body, reject = false, own = false) => { const file = path.join(directory, `${name}.lean`); fs.writeFileSync(file, (own ? "" : `import ${moduleName}\n`) + body); const result = run(["env", "lean", file]), output = result.stdout + result.stderr; if (reject) { assert.notEqual(result.status, 0, `accepted ${name}`); assert.match(output, /error(?:\([^)]*\))?:/u); } else assert.equal(result.status, 0, output); return output; };
  const build = run(["build", moduleName, "--wfail"]); assert.equal(build.status, 0, build.stdout + build.stderr);
  const audits = ["Countermodel.completeProtectsEvery", "Countermodel.missingProvenanceFails", "Countermodel.missingResidualFails", "Countermodel.missingFactorizationFails", "Countermodel.missingRecoveryFails", "Countermodel.missingUnlockFails"];
  const output = probe("contracts", audits.map((name) => `#print axioms ${moduleName}.${name}`).join("\n"));
  for (const name of audits) assert.match(output, new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.${name.replaceAll(".", "\\.")}' does not depend on any axioms`));
  const ablations = [["provenance", "  provenance : Requirement → Prop", "  provenance : True"], ["residual", "  residualDistinction : Requirement → Prop", "  residualDistinction : True"], ["factorization", "  factorizationRoute : Requirement → Prop", "  factorizationRoute : True"], ["recovery", "  recoveryRoute : Requirement → Prop", "  recoveryRoute : True"], ["unlock", "  unlockTrigger : Requirement → Prop", "  unlockTrigger : True"], ["retained-enough", "def RetainsEnough", "def RetainsEnoughRemoved"], ["protected-universal", "def ProtectsEvery", "def ProtectsEveryRemoved"]];
  for (const [name, before, after] of ablations) { const changed = lean.replace(before, after); assert.notEqual(changed, lean, name); probe(`drop-${name}`, changed, true, true); }
  console.log(`PASS seven recovery/reopening ablations and ${audits.length} axiom-free proof audits`);
}
main();
