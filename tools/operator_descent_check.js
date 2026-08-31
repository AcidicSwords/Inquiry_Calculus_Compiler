#!/usr/bin/env node
"use strict";

// Independently regenerate source ancestry and check the fixed-representation,
// whole-image relational descent contracts. Matching source text or proof names
// does not establish the formulas checked below.
const assert = require("node:assert/strict");
const fs = require("node:fs"), path = require("node:path"), os = require("node:os");
const cp = require("node:child_process"), crypto = require("node:crypto");
const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.OperatorDescent";
const modulePath = "formal/InquiryCalculus/Legacy/V20/OperatorDescent.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const hash = (data) => crypto.createHash("sha256").update(data).digest("hex");
const sourceContracts = [
  ["PRED-TEX-PROSE-FB082A57E7A83268", 4110, 4110, "LegacyObligation", "Unproved"],
  ["PRED-TEX-PROSE-403D87CD771923C0-02", 4112, 4112, "LegacyObligation", "Ambiguous"],
  ["PRED-TEX-DISPLAY-D85F60F3BD86A29F", 4113, 4115, "LegacyObligation", "Ambiguous"],
  ["PRED-TEX-PROSE-DA48910D06B9AE1A", 4116, 4116, "LegacyObligation", "Ambiguous"],
  ["PRED-TEX-DISPLAY-E062943987A89C30", 4117, 4119, "LegacyObligation", "Ambiguous"],
  ["PRED-TEX-PROSE-1BD1A5A82DED8417", 4120, 4120, "LegacyObligation", "Ambiguous"],
  ["PRED-TEX-DECL-60B874576BE3C6DC", 4122, 4139, "FormalDefinition", null],
  ["PRED-TEX-PROSE-51B5522AD4A8222A", 4141, 4141, "LegacyObligation", "Unproved"],
  ["PRED-TEX-DECL-0C77C2F920F7005A", 4143, 4151, "LegacyObligation", "Unproved"],
];

function regenerateSources(tex, classification) {
  assert.equal(hash(tex), texDigest, "canonical TeX bytes changed");
  const lines = tex.toString("utf8").replace(/\r\n?/gu, "\n").split("\n");
  const records = classification.records.filter((r) => r.source.start_line >= 4110 && r.source.end_line <= 4151)
    .sort((a, b) => a.source.start_line - b.source.start_line);
  assert.deepEqual(records.map((r) => r.source_id), sourceContracts.map(([id]) => id), "source coverage changed");
  return sourceContracts.map(([id, start, end, disposition, status]) => {
    const matches = classification.records.filter((r) => r.source_id === id);
    assert.equal(matches.length, 1, "source identity must be unique");
    const r = matches[0];
    assert.equal(r.source.path, "Inquiry_Calculus_v2_0.tex");
    assert.equal(r.source.revision, `sha256:${texDigest}`);
    assert.equal(r.source.start_line, start); assert.equal(r.source.end_line, end);
    assert.equal(r.disposition, disposition); assert.equal(r.legacy_obligation?.status ?? null, status);
    const excerpt = lines.slice(start - 1, end).map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(r.source.sha256, hash(excerpt)); assert.equal(r.source_excerpt_sha256, hash(excerpt));
    return { source_id: id, source: r.source, disposition, legacy_status: status, excerpt };
  });
}

