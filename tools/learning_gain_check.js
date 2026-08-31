#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs"), path = require("node:path"), os = require("node:os");
const cp = require("node:child_process"), crypto = require("node:crypto");
const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.LearningGain";
const modulePath = "formal/InquiryCalculus/Legacy/V20/LearningGain.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");

function sourceRecord(tex, classification, id, start, end, disposition, status = null) {
  assert.equal(hash(tex), texDigest);
  const records = classification.records.filter((record) => record.source_id === id);
  assert.equal(records.length, 1);
  const record = records[0], lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  const excerpt = lines.slice(start - 1, end).map((line) => line.trimEnd()).join("\n").trim();
  assert.equal(record.source.start_line, start); assert.equal(record.source.end_line, end);
  assert.equal(record.disposition, disposition);
  if (status) assert.equal(record.legacy_obligation.status, status);
  assert.equal(record.source.revision, `sha256:${texDigest}`);
  assert.equal(record.source.sha256, hash(excerpt)); assert.equal(record.source_excerpt_sha256, hash(excerpt));
  return record;
}

const opens = `open InquiryCalculus.Legacy.V20.LearningGain\nopen Countermodel\n`;
const examples = `
example : PromotionAdmissible context overcomplete := overcompleteCrosses
example : PromotionAdmissible context minimal := contractedToOneGain
example : ¬ PromotionAdmissible context historyOnly := historyAloneRejected
example : ¬ PromotionAdmissible context equalCapacity := equalCapacityRejected
example : ¬ PromotionAdmissible context breaksProtected := breaksProtectedRejected
example : ¬ PromotionAdmissible context breaksWarrant := breaksWarrantRejected
example : ¬ PromotionAdmissible context lacksStanding := lacksStandingRejected
`;
const noAxiomAudits = ["Countermodel.overcompleteCrosses",
  "Countermodel.historyAloneRejected", "Countermodel.equalCapacityRejected"];
const propextAudits = ["Countermodel.contractedToOneGain",
  "Countermodel.breaksProtectedRejected", "Countermodel.breaksWarrantRejected",
  "Countermodel.lacksStandingRejected"];
const audits = [...noAxiomAudits, ...propextAudits];

function finiteModel() {
  const gain = (c) => c.resource <= 1 && c.gains.some(Boolean) && c.protected && c.warrant;
  const promote = (c) => gain(c) && c.standing;
  const all = { resource: 0, gains: [1, 1, 1, 1, 1, 1], protected: true, warrant: true, standing: true };
  const one = { ...all, gains: [1, 0, 0, 0, 0, 0] };
  assert.equal(promote(all), true); assert.equal(promote(one), true);
  assert.equal(promote({ ...one, gains: [0, 0, 0, 0, 0, 0] }), false);
  assert.equal(promote({ ...one, protected: false }), false);
  assert.equal(promote({ ...one, warrant: false }), false);
  assert.equal(promote({ ...one, standing: false }), false);
}

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const read = (relative) => fs.readFileSync(path.join(root, relative));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const law = sourceRecord(tex, classification, "PRED-TEX-DECL-1FD46E66A0EBC1D2", 4248, 4253, "LegacyObligation", "Unproved");
  const definition = sourceRecord(tex, classification, "PRED-TEX-DECL-42BCE49D848B36BE", 4255, 4266, "FormalDefinition");
  const lean = read(modulePath).toString(), doc = read("formal-successor/PHASE_B_LEARNING_GAIN.md").toString();
  assert.match(doc, /PRED-TEX-DECL-1FD46E66A0EBC1D2 \| 4248–4253 \| LegacyObligation \/ Unproved/u);
  assert.match(doc, /PRED-TEX-DECL-42BCE49D848B36BE \| 4255–4266 \| FormalDefinition/u);
  for (const [record, mutate] of [[law, (r) => { r.legacy_obligation.status = "Proved"; }],
    [definition, (r) => { r.disposition = "LegacyObligation"; }]]) {
    const copy = structuredClone(classification); mutate(copy.records.find((item) => item.source_id === record.source_id));
    assert.throws(() => sourceRecord(tex, copy, record.source_id, record.source.start_line,
      record.source.end_line, record.disposition, record === law ? "Unproved" : null));
  }
  assert.throws(() => sourceRecord(Buffer.concat([tex, Buffer.from("\n")]), classification,
    definition.source_id, 4255, 4266, "FormalDefinition"));
  finiteModel();
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(), /^import InquiryCalculus\.Legacy\.V20\.LearningGain\r?$/mu);
  console.log(`PASS exact learning sources and finite gain checks; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-learning-"));
  const run = (args) => cp.spawnSync("lake", args,
    { cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true });
  function probe(name, body, reject = false, ownSource = false) {
    const file = path.join(directory, `${name}.lean`);
    fs.writeFileSync(file, (ownSource ? "" : `import ${moduleName}\n${opens}`) + body);
    const result = run(["env", "lean", file]), output = result.stdout + result.stderr;
    if (reject) { assert.notEqual(result.status, 0, `accepted ${name}`); assert.match(output, /error:/u); }
    else assert.equal(result.status, 0, output);
    return output;
  }
  const build = run(["build", moduleName, "--wfail"]);
  assert.equal(build.status, 0, build.stdout + build.stderr);
  const output = probe("contracts", examples + audits.map((name) => `\n#print axioms ${moduleName}.${name}`).join(""));
  for (const name of noAxiomAudits) assert.match(output,
    new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.${name.replaceAll(".", "\\.")}' does not depend on any axioms`));
  for (const name of propextAudits) assert.match(output,
    new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.${name.replaceAll(".", "\\.")}' depends on axioms: \\[propext\\]`));
  for (const [name, claim] of [["history-is-learning", "example : PromotionAdmissible context historyOnly := by decide"],
    ["equality-is-gain", "example : PromotionAdmissible context equalCapacity := by decide"],
    ["broken-protection", "example : PromotionAdmissible context breaksProtected := by decide"],
    ["broken-warrant", "example : PromotionAdmissible context breaksWarrant := by decide"],
    ["gain-self-warrants", "example : PromotionAdmissible context lacksStanding := by decide"]]) probe(name, claim, true);
  for (const [name, mutate] of [
    ["drop-standing", (source) => source.replace("HasLearningGain context candidate ∧ context.standing candidate", "HasLearningGain context candidate")],
    ["drop-protection", (source) => source.replace("context.preservesProtectedBehavior candidate region ∧", "True ∧")],
    ["drop-warrant", (source) => source.replace("context.preservesWarrantBoundary candidate region", "True")],
  ]) {
    const changed = mutate(lean); assert.notEqual(changed, lean); probe(name, changed, true, true);
  }
  console.log(`PASS standing learning gain; 5 rejected counterclaims; 3 rejected ablations; ${audits.length} exact proof audits`);
}

main();
