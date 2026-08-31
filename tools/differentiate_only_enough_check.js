#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs"), path = require("node:path"), os = require("node:os");
const cp = require("node:child_process"), crypto = require("node:crypto");
const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.DifferentiateOnlyEnough";
const modulePath = "formal/InquiryCalculus/Legacy/V20/DifferentiateOnlyEnough.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");

function regenerate(tex, classification) {
  assert.equal(hash(tex), texDigest);
  const matches = classification.records.filter((record) =>
    record.source_id === "PRED-TEX-DECL-LAW-REGENERATIVE-ECONOMY");
  assert.equal(matches.length, 1);
  const record = matches[0];
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  const excerpt = lines.slice(4208, 4216).map((line) => line.trimEnd()).join("\n").trim();
  assert.equal(record.source.start_line, 4209); assert.equal(record.source.end_line, 4216);
  assert.equal(record.disposition, "LegacyObligation");
  assert.equal(record.legacy_obligation.status, "Unproved");
  assert.equal(record.source.revision, `sha256:${texDigest}`);
  assert.equal(record.source.sha256, hash(excerpt));
  assert.equal(record.source_excerpt_sha256, hash(excerpt));
  return record;
}

const opens = `open InquiryCalculus.Legacy.V20.DifferentiateOnlyEnough\n`;
const finite = `
open Countermodel Countermodel.Distinction3
example : ContractSatisfied contract fullPresentation.active := fullContract
example : PreservingAblation contract fullPresentation (fun d => d = redundant) := removeRedundantPreserves
example : (ablate (fun d => d = redundant) fullPresentation).ancestry = fullPresentation.ancestry := removeRedundantKeepsAncestry
example : DifferentiatedOnlyEnough contract protectedLosses minimalPresentation := minimalIsDifferentiatedOnlyEnough
example : ¬ DifferentiatedOnlyEnough contract protectedLosses fullPresentation := fullIsNotYetDifferentiatedOnlyEnough
`;
const noAxiomAudits = [
  "ablationPreservesAuthoritativeAncestry", "witnessedLossBlocksSubtraction",
  "Countermodel.fullContract", "Countermodel.removeRedundantPreserves",
  "Countermodel.removeRedundantKeepsAncestry", "Countermodel.minimalContract",
  "Countermodel.economyMembershipDoesNotProveEveryDistinctionNecessary",
];
const propextAudits = [
  "Countermodel.minimalIsDifferentiatedOnlyEnough",
  "Countermodel.fullIsNotYetDifferentiatedOnlyEnough",
];

function finiteModel() {
  const distinctions = ["separates", "regenerates", "redundant"];
  const satisfies = (active) => active.has("separates") && active.has("regenerates");
  const remove = (active, distinction) => new Set([...active].filter((item) => item !== distinction));
  const full = new Set(distinctions), minimal = remove(full, "redundant");
  assert.equal(satisfies(full), true); assert.equal(satisfies(minimal), true);
  assert.equal(satisfies(remove(minimal, "separates")), false);
  assert.equal(satisfies(remove(minimal, "regenerates")), false);
  assert.equal(satisfies(remove(full, "redundant")), true);
}

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const read = (relative) => fs.readFileSync(path.join(root, relative));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const record = regenerate(tex, classification);
  const lean = read(modulePath).toString();
  const doc = read("formal-successor/PHASE_B_DIFFERENTIATE_ONLY_ENOUGH.md").toString();
  assert.match(doc, /PRED-TEX-DECL-LAW-REGENERATIVE-ECONOMY \| 4209–4216 \| LegacyObligation \/ Unproved/u);
  for (const mutate of [
    (copy) => { copy.records = copy.records.filter((item) => item.source_id !== record.source_id); },
    (copy) => { copy.records.find((item) => item.source_id === record.source_id).legacy_obligation.status = "Proved"; },
    (copy) => { copy.records.find((item) => item.source_id === record.source_id).disposition = "FormalTheorem"; },
    (copy) => { copy.records.find((item) => item.source_id === record.source_id).source.end_line++; },
  ]) {
    const copy = structuredClone(classification); mutate(copy);
    assert.throws(() => regenerate(tex, copy));
  }
  assert.throws(() => regenerate(Buffer.concat([tex, Buffer.from("\n")]), classification));
  finiteModel();
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.DifferentiateOnlyEnough\r?$/mu);
  console.log(`PASS source and finite ablation checks; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-ablation-"));
  const run = (args) => cp.spawnSync("lake", args,
    { cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true });
  function probe(name, body, reject = false, ownSource = false) {
    const file = path.join(directory, `${name}.lean`);
    fs.writeFileSync(file, (ownSource ? "" : `import ${moduleName}\n${opens}`) + body);
    const result = run(["env", "lean", file]), output = result.stdout + result.stderr;
    if (reject) {
      assert.notEqual(result.status, 0, `accepted ${name}`); assert.match(output, /error:/u);
    } else assert.equal(result.status, 0, output);
    return output;
  }
  const build = run(["build", moduleName, "--wfail"]);
  assert.equal(build.status, 0, build.stdout + build.stderr);
  const allAudits = [...noAxiomAudits, ...propextAudits];
  const output = probe("contracts", finite + allAudits.map((name) =>
    `\n#print axioms ${moduleName}.${name}`).join(""));
  for (const name of noAxiomAudits) {
    assert.match(output, new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.${name.replaceAll(".", "\\.")}' does not depend on any axioms`));
  }
  for (const name of propextAudits) {
    assert.match(output, new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.${name.replaceAll(".", "\\.")}' depends on axioms: \\[propext\\]`));
  }
  for (const [name, body] of [
    ["economy-not-necessity", finite + `example : DifferentiatedOnlyEnough contract protectedLosses fullPresentation := fullContract`],
    ["redundant-not-loss", finite + `example : ¬ ContractSatisfied contract (removeOne fullPresentation redundant).active := by exact fun _ => trivial`],
    ["ancestry-not-active", finite + `example : (removeOne fullPresentation redundant).ancestry = [] := rfl`],
    ["unknown-not-witness", finite + `example : Nonempty (WitnessedProtectedLoss contract protectedLosses minimalPresentation.active (removeOne minimalPresentation separates).active) := ⟨()⟩`],
  ]) probe(name, body, true);
  for (const [name, mutate] of [
    ["drop-ancestry-retention", (source) => source.replace("  ancestry := presentation.ancestry\n", "")],
    ["drop-positive-evidence", (source) => source.replace("evidence : loss.loses kind before after", "evidence : True")],
  ]) {
    const changed = mutate(lean); assert.notEqual(changed, lean);
    probe(name, changed + `\n${opens}${finite}`, true, true);
  }
  console.log(`PASS conditional witnessed ablation; 4 rejected counterclaims; 2 rejected ablations; ${noAxiomAudits.length} axiom-free and ${propextAudits.length} propext-only audits`);
}

try { main(); } catch (error) { console.error(error.message); process.exitCode = 1; }