const opens = `open InquiryCalculus.Legacy.V20.OperatorDescent\n`;
const general = `
universe u
variable {X S X' S' : Type u}
variable (q : X → S) (a : Relation X X') (q' : X' → S')
example (f : X → S) : functionGraph f = (fun x s => f x = s) := rfl
example (R : Relation X X') (T : Relation X' S') : serialCompose R T =
    (fun x s' => ∃ x', R x x' ∧ T x' s') := rfl
example (bar : Relation S S') : DescentSquare q a q' bar =
    RelationallyEqual (serialCompose a (functionGraph q'))
      (serialCompose (functionGraph q) bar) := rfl
example : continuationDescentDefinitionShape q a = Nonempty (ContinuationDescentWitness q a) := rfl
example : DescendsTo q a q' ↔ FiberStable q a q' := descentIffFiberStable q a q'
example (d : DescendsTo q a q') : FiberStable q a q' := descentImpliesFiberStable q a q' d
example (stable : FiberStable q a q') : DescentSquare q a q' (canonicalDescended q a q') :=
  fiberStableCanonicalSquare q a q' stable
`;
const finite = `
open Countermodel Countermodel.State2
example : currentQuotient left = currentQuotient right := presentEquivalent
example : wholeNextImage continuation protectedNext left false ∧
    ¬ wholeNextImage continuation protectedNext right false := protectedFutureSeparated
example : ¬ FiberStable currentQuotient continuation protectedNext := notFiberStable
example : ¬ DescendsTo currentQuotient continuation protectedNext := noProtectedFixedDescent
example : DescentSquare currentQuotient continuation collapsedNext collapsedDescended := collapsedSquare
example : continuationDescentDefinitionShape currentQuotient continuation := existentialDefinitionStillHolds
`;
const audits = {
  descentSquareIffPointwise: ["propext", "Quot.sound"],
  descentImpliesFiberStable: ["propext", "Quot.sound"],
  fiberStableCanonicalSquare: ["propext", "Quot.sound"],
  descentIffFiberStable: ["propext", "Quot.sound"],
  "Countermodel.presentEquivalent": [],
  "Countermodel.protectedFutureSeparated": ["propext"],
  "Countermodel.notFiberStable": ["propext"],
  "Countermodel.noProtectedFixedDescent": ["propext", "Quot.sound"],
  "Countermodel.collapsedSquare": ["propext", "Quot.sound"],
  "Countermodel.existentialDefinitionStillHolds": ["propext", "Quot.sound"],
};
const negatives = [
  ["current-output-is-not-descent", finite + `example : DescendsTo currentQuotient continuation protectedNext := ⟨collapsedDescended, collapsedSquare⟩`],
  ["existential-is-not-fixed", finite + `example : DescendsTo currentQuotient continuation protectedNext := existentialDefinitionStillHolds`],
  ["reverse-square-orientation", general + `example (bar : Relation S S') (h : DescentSquare q a q' bar) :
      RelationallyEqual (serialCompose (functionGraph q) bar) (serialCompose a (functionGraph q')) := h`],
  ["selected-left-return", finite + `example : FiberStable currentQuotient continuation protectedNext := by
      intro _ _ _ _; constructor <;> intro _ <;> exact protectedFutureSeparated.1`],
  ["drop-right-branch", finite + `example : wholeNextImage continuation protectedNext right false := protectedFutureSeparated.1`],
  ["wrong-next-representation", finite + `example : ¬ DescendsTo currentQuotient continuation collapsedNext := noProtectedFixedDescent`],
  ["function-instead-of-relation", general + `example (bar : S → S') : DescentSquare q a q' bar := by intro _ _; rfl`],
];

function finiteContrast() {
  const states = ["left", "right"], q = () => "unit", next = (x) => x === "left" ? false : true;
  const image = (x, qNext) => new Set([qNext(next(x))]);
  const subsets = [new Set(), new Set([false]), new Set([true]), new Set([false, true])];
  const collapsedSubsets = [new Set(), new Set(["unit"])];
  const same = (a, b) => a.size === b.size && [...a].every((x) => b.has(x));
  assert.equal(q(states[0]), q(states[1]), "current quotient must merge the foil states");
  assert.equal(subsets.filter((bar) => states.every((x) => same(image(x, (v) => v), bar))).length, 0,
    "no whole-image relation on the merged state may realize the fixed protected next map");
  assert.equal(collapsedSubsets.filter((bar) => states.every((x) => same(image(x, () => "unit"), bar))).length, 1,
    "collapsing the next representation should leave exactly one descended whole image");
  assert.notDeepEqual([...image("left", (v) => v)], [...image("right", (v) => v)]);
}

