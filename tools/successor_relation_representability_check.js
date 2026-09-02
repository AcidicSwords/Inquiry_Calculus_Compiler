#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Successor.Relational.Representability";
const modulePath = "formal/InquiryCalculus/Successor/Relational/Representability.lean";
const read = (name) => fs.readFileSync(path.join(root, name), "utf8");

const required = [
  "Satisfies", "ContextualBridge", "base", "encode", "domainCtx", "codomainTy", "relationCtx",
  "domainSub", "domainSub_projection", "codomainTy_reindex", "relationSub", "Represents",
  "RegularRepresentation", "sound", "DenotationInvariant", "denote_invariant",
  "NonRepresentabilityWitness", "separates", "not_representable", "BindingExtensionRoute",
  "domainVar", "codomainVar", "relationSub_projection", "codomainVar_subst", "domainVar_subst",
  "identityFormula", "identity_ports_agree", "identity_holds_on_diagonal", "FaithfulEncoding",
  "reflect", "identityRepresentation",
  "exists_unit", "reindex_projection_pair", "exists_intro", "ExistentialReflection",
  "exists_iff_witness",
  "identityRepresentableUnderFaithfulEncoding", "existentialCarrierUnderReflection", "predecessorMediatorCoverage",
  "compositeTelescopeConstruction",
  "concreteComplementCountermodel", "converseRequiresSuppliedBoundary",
];

