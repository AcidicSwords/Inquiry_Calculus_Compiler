#!/usr/bin/env node
"use strict";

// Independently regenerate the two source declarations and challenge the
// supplied-family, every-component, separate-revision formal boundary.
const assert = require("node:assert/strict");
const fs = require("node:fs"), path = require("node:path"), os = require("node:os");
const cp = require("node:child_process"), crypto = require("node:crypto");
const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.RegenerativeSufficiency";
const modulePath = "formal/InquiryCalculus/Legacy/V20/RegenerativeSufficiency.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const hash = (data) => crypto.createHash("sha256").update(data).digest("hex");
const sourceContracts = [
  ["PRED-TEX-DECL-6B5293AC8D6CC5AD", 4156, 4173, "FormalDefinition"],
  ["PRED-TEX-DECL-FB64B23D4F342D30", 4175, 4182, "FormalDefinition"],
];

function regenerateSources(tex, classification) {
  assert.equal(hash(tex), texDigest, "canonical TeX bytes changed");
  const lines = tex.toString("utf8").replace(/\r\n?/gu, "\n").split("\n");
  return sourceContracts.map(([id, start, end, disposition]) => {
    const matches = classification.records.filter((record) => record.source_id === id);
    assert.equal(matches.length, 1, `source identity ${id} must be unique`);
    const record = matches[0];
    assert.equal(record.source.path, "Inquiry_Calculus_v2_0.tex");
    assert.equal(record.source.revision, `sha256:${texDigest}`);
    assert.equal(record.source.start_line, start); assert.equal(record.source.end_line, end);
    assert.equal(record.disposition, disposition);
    assert.equal(record.source_role, "standalone_declaration");
    const excerpt = lines.slice(start - 1, end).map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(record.source.sha256, hash(excerpt));
    assert.equal(record.source_excerpt_sha256, hash(excerpt));
    return { id, start, end, disposition, excerpt };
  });
}

const opens = `open InquiryCalculus.Legacy.V20.RegenerativeSufficiency\n`;
const general = `
universe u v w x y
variable {Component : Type u} {Value : Component → Type v}
variable {Source : Type w} {Representation : Type x} {RevisionRole : Type y}
variable (components : ProtectedComponentFamily Component Value Source)
variable (reconstruction : TypedReconstructionFamily Component Value Representation)
variable (revision : InquiryRevisionFamily RevisionRole Representation Source)
variable (representation : Representation) (source : Source)
example : RegenerativeSufficient components reconstruction representation source =
    (∀ component, components.requiredAtHorizon component →
      ∃ reconstructed,
        reconstruction.reconstructs component representation reconstructed ∧
          components.protectedEquivalent component reconstructed
            (components.sourceComponent component source)) := rfl
example : InquiryRegenerativeSufficient components reconstruction revision representation source =
    (RegenerativeSufficient components reconstruction representation source ∧
      ∀ role, revision.requiredForRevision role →
        revision.retainsOrRegenerates role representation source) := rfl
example (h : InquiryRegenerativeSufficient components reconstruction revision representation source) :
    RegenerativeSufficient components reconstruction representation source :=
  inquiryRegenerativeImpliesRegenerative components reconstruction revision representation source h
`;
const finite = `
open Countermodel Countermodel.Component2 Countermodel.RevisionRole1
example (b : Bool) (n : Nat) :
    RecoversComponent protectedComponents observedOnlyReconstruction b ⟨b, n⟩ observed :=
  observedComponentRecovers b n
example (b : Bool) (n : Nat) :
    ¬ RegenerativeSufficient protectedComponents observedOnlyReconstruction b ⟨b, n⟩ :=
  observedOnlyNotRegenerative b n
example (source : Source2) :
    RegenerativeSufficient protectedComponents completeReconstruction source source :=
  completeIsRegenerative source
example (source : Source2) :
    ¬ InquiryRegenerativeSufficient protectedComponents completeReconstruction
      missingRevision source source :=
  completeButNotInquiryRegenerative source
`;
const audits = [
  "inquiryRegenerativeImpliesRegenerative",
  "Countermodel.observedComponentRecovers",
  "Countermodel.observedOnlyNotRegenerative",
  "Countermodel.completeIsRegenerative",
  "Countermodel.completeButNotInquiryRegenerative",
];
const negatives = [
  ["one-component-is-not-every-component", finite + `example (b : Bool) (n : Nat) :
      RegenerativeSufficient protectedComponents observedOnlyReconstruction b ⟨b, n⟩ :=
    observedComponentRecovers b n`],
  ["future-component-is-not-reconstructed", finite + `example (b : Bool) (n : Nat) :
      RecoversComponent protectedComponents observedOnlyReconstruction b ⟨b, n⟩ future := by
    exact ⟨n, trivial, rfl⟩`],
  ["regenerative-is-not-inquiry-regenerative", finite + `example (source : Source2) :
      InquiryRegenerativeSufficient protectedComponents completeReconstruction
        missingRevision source source :=
    completeIsRegenerative source`],
  ["missing-route-cannot-be-invented", finite + `example (source : Source2) :
      missingRevision.retainsOrRegenerates reopeningRoute source source := trivial`],
  ["converse-does-not-follow", general + `example
      (h : RegenerativeSufficient components reconstruction representation source) :
      InquiryRegenerativeSufficient components reconstruction revision representation source := h`],
  ["wrong-component-value-type", finite + `example (b : Bool) (n : Nat) :
      RecoversComponent protectedComponents observedOnlyReconstruction b ⟨b, n⟩ future :=
    ⟨b, rfl, rfl⟩`],
];

