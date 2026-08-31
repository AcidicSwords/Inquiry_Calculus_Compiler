#!/usr/bin/env node
"use strict";

// Independent statement probes against exported Lean proofs. This does not
// duplicate their proof bodies or accept a declaration merely by its name.
const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const cp = require("node:child_process");
const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.ProtectedBehavioralEquivalenceLaws";
const modulePath = "formal/InquiryCalculus/Legacy/V20/ProtectedBehavioralEquivalenceLaws.lean";
const header = `import ${moduleName}\nopen InquiryCalculus.Legacy.V20\nopen ${moduleName}\nopen FiniteCoverage\n`;
const finiteContract = `
example : observation.consequence (A := ()) (B := ()) false false = false := rfl
example : observation.consequence (A := ()) (B := ()) false true = false := rfl
example : observation.consequence (A := ()) (B := ()) true false = false := rfl
example : observation.consequence (A := ()) (B := ()) true true = true := rfl
example : tested = [false] := rfl
example : workingNondistinctionDefinitionShape observation (A := ()) (B := ()) [false] false true := testedAgreement
example : separatorFamilyDefinitionShape observation (A := ()) (B := ()) whole false true true := protectedSeparator
example : true ∉ tested := separatorUntested
example : ∀ context, context ∈ tested → whole context := testedWithinWhole
example : ∀ context, narrow context → whole context := narrowWithinWhole
example : protectedEquivalenceDefinitionShape observation (A := ()) (B := ()) narrow false true := narrowEquivalent
example : ¬ protectedEquivalenceDefinitionShape observation (A := ()) (B := ()) whole false true := notWholeEquivalent
example : ¬ workingNondistinctionDefinitionShape observation (A := ()) (B := ()) [false, true] false true := completeTestsDistinguish
example : workingNondistinctionDefinitionShape observation (A := ()) (B := ()) [false] false true ∧
  ¬ protectedEquivalenceDefinitionShape observation (A := ()) (B := ()) whole false true := finiteCoverageGap
#print axioms ${moduleName}.FiniteCoverage.finiteCoverageGap
#print axioms ${moduleName}.FiniteCoverage.completeTestsDistinguish
example : workingNondistinctionDefinitionShape oneContextObservation (A := ()) (B := ()) [] false true := emptyTestsAgree
example : ¬ protectedEquivalenceDefinitionShape oneContextObservation (A := ()) (B := ()) (fun _ => True) false true := oneContextEmptyTestGap
`;
const generalContract = `
universe u
variable {Object : Type u} {Term : Object → Object → Type u}
variable (S : ProtectedBehavioralEquivalenceContext Object Term) {A B : Object}
variable (H H' : S.Context A B → Prop) (D : List (S.Context A B)) (f g h : Term A B)
example : ∀ K, H K → S.consequence K f = S.consequence K f := equivalenceReflexive S H f
example : (∀ K, H K → S.consequence K f = S.consequence K g) →
    ∀ K, H K → S.consequence K g = S.consequence K f := equivalenceSymmetric S H f g
example : (∀ K, H K → S.consequence K f = S.consequence K g) →
    (∀ K, H K → S.consequence K g = S.consequence K h) →
    ∀ K, H K → S.consequence K f = S.consequence K h := equivalenceTransitive S H f g h
example : (∀ K, H K → H' K) → ∀ (left right : Term A B),
    (∀ K, H' K → S.consequence K left = S.consequence K right) →
    ∀ K, H K → S.consequence K left = S.consequence K right := horizonRestriction S H H'
example (K : S.Context A B) : (H K ∧ S.consequence K f ≠ S.consequence K g) →
    ¬ (∀ J, H J → S.consequence J f = S.consequence J g) := separatorRefutesEquivalence S H f g K
example : (¬ (∀ K, H K → S.consequence K f = S.consequence K g)) ↔
    ∃ K, H K ∧ S.consequence K f ≠ S.consequence K g := separatorCharacterizationClassical S H f g
example : (∀ K, K ∈ D → H K) → (∀ K, H K → S.consequence K f = S.consequence K g) →
    ∀ K, K ∈ D → S.consequence K f = S.consequence K g := exactImpliesTested S H D f g
example : (∀ K, H K → K ∈ D) → (∀ K, K ∈ D → S.consequence K f = S.consequence K g) →
    ∀ K, H K → S.consequence K f = S.consequence K g := testedImpliesExactUnderCoverage S H D f g
example : (∀ K, K ∈ D → S.consequence K f = S.consequence K g) →
    ∀ K, (H K ∧ S.consequence K f ≠ S.consequence K g) → K ∉ D := separatorOutsideTests S H D f g
example [Subsingleton (S.Context A B)] : ∀ sample, sample ∈ D →
    (∀ K, K ∈ D → S.consequence K f = S.consequence K g) →
    ∀ K, H K → S.consequence K f = S.consequence K g := oneContextSampleComplete S H D f g
`;
const constructiveProofs = ["equivalenceReflexive", "equivalenceSymmetric", "equivalenceTransitive", "horizonRestriction",
  "separatorRefutesEquivalence", "exactImpliesTested", "testedImpliesExactUnderCoverage", "separatorOutsideTests", "oneContextSampleComplete",
  "FiniteCoverage.testedAgreement", "FiniteCoverage.testedWithinWhole", "FiniteCoverage.protectedSeparator",
  "FiniteCoverage.separatorUntested", "FiniteCoverage.notWholeEquivalent", "FiniteCoverage.narrowEquivalent", "FiniteCoverage.narrowWithinWhole",
  "FiniteCoverage.emptyTestsAgree", "FiniteCoverage.oneContextEmptyTestGap"];