function validate(source) {
  for (const name of required) assert.match(source, new RegExp(`\\b${name}\\b`), name);
  assert.doesNotMatch(source, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  // Dependencies must not be imported before they exist.
  assert.doesNotMatch(source, /^import .*?(?:PartialBinding|Question|Support|Runtime|QSucc)/mu);
  // Satisfaction must stay entailment from top: a regular doctrine has no truth-value carrier.
  assert.match(source, /Entails\s+predicates\.top/u, "Satisfies must be entailment from top");
  // Representation must be a two-way equivalence, never a one-directional translation.
  assert.match(source, /R\.holds a c ↔/u, "Represents must be pointwise equivalence");

  // FaithfulEncoding must supply ONLY the reflection direction. If it also supplied the
  // assertion, identity representability would be assumed rather than characterized, and a
  // doctrine collapsing every encoded value would still pass.
  const faithful = source.match(/structure FaithfulEncoding[\s\S]*?\n(?=\/-|def |theorem |structure |inductive |namespace |end )/u);
  assert.ok(faithful, "FaithfulEncoding is not declared");
  const fields = [...faithful[0].matchAll(/^\s{2}([A-Za-z_][A-Za-z0-9_']*)\s*:/gmu)].map((m) => m[1]);
  assert.deepEqual(fields, ["reflect"],
    `FaithfulEncoding must carry only the reflection condition, found: ${fields.join(",")}`);
  // The assertion direction must be a proved theorem, not a supplied field.
  assert.match(source, /^theorem identity_holds_on_diagonal/mu,
    "the diagonal assertion must be derived, not assumed");
  assert.match(source, /predicates\.equal_refl/u,
    "the diagonal assertion must use equality introduction");

  // ExistentialReflection must supply ONLY witness extraction. Existential introduction is
  // derivable from the declared adjunction and must remain a theorem.
  const reflection = source.match(/structure ExistentialReflection[\s\S]*?\n(?=\/-|def |theorem |structure |inductive |namespace |end )/u);
  assert.ok(reflection, "ExistentialReflection is not declared");
  const reflectionFields = [...reflection[0].matchAll(/^\s{2}([A-Za-z_][A-Za-z0-9_']*)\s*:/gmu)]
    .map((m) => m[1]);
  assert.deepEqual(reflectionFields, ["witness"],
    `ExistentialReflection must carry only witness extraction, found: ${reflectionFields.join(",")}`);
  assert.match(source, /^theorem exists_intro/mu,
    "existential introduction must be derived, not assumed");
  assert.match(source, /predicates\.exists_adjunction/u,
    "existential introduction must come from the declared adjunction");
}

function main() {
  assert.ok(process.argv.slice(2).every((arg) => arg === "--compile"));
  const source = read(modulePath);
  validate(source);

  // The doctrine gained exactly one equality introduction rule, and nothing stronger.
  const basis = read("formal/InquiryCalculus/Successor/Ambient/CapabilityBasis.lean");
  assert.match(basis, /^\s+equal_refl\s*:/mu, "RegPred lacks the equality introduction rule");
  // Match declared fields only; prose naming an operator is not the operator.
  for (const forbidden of ["complement", "implies", "negation", "forallAlong", "not", "bot"]) {
    assert.doesNotMatch(basis, new RegExp(`^\\s+${forbidden}\\s*:`, "mu"),
      `RegPred gained a nonregular operator: ${forbidden}`);
  }

  // The obligation is derived, and its capability still records the open correspondence.
  const memory = JSON.parse(read("formal-successor/REGENERATIVE_SPINE.json"));
  const capability = memory.protected_predecessor_capabilities
    .find((entry) => entry.id === "typed-relational-surface");
  const correspondence = capability.remaining_correspondence
    .find((entry) => entry.id === "FORMAL-C-REGULAR-RELATION-REPRESENTABILITY");
  assert.ok(correspondence, "the derived correspondence obligation is not represented");
  assert.equal(capability.regeneration_status, "OPEN_NO_SUCCESSOR_CONSTRUCTION_OR_CORRESPONDENCE",
    "a partial representability result must not mark the capability regenerated");

  for (const name of [
    "denote_invariant", "not_representable", "codomainTy_reindex", "Represents", "separates",
  ]) {
    const mutant = source.replace(new RegExp(`\\b${name}\\b`, "gu"), `${name}Ablated`);
    assert.throws(() => validate(mutant), `${name} ablation escaped`);
  }

  if (process.argv.includes("--compile")) {
    const build = childProcess.spawnSync("lake", ["build", moduleName, "--wfail"], {
      cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true,
    });
    assert.equal(build.status, 0, build.stdout + build.stderr);
    const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "ic-relation-representability-"));
    for (const [index, proof] of [
      "Representability.domainSub_projection",
      "Representability.codomainTy_reindex",
      "Representability.denote_invariant",
      "Representability.not_representable",
      "Representability.domainVar_subst",
      "Representability.codomainVar_subst",
      "Representability.identity_ports_agree",
      "Representability.identity_holds_on_diagonal",
      "Representability.identityRepresentation",
      "Representability.exists_unit",
      "Representability.exists_intro",
      "Representability.exists_iff_witness",
      "Representability.BindingExtensionRoute.representation",
    ].entries()) {
      const auditPath = path.join(temporary, `audit-${index}.lean`);
      fs.writeFileSync(auditPath,
        `import ${moduleName}\n#print axioms InquiryCalculus.Successor.Relational.${proof}\n`);
      const audit = childProcess.spawnSync("lake", ["env", "lean", auditPath], {
        cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true,
      });
      assert.equal(audit.status, 0, audit.stdout + audit.stderr);
      const output = audit.stdout + audit.stderr;
      const list = output.match(/depends on axioms:\s*\[([^\]]*)\]/u);
      if (list) {
        const axioms = list[1].split(",").map((entry) => entry.trim()).filter(Boolean);
        assert.ok(axioms.every((axiom) => ["propext", "Quot.sound"].includes(axiom)),
          `${proof}: ${axioms}`);
      }
      assert.doesNotMatch(output, /Classical|sorryAx|choice/u, proof);
    }
    fs.rmSync(temporary, { recursive: true, force: true });
  }
  console.log("PASS contextual relation telescope, encoded assignment, two-way representability, " +
    "generic invariant obstruction, conditional identity and existential witness equivalence, " +
    "and typed binding-extension retention route (not composition closure or a concrete countermodel)");
}

main();
