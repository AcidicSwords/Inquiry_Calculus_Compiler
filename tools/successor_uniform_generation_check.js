#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Successor.Ambient.UniformGeneration";
const modulePath = "formal/InquiryCalculus/Successor/Ambient/UniformGeneration.lean";
const read = (name) => fs.readFileSync(path.join(root, name), "utf8");

const required = [
  "RegenerationWitness", "construction", "source", "corresponds", "covered",
  "UniformGenerationBoundary", "Extends", "extendsRefl", "extendsTrans", "Target", "Protected",
  "transportTarget", "transportProtected", "transportWitness", "transportTarget_id",
  "transportTarget_comp", "PointwiseGenerates", "UniformlyGenerates",
  "uniformGenerationIncludesPointwise", "atomizedCurrentTargetsArePointwise",
  "freshProtectedTargetBreaksUniformGeneration", "pointwiseGenerationDoesNotImplyUniformGeneration",
  "instantiateAmbientPresentations", "mapEveryProtectedPredecessorTarget",
  "constructFourSemanticAblations", "noBasisPromotionFromBoundaryAlone"
];

function validate(source) {
  for (const name of required) assert.match(source, new RegExp(`\\b${name}\\b`), name);
  assert.doesNotMatch(source, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
}

function main() {
  assert.ok(process.argv.slice(2).every((arg) => arg === "--compile"));
  const source = read(modulePath);
  validate(source);
  assert.match(read("formal/InquiryCalculus.lean"),
    /^import InquiryCalculus\.Successor\.Ambient\.UniformGeneration\r?$/mu);
  const document = read("formal-successor/GATE_C_UNIFORM_GENERATION.md");
  for (const phrase of [
    "conservative extension", "source correspondence", "coverage", "atomization",
    "PointwiseGenerates", "UniformlyGenerates", "does not complete"
  ]) assert.match(document, new RegExp(phrase, "iu"), phrase);

  for (const name of ["transportWitness", "corresponds", "covered", "freshProtectedTargetBreaksUniformGeneration"]) {
    const mutant = source.replace(new RegExp(`\\b${name}\\b`, "gu"), `${name}Ablated`);
    assert.throws(() => validate(mutant), `${name} ablation escaped`);
  }

  if (process.argv.includes("--compile")) {
    const build = childProcess.spawnSync("lake", ["build", moduleName, "--wfail"], {
      cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true
    });
    assert.equal(build.status, 0, build.stdout + build.stderr);
    const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "ic-uniform-generation-"));
    for (const [index, proof] of [
      "uniformGenerationIncludesPointwise",
      "AtomizationFoil.atomizedCurrentTargetsArePointwise",
      "AtomizationFoil.freshProtectedTargetBreaksUniformGeneration",
      "AtomizationFoil.pointwiseGenerationDoesNotImplyUniformGeneration"
    ].entries()) {
      const auditPath = path.join(temporary, `audit-${index}.lean`);
      fs.writeFileSync(auditPath, `import ${moduleName}\n#print axioms InquiryCalculus.Successor.Ambient.${proof}\n`);
      const audit = childProcess.spawnSync("lake", ["env", "lean", auditPath], {
        cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true
      });
      assert.equal(audit.status, 0, audit.stdout + audit.stderr);
      const output = audit.stdout + audit.stderr;
      const axiomList = output.match(/depends on axioms:\s*\[([^\]]*)\]/u);
      if (axiomList) {
        const axioms = axiomList[1].split(",").map((entry) => entry.trim()).filter(Boolean);
        assert.ok(axioms.every((axiom) => axiom === "propext"), `${proof}: ${axioms}`);
      }
      assert.doesNotMatch(output, /Classical|sorryAx|choice|Quot\.sound/u, proof);
    }
  }
  console.log("PASS typed UniformGeneration boundary, conservative witness transport, four structural ablations, and atomization foil proof audits");
}

main();