function finiteContrast() {
  const componentMasks = [0, 1, 2, 3];
  const recovers = (mask, component) => Boolean(mask & (component === "observed" ? 1 : 2));
  const regenerative = (mask) => ["observed", "future"].every((component) => recovers(mask, component));
  assert.equal(recovers(1, "observed"), true);
  assert.equal(recovers(1, "future"), false);
  assert.equal(regenerative(1), false, "one current component must not cover the horizon");
  assert.deepEqual(componentMasks.filter(regenerative), [3], "both required components are necessary");
  const candidates = componentMasks.flatMap((mask) => [false, true].map((revision) => ({ mask, revision })));
  const inquiryRegenerative = ({ mask, revision }) => regenerative(mask) && revision;
  assert.deepEqual(candidates.filter(inquiryRegenerative), [{ mask: 3, revision: true }],
    "component completeness and revision availability are independent coordinates");
  assert.equal(inquiryRegenerative({ mask: 3, revision: false }), false);
}

function main() {
  assert.ok(process.argv.slice(2).every((arg) => arg === "--compile"), "expected optional --compile");
  const read = (relative) => fs.readFileSync(path.join(root, relative));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const sources = regenerateSources(tex, classification);
  const doc = read("formal-successor/PHASE_B_REGENERATIVE_SUFFICIENCY.md").toString("utf8");
  for (const [id, start, end, disposition] of sourceContracts) {
    assert.ok(doc.includes(`| ${id} | ${start}–${end} | ${disposition} |`),
      `source-to-document mapping changed for ${id}`);
  }
  let sourceMutations = 0;
  for (const mutate of [
    (copy) => { copy.records = copy.records.filter((record) => record.source_id !== sourceContracts[0][0]); },
    (copy) => { copy.records.push(structuredClone(copy.records.find((record) => record.source_id === sourceContracts[1][0]))); },
    (copy) => { copy.records.find((record) => record.source_id === sourceContracts[0][0]).disposition = "FormalTheorem"; },
    (copy) => { copy.records.find((record) => record.source_id === sourceContracts[1][0]).source.start_line--; },
    (copy) => { copy.records.find((record) => record.source_id === sourceContracts[0][0]).source.revision = "unpinned"; },
    (copy) => { copy.records.find((record) => record.source_id === sourceContracts[1][0]).source_excerpt_sha256 = "0".repeat(64); },
    (copy) => { copy.records.find((record) => record.source_id === sourceContracts[0][0]).source_role = "prose_context"; },
  ]) {
    const copy = structuredClone(classification); mutate(copy);
    assert.throws(() => regenerateSources(tex, copy)); sourceMutations++;
  }
  assert.throws(() => regenerateSources(Buffer.concat([tex, Buffer.from("\n")]), classification)); sourceMutations++;
  finiteContrast();
  const source = read(modulePath).toString("utf8");
  assert.doesNotMatch(source, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.deepEqual([...source.matchAll(/^theorem ([\w.]+)/gmu)].map((match) => match[1]).sort(),
    audits.map((name) => name.split(".").at(-1)).sort(), "every theorem must be audited");
  assert.match(read("formal/InquiryCalculus.lean").toString("utf8"),
    /^import InquiryCalculus\.Legacy\.V20\.RegenerativeSufficiency\r?$/mu);
  console.log(`Source regeneration: ${sources.length} exact FormalDefinitions; ${sourceMutations} rejected source mutations; canonical sha256 ${hash(tex)}; module sha256 ${hash(source)}`);
  if (!process.argv.includes("--compile")) {
    console.log("PASS source/data and complete finite-mask checks; Lean contracts and dependency audits NOT RUN (use --compile)");
    return;
  }
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-regenerative-sufficiency-"));
  const run = (args) => cp.spawnSync("lake", args, { cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true });
  const build = run(["build", moduleName, "--wfail"]);
  assert.equal(build.status, 0, build.stdout + build.stderr);
  function probe(name, body, reject = false, ownSource = false) {
    const file = path.join(directory, `${name}.lean`);
    fs.writeFileSync(file, (ownSource ? "" : `import ${moduleName}\n${opens}`) + body);
    const result = run(["env", "lean", file]), output = result.stdout + result.stderr;
    assert.equal(result.error, undefined, `could not execute Lean: ${result.error}`);
    if (reject) {
      assert.notEqual(result.status, 0, `invalid contract accepted: ${name}`);
      assert.match(output, /error:/u);
      assert.doesNotMatch(output, /unknown module|object file.*does not exist|failed to open/u);
    } else assert.equal(result.status, 0, output);
    return output;
  }
  const audit = probe("independent-contracts", general + finite +
    audits.map((name) => `\n#print axioms ${moduleName}.${name}`).join(""));
  const normalized = audit.replace(/\r?\n(?!')\s*/gu, " ");
  for (const name of audits) {
    const line = normalized.split(/\r?\n/u).find((entry) => entry.startsWith(`'${moduleName}.${name}' `));
    assert.ok(line, `missing audit: ${name}`);
    assert.match(line, /does not depend on any axioms/u, line);
  }
  for (const [name, body] of negatives) probe(name, body, true);
  const leanAblations = [
    ["some-component-instead-of-every", (text) => text.replace("∀ component, components.requiredAtHorizon component →", "∃ component, components.requiredAtHorizon component ∧")],
    ["drop-horizon-requirement", (text) => text.replace("∀ component, components.requiredAtHorizon component →", "∀ component,")],
    ["universal-reconstruction-value", (text) => text.replace("∃ reconstructed,", "∀ reconstructed,")],
    ["collapse-revision-conjunction", (text) => text.replace("RegenerativeSufficient components reconstruction representation source ∧\n", "RegenerativeSufficient components reconstruction representation source ∨\n")],
  ];
  for (const [name, mutate] of leanAblations) {
    const changed = mutate(source); assert.notEqual(changed, source, `inert mutation: ${name}`);
    probe(name, changed + "\n" + opens + general + finite, true, true);
  }
  console.log(audit.trim());
  console.log(`PASS dependent regenerative sufficiency; ${negatives.length} rejected type/counterclaims; ${leanAblations.length} rejected Lean ablations; ${audits.length} axiom-free proof audits`);
}

if (require.main === module) {
  try { main(); } catch (error) { console.error(error.message); process.exitCode = 1; }
}
module.exports = { regenerateSources };