function main() {
  assert.ok(process.argv.slice(2).every((arg) => arg === "--compile"), "expected optional --compile");
  const read = (p) => fs.readFileSync(path.join(root, p));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const sources = regenerateSources(tex, classification);
  const doc = read("formal-successor/PHASE_B_OPERATOR_DESCENT.md").toString("utf8");
  for (const [id, start, end, disposition, status] of sourceContracts) {
    assert.ok(doc.includes(`| ${id} | ${start}–${end} | ${status ?? disposition} |`), "source-to-document mapping changed");
  }
  let mutations = 0;
  for (const mutate of [
    (c) => { c.records = c.records.filter((r) => r.source_id !== sourceContracts[0][0]); },
    (c) => { c.records.push(structuredClone(c.records.find((r) => r.source_id === sourceContracts[0][0]))); },
    (c) => { c.records.find((r) => r.source_id === sourceContracts[8][0]).legacy_obligation.status = "Proved"; },
    (c) => { c.records.find((r) => r.source_id === sourceContracts[6][0]).disposition = "FormalTheorem"; },
    (c) => { c.records.find((r) => r.source_id === sourceContracts[2][0]).source.end_line++; },
    (c) => { c.records.find((r) => r.source_id === sourceContracts[4][0]).source.revision = "unpinned"; },
    (c) => { c.records.find((r) => r.source_id === sourceContracts[6][0]).source_excerpt_sha256 = "0".repeat(64); },
    (c) => { c.records.find((r) => r.source_id === sourceContracts[7][0]).legacy_obligation.status = "Ambiguous"; },
  ]) {
    const c = structuredClone(classification); mutate(c);
    assert.throws(() => regenerateSources(tex, c)); mutations++;
  }
  assert.throws(() => regenerateSources(Buffer.concat([tex, Buffer.from("\n")]), classification)); mutations++;
  finiteContrast();
  const source = read(modulePath).toString("utf8");
  assert.doesNotMatch(source, /\b(?:sorry|admit|axiom|unsafe|native_decide)\b/u);
  assert.deepEqual([...source.matchAll(/^theorem ([\w.]+)/gmu)].map((m) => m[1]).sort(),
    Object.keys(audits).map((n) => n.split(".").at(-1)).sort(), "every theorem must be audited");
  assert.match(read("formal/InquiryCalculus.lean").toString("utf8"), /^import InquiryCalculus\.Legacy\.V20\.OperatorDescent\r?$/mu);
  console.log(`Source regeneration: ${sources.length} exact identities; ${mutations} rejected source mutations; canonical sha256 ${hash(tex)}; module sha256 ${hash(source)}`);
  if (!process.argv.includes("--compile")) {
    console.log("PASS source/data checks; Lean contracts and dependency audits NOT RUN (use --compile)"); return;
  }
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "ic-operator-descent-"));
  const run = (args) => cp.spawnSync("lake", args, { cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true });
  const build = run(["build", moduleName, "--wfail"]); assert.equal(build.status, 0, build.stdout + build.stderr);
  function probe(name, body, reject = false, ownSource = false) {
    const file = path.join(dir, `${name}.lean`); fs.writeFileSync(file, (ownSource ? "" : `import ${moduleName}\n${opens}`) + body);
    const result = run(["env", "lean", file]), output = result.stdout + result.stderr;
    assert.equal(result.error, undefined, `could not execute Lean: ${result.error}`);
    if (reject) {
      assert.notEqual(result.status, 0, `invalid contract accepted: ${name}`); assert.match(output, /error:/u);
      assert.doesNotMatch(output, /unknown module|object file.*does not exist|failed to open/u);
    } else assert.equal(result.status, 0, output);
    return output;
  }
  const audit = probe("independent-contracts", general + finite + Object.keys(audits).map((n) => `\n#print axioms ${moduleName}.${n}`).join(""));
  const normalizedAudit = audit.replace(/\r?\n(?!')\s*/gu, " ");
  for (const [name, expected] of Object.entries(audits)) {
    const line = normalizedAudit.split(/\r?\n/u).find((s) => s.startsWith(`'${moduleName}.${name}' `));
    assert.ok(line, `missing audit: ${name}`);
    const actual = line.includes("does not depend on any axioms") ? [] : line.match(/\[([^\]]*)\]/u)?.[1].split(",").map((s) => s.trim());
    assert.deepEqual(actual?.sort(), [...expected].sort(), line);
  }
  for (const [name, body] of negatives) probe(name, body, true);
  const sourceMutations = [
    ["erase-whole-image", (s) => s.replace("∃ x', a x x' ∧ q' x' = s'", "True")],
    ["erase-fiber-equality", (s) => s.replace("q x = q y → ∀ s'", "True → ∀ s'")],
    ["reverse-square", (s) => s.replace("(serialCompose a (functionGraph q'))\n    (serialCompose (functionGraph q) descended)", "(serialCompose (functionGraph q) descended)\n    (serialCompose a (functionGraph q'))")],
    ["erase-fixed-breaker", (s) => s.replace("protectedNext := by", "collapsedNext := by")],
  ];
  for (const [name, mutate] of sourceMutations) {
    const changed = mutate(source); assert.notEqual(changed, source, `inert mutation: ${name}`);
    probe(name, changed + "\n" + opens + general + finite, true, true);
  }
  console.log(audit.trim());
  console.log(`PASS whole-image operator descent; ${negatives.length} rejected type/counterclaims; ${sourceMutations.length} rejected source ablations; ${Object.keys(audits).length} exact proof-dependency audits; no Classical.choice`);
}

if (require.main === module) {
  try { main(); } catch (error) { console.error(error.message); process.exitCode = 1; }
}
module.exports = { regenerateSources };
