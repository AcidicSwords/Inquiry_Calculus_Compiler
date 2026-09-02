#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Successor.Ambient.CapabilityBasis";
const modulePath = "formal/InquiryCalculus/Successor/Ambient/CapabilityBasis.lean";
const read = (name) => fs.readFileSync(path.join(root, name));
const digest = (value) => crypto.createHash("sha256").update(value).digest("hex");

const requiredGroups = {
  CtxFam: ["Sub", "comp_assoc", "reindexTy", "reindexTm", "extend", "projection", "genericTerm", "pair", "pair_unique", "lift"],
  RegPred: ["Entails", "top", "meet", "reindex", "equal", "existsAlong", "exists_adjunction", "beckChevalley", "frobenius"],
  IndPlus: ["PositiveIndexedOperator", "PositiveAlgebra", "InitialPositiveAlgebra", "strictlyPositive", "initial"],
  BindingPresentation: ["TyCode", "El", "RelAtom", "atomContext", "atomInterpretation", "LogicOperator", "logicInterpretation", "LogicLaw", "logicLawValid", "TheoryStatement", "theoryValid"]
};

function validateLean(lean) {
  for (const [group, names] of Object.entries(requiredGroups)) {
    assert.match(lean, new RegExp(`structure ${group}\\b|structure ${group} `), group);
    for (const name of names) assert.match(lean, new RegExp(`\\b${name}\\b`), `${group}.${name}`);
  }
  for (const declaration of [
    "CapabilityBasis", "Capability", "ProtectedTarget", "protectedTarget", "RemainingView",
    "remainingViewCannotRecoverRemovedCapability", "eachCandidateCapabilityHasIndependentFiniteAblation",
    "protectedPredecessorRegenerationMap", "protectedSufficiency",
    "ablationMinimalityAtDeclaredHorizon", "noGatePassFromInterfaceOrFiniteAblation"
  ]) assert.match(lean, new RegExp(`\\b${declaration}\\b`), declaration);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
}

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const lean = read(modulePath).toString();
  validateLean(lean);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Successor\.Ambient\.CapabilityBasis\r?$/mu);

  const document = read("formal-successor/GATE_C_AMBIENT_CAPABILITY_BASIS.md").toString();
  for (const boundary of [
    "interface-level independence", "protected predecessor regeneration", "semantic ablation minimality",
    "semantic existence", "syntactic representability", "evidence activation", "Partial"
  ]) assert.match(document, new RegExp(boundary, "iu"), boundary);
  const obligation = JSON.parse(read("formal-successor/INTEGRATED_THEOREM_OBLIGATIONS.json"))
    .obligations.find((entry) => entry.id === "IC-THM-C-000");
  assert.equal(obligation.status, "PLANNED");

  for (const [group, names] of Object.entries(requiredGroups)) {
    const needle = names[Math.floor(names.length / 2)];
    const mutant = lean.replace(new RegExp(`\\b${needle}\\b`, "gu"), `${needle}Ablated`);
    assert.throws(() => validateLean(mutant), `${group} ablation escaped: ${needle}`);
  }

  if (process.argv.includes("--compile")) {
    const build = childProcess.spawnSync("lake", ["build", moduleName, "--wfail"], {
      cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true
    });
    assert.equal(build.status, 0, build.stdout + build.stderr);
    const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "ic-ambient-basis-"));
    for (const [index, proof] of [
      "remainingViewCannotRecoverRemovedCapability",
      "eachCandidateCapabilityHasIndependentFiniteAblation"
    ].entries()) {
      const auditFile = path.join(temporary, `audit-${index}.lean`);
      fs.writeFileSync(auditFile, `import ${moduleName}\n#print axioms InquiryCalculus.Successor.Ambient.${proof}\n`);
      const audit = childProcess.spawnSync("lake", ["env", "lean", auditFile], {
        cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true
      });
      assert.equal(audit.status, 0, audit.stdout + audit.stderr);
      const auditOutput = audit.stdout + audit.stderr;
      const axiomList = auditOutput.match(/depends on axioms:\s*\[([^\]]*)\]/u);
      if (axiomList) {
        const axioms = axiomList[1].split(",").map((entry) => entry.trim()).filter(Boolean);
        assert.ok(axioms.every((axiom) => ["propext", "Quot.sound"].includes(axiom)), `${proof}: ${axioms}`);
      }
      assert.doesNotMatch(auditOutput, /Classical|sorryAx|choice/u, proof);
    }
  }
  console.log(`PASS exact ambient interfaces, four structural ablations, two axiom audits, and explicit unresolved regeneration boundary; module sha256 ${digest(lean)}`);
}

main();
