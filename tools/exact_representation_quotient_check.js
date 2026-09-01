#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.ExactRepresentationQuotient";
const modulePath = "formal/InquiryCalculus/Legacy/V20/ExactRepresentationQuotient.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-PROSE-403D87CD771923C0-04", [4638, 4638, "Ambiguous", "exact-representation-quotient"]],
  ["PRED-TEX-DISPLAY-238FB98DB8880A2D", [4639, 4641, "Ambiguous", "exact-representation-quotient"]],
  ["PRED-TEX-PROSE-E3ED61C44816FEB4", [4642, 4642, "Ambiguous", "exact-representation-quotient"]],
  ["PRED-TEX-PROSE-8D18395B5F993E96", [4646, 4646, "Ambiguous", "consequence-sufficiency"]],
  ["PRED-TEX-DISPLAY-DA7543401DA6F7C6", [4647, 4653, "Ambiguous", "consequence-sufficiency"]],
  ["PRED-TEX-PROSE-A7EEB443679E828D", [4655, 4655, "Ambiguous", "consequence-sufficiency"]],
  ["PRED-TEX-DISPLAY-C3EA150FD3411AE3", [4656, 4660, "Ambiguous", "consequence-sufficiency"]],
  ["PRED-TEX-PROSE-28CADB2E8391A931", [4664, 4664, "Ambiguous", "continuation-sufficiency"]]
]);
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");

function verifySources(tex, classification) {
  assert.equal(hash(tex), texDigest);
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  for (const [id, [start, end, status, destination]] of sources) {
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
    assert.match(record.destination, new RegExp(`/Obligations/${destination}/${id}$`, "u"));
  }
}

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const read = (relative) => fs.readFileSync(path.join(root, relative));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const classification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
  const lean = read(modulePath).toString();
  const doc = read("formal-successor/PHASE_B_EXACT_REPRESENTATION_QUOTIENT.md").toString();
  verifySources(tex, classification);
  for (const token of ["ProposedQuotient", "QuotientContext", "ConsequenceSufficient",
    "CoarsestCharacterization", "TestedNondistinction", "coarsestImpliesConsequenceSufficient",
    "exactMapIsConsequenceSufficient", "overcoarseMapFailsConsequenceSufficiency",
    "fineMapIsConsequenceSufficient", "fineMapIsNotCoarsest", "testedABIsNondistinct"]) {
    assert.match(lean, new RegExp(`\\b${token}\\b`, "u"));
  }
  assert.match(doc, /eight exact `LegacyObligation` records\s+at v2\.0 lines 4638–4664/u);
  assert.match(doc, /three quotient records, four\s+consequence-sufficiency records, and one continuation-sufficiency record/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.ExactRepresentationQuotient\r?$/mu);
  console.log(`PASS exact quotient source boundary and contrasts; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-exact-quotient-"));
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
    "coarsestImpliesConsequenceSufficient",
    "Countermodel.exactMapIsConsequenceSufficient",
    "Countermodel.overcoarseMapFailsConsequenceSufficiency",
    "Countermodel.fineMapIsConsequenceSufficient",
    "Countermodel.fineMapIsNotCoarsest",
    "Countermodel.testedABIsNondistinct"
  ];
  const output = probe("contracts", `open ${moduleName}\n` + audits
    .map((name) => `#print axioms ${moduleName}.${name}`).join("\n"));
  for (const name of audits) assert.match(output, new RegExp(
    `'${moduleName.replaceAll(".", "\\.")}\\.${name.replaceAll(".", "\\.")}' does not depend on any axioms`));
  const ablations = [
    ["quotient-map", "  map : Source → Target", "  map : True"],
    ["protected-equivalence", "  protectedEquivalent : Source → Source → Prop", "  protectedEquivalent : True"],
    ["consequence-sufficiency", "def ConsequenceSufficient", "def ConsequenceSufficientRemoved"],
    ["coarsest-characterization", "def CoarsestCharacterization", "def CoarsestCharacterizationRemoved"],
    ["tested-nondistinction", "def TestedNondistinction", "def TestedNondistinctionRemoved"],
    ["overcoarse-foil", "map := fun _ => .only", "map := fun _ => true"]
  ];
  for (const [name, before, after] of ablations) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS six quotient ablations and ${audits.length} axiom-free proof audits`);
}

main();
