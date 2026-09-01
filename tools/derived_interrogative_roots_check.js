#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.DerivedInterrogativeRoots";
const modulePath = "formal/InquiryCalculus/Legacy/V20/DerivedInterrogativeRoots.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-PROSE-35990033BB3E13A3", [5072, 5074, "Unproved"]],
  ["PRED-TEX-PROSE-DB08B79A75116933", [5078, 5078, "Ambiguous"]],
  ["PRED-TEX-DISPLAY-8E859D138E54C871", [5079, 5085, "Ambiguous"]],
  ["PRED-TEX-PROSE-0CACB168C72FDF00", [5086, 5086, "Ambiguous"]],
  ["PRED-TEX-ITEM-3FBDF33029AE65FD", [5088, 5088, "Ambiguous"]],
  ["PRED-TEX-ITEM-98AC0CB9F4F4DD91", [5089, 5090, "Ambiguous"]],
  ["PRED-TEX-PROSE-AFBC48E97D1735B9", [5090, 5090, "Ambiguous"]],
  ["PRED-TEX-ITEM-423C69F2DB45AEED", [5091, 5092, "Ambiguous"]],
  ["PRED-TEX-PROSE-B7318309EF77D763", [5092, 5092, "Ambiguous"]],
  ["PRED-TEX-ITEM-2151C8203B4718B8", [5093, 5094, "Ambiguous"]],
  ["PRED-TEX-PROSE-81263BAA7DF041D8", [5094, 5094, "Ambiguous"]],
  ["PRED-TEX-ITEM-03F49905960F607C", [5095, 5096, "Ambiguous"]],
  ["PRED-TEX-PROSE-CBB735CA9965C558", [5096, 5096, "Ambiguous"]],
  ["PRED-TEX-ITEM-845A3D4E5ED06E3D", [5097, 5098, "Unproved"]],
  ["PRED-TEX-PROSE-0BAAE6A5345BEE07", [5098, 5098, "Unproved"]],
  ["PRED-TEX-PROSE-0AA8ABAF74A9831A", [5100, 5101, "Ambiguous"]]
]);

const read = (name) => fs.readFileSync(path.join(root, name));
const digest = (value) => crypto.createHash("sha256").update(value).digest("hex");

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const document = read("formal-successor/PHASE_B_DERIVED_INTERROGATIVE_ROOTS.md").toString();
  assert.equal(digest(tex), texDigest);
  for (const [id, [start, end, status]] of sources) {
    const matches = classification.records.filter((record) => record.source_id === id);
    assert.equal(matches.length, 1, id);
    const record = matches[0];
    const excerpt = lines.slice(start - 1, end).map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(record.disposition, "LegacyObligation", id);
    assert.equal(record.legacy_obligation.status, status, id);
    assert.deepEqual([record.source.start_line, record.source.end_line], [start, end], id);
    assert.equal(record.source.sha256, digest(excerpt), id);
    assert.equal(record.source_excerpt_sha256, digest(excerpt), id);
  }
  for (const declaration of [
    "Root", "Orientation", "CanonicalForm", "RootPresentation", "Transparent", "canonicalFor",
    "ReifiedQuestionSurface", "transparentDoesNotAuthorize", "everyRootHasCanonicalForm",
    "transparentPresentationIsTransparent", "orientationIsNotInverseSynthesis", "bareOrientIsIllShaped",
    "nonOrientCannotClaimOrientation", "exposeIsQuestionConstructor", "factorIsFactorExposure",
    "polarizeIsPositiveAlternative", "varyIsAdmissibleVariation", "groundIsSupportCheckWarrant",
    "reifiedQuestionReusesRootFamily"
  ]) assert.match(lean, new RegExp(`\\b${declaration}\\b`));
  assert.match(lean, /inductive Root where \| expose \| orient \| factor \| polarize \| vary \| ground/u);
  assert.doesNotMatch(lean, /orientForward|orientConverse/u);
  assert.match(document, /six roots/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.DerivedInterrogativeRoots\r?$/mu);
  console.log(`PASS exact derived-root sources and six-root primitive elimination; module sha256 ${digest(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "ic-derived-roots-"));
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
    "transparentDoesNotAuthorize", "Countermodel.everyRootHasCanonicalForm",
    "Countermodel.transparentPresentationIsTransparent", "Countermodel.orientationIsNotInverseSynthesis",
    "Countermodel.bareOrientIsIllShaped", "Countermodel.nonOrientCannotClaimOrientation",
    "Countermodel.exposeIsQuestionConstructor", "Countermodel.factorIsFactorExposure",
    "Countermodel.polarizeIsPositiveAlternative", "Countermodel.varyIsAdmissibleVariation",
    "Countermodel.groundIsSupportCheckWarrant", "Countermodel.reifiedQuestionReusesRootFamily"
  ];
  const audit = probe("axioms", proofs.map((proof) => `#print axioms ${moduleName}.${proof}`).join("\n"));
  for (const proof of proofs) assert.match(audit, /does not depend on any axioms/u);
  for (const [name, before, after] of [
    ["root", "  root : Root", "  root : True"],
    ["orientation", "  orientation : Option Orientation", "  orientation : True"],
    ["primitive", "  addsPrimitive : Prop", "  addsPrimitive : True"],
    ["actuality", "  addsActuality : Prop", "  addsActuality : True"],
    ["authority", "  addsAuthority : Prop", "  addsAuthority : True"],
    ["scheduler", "  schedules : Prop", "  schedules : True"],
    ["canonical", "def canonicalFor", "def canonicalForRemoved"]
  ]) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS seven derived-root ablations and ${proofs.length} axiom-free proof audits`);
}

main();
