#!/usr/bin/env node
"use strict";

// Independent, deliberately bounded source-shape checker. It does not import
// the generator or infer mathematical equivalence between arbitrary Lean texts.
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const crypto = require("node:crypto");
const cp = require("node:child_process");
const root = path.resolve(__dirname, "..");
const hash = (bytes) => crypto.createHash("sha256").update(bytes).digest("hex");
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sourceContracts = [
  ["PRED-TEX-PROSE-60E139ED25B0C85E", 4004, 4004, "LegacyObligation", "Ambiguous"],
  ["PRED-TEX-DECL-CA927522E6D4370C", 4006, 4016, "FormalDefinition", null],
  ["PRED-TEX-DECL-34FF1494752E4183", 4018, 4035, "FormalDefinition", null],
  ["PRED-TEX-DECL-E862C58481AED1AA", 4037, 4051, "LegacyObligation", "Unproved"],
  ["PRED-TEX-DECL-2FF1BAC4CB0C0B75", 4053, 4059, "FormalDefinition", null],
];
const obligations = [
  "protectedEquivalenceDefinitionShape", "separatorFamilyDefinitionShape",
  "workingNondistinctionDefinitionShape", "horizonMonotonicityUnproved",
  "separatorCharacterizationUnproved", "completenessLicenseAbsent",
  "noContextExecution", "noConsequenceEvaluation", "noEquivalenceDecision",
  "noSeparatorFinding", "noHorizonInclusionProof", "noSemanticAuthorityPromotion",
];

// Independently transcribed contract from the five source boundaries. Every
// executable token must belong to this reviewed, non-executing definition slice.
const expectedLean = `
import InquiryCalculus.Legacy.V20.QuestionOrderDiagnostics
namespace InquiryCalculus.Legacy.V20
universe u
structure ProtectedBehavioralEquivalenceContext (Object : Type u)
    (Term : Object → Object → Type u) where
  Context : Object → Object → Type u
  Consequence : {A B : Object} → Context A B → Type u
  consequence : {A B : Object} → (K : Context A B) → Term A B → Consequence K
variable {Object : Type u} {Term : Object → Object → Type u}
def protectedEquivalenceDefinitionShape
    (S : ProtectedBehavioralEquivalenceContext Object Term) {A B : Object}
    (H : S.Context A B → Prop) (f g : Term A B) : Prop :=
  ∀ K, H K → S.consequence K f = S.consequence K g
def separatorFamilyDefinitionShape
    (S : ProtectedBehavioralEquivalenceContext Object Term) {A B : Object}
    (H : S.Context A B → Prop) (f g : Term A B) : S.Context A B → Prop :=
  fun K => H K ∧ S.consequence K f ≠ S.consequence K g
def separatorCharacterizationUnproved
    (S : ProtectedBehavioralEquivalenceContext Object Term) {A B : Object}
    (H : S.Context A B → Prop) (f g : Term A B) : Prop :=
  (¬ protectedEquivalenceDefinitionShape S H f g) ↔
    ∃ K, separatorFamilyDefinitionShape S H f g K
def workingNondistinctionDefinitionShape
    (S : ProtectedBehavioralEquivalenceContext Object Term) {A B : Object}
    (D : List (S.Context A B)) (f g : Term A B) : Prop :=
  ∀ K, K ∈ D → S.consequence K f = S.consequence K g
def horizonMonotonicityUnproved
    (S : ProtectedBehavioralEquivalenceContext Object Term) {A B : Object}
    (H H' : S.Context A B → Prop) : Prop :=
  (∀ K, H K → H' K) → ∀ f g,
    protectedEquivalenceDefinitionShape S H' f g →
      protectedEquivalenceDefinitionShape S H f g
structure ProtectedBehavioralEquivalenceSyntax (Object : Type u)
    (Term : Object → Object → Type u) where
  signature : ProtectedBehavioralEquivalenceContext Object Term
  sourceObject : Object
  targetObject : Object
  leftTerm : Term sourceObject targetObject
  rightTerm : Term sourceObject targetObject
  protectedHorizon : signature.Context sourceObject targetObject → Prop
  testedDiscriminatorSet : List (signature.Context sourceObject targetObject)
  testedWithinHorizon : ∀ K, K ∈ testedDiscriminatorSet → protectedHorizon K
inductive ProtectedBehavioralEquivalenceObligation where
${obligations.map((name) => `  | ${name}`).join("\n")}
  deriving DecidableEq, Repr
end InquiryCalculus.Legacy.V20
`;