const negativeContracts = [
  ["wrong-horizon-direction", "example : protectedEquivalenceDefinitionShape observation (A := ()) (B := ()) whole false true := narrowEquivalent"],
  ["tests-promoted-to-completeness", "example : workingNondistinctionDefinitionShape observation (A := ()) (B := ()) [false, true] false true := testedAgreement"],
  ["separator-claimed-tested", "example : true ∈ tested := by decide"],
  ["equal-terms-separated", "example : separatorFamilyDefinitionShape observation (A := ()) (B := ()) whole false false true := protectedSeparator"],
  ["separator-outside-horizon", "example : separatorFamilyDefinitionShape observation (A := ()) (B := ()) narrow false true true := protectedSeparator"],
  ["empty-tests-on-one-context", "example : protectedEquivalenceDefinitionShape oneContextObservation (A := ()) (B := ()) (fun _ => True) false true := by intro _ _; rfl"],
];

function main() {
  const args = process.argv.slice(2);
  if (args.some((arg) => arg !== "--compile")) throw new Error("expected optional --compile");
  const source = fs.readFileSync(path.join(root, modulePath), "utf8");
  assert.match(source, /^import InquiryCalculus\.Legacy\.V20\.ProtectedBehavioralEquivalence\r?$/mu);
  assert.doesNotMatch(source, /\b(?:sorry|admit|axiom|unsafe|native_decide)\b/u, "unchecked proof mechanism in proof boundary");
  const registered = ["FiniteCoverage.finiteCoverageGap", "FiniteCoverage.completeTestsDistinguish",
    ...constructiveProofs, "separatorCharacterizationClassical"];
  assert.deepEqual([...source.matchAll(/^theorem (\w+)/gmu)].map((match) => match[1]).sort(),
    registered.map((name) => name.split(".").at(-1)).sort(), "every theorem must have an explicit dependency audit");
  // A separate finite interpretation of the proposed contrast and its ablations.
  const contexts = [false, true];
  const con = (context, term) => context && term;
  const agrees = (horizon, left, right) => horizon.every((context) => con(context, left) === con(context, right));
  assert.deepEqual(contexts.flatMap((context) => contexts.map((term) => con(context, term))), [false, false, false, true]);
  assert.equal(agrees([false], false, true), true);
  assert.equal(agrees(contexts, false, true), false);
  assert.equal(agrees([true], false, true), false);
  assert.equal(agrees(contexts, false, false), true);
  if (!args.includes("--compile")) {
    console.log("protected-equivalence proof-boundary static/data checks passed; Lean proof probes NOT RUN (use --compile)");
    return;
  }
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-equivalence-proofs-"));
  const run = (arguments_) => cp.spawnSync("lake", arguments_, { cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true });
  const built = run(["build", moduleName, "--wfail"]);
  assert.equal(built.status, 0, `${built.stdout}${built.stderr}`);
  function probe(name, text, reject = false) {
    const file = path.join(directory, `${name}.lean`);
    fs.writeFileSync(file, header + text);
    const result = run(["env", "lean", file]);
    const output = `${result.stdout}${result.stderr}`;
    if (reject) {
      assert.notEqual(result.status, 0, `false statement accepted: ${name}`);
      assert.match(output, /error:/u, `no Lean rejection for ${name}`);
      assert.doesNotMatch(output, /unknown module|object file.*does not exist|failed to open/u, "environment failure is not a counterclaim rejection");
    } else assert.equal(result.status, 0, output);
    return output;
  }
  const audit = probe("independent-statements", finiteContract + generalContract +
    constructiveProofs.map((name) => `\n#print axioms ${moduleName}.${name}`).join("") +
    `\n#print axioms ${moduleName}.separatorCharacterizationClassical\n`);
  for (const name of ["FiniteCoverage.finiteCoverageGap", "FiniteCoverage.completeTestsDistinguish", ...constructiveProofs]) {
    assert.ok(audit.includes(`'${moduleName}.${name}' does not depend on any axioms`), audit);
  }
  const classical = audit.match(/separatorCharacterizationClassical' depends on axioms:\s*\[([^\]]+)\]/u);
  assert.ok(classical, audit);
  assert.deepEqual(classical[1].split(",").map((name) => name.trim()).sort(), ["Classical.choice", "Quot.sound", "propext"].sort(), audit);
  assert.doesNotMatch(audit, /sorryAx/u, audit);
  for (const [name, contract] of negativeContracts) probe(name, contract, true);
  console.log(audit.trim());
  console.log(`PASS independent conditional-law and finite-model contracts; ${negativeContracts.length} rejected counterclaims; classical premises confined to separator characterization`);
}

if (require.main === module) {
  try { main(); } catch (error) { console.error(error.message); process.exitCode = 1; }
}
