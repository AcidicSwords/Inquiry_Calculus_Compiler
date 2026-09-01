#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.LeastFixedPointStanding";
const modulePath = "formal/InquiryCalculus/Legacy/V20/LeastFixedPointStanding.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-PROSE-F43286975087BA44", [4579, 4579, "LegacyObligation", "Unproved"]],
  ["PRED-TEX-PROSE-981FA8137E0CE57E", [4581, 4581, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-DISPLAY-05EE422A5B2F0E8D", [4582, 4594, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-PROSE-B06D4CEEFBDCB356", [4596, 4596, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-DISPLAY-EAC5E0B4978C07E2", [4597, 4603, "LegacyObligation", "Ambiguous"]],
  ["PRED-TEX-DECL-216360B2EDCA8ADB", [4605, 4607, "FormalTheorem", null]]
]);
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");

function verifySources(tex, classification) {
  assert.equal(hash(tex), texDigest);
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  for (const [id, [start, end, disposition, status]] of sources) {
    const records = classification.records.filter((record) => record.source_id === id);
    assert.equal(records.length, 1, id);
    const record = records[0];
    const excerpt = lines.slice(start - 1, end).map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(record.disposition, disposition, id);
    if (status === null) assert.equal(record.legacy_obligation, undefined, id);
    else assert.equal(record.legacy_obligation.status, status, id);
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
  const doc = read("formal-successor/PHASE_B_LEAST_FIXED_POINT_STANDING.md").toString();
  verifySources(tex, classification);
  for (const token of ["CandidateSet", "Included", "MonotoneOperator", "IsPreFixedPoint",
    "IsFixedPoint", "LeastFixedPoint", "StandingContext", "ingress", "closedSupport",
    "StandingOperator", "Standing", "PositiveRootlessRegion", "noRootlessPositiveSupportCycle",
    "Iterate", "leastStandingExactlyRooted", "rootlessCycleExcluded",
    "arbitraryFixedPointIsNotStanding", "emptyDependenciesAreNotIngress",
    "finiteModelStabilizesAtTwo"]) {
    assert.match(lean, new RegExp(`\\b${token}\\b`, "u"));
  }
  assert.match(doc, /six exact source records at v2\.0 lines 4579–4607/u);
  assert.match(doc, /one remains `Unproved`, four remain `Ambiguous`, and one is a `FormalTheorem`/u);
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.doesNotMatch(lean, /algorithmicConvergence|automaticStanding|greatestFixedPoint/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.LeastFixedPointStanding\r?$/mu);
  console.log(`PASS exact standing sources and grounded least-fixed-point contrast; module sha256 ${hash(lean)}`);
  if (!process.argv.includes("--compile")) return;

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "ic-least-standing-"));
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
    "leastIncludedInPreFixedPoint", "operatorLeastIncludedInLeast",
    "leastIncludedInOperatorLeast", "leastIsFixedPoint", "leastAmongFixedPoints",
    "standingOperatorMonotone", "standingIsLeastFixedPoint", "ingressStands",
    "closedSupportIntoStandingStands", "noRootlessPositiveSupportCycle",
    "iterateIncludedInLeast", "Countermodel.leastStandingExactlyRooted",
    "Countermodel.rootlessCycleExcluded", "Countermodel.overlargeIsFixedPoint",
    "Countermodel.arbitraryFixedPointIsNotStanding",
    "Countermodel.emptyDependenciesAreNotIngress",
    "Countermodel.finiteModelStabilizesAtTwo"
  ];
  const output = probe("contracts", `open ${moduleName}\n` + audits
    .map((name) => `#print axioms ${moduleName}.${name}`).join("\n"));
  for (const name of audits) {
    assert.match(output, new RegExp(`'${moduleName.replaceAll(".", "\\.")}\\.${name.replaceAll(".", "\\.")}' does not depend on any axioms`));
  }
  const ablations = [
    ["included-relation", "def Included {Candidate", "def IncludedRemoved {Candidate"],
    ["monotonicity-relation", "def MonotoneOperator {Candidate", "def MonotoneOperatorRemoved {Candidate"],
    ["prefixed-relation", "def IsPreFixedPoint {Candidate", "def IsPreFixedPointRemoved {Candidate"],
    ["fixed-relation", "def IsFixedPoint {Candidate", "def IsFixedPointRemoved {Candidate"],
    ["least-construction", "def LeastFixedPoint {Candidate", "def LeastFixedPointRemoved {Candidate"],
    ["ingress-coordinate", "  ingress : Candidate → Prop", "  ingress : True"],
    ["route-target-coordinate", "  routeTarget : Route → Candidate → Prop", "  routeTarget : True"],
    ["requires-coordinate", "  requires : Route → Candidate → Prop", "  requires : True"],
    ["closed-support-coordinate", "  closedSupport : CandidateSet Candidate → Route → Candidate → Prop", "  closedSupport : True"],
    ["closed-support-monotonicity", "  closedSupportMonotone : ∀", "  closedSupportMonotoneRemoved : ∀"],
    ["ingress-operator-branch", "  fun candidate => context.ingress candidate ∨", "  fun candidate => False ∨"],
    ["standing-is-least", "  LeastFixedPoint (StandingOperator context)", "  fun _ => False"],
    ["finite-enumeration", "  finiteEnumeration : List Candidate", "  finiteEnumeration : True"],
    ["no-ingress-region", "  noIngress : ∀", "  noIngressRemoved : ∀"],
    ["positive-cycle-premise", "  closedRouteRequiresRegionMember : ∀", "  closedRouteRequiresRegionMemberRemoved : ∀"],
    ["rootless-theorem", "theorem noRootlessPositiveSupportCycle", "theorem noRootlessPositiveSupportCycleRemoved"],
    ["iteration-boundary", "def Iterate {Candidate", "def IterateRemoved {Candidate"]
  ];
  for (const [name, before, after] of ablations) {
    const changed = lean.replace(before, after);
    assert.notEqual(changed, lean, name);
    probe(`drop-${name}`, changed, true, true);
  }
  console.log(`PASS least fixed point and rootless-cycle exclusion; ${ablations.length} rejected source ablations; ${audits.length} axiom-free proof audits`);
}

main();
