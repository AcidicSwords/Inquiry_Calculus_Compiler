#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.OpenDependencyBoundary";
const modulePath = "formal/InquiryCalculus/Legacy/V20/OpenDependencyBoundary.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-DECL-CC5C1DB6DF82D287", [4537, 4551, "FormalDefinition", null]],
  ["PRED-TEX-PROSE-B95D4E3A1BBF210D", [4553, 4553, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-DISPLAY-67BAEFF846B23C7A", [4554, 4560, "LegacyObligation", "Ambiguous"]]
]);
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");

function verifySources(tex, classification) {
  assert.equal(hash(tex), texDigest);
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  for (const [id, [start, end, disposition, status]] of sources) {
    const records = classification.records.filter((record) => record.source_id === id);
    assert.equal(records.length, 1, id);
    const record = records[0];
    const excerpt = lines.slice(start - 1, end).map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(record.disposition, disposition, id);
    if (status === null) assert.equal(record.legacy_obligation, undefined, id);
    else assert.equal(record.legacy_obligation.status, status, id);
    assert.deepEqual([record.source.start_line, record.source.end_line], [start, end], id);
    assert.equal(record.source.revision, `sha256:${texDigest}`, id);
    assert.equal(record.source.sha256, hash(excerpt), id);
    assert.equal(record.source_excerpt_sha256, hash(excerpt), id);
  }
}

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const read = (relative) => fs.readFileSync(path.join(root, relative));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const doc = read("formal-successor/PHASE_B_OPEN_DEPENDENCY_BOUNDARY.md").toString();
  verifySources(tex, classification);
  for (const token of ["DependencyContext", "required", "suppliedBy", "independentlyDischarged",
    "questionFor", "IsOpenDependency", "OpenDependencyQuestionTarget", "openAtEnvironment",
    "exactQuestion", "toQuestionTarget", "onlyUnresolvedIsOpenAtBase",
    "notSuppliedDoesNotEstablishRequirementOrOpenness", "unresolvedDependencyIsPositivelyOpen",
    "environmentSupplyClosesOnlyTheLocalBoundary", "independentDischargeClosesWithoutSupplying"]) {
    assert.match(lean, new RegExp(`\\b${token}\\b`, "u"));
  }
  assert.match(doc, /`FormalDefinition` at v2\.0 lines 4537–4551/u);
  assert.match(doc, /two adjacent\s+`Ambiguous` obligations at lines 4553–4560/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.doesNotMatch(lean, /^\s*answer\s*:/mu);
  assert.doesNotMatch(lean, /automaticAsk|executeQuestion|negativeAnswer|globalAbsence/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(), /^import InquiryCalculus\.Legacy\.V20\.OpenDependencyBoundary\r?$/mu);
  console.log(`PASS exact open-dependency sources and relative-question-target contrast; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-open-dependency-"));
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
    } else {
      assert.equal(result.status, 0, output);
    }
    return output;
  }
  const build = run(["build", moduleName, "--wfail"]);
  assert.equal(build.status, 0, build.stdout + build.stderr);
  const audits = [
    "toQuestionTarget", "questionTargetRetainsPositiveBoundary",
    "questionTargetRetainsExactRendering", "Countermodel.onlyUnresolvedIsOpenAtBase",
    "Countermodel.suppliedDependencyIsRequiredAndSupplied",
    "Countermodel.independentDischargeIsNotEnvironmentSupply",
    "Countermodel.notSuppliedDoesNotEstablishRequirementOrOpenness",
    "Countermodel.unresolvedDependencyIsPositivelyOpen",
    "Countermodel.targetPreservesExactEnvironmentCandidateAndDependency",
    "Countermodel.environmentSupplyClosesOnlyTheLocalBoundary",
    "Countermodel.independentDischargeClosesWithoutSupplying"
  ];
  const output = probe("contracts", `open ${moduleName}\n` + audits
    .map((name) => `#print axioms ${moduleName}.${name}`).join("\n"));
  for (const name of audits) {
    assert.match(output, new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.${name.replaceAll(".", "\\.")}' does not depend on any axioms`));
  }
  const ablations = [
    ["required-relation", "required : Candidate → Dependency → Prop", "required : True"],
    ["supply-relation", "suppliedBy : Environment → Dependency → Prop", "suppliedBy : True"],
    ["independent-discharge", "independentlyDischarged : Candidate → Dependency → Prop", "independentlyDischarged : True"],
    ["question-rendering", "questionFor : Environment → Candidate → Dependency → Question", "questionFor : True"],
    ["required-coordinate", "context.required candidate dependency ∧", "True ∧"],
    ["supply-coordinate", "¬ context.suppliedBy environment dependency ∧", "True ∧"],
    ["discharge-coordinate", "¬ context.independentlyDischarged candidate dependency", "True"],
    ["positive-boundary", "openAtEnvironment : IsOpenDependency context environment candidate dependency", "openAtEnvironment : True"],
    ["exact-question", "exactQuestion : question = context.questionFor environment candidate dependency", "exactQuestion : True"],
    ["question-dependency", "dependency : Dependency\n  openAtEnvironment", "dependency : Unit\n  openAtEnvironment"]
  ];
  for (const [name, before, after] of ablations) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS environment-relative open boundary; ${ablations.length} rejected source ablations; ${audits.length} axiom-free proof audits`);
}

main();
