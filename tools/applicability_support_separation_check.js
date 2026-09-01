#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.ApplicabilitySupportSeparation";
const modulePath = "formal/InquiryCalculus/Legacy/V20/ApplicabilitySupportSeparation.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-PROSE-4205930105721260", [4615, 4615, "Unproved"]],
  ["PRED-TEX-PROSE-7A5D12A87FC72D9C", [4617, 4617, "Ambiguous"]],
  ["PRED-TEX-DISPLAY-7D57533114DEC258", [4618, 4630, "Ambiguous"]],
  ["PRED-TEX-PROSE-E67DBAF487757D8F", [4632, 4632, "Unproved"]]
]);
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");

function verifySources(tex, classification) {
  assert.equal(hash(tex), texDigest);
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  for (const [id, [start, end, status]] of sources) {
    const records = classification.records.filter((record) => record.source_id === id);
    assert.equal(records.length, 1, id);
    const record = records[0];
    const excerpt = lines.slice(start - 1, end).map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(record.disposition, "LegacyObligation", id);
    assert.equal(record.legacy_obligation.status, status, id);
    assert.deepEqual([record.source.start_line, record.source.end_line], [start, end], id);
    assert.equal(record.source.revision, `sha256:${texDigest}`, id);
    assert.equal(record.source.sha256, hash(excerpt), id);
    assert.equal(record.source_excerpt_sha256, hash(excerpt), id);
  }
}

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const read = (relative) => fs.readFileSync(path.join(root, relative));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const doc = read("formal-successor/PHASE_B_APPLICABILITY_SUPPORT_SEPARATION.md").toString();
  verifySources(tex, classification);
  for (const token of ["RetainedRelation", "relation", "scope", "applicability",
    "supportFamily", "negativeBoundary", "warrantClass", "certificateRefs",
    "RetainedRelationContext", "MayUse", "HasEvidentialSupport", "Deactivate",
    "activeAndInactiveDifferOnlyInApplicability", "deactivationPreservesHistoricalWarrant",
    "applicabilityDoesNotEstablishSupport", "deactivationDoesNotEraseSupport",
    "supportDoesNotImplyCurrentUse"]) {
    assert.match(lean, new RegExp(`\\b${token}\\b`, "u"));
  }
  assert.match(doc, /four exact `LegacyObligation` records at v2\.0 lines 4615–4632/u);
  assert.match(doc, /Two\s+remain `Unproved` and two remain `Ambiguous`/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.doesNotMatch(lean, /applicabilityImpliesSupport|deactivationErasesWarrant|inactiveMeansUnsupported/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.ApplicabilitySupportSeparation\r?$/mu);
  console.log(`PASS exact applicability/support sources and opposed-use contrast; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-applicability-support-"));
  const run = (args) => cp.spawnSync("lake", args, {
    cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true
  });
  function probe(name, body, reject = false, own = false) {
    const file = path.join(directory, `${name}.lean`);
    fs.writeFileSync(file, (own ? "" : `import ${moduleName}\n`) + body);
    const result = run(["env", "lean", file]);
    const output = result.stdout + result.stderr;
    if (reject) {
      assert.notEqual(result.status, 0, `accepted ${name}`);
      assert.match(output, /error(?:\([^)]*\))?:/u);
    } else assert.equal(result.status, 0, output);
    return output;
  }
  const build = run(["build", moduleName, "--wfail"]);
  assert.equal(build.status, 0, build.stdout + build.stderr);
  const audits = [
    "deactivateRetainsRelation", "deactivateRetainsScope", "deactivateRetainsSupport",
    "deactivateRetainsNegativeBoundary", "deactivateRetainsWarrantClass",
    "deactivateRetainsCertificateRefs", "Countermodel.activeRecordMayBeUsed",
    "Countermodel.inactiveRecordMayNotBeUsed",
    "Countermodel.activeAndInactiveDifferOnlyInApplicability",
    "Countermodel.deactivationPreservesHistoricalWarrant",
    "Countermodel.applicabilityDoesNotEstablishSupport",
    "Countermodel.deactivationDoesNotEraseSupport",
    "Countermodel.supportDoesNotImplyCurrentUse"
  ];
  const output = probe("contracts", `open ${moduleName}\n` + audits
    .map((name) => `#print axioms ${moduleName}.${name}`).join("\n"));
  for (const name of audits) assert.match(output, new RegExp(
    `'${moduleName.replaceAll(".", "\\.")}\\.${name.replaceAll(".", "\\.")}' does not depend on any axioms`));
  const ablations = [
    ["relation-coordinate", "  relation : Relation", "  relation : True"],
    ["scope-coordinate", "  scope : Scope", "  scope : True"],
    ["applicability-coordinate", "  applicability : Applicability", "  applicability : True"],
    ["support-coordinate", "  supportFamily : Support", "  supportFamily : True"],
    ["negative-boundary-coordinate", "  negativeBoundary : NegativeBoundary", "  negativeBoundary : True"],
    ["warrant-coordinate", "  warrantClass : WarrantClass", "  warrantClass : True"],
    ["certificate-coordinate", "  certificateRefs : List CertificateRef", "  certificateRefs : True"],
    ["use-predicate", "def MayUse", "def MayUseRemoved"],
    ["support-predicate", "def HasEvidentialSupport", "def HasEvidentialSupportRemoved"],
    ["deactivation", "def Deactivate", "def DeactivateRemoved"],
    ["use-context", "  applicable : Applicability → Prop", "  applicable : True"],
    ["support-context", "  evidentiallySupported : Support → Prop", "  evidentiallySupported : True"],
    ["opposed-applicability-contrast", "def warrantedInactive", "def warrantedInactiveRemoved"],
    ["applicable-unsupported-contrast", "def applicableUnsupported", "def applicableUnsupportedRemoved"]
  ];
  for (const [name, before, after] of ablations) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS retained profile and applicability/support noncollapse; ${ablations.length} rejected source ablations; ${audits.length} axiom-free proof audits`);
}

main();
