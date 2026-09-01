#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.ContinuationSufficiency";
const modulePath = "formal/InquiryCalculus/Legacy/V20/ContinuationSufficiency.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const source = ["PRED-TEX-PROSE-28CADB2E8391A931", 4664, 4664, "Ambiguous"];
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");

function verifySource(tex, classification) {
  assert.equal(hash(tex), texDigest);
  const [id, start, end, status] = source;
  const records = classification.records.filter((record) => record.source_id === id);
  assert.equal(records.length, 1, id);
  const record = records[0];
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  const excerpt = lines.slice(start - 1, end).map((line) => line.trimEnd()).join("\n").trim();
  assert.equal(record.disposition, "LegacyObligation");
  assert.equal(record.legacy_obligation.status, status);
  assert.deepEqual([record.source.start_line, record.source.end_line], [start, end]);
  assert.equal(record.source.revision, `sha256:${texDigest}`);
  assert.equal(record.source.sha256, hash(excerpt));
  assert.equal(record.source_excerpt_sha256, hash(excerpt));
  assert.match(record.destination, new RegExp(`/Obligations/continuation-sufficiency/${id}$`, "u"));
}

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const read = (relative) => fs.readFileSync(path.join(root, relative));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const doc = read("formal-successor/PHASE_B_CONTINUATION_SUFFICIENCY.md").toString();
  verifySource(tex, classification);
  for (const token of ["Continuation", "ContinuationScope", "DescendsThrough",
    "HasDescendedContinuation", "AllProtectedContinuationsDescend", "compatibleDescends",
    "compatibleScopeDescends", "splittingHasNoDescendedContinuation", "splittingScopeFails"]) {
    assert.match(lean, new RegExp(`\\b${token}\\b`, "u"));
  }
  assert.match(doc, /one exact `LegacyObligation` record at v2\.0 line\s+4664/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.ContinuationSufficiency\r?$/mu);
  console.log(`PASS exact continuation-sufficiency source and fiber-breaking contrast; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-continuation-sufficiency-"));
  const run = (args) => cp.spawnSync("lake", args, {
    cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true
  });
  function probe(name, body, reject = false, own = false) {
    const file = path.join(directory, `${name}.lean`);
    fs.writeFileSync(file, (own ? "" : `import ${moduleName}\n`) + body);
    const result = run(["env", "lean", file]);
    const output = result.stdout + result.stderr;
    if (reject) {
      assert.notEqual(result.status, 0, `accepted ${name}`);
      assert.match(output, /error(?:\([^)]*\))?:/u);
    } else assert.equal(result.status, 0, output);
    return output;
  }
  const build = run(["build", moduleName, "--wfail"]);
  assert.equal(build.status, 0, build.stdout + build.stderr);
  const audits = ["Countermodel.compatibleDescends", "Countermodel.compatibleScopeDescends",
    "Countermodel.splittingHasNoDescendedContinuation", "Countermodel.splittingScopeFails"];
  const output = probe("contracts", audits.map((name) => `#print axioms ${moduleName}.${name}`).join("\n"));
  for (const name of audits) assert.match(output, new RegExp(
    `'${moduleName.replaceAll(".", "\\.")}\\.${name.replaceAll(".", "\\.")}' does not depend on any axioms`));
  const ablations = [
    ["continuation-action", "  step : Carrier → Carrier", "  step : True"],
    ["protected-scope", "  protectedContinuation : Continuation Carrier → Prop", "  protectedContinuation : True"],
    ["descent-law", "def DescendsThrough", "def DescendsThroughRemoved"],
    ["existence-separation", "def HasDescendedContinuation", "def HasDescendedContinuationRemoved"],
    ["all-protected-law", "def AllProtectedContinuationsDescend", "def AllProtectedContinuationsDescendRemoved"],
    ["fiber-splitting-foil", "def splittingSource", "def splittingSourceRemoved"]
  ];
  for (const [name, before, after] of ablations) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS six continuation ablations and ${audits.length} axiom-free proof audits`);
}

main();
