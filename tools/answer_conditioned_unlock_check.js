#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.AnswerConditionedUnlock";
const modulePath = "formal/InquiryCalculus/Legacy/V20/AnswerConditionedUnlock.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-DECL-DEF-Q-UNLOCK", { lines: [5140, 5153], disposition: "FormalDefinition" }],
  ["PRED-TEX-PROSE-2002AA04D3056BFC", {
    lines: [5155, 5159], disposition: "LegacyObligation", status: "Unproved"
  }]
]);
const read = (name) => fs.readFileSync(path.join(root, name));
const digest = (value) => crypto.createHash("sha256").update(value).digest("hex");

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const document = read("formal-successor/PHASE_B_ANSWER_CONDITIONED_UNLOCK.md").toString();
  assert.equal(digest(tex), texDigest);
  for (const [sourceId, expected] of sources) {
    const matches = classification.records.filter((record) => record.source_id === sourceId);
    assert.equal(matches.length, 1, sourceId);
    const record = matches[0];
    const excerpt = lines.slice(expected.lines[0] - 1, expected.lines[1])
      .map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(record.disposition, expected.disposition);
    if (expected.status) assert.equal(record.legacy_obligation.status, expected.status);
    assert.deepEqual([record.source.start_line, record.source.end_line], expected.lines);
    assert.equal(record.source.sha256, digest(excerpt));
    assert.equal(record.source_excerpt_sha256, digest(excerpt));
  }
  for (const declaration of [
    "Unlock", "RouteAnnotations", "RouteAuthority", "LawfulRouteExplanation",
    "eraseRouteAnnotations", "AnswerConditionedUnlockObligation", "crossingUnlocks",
    "alreadyReadyDoesNotUnlock", "stalledStepDoesNotUnlock", "wrongContractDoesNotUnlock",
    "wrongContinuationDoesNotUnlock", "unlockDoesNotResolve",
    "overlappingAnnotationsAreRepresentable", "deletingRouteLabelsCannotChangeExecution",
    "routeLabelsDoNotCreateUnlock", "completeExplanationIsLawful",
    "missingDefeatAuthorityIsNotLawful", "missingRevisionIsNotLawful",
    "missingSeedIsNotLawful", "missingSuccessorIsNotLawful"
  ]) assert.match(lean, new RegExp(`\\b${declaration}\\b`));
  assert.match(document, /not a runtime enum/iu);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.AnswerConditionedUnlock\r?$/mu);
  console.log(`PASS exact answer-conditioned-unlock sources and readiness/route contrasts; module sha256 ${digest(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "ic-answer-unlock-"));
  const run = (arguments_) => childProcess.spawnSync("lake", arguments_, {
    cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true
  });
  const probe = (name, body, reject = false, ownModule = false) => {
    const filename = path.join(temporary, `${name}.lean`);
    fs.writeFileSync(filename, `${ownModule ? "" : `import ${moduleName}\n`}${body}`);
    const result = run(["env", "lean", filename]);
    const output = result.stdout + result.stderr;
    if (reject) {
      assert.notEqual(result.status, 0, `accepted ${name}`);
      assert.match(output, /error(?:\([^)]*\))?:/u);
    } else assert.equal(result.status, 0, output);
    return output;
  };
  const build = run(["build", moduleName, "--wfail"]);
  assert.equal(build.status, 0, build.stdout + build.stderr);
  const proofs = [
    "Countermodel.crossingUnlocks", "Countermodel.alreadyReadyDoesNotUnlock",
    "Countermodel.stalledStepDoesNotUnlock", "Countermodel.wrongContractDoesNotUnlock",
    "Countermodel.wrongContinuationDoesNotUnlock", "Countermodel.unlockDoesNotResolve",
    "Countermodel.overlappingAnnotationsAreRepresentable",
    "Countermodel.deletingRouteLabelsCannotChangeExecution",
    "Countermodel.routeLabelsDoNotCreateUnlock", "Countermodel.completeExplanationIsLawful",
    "Countermodel.missingDefeatAuthorityIsNotLawful", "Countermodel.missingRevisionIsNotLawful",
    "Countermodel.missingSeedIsNotLawful", "Countermodel.missingSuccessorIsNotLawful"
  ];
  for (const [index, proof] of proofs.entries()) {
    const audit = probe(`axiom-${index}`, `#print axioms ${moduleName}.${proof}`);
    assert.match(audit, /does not depend on any axioms/u, proof);
  }
  for (const [name, before, after] of [
    ["pre-ready", "  ¬ ready state contract continuation", "  ready state contract continuation"],
    ["post-ready", "    ready (qstep state askReference answer) contract continuation", "    True"],
    ["ask-question", "(askQuestion : AskReference → Question)", "(askQuestion : True)"],
    ["readiness", "(ready : State → Contract → Continuation → Prop)", "(ready : True)"],
    ["qstep", "(qstep : (state : State) → (askReference : AskReference) →", "(qstep : True) --"],
    ["contract", "    (contract : Contract) (state : State)", "    (contract : True) (state : State)"],
    ["state", "(state : State) (askReference : AskReference)", "(state : True) (askReference : AskReference)"],
    ["ask-reference", "(askReference : AskReference)\n", "(askReference : True)\n"],
    ["answer", "    (answer : SupportedAnswer (askQuestion askReference))", "    (answer : True)"],
    ["continuation", "(continuation : Continuation) : Prop", "(continuation : True) : Prop"],
    ["resolvedness", "def resolved : State → Continuation → Prop", "def resolved : True"],
    ["narrowing", "  narrowing : Bool", "  narrowing : True"],
    ["discharge", "  discharge : Bool", "  discharge : True"],
    ["defeat", "  defeat : Bool", "  defeat : True"],
    ["reframe", "  reframe : Bool", "  reframe : True"],
    ["reorientation", "  reorientation : Bool", "  reorientation : True"],
    ["extension", "  extension : Bool", "  extension : True"],
    ["defeat-authority", "  defeatAuthorized : Bool", "  defeatAuthorized : True"],
    ["revision", "  representationOrBindingRevisionAccepted : Bool", "  representationOrBindingRevisionAccepted : True"],
    ["seed", "  canonicalSeedRetained : Bool", "  canonicalSeedRetained : True"],
    ["successor", "  successorAccepted : Bool", "  successorAccepted : True"],
    ["route-erasure", "def eraseRouteAnnotations (transitionIdentity : Nat) (_ : RouteAnnotations) : Nat :=",
      "def eraseRouteAnnotations (transitionIdentity : Nat) (_ : True) : Nat :="]
  ]) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS twenty-two answer-conditioned-unlock ablations and ${proofs.length} axiom-free proof audits`);
}

main();
