#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Successor.Relational.TypedSurface";
const modulePath = "formal/InquiryCalculus/Successor/Relational/TypedSurface.lean";
const read = (name) => fs.readFileSync(path.join(root, name), "utf8");

const required = [
  "BindingForm", "typeCode", "ContextualForm", "RegularFormula", "atom", "equal", "top", "meet",
  "exists", "extension", "RegularFormula.reindex", "RegularFormula.denote",
  "RegularFormula.denote_reindex", "distinctTaggedForms", "erasureCollides",
  "typeTagErasureIsNotInjective", "TypeInterpretationBridge", "mapType_injective", "mapValue",
  "mapForm", "mapForm_preserves_type_code", "RelationRepresentabilityBoundary",
  "represents", "arbitraryRelationRegularRepresentability", "relationRepresentationSoundness", "noQuestionOrRuntimePromotion"
];

function validate(source) {
  for (const name of required) assert.match(source, new RegExp(`\\b${name}\\b`), name);
  assert.doesNotMatch(source, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.doesNotMatch(source, /^import .*?(?:PartialBinding|Question|Support|Runtime|QSucc)/mu);
}

function main() {
  assert.ok(process.argv.slice(2).every((arg) => arg === "--compile"));
  const source = read(modulePath);
  validate(source);
  const memory = JSON.parse(read("formal-successor/REGENERATIVE_SPINE.json"));
  const target = memory.protected_predecessor_capabilities.find((entry) => entry.id === "typed-relational-surface");
  assert.deepEqual(target.artifacts, [
    "formal/InquiryCalculus/Legacy/V20/Types.lean",
    "formal/InquiryCalculus/Legacy/V20/Forms.lean",
    "formal/InquiryCalculus/Legacy/V20/Relations.lean"
  ]);
  assert.equal(target.regeneration_status, "OPEN_NO_SUCCESSOR_CONSTRUCTION_OR_CORRESPONDENCE");
  for (const artifact of target.artifacts) assert.ok(fs.existsSync(path.join(root, artifact)));
  for (const phrase of [
    "syntax is not denotation", "type-tag", "binding-conditional", "arbitrary predecessor relations",
    "not yet regenerated"
  ]) assert.match(read("formal-successor/GATE_C_TYPED_RELATIONAL_SURFACE.md"), new RegExp(phrase, "iu"), phrase);

  for (const name of ["typeCode", "atom", "reindex", "denote", "mapType_injective", "represents"]) {
    const mutant = source.replace(new RegExp(`\\b${name}\\b`, "gu"), `${name}Ablated`);
    assert.throws(() => validate(mutant), `${name} ablation escaped`);
  }

  if (process.argv.includes("--compile")) {
    const build = childProcess.spawnSync("lake", ["build", moduleName, "--wfail"], {
      cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true
    });
    assert.equal(build.status, 0, build.stdout + build.stderr);
    const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "ic-typed-surface-"));
    for (const [index, proof] of [
      "RegularFormula.denote_reindex", "TypeTagFoil.distinctTaggedForms",
      "TypeTagFoil.erasureCollides", "TypeTagFoil.typeTagErasureIsNotInjective",
      "LegacyBridge.mapForm_preserves_type_code"
    ].entries()) {
      const auditPath = path.join(temporary, `audit-${index}.lean`);
      fs.writeFileSync(auditPath, `import ${moduleName}\n#print axioms InquiryCalculus.Successor.Relational.${proof}\n`);
      const audit = childProcess.spawnSync("lake", ["env", "lean", auditPath], {
        cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true
      });
      assert.equal(audit.status, 0, audit.stdout + audit.stderr);
      const output = audit.stdout + audit.stderr;
      const list = output.match(/depends on axioms:\s*\[([^\]]*)\]/u);
      if (list) {
        const axioms = list[1].split(",").map((entry) => entry.trim()).filter(Boolean);
        assert.ok(axioms.every((axiom) => ["propext", "Quot.sound"].includes(axiom)), `${proof}: ${axioms}`);
      }
      assert.doesNotMatch(output, /Classical|sorryAx|choice/u, proof);
    }
  }
  console.log("PASS successor type/form/free-regular syntax, derived denotation reindexing, type-tag foil, conditional predecessor bridge, and explicit relation-representability residual");
}

main();
