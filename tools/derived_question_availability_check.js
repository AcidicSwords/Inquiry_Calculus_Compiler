#!/usr/bin/env node
"use strict";
const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.DerivedQuestionAvailability";
const modulePath = "formal/InquiryCalculus/Legacy/V20/DerivedQuestionAvailability.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-PROSE-5DA3128426877149", [4908, 4908, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-ITEM-96352E6F01948237", [4910, 4911, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-PROSE-9B142846616D8F1D", [4911, 4911, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-ITEM-31560A6E41EAF4B3", [4912, 4913, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-PROSE-6E9C44CA224E7DFF", [4913, 4913, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-ITEM-859BDD44D37F6650", [4914, 4915, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-PROSE-5DDE160E5A21FFC6", [4915, 4915, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-ITEM-9967FB7E085FF288", [4916, 4917, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-PROSE-7BBF286EBB4271FF", [4917, 4917, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-ITEM-64630939276FBFF8", [4918, 4919, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-PROSE-8EDF7CF9CE91CB59", [4919, 4919, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-ITEM-196244D8DA57E3A6", [4920, 4921, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-PROSE-2B5E7E4E694FA5A4", [4921, 4921, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-ITEM-6B84FD03AE7882AE", [4922, 4923, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-PROSE-5DE8E860D7B42D80", [4923, 4923, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-PROSE-284C26F800FABD69", [4925, 4926, "LegacyObligation", "Unproved"]],
  ["PRED-TEX-DECL-DEF-REQUIRED-DISCHARGE", [4928, 4934, "FormalDefinition", null]],
  ["PRED-TEX-DECL-LAW-PRODUCTIVE-REQUIRED-DISCHARGE", [4936, 4941, "LegacyObligation", "Unproved"]]
]);
const digest = value => crypto.createHash("sha256").update(value).digest("hex");
const read = relative => fs.readFileSync(path.join(root, relative));

function main() {
  assert.ok(process.argv.slice(2).every(argument => argument === "--compile"));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const document = read("formal-successor/PHASE_B_DERIVED_QUESTION_AVAILABILITY.md").toString();
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  assert.equal(digest(tex), texDigest);
  for (const [id, [start, end, disposition, status]] of sources) {
    const matches = classification.records.filter(record => record.source_id === id);
    assert.equal(matches.length, 1, id);
    const record = matches[0];
    const excerpt = lines.slice(start - 1, end).map(line => line.trimEnd()).join("\n").trim();
    assert.equal(record.disposition, disposition, id);
    assert.deepEqual([record.source.start_line, record.source.end_line], [start, end], id);
    assert.equal(record.source.sha256, digest(excerpt), id);
    assert.equal(record.source_excerpt_sha256, digest(excerpt), id);
    if (status !== null) assert.equal(record.legacy_obligation.status, status, id);
  }
  for (const declaration of [
    "AvailabilityProfile", "formable", "applicable", "executable", "answerable",
    "productiveAlternatives", "allLiveAnswersProtectedEquivalent", "sufficientCoverage",
    "Productive", "ResolvedQ", "Ready", "requiredDischargeAt", "resolvedRequiresCoverage",
    "formableButInapplicable", "formableButNonexecutable", "executableEmptyButNotAnswerable",
    "incompleteCoverageDoesNotResolve", "formableApplicableButNotReady", "nonproductiveRequired",
    "productiveNotRequired", "requiredDoesNotAssertExecution"
  ]) assert.match(lean, new RegExp(`\\b${declaration}\\b`));
  assert.match(document, /18 exact source records/u);
  assert.match(document, /15 `Ambiguous` `LegacyObligation` records, and two `Unproved`/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(), /^import InquiryCalculus\.Legacy\.V20\.DerivedQuestionAvailability\r?$/mu);
  console.log(`PASS exact derived-question-availability sources and non-collapsing contrasts; module sha256 ${digest(lean)}`);
  if (!process.argv.includes("--compile")) return;
  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "ic-derived-question-availability-"));
  const run = arguments_ => childProcess.spawnSync("lake", arguments_, { cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true });
  const compile = run(["build", moduleName, "--wfail"]);
  assert.equal(compile.status, 0, compile.stdout + compile.stderr);
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
    "resolvedRequiresCoverage", "Countermodel.formableButInapplicable", "Countermodel.formableButNonexecutable",
    "Countermodel.executableEmptyButNotAnswerable", "Countermodel.incompleteCoverageDoesNotResolve",
    "Countermodel.formableApplicableButNotReady", "Countermodel.nonproductiveRequired",
    "Countermodel.productiveNotRequired", "Countermodel.requiredDoesNotAssertExecution"
  ];
  const audit = probe("axioms", proofs.map(proof => `#print axioms ${moduleName}.${proof}`).join("\n"));
  for (const proof of proofs) assert.match(audit, /does not depend on any axioms/u);
  for (const [name, before, after] of [
    ["formable", "  formable : Question → Prop", "  formable : Question → True"],
    ["applicable", "  applicable : Question → Prop", "  applicable : Question → True"],
    ["executable", "  executable : Question → Prop", "  executable : Question → True"],
    ["answerable", "  answerable : Question → Prop", "  answerable : Question → True"],
    ["productive", "  productiveAlternatives : Occurrence → Prop", "  productiveAlternatives : Occurrence → True"],
    ["coverage", "  sufficientCoverage : Occurrence → Prop", "  sufficientCoverage : Occurrence → True"],
    ["ready", "def Ready", "def ReadyRemoved"],
    ["required", "def requiredDischargeAt", "def requiredDischargeAtRemoved"]
  ]) {
    const altered = lean.replace(before, after);
    assert.notEqual(altered, lean, name);
    probe(`drop-${name}`, altered, true, true);
  }
  console.log(`PASS eight derived-question-availability ablations and ${proofs.length} axiom-free proof audits`);
}

main();
