#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.ClaimLifecycle";
const modulePath = "formal/InquiryCalculus/Legacy/V20/ClaimLifecycle.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-DECL-CC1075C537ECA3C2", [4503, 4512, "Unproved"]],
  ["PRED-TEX-DISPLAY-542AD7F8E17A399E", [4492, 4497, "Ambiguous"]],
  ["PRED-TEX-DISPLAY-F64B1969D457C1EC", [4484, 4488, "Ambiguous"]],
  ["PRED-TEX-PROSE-272E22CCAD27EB56", [4501, 4501, "Unproved"]],
  ["PRED-TEX-PROSE-498C607E5D2A1098", [4489, 4489, "Ambiguous"]],
  ["PRED-TEX-PROSE-50B6A1EA65393B5E", [4499, 4499, "Ambiguous"]],
  ["PRED-TEX-PROSE-79E53B0C27002E4E", [4483, 4483, "Ambiguous"]],
  ["PRED-TEX-PROSE-C4E2514522BF547C", [4491, 4491, "Ambiguous"]]
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
  const doc = read("formal-successor/PHASE_B_CLAIM_LIFECYCLE.md").toString();
  verifySources(tex, classification);
  for (const token of ["Claim", "Candidate", "CandidateFor", "ReificationFailureKind", "ReificationResidual",
    "ReificationResult", "UnsupportedPromotionBasis", "Lifecycle", "truth", "generated", "reify",
    "warranted", "standing", "standingRequiresWarrant", "failedReificationIsNotSemanticNegation"]) {
    assert.match(lean, new RegExp(`\\b${token}\\b`, "u"));
  }
  assert.match(doc, /eight exact `LegacyObligation` records at v2\.0 lines 4483–4512/u);
  assert.match(doc, /six\s+remain `Ambiguous`/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.doesNotMatch(lean, /confidence\s*:\s*(?:Nat|Float)|automaticStanding|totalReify|selectedCandidate/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(), /^import InquiryCalculus\.Legacy\.V20\.ClaimLifecycle\r?$/mu);
  console.log(`PASS exact claim-lifecycle sources and non-promotion contrast; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-claim-lifecycle-"));
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
    } else {
      assert.equal(result.status, 0, output);
    }
    return output;
  }
  const build = run(["build", moduleName, "--wfail"]);
  assert.equal(build.status, 0, build.stdout + build.stderr);
  const audits = [
    "Candidate.retainsStatement", "standingRequiresWarrant", "noStandingWithoutWarrantedCandidate",
    "unsupportedBasisNeverAuthorizesStanding", "Countermodel.generatedClaimDoesNotEstablishTruthOrStanding",
    "Countermodel.successfulReificationProducesCandidate", "Countermodel.successfulCandidateIsNotWarranted",
    "Countermodel.successfulCandidateDoesNotStand", "Countermodel.failedReificationReturnsTypedInquiry",
    "Countermodel.failedReificationIsNotSemanticNegation", "Countermodel.everyUnsupportedBasisIsRejected"
  ];
  const output = probe("contracts", `open ${moduleName}\n` + audits
    .map((name) => `#print axioms ${moduleName}.${name}`).join("\n"));
  for (const name of audits) {
    assert.match(output, new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.${name.replaceAll(".", "\\.")}' does not depend on any axioms`));
  }
  const ablations = [
    ["provenance", "preservesStatement : statement = source.statement", "preservesStatement : True"],
    ["claim-source", "preservesSource : candidate.source = claim", "preservesSource : True"],
    ["reify", "reify : (claim : Claim Statement) → ReificationResult (CandidateFor claim) Inquiry", "reify : True"],
    ["failure", "| failure : ReificationResidual Inquiry → ReificationResult Candidate Inquiry", "| failed : ReificationResidual Inquiry → ReificationResult Candidate Inquiry"],
    ["residual", "inquiry : Inquiry", "inquiry : True"],
    ["truth", "truth : Statement → Prop", "truth : True"],
    ["generated", "generated : Claim Statement → Prop", "generated : True"],
    ["warrant", "standingRequiresWarrant : ∀ statement, standing statement →", "standingRequiresWarrant : ∀ statement, standing statement → True →"],
    ["weak-promotion", "(_ : Candidate Statement) : Prop :=\n  False", "(_ : Candidate Statement) : Prop :=\n  True"]
  ];
  for (const [name, before, after] of ablations) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS partial claim lifecycle; ${ablations.length} rejected source ablations; ${audits.length} axiom-free proof audits`);
}

main();
