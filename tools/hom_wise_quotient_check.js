#!/usr/bin/env node
"use strict";

// Source ancestry is regenerated from the independently checked predecessor
// classification. Lean contracts below check formulas, not matching proof text.
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const os = require("node:os");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.HomWiseQuotient";
const modulePath = "formal/InquiryCalculus/Legacy/V20/HomWiseQuotient.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const hash = (data) => crypto.createHash("sha256").update(data).digest("hex");
const sourceContracts = [
  ["PRED-TEX-PROSE-BBCC9442AAAE4320", 4063, 4063, "LegacyObligation", "Ambiguous"],
  ["PRED-TEX-DISPLAY-2B3AD19A08A4881B", 4064, 4071, "LegacyObligation", "Ambiguous"],
  ["PRED-TEX-PROSE-403D87CD771923C0", 4073, 4073, "LegacyObligation", "Ambiguous"],
  ["PRED-TEX-DISPLAY-4A6F7E465E68AE70", 4074, 4079, "LegacyObligation", "Ambiguous"],
  ["PRED-TEX-PROSE-C359B4BAAFC28DB3", 4080, 4080, "LegacyObligation", "Ambiguous"],
  ["PRED-TEX-DECL-2DCEA31A4E4CED7D", 4082, 4097, "FormalDefinition", null],
  ["PRED-TEX-PROSE-04C6F935934FDBCF", 4099, 4099, "LegacyObligation", "Ambiguous"],
  ["PRED-TEX-DISPLAY-D96A01F40D99D46A", 4100, 4106, "LegacyObligation", "Ambiguous"],
];

