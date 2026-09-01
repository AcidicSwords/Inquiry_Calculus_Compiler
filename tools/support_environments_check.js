#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.SupportEnvironments";
const modulePath = "formal/InquiryCalculus/Legacy/V20/SupportEnvironments.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-DECL-2AEFA5353255762F", [4516, 4518]],
  ["PRED-TEX-DECL-86620E105C0A1E88", [4520, 4533]]
]);
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");

function verifySources(tex, classification) {
  assert.equal(hash(tex), texDigest);
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  for (const [id, [start, end]] of sources) {
    const records = classification.records.filter((record) => record.source_id === id);
    assert.equal(records.length, 1, id);
    const record = records[0];
    const excerpt = lines.slice(start - 1, end).map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(record.disposition, "FormalDefinition", id);
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
  const doc = read("formal-successor/PHASE_B_SUPPORT_ENVIRONMENTS.md").toString();
  verifySources(tex, classification);
  for (const token of ["TypedSupportAtom", "premise", "actualReturn", "checkerResult", "assumption",
    "standingRelation", "SupportContent", "finite", "represented", "CandidateSupportEnvironment",
    "SupportContext", "contains", "supports", "warranted", "standing", "EnvironmentSubset",
    "ProperEnvironmentSubset", "IsMinimalSupport", "MinimalSupportFamily",
    "twoIncomparableMinimalEnvironments", "unionIsSupportedButNotMinimal",
    "oneElementAblationsBreakSupport", "candidacyDoesNotEstablishSupport",
    "supportDoesNotEstablishWarrantOrStanding"]) {
    assert.match(lean, new RegExp(`\\b${token}\\b`, "u"));
  }
  assert.match(doc, /two `FormalDefinition` records at v2\.0 lines 4516–4533/u);
  assert.match(doc, /support is an explicit relation/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.doesNotMatch(lean, /selectedMinimum|uniqueMinimum|supportImpliesStanding|membershipSupports/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(), /^import InquiryCalculus\.Legacy\.V20\.SupportEnvironments\r?$/mu);
  console.log(`PASS exact support-environment sources and plural-minimality contrast; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-support-environments-"));
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
    "finiteMembership", "environmentSubsetRetainsTarget", "minimalFamilyTargetsCandidate",
    "environmentSubsetRefl", "Countermodel.leftSupports", "Countermodel.rightSupports",
    "Countermodel.unionSupports", "Countermodel.leftSubsetUnion", "Countermodel.rightSubsetUnion",
    "Countermodel.notRightSubsetLeft", "Countermodel.notLeftSubsetRight",
    "Countermodel.notUnionSubsetLeft", "Countermodel.notUnionSubsetRight",
    "Countermodel.leftMinimal", "Countermodel.rightMinimal",
    "Countermodel.twoIncomparableMinimalEnvironments",
    "Countermodel.unionIsSupportedButNotMinimal",
    "Countermodel.oneElementAblationsBreakSupport",
    "Countermodel.candidacyDoesNotEstablishSupport",
    "Countermodel.supportDoesNotEstablishWarrantOrStanding"
  ];
  const output = probe("contracts", `open ${moduleName}\n` + audits
    .map((name) => `#print axioms ${moduleName}.${name}`).join("\n"));
  for (const name of audits) {
    assert.match(output, new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.${name.replaceAll(".", "\\.")}' does not depend on any axioms`));
  }
  const ablations = [
    ["actual-return-role", "| actualReturn : ActualReturn →", "| returned : ActualReturn →"],
    ["represented-content", "| represented : FiniteRepresentation →", "| encoded : FiniteRepresentation →"],
    ["environment-target", "target : Candidate", "target : Unit"],
    ["environment-content", "content : SupportContent Atom FiniteRepresentation", "content : True"],
    ["contains-relation", "contains : SupportContent Atom FiniteRepresentation → Atom → Prop", "contains : True"],
    ["support-relation", "supports : CandidateSupportEnvironment Candidate Atom FiniteRepresentation → Prop", "supports : True"],
    ["warrant-relation", "warranted : Candidate → Prop", "warranted : True"],
    ["standing-relation", "standing : Candidate → Prop", "standing : True"],
    ["subset-target", "left.target = right.target ∧", "True ∧"],
    ["proper-subset", "EnvironmentSubset context left right ∧ ¬ EnvironmentSubset context right left", "EnvironmentSubset context left right ∧ True"],
    ["minimal-support", "context.supports environment ∧\n    ∀ smaller", "True ∧\n    ∀ smaller"],
    ["family-target", "environment.target = candidate ∧ IsMinimalSupport", "True ∧ IsMinimalSupport"]
  ];
  for (const [name, before, after] of ablations) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS typed plural support family; ${ablations.length} rejected source ablations; ${audits.length} axiom-free proof audits`);
}

main();
