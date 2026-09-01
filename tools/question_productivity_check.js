#!/usr/bin/env node
"use strict";
const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.QuestionProductivity";
const modulePath = "formal/InquiryCalculus/Legacy/V20/QuestionProductivity.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-PROSE-F39B0F158B458BD9", [4945, 4947, "Ambiguous"]],
  ["PRED-TEX-PROSE-1CB20D72DBF554D9", [4949, 4951, "Ambiguous"]],
  ["PRED-TEX-DISPLAY-ED152586A0C6D1EE", [4952, 4958, "Ambiguous"]],
  ["PRED-TEX-PROSE-7F203AB95306527E", [4960, 4961, "Unproved"]],
  ["PRED-TEX-PROSE-DD102955533EE113", [4963, 4963, "Ambiguous"]],
  ["PRED-TEX-DISPLAY-57BA7B89366BB653", [4964, 4968, "Ambiguous"]],
  ["PRED-TEX-PROSE-8B50388D03A3D164", [4969, 4969, "Ambiguous"]],
  ["PRED-TEX-DISPLAY-6A32BB388DA90BCA", [4970, 4975, "Ambiguous"]],
  ["PRED-TEX-PROSE-DEDB525027CF7954", [4977, 4977, "Ambiguous"]]
]);
const digest = value => crypto.createHash("sha256").update(value).digest("hex");
const read = relative => fs.readFileSync(path.join(root, relative));

function main() {
  assert.ok(process.argv.slice(2).every(argument => argument === "--compile"));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const document = read("formal-successor/PHASE_B_QUESTION_PRODUCTIVITY.md").toString();
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
    "ProductivityContext", "normalizedQuestion", "returnClass", "lawful", "supported", "successor",
    "sufficientCoverage", "Productive", "Resolved", "consequenceFiber", "referenceFiniteSymmetricLifting",
    "resolvedNotProductive", "sameNormalizedQuestionAndReturnClass", "sensitiveProductive",
    "neutralNotProductive", "occurrenceIndexMatters", "suppliedLiftingChangesProductivity",
    "neutralResolved", "referenceLiftingSeparatesFibers"
  ]) assert.match(lean, new RegExp(`\\b${declaration}\\b`));
  assert.match(document, /nine exact `LegacyObligation` records/u);
  assert.match(document, /eight `Ambiguous` and one `Unproved`/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(), /^import InquiryCalculus\.Legacy\.V20\.QuestionProductivity\r?$/mu);
  console.log(`PASS exact question-productivity sources and occurrence/lifting contrasts; module sha256 ${digest(lean)}`);
  if (!process.argv.includes("--compile")) return;
  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "ic-question-productivity-"));
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
    "resolvedNotProductive", "Countermodel.sameNormalizedQuestionAndReturnClass",
    "Countermodel.sensitiveProductive", "Countermodel.neutralNotProductive",
    "Countermodel.occurrenceIndexMatters", "Countermodel.suppliedLiftingChangesProductivity",
    "Countermodel.neutralResolved", "Countermodel.referenceLiftingSeparatesFibers"
  ];
  const audit = probe("axioms", proofs.map(proof => `#print axioms ${moduleName}.${proof}`).join("\n"));
  for (const proof of proofs) assert.match(audit, /does not depend on any axioms/u);
  for (const [name, before, after] of [
    ["normalized-question", "  normalizedQuestion : Occurrence → Question", "  normalizedQuestion : Occurrence → True"],
    ["return-class", "  returnClass : Occurrence → Nat", "  returnClass : Occurrence → True"],
    ["support", "  supported : Occurrence → Answer → Prop", "  supported : Occurrence → Answer → True"],
    ["successor", "  successor : Occurrence → Answer → Successor", "  successor : Occurrence → Answer → Question"],
    ["coverage", "  sufficientCoverage : Occurrence → Prop", "  sufficientCoverage : Occurrence → True"],
    ["productive", "def Productive", "def ProductiveRemoved"],
    ["resolved", "def Resolved", "def ResolvedRemoved"],
    ["lifting", "def referenceFiniteSymmetricLifting", "def referenceFiniteSymmetricLiftingRemoved"]
  ]) {
    const altered = lean.replace(before, after);
    assert.notEqual(altered, lean, name);
    probe(`drop-${name}`, altered, true, true);
  }
  console.log(`PASS eight question-productivity ablations and ${proofs.length} axiom-free proof audits`);
}

main();