function regenerateSources(tex, classification) {
  assert.equal(hash(tex), texDigest, "canonical TeX bytes changed");
  const lines = tex.toString("utf8").replace(/\r\n?/gu, "\n").split("\n");
  const records = classification.records.filter((r) => r.source.start_line >= 4063 && r.source.end_line <= 4106)
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

const opens = `open InquiryCalculus.Legacy.V20\nopen ${moduleName}\n`;
const general = `
universe u
variable {Object : Type u} {Term : Object → Object → Type u}
variable (S : ProtectedBehavioralEquivalenceContext Object Term)
variable (H : (A B : Object) → S.Context A B → Prop) {A B C : Object}
variable (compose : Term B C → Term A B → Term A C)
example (f g : Term A B) : (protectedSetoid S H A B).r f g =
    (∀ K, H A B K → S.consequence K f = S.consequence K g) := rfl
example : HomQuotient S H A B = Quotient (protectedSetoid S H A B) := rfl
example (f : Term A B) : quotientMap S H f = Quotient.mk (protectedSetoid S H A B) f := rfl
example (f g : Term A B) : (∀ K, H A B K → S.consequence K f = S.consequence K g) →
    quotientMap S H f = quotientMap S H g := quotientMapSound S H f g
example (f g : Term A B) : quotientMap S H f = quotientMap S H g →
    ∀ K, H A B K → S.consequence K f = S.consequence K g := quotientMapExact S H f g
example (f g : Term A B) : quotientMap S H f = quotientMap S H g ↔
    (∀ K, H A B K → S.consequence K f = S.consequence K g) := quotientMapEqIff S H f g
example : Congruent S H compose = (∀ g g' f f',
    (∀ K, H B C K → S.consequence K g = S.consequence K g') →
    (∀ K, H A B K → S.consequence K f = S.consequence K f') →
    ∀ K, H A C K → S.consequence K (compose g f) = S.consequence K (compose g' f')) := rfl
example (respects : Congruent S H compose) (g : Term B C) (f : Term A B) :
    descendedComposition S H compose respects (quotientMap S H g) (quotientMap S H f) =
      quotientMap S H (compose g f) := descendedCompositionOnRepresentatives S H compose respects g f
example (d : HomQuotient S H B C → HomQuotient S H A B → HomQuotient S H A C) :
    (∀ g f, d (quotientMap S H g) (quotientMap S H f) = quotientMap S H (compose g f)) →
      Congruent S H compose := descentRequiresCongruence S H compose d
example : (∃ d : HomQuotient S H B C → HomQuotient S H A B → HomQuotient S H A C,
    ∀ g f, d (quotientMap S H g) (quotientMap S H f) = quotientMap S H (compose g f)) ↔
      Congruent S H compose := descentIffCongruence S H compose
`;
const finite = `
open Countermodel Countermodel.Term3
example : observe ordinary = false := rfl
example : observe hidden = false := rfl
example : observe visible = true := rfl
example (g : Term3) : Countermodel.compose g ordinary = ordinary := rfl
example (g : Term3) : Countermodel.compose g hidden = visible := rfl
example (g : Term3) : Countermodel.compose g visible = visible := rfl
example : protectedEquivalenceDefinitionShape observation (A := ()) (B := ()) (horizon () ()) ordinary hidden := equivalentRepresentatives
example : ¬ protectedEquivalenceDefinitionShape observation (A := ()) (B := ()) (horizon () ()) ordinary visible := separatedComposites
example : ¬ Congruent observation horizon (A := ()) (B := ()) (C := ()) Countermodel.compose := notCongruent
example : ¬ ∃ d : HomQuotient observation horizon () () → HomQuotient observation horizon () () → HomQuotient observation horizon () (),
    ∀ g f, d (quotientMap observation horizon g) (quotientMap observation horizon f) =
      quotientMap observation horizon (Countermodel.compose g f) := noDescent
`;
const audits = {
  quotientMapSound: ["Quot.sound"], quotientMapExact: ["propext"],
  quotientMapEqIff: ["propext", "Quot.sound"], descendedCompositionOnRepresentatives: ["Quot.sound"],
  descentRequiresCongruence: ["propext", "Quot.sound"], descentIffCongruence: ["propext", "Quot.sound"],
  "Countermodel.equivalentRepresentatives": [], "Countermodel.separatedComposites": [],
  "Countermodel.notCongruent": [], "Countermodel.noDescent": ["propext", "Quot.sound"],
};
const negatives = [
  ["cross-hom-map", general + `example (f : Term A B) : HomQuotient S H B A := quotientMap S H f`],
  ["cross-hom-equivalence", general + `example (f : Term A B) (g : Term B A) : Prop := protectedEquivalenceDefinitionShape S (H A B) f g`],
  ["reversed-composition-endpoints", general + `example (p : Congruent S H compose) (g : HomQuotient S H B C) (f : HomQuotient S H A B) : HomQuotient S H A C := descendedComposition S H compose p f g`],
  ["missing-congruence-premise", general + `example (g : HomQuotient S H B C) (f : HomQuotient S H A B) : HomQuotient S H A C := descendedComposition S H compose g f`],
  ["wrong-horizon", general + `example (J : (A B : Object) → S.Context A B → Prop) (f g : Term A B) : quotientMap S H f = quotientMap S H g → protectedEquivalenceDefinitionShape S (J A B) f g := quotientMapExact S H f g`],
  ["false-congruence", finite + `example : Congruent observation horizon (A := ()) (B := ()) (C := ()) Countermodel.compose := by intro _ _ _ _ _ _ _ _; rfl`],
  ["separated-classes-collapsed", finite + `example : quotientMap observation horizon (A := ()) (B := ()) ordinary = quotientMap observation horizon visible := quotientMapSound observation horizon ordinary visible equivalentRepresentatives`],
];

function finiteContrast() {
  const states = [0, 1, 2], observe = (x) => x === 2, compose = (_g, f) => f === 0 ? 0 : 2;
  const countBreakers = (obs, op) => {
    let count = 0;
    for (const g of states) for (const gp of states) for (const f of states) for (const fp of states) {
      if (obs(g) === obs(gp) && obs(f) === obs(fp) && obs(op(g, f)) !== obs(op(gp, fp))) count++;
    }
    return count;
  };
  assert.equal(countBreakers(observe, compose), 10);
  assert.equal(countBreakers(() => false, compose), 0, "removing protected observation erases the breaker");
  assert.equal(countBreakers(observe, (_g, f) => observe(f) ? 2 : 0), 0, "factoring through observations restores congruence");
  for (const g of states) for (const f of states) assert.ok(states.includes(compose(g, f)), "contracted carrier must be closed");
}

function main() {
  assert.ok(process.argv.slice(2).every((arg) => arg === "--compile"), "expected optional --compile");
  const read = (p) => fs.readFileSync(path.join(root, p));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const sources = regenerateSources(tex, classification);
  const doc = read("formal-successor/PHASE_B_HOM_WISE_QUOTIENT.md").toString("utf8");
  for (const [id, start, end, disposition, status] of sourceContracts) {
    assert.ok(doc.includes(`| ${id} | ${start}–${end} | ${status ?? disposition} |`), "source-to-document mapping changed");
  }
  let mutations = 0;
  for (const mutate of [
    (c) => { c.records = c.records.filter((r) => r.source_id !== sourceContracts[0][0]); },
    (c) => { c.records.push(structuredClone(c.records.find((r) => r.source_id === sourceContracts[0][0]))); },
    (c) => { c.records.find((r) => r.source_id === sourceContracts[1][0]).legacy_obligation.status = "Proved"; },
    (c) => { c.records.find((r) => r.source_id === sourceContracts[5][0]).disposition = "Theorem"; },
    (c) => { c.records.find((r) => r.source_id === sourceContracts[1][0]).source.end_line++; },
    (c) => { c.records.find((r) => r.source_id === sourceContracts[1][0]).source.revision = "unpinned"; },
    (c) => { c.records.find((r) => r.source_id === sourceContracts[1][0]).source_excerpt_sha256 = "0".repeat(64); },
  ]) {
    const c = structuredClone(classification); mutate(c);
    assert.throws(() => regenerateSources(tex, c)); mutations++;
  }
  assert.throws(() => regenerateSources(Buffer.concat([tex, Buffer.from("\n")]), classification)); mutations++;
  finiteContrast();
  const source = read(modulePath).toString("utf8");
  assert.doesNotMatch(source, /\b(?:sorry|admit|axiom|unsafe|native_decide)\b/u);
  assert.deepEqual([...source.matchAll(/^theorem (\w+)/gmu)].map((m) => m[1]).sort(),
    Object.keys(audits).map((n) => n.split(".").at(-1)).sort(), "every theorem must be audited");
  assert.match(read("formal/InquiryCalculus.lean").toString("utf8"), /^import InquiryCalculus\.Legacy\.V20\.HomWiseQuotient\r?$/mu);
  console.log(`Source regeneration: ${sources.length} exact identities; ${mutations} rejected source mutations; canonical sha256 ${hash(tex)}; module sha256 ${hash(source)}`);
  if (!process.argv.includes("--compile")) {
    console.log("PASS source/data checks; Lean statement, mutation and dependency probes NOT RUN (use --compile)"); return;
  }
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "ic-quotient-check-"));
  const run = (args) => cp.spawnSync("lake", args, { cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true });
  const build = run(["build", moduleName, "--wfail"]);
  assert.equal(build.status, 0, build.stdout + build.stderr);
  function probe(name, body, reject = false, ownSource = false) {
    const file = path.join(dir, `${name}.lean`);
    fs.writeFileSync(file, (ownSource ? "" : `import ${moduleName}\n${opens}`) + body);
    const result = run(["env", "lean", file]), output = result.stdout + result.stderr;
    assert.equal(result.error, undefined, `could not execute Lean: ${result.error}`);
    if (reject) {
      assert.notEqual(result.status, 0, `invalid contract accepted: ${name}`);
      assert.match(output, /error:/u);
      assert.doesNotMatch(output, /unknown module|object file.*does not exist|failed to open/u, "environment failure is not a type rejection");
    } else assert.equal(result.status, 0, output);
    return output;
  }
  const audit = probe("independent-contracts", general + finite + Object.keys(audits).map((n) => `\n#print axioms ${moduleName}.${n}`).join(""));
  for (const [name, expected] of Object.entries(audits)) {
    const line = audit.split(/\r?\n/u).find((s) => s.startsWith(`'${moduleName}.${name}' `));
    assert.ok(line, `missing audit: ${name}`);
    const actual = line.includes("does not depend on any axioms") ? [] : line.match(/\[([^\]]*)\]/u)?.[1].split(",").map((s) => s.trim());
    assert.deepEqual(actual?.sort(), [...expected].sort(), line);
  }
  for (const [name, body] of negatives) probe(name, body, true);
  const sourceMutations = [
    ["erase-horizon", (s) => s.replace("r := protectedEquivalenceDefinitionShape S (H A B)", "r := fun _ _ => True")],
    ["erase-congruence", (s) => s.replace("(respects : Congruent S H compose)", "(respects : True)")],
    ["reverse-output-hom", (s) => s.replaceAll("Term A C", "Term C A")],
    ["erase-composite-discrimination", (s) => s.replace("(compose g f) (compose g' f')", "(compose g f) (compose g f)")],
  ];
  for (const [name, mutate] of sourceMutations) {
    const changed = mutate(source); assert.notEqual(changed, source, `inert mutation: ${name}`);
    probe(name, changed + "\n" + opens + general + finite, true, true);
  }
  console.log(audit.trim());
  console.log(`PASS exact quotient/congruence formulas; ${negatives.length} rejected type/counterclaims; ${sourceMutations.length} rejected source ablations; ${Object.keys(audits).length} exact proof-dependency audits; no Classical.choice`);
}

if (require.main === module) {
  try { main(); } catch (e) { console.error(e.message); process.exitCode = 1; }
}
module.exports = { regenerateSources };
