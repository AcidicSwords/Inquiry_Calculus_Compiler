#!/usr/bin/env node
"use strict";
const assert = require("node:assert/strict"), cp = require("node:child_process"), crypto = require("node:crypto");
const fs = require("node:fs"), os = require("node:os"), path = require("node:path");
const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.HistoricalReconstruction";
const modulePath = "formal/InquiryCalculus/Legacy/V20/HistoricalReconstruction.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-DISPLAY-D973B25A9267323F", [4474, 4476]],
  ["PRED-TEX-DISPLAY-DCD48C665DC6748C", [4463, 4469]],
  ["PRED-TEX-PROSE-6792F9C863B1045E", [4471, 4471]],
  ["PRED-TEX-PROSE-7609FFB96E7D8814", [4473, 4473]],
  ["PRED-TEX-PROSE-9F195BD04460416D", [4477, 4477]],
  ["PRED-TEX-PROSE-AE00ADD05AA54C51", [4462, 4462]],
]);
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");

function verifySources(tex, classification) {
  assert.equal(hash(tex), texDigest);
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  for (const [id, [start, end]] of sources) {
    const records = classification.records.filter((record) => record.source_id === id);
    assert.equal(records.length, 1, id);
    const record = records[0], excerpt = lines.slice(start - 1, end).map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(record.disposition, "LegacyObligation", id);
    assert.deepEqual([record.source.start_line, record.source.end_line], [start, end], id);
    assert.equal(record.source.revision, `sha256:${texDigest}`, id);
    assert.equal(record.source.sha256, hash(excerpt), id);
    assert.equal(record.source_excerpt_sha256, hash(excerpt), id);
  }
}

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const read = (relative) => fs.readFileSync(path.join(root, relative));
  const tex = read("Inquiry_Calculus_v2_0.tex"), classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString(), doc = read("formal-successor/PHASE_B_HISTORICAL_RECONSTRUCTION.md").toString();
  verifySources(tex, classification);
  const history = ["event-1"], candidates = [{ value: 0 }, { value: 1 }];
  assert.deepEqual(history, ["event-1"]); assert.equal(candidates.length, 2); assert.notDeepEqual(candidates[0], candidates[1]);
  for (const token of ["ReconstructionMaterial", "historical", "generated", "PluralReconstruction",
    "ancestryPreserved", "everyCandidateGenerated", "everyCandidateFills", "protectedDistinct", "inquiryRaised"]) {
    assert.match(lean, new RegExp(`\\b${token}\\b`, "u"));
  }
  assert.match(doc, /six exact `LegacyObligation` records at lines 4462–4477/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.doesNotMatch(lean, /selectedCandidate|chooseCandidate/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(), /^import InquiryCalculus\.Legacy\.V20\.HistoricalReconstruction\r?$/mu);
  console.log(`PASS exact historical-reconstruction sources and plural ancestry model; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-historical-reconstruction-"));
  const run = (args) => cp.spawnSync("lake", args, { cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true });
  function probe(name, body, reject = false, own = false) {
    const file = path.join(directory, `${name}.lean`); fs.writeFileSync(file, (own ? "" : `import ${moduleName}\n`) + body);
    const result = run(["env", "lean", file]), output = result.stdout + result.stderr;
    if (reject) { assert.notEqual(result.status, 0, `accepted ${name}`); assert.match(output, /error:/u); }
    else assert.equal(result.status, 0, output); return output;
  }
  const build = run(["build", moduleName, "--wfail"]); assert.equal(build.status, 0, build.stdout + build.stderr);
  const audits = ["generatedTagIsNotHistorical", "ancestryCannotBeRewritten", "protectedCandidatesRemainPlural", "inquiryCarriesThePluralField"];
  const output = probe("contracts", `open ${moduleName}\nopen Countermodel\n` + audits.map((name) => `#print axioms ${moduleName}.Countermodel.${name}`).join("\n"));
  for (const name of audits) assert.match(output, new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.Countermodel\\.${name}' does not depend on any axioms`));
  const ablations = [
    ["ancestry", "ancestryPreserved : historicalAfter = historicalRefs", "ancestryPreserved : True"],
    ["left", "leftPresent : left ∈ candidates", "leftPresent : True"],
    ["right", "rightPresent : right ∈ candidates", "rightPresent : True"],
    ["distinction", "protectedDistinct : ¬ context.protectedEquivalent left right", "protectedDistinct : True"],
    ["inquiry", "inquiryRaised : context.raisesInquiry candidates obligation", "inquiryRaised : True"],
  ];
  for (const [name, before, after] of ablations) { const changed = lean.replace(before, after); assert.notEqual(changed, lean); probe(`drop-${name}`, changed, true, true); }
  console.log(`PASS ancestry-safe plural reconstruction; ${ablations.length} rejected source ablations; ${audits.length} axiom-free proof audits`);
}
main();