function tokens(text) {
  // Nested block comments and line comments cannot stand in for declarations.
  let clean = "", depth = 0;
  for (let i = 0; i < text.length;) {
    if (text.startsWith("/-", i)) { depth += 1; i += 2; clean += " "; }
    else if (depth && text.startsWith("-/", i)) { depth -= 1; i += 2; }
    else if (depth) i += 1;
    else if (text.startsWith("--", i)) {
      while (i < text.length && text[i] !== "\n") i += 1;
      clean += " ";
    } else clean += text[i++];
  }
  if (depth) throw new Error("unterminated Lean comment");
  return (clean.match(/[A-Za-z_][A-Za-z_0-9']*|[0-9]+|:=|=>|[^\s]/gu) ?? []).join(" ");
}

function inspect({ surface, schemaBytes, classificationBytes, texBytes, moduleText }) {
  const schema = JSON.parse(schemaBytes), classification = JSON.parse(classificationBytes);
  const expectedIds = sourceContracts.map(([id]) => id);
  assert.deepEqual(schema.sources, expectedIds, "schema source boundary changed");
  assert.deepEqual(schema.obligations, obligations, "schema obligations changed");
  assert.deepEqual(schema.required_declarations, [...expectedLean.matchAll(/^(?:structure|def|inductive) (\w+)/gmu)].map((match) => match[1]));
  assert.equal(schema.inputs.canonical_tex_sha256, texDigest);
  assert.equal(hash(texBytes), texDigest, "canonical source bytes changed");
  assert.equal(surface.schema, 1);
  assert.equal(surface.status, "generated_phase_b_protected_behavioral_equivalence_surface_not_successor_semantics");
  assert.deepEqual(surface.generated_from, {
    schema_sha256: hash(schemaBytes), tex_classification_sha256: hash(classificationBytes),
    canonical_tex_sha256: hash(texBytes), module_sha256: hash(moduleText),
  }, "surface input ancestry is detached");
  assert.deepEqual(surface.records.map((record) => record.source_id), expectedIds, "source coverage changed");
  const lines = texBytes.toString("utf8").replace(/\r\n?/gu, "\n").split("\n");
  for (const [index, [id, start, end, disposition, status]] of sourceContracts.entries()) {
    const matches = classification.records.filter((record) => record.source_id === id);
    assert.equal(matches.length, 1, `classified identity must be unique: ${id}`);
    const classified = matches[0], source = classified.source;
    assert.equal(source.path, "Inquiry_Calculus_v2_0.tex");
    assert.equal(source.revision, `sha256:${texDigest}`);
    assert.equal(source.start_line, start); assert.equal(source.end_line, end);
    const excerpt = lines.slice(start - 1, end).map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(source.sha256, hash(excerpt));
    assert.equal(classified.source_excerpt_sha256, hash(excerpt));
    assert.equal(classified.disposition, disposition, `source role changed: ${id}`);
    assert.equal(classified.legacy_obligation?.status ?? null, status, `source status promoted: ${id}`);
    assert.deepEqual(surface.records[index], { source_id: id, disposition, destination: classified.destination,
      source, legacy_status: status }, `surface source ancestry changed: ${id}`);
  }
  assert.deepEqual(surface.formal_gate_b, { status: "PENDING" });
  assert.deepEqual(schema.gate_b, { status: "PENDING" });
  assert.equal(surface.next_residual, "FORMAL-B-PROTECTED-BEHAVIORAL-EQUIVALENCE-CONTINUATION");
  assert.equal(schema.next_residual, surface.next_residual);
  assert.equal(tokens(moduleText), tokens(expectedLean), "typed declaration structure differs from source contract");
}

function main() {
  const args = process.argv.slice(2);
  if (args.some((arg) => arg !== "--compile")) throw new Error("unknown argument");
  const read = (file) => fs.readFileSync(path.join(root, file));
  const base = {
    surface: JSON.parse(read("formal-successor/PHASE_B_PROTECTED_BEHAVIORAL_EQUIVALENCE_SURFACE.json")),
    schemaBytes: read("formal-successor/PHASE_B_PROTECTED_BEHAVIORAL_EQUIVALENCE_SCHEMA.json"),
    classificationBytes: read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"),
    texBytes: read("Inquiry_Calculus_v2_0.tex"),
    moduleText: read("formal/InquiryCalculus/Legacy/V20/ProtectedBehavioralEquivalence.lean").toString("utf8"),
  };
  inspect(base);
  let rejected = 0;
  function foil(name, change, expectedError) {
    const candidate = { ...base, surface: structuredClone(base.surface) };
    change(candidate);
    assert.throws(() => inspect(candidate), expectedError, `mutation escaped: ${name}`);
    rejected += 1;
  }
  function moduleFoil(name, change) {
    foil(name, (candidate) => {
      candidate.moduleText = change(candidate.moduleText);
      assert.notEqual(candidate.moduleText, base.moduleText, `inert mutation: ${name}`);
      // Recompute the digest so rejection must inspect the relation, not freshness.
      candidate.surface.generated_from.module_sha256 = hash(candidate.moduleText);
    }, /typed declaration structure|unterminated Lean comment/u);
  }
  moduleFoil("names only", () => `/- ${expectedLean} -/`);
  moduleFoil("missing horizon", (s) => s.replace("  protectedHorizon :", "  -- protectedHorizon :"));
  moduleFoil("arbitrary proposition carriers", (s) => s.replaceAll(": Prop", ": Nat"));
  moduleFoil("exists instead of every", (s) => s.replace("∀ K, H K →", "∃ K, H K →"));
  moduleFoil("unrestricted contexts", (s) => s.replace("∀ K, H K →", "∀ K,"));
  moduleFoil("equality separator", (s) => s.replace("f ≠ S.consequence", "f = S.consequence"));
  moduleFoil("separator outside horizon", (s) => s.replace("H K ∧ S.consequence", "S.consequence"));
  moduleFoil("nonparallel terms", (s) => s.replace("rightTerm : Term sourceObject targetObject", "rightTerm : Term targetObject sourceObject"));
  moduleFoil("unrestricted tested coverage", (s) => s.replace("K ∈ D →", "True →"));
  moduleFoil("nonfinite tested family", (s) => s.replaceAll("List (", "Option ("));
  moduleFoil("lost tested horizon containment", (s) => s.replace("  testedWithinHorizon :", "  -- testedWithinHorizon :"));
  moduleFoil("wrong horizon direction", (s) => s.replace("(∀ K, H K → H' K)", "(∀ K, H' K → H K)"));
  moduleFoil("proved law promotion", (s) => s.replace("def horizonMonotonicityUnproved", "theorem horizonMonotonicityUnproved"));
  moduleFoil("new decision procedure", (s) => s + "\ndef decideEquivalence : Bool := true\n");
  moduleFoil("lost completeness obligation", (s) => s.replace("  | completenessLicenseAbsent", ""));
  moduleFoil("nested comment erases declaration", (s) => `/- outer /- inner -/ ${s} -/`);
  foil("source omission", (c) => c.surface.records.pop());
  foil("source duplicate", (c) => c.surface.records[1] = c.surface.records[0]);
  foil("source range changed", (c) => c.surface.records[1].source.end_line += 1);
  foil("law promoted", (c) => c.surface.records[3].legacy_status = "Proved");
  foil("gate promoted", (c) => c.surface.formal_gate_b.status = "PASS");
  foil("source bytes detached", (c) => c.texBytes = Buffer.from(c.texBytes.toString().replace("\\forall K\\in\\mathcal H", "\\exists K\\in\\mathcal H")));
  foil("module digest detached", (c) => c.surface.generated_from.module_sha256 = "0".repeat(64));
  foil("schema deletes source too", (c) => {
    const schema = JSON.parse(c.schemaBytes); schema.sources.pop(); c.surface.records.pop();
    c.schemaBytes = Buffer.from(JSON.stringify(schema)); c.surface.generated_from.schema_sha256 = hash(c.schemaBytes);
  });
  // Nonsemantic rendering is allowed; a comment cannot create or destroy a declaration.
  const spaced = { ...base, surface: structuredClone(base.surface), moduleText: base.moduleText.replace("∀ K, H K →", "∀  /- outer /- inner -/ -/ K,\n H K →") };
  spaced.surface.generated_from.module_sha256 = hash(spaced.moduleText); inspect(spaced);
  if (args.includes("--compile")) cp.execFileSync("lake", ["env", "lean", "InquiryCalculus/Legacy/V20/ProtectedBehavioralEquivalence.lean"], { cwd: path.join(root, "formal"), stdio: "pipe", windowsHide: true });
  console.log(`protected-behavioral-equivalence checks passed (5 source boundaries; ${rejected} adversarial mutations; typed definitions, not proved laws)`);
}

if (require.main === module) {
  try { main(); } catch (error) { console.error(error.message); process.exitCode = 1; }
}
module.exports = { inspect, tokens };
