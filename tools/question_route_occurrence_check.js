#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const moduleName = "InquiryCalculus.Legacy.V20.QuestionRouteOccurrence";
const modulePath = "formal/InquiryCalculus/Legacy/V20/QuestionRouteOccurrence.lean";
const texDigest = "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const sources = new Map([
  ["PRED-TEX-PROSE-74AB75710205500E", [[5201, 5201], "Ambiguous"]],
  ["PRED-TEX-DISPLAY-B2142B8B08D206FE", [[5202, 5212], "Ambiguous"]],
  ["PRED-TEX-PROSE-7A9455416E4755C4", [[5213, 5216], "Unproved"]],
  ["PRED-TEX-PROSE-18B389A82123A460", [[5218, 5218], "Ambiguous"]],
  ["PRED-TEX-DISPLAY-C163E6F370A9CE89", [[5219, 5221], "Ambiguous"]],
  ["PRED-TEX-PROSE-BBF25F08C060FB73", [[5222, 5225], "Unproved"]],
  ["PRED-TEX-PROSE-F80642D41AC2801D", [[5227, 5232], "Unproved"]],
  ["PRED-TEX-DECL-LAW-NO-SECOND-QUESTION-LANGUAGE", [[5234, 5239], "Unproved"]]
]);
const read = (name) => fs.readFileSync(path.join(root, name));
const digest = (value) => crypto.createHash("sha256").update(value).digest("hex");

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--compile"));
  const tex = read("Inquiry_Calculus_v2_0.tex");
  const lines = tex.toString().replace(/\r\n?/gu, "\n").split("\n");
  const records = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json")).records;
  const lean = read(modulePath).toString();
  assert.equal(digest(tex), texDigest);
  for (const [sourceId, [span, status]] of sources) {
    const matches = records.filter((record) => record.source_id === sourceId);
    assert.equal(matches.length, 1, sourceId);
    const record = matches[0];
    const excerpt = lines.slice(span[0] - 1, span[1]).map((line) => line.trimEnd()).join("\n").trim();
    assert.equal(record.disposition, "LegacyObligation");
    assert.equal(record.legacy_obligation.status, status);
    assert.deepEqual([record.source.start_line, record.source.end_line], span);
    assert.equal(record.source.sha256, digest(excerpt));
    assert.equal(record.source_excerpt_sha256, digest(excerpt));
  }
  for (const declaration of [
    "ActualCoordinates", "PureCoordinates", "QuestionRouteOccurrence", "IsPureRoute",
    "pureCoordinatesAreNonactual", "SupportedAnswerCueFiber", "CheckedOccurrenceFiber",
    "ExactRegeneration", "QuestionRouteOccurrenceObligation", "CompactRoute", "semanticProjection",
    "equalSemanticProjection", "distinctQuestionRouteOccurrences",
    "supportedAnswerRemovalIsOccurrenceSpecific", "questionRemovalRetainsDistinctCheckedOccurrences",
    "eachProtectedCoordinateSeparatesRightRoute"
  ]) assert.match(lean, new RegExp(`\\b${declaration}\\b`));
  assert.doesNotMatch(lean, /\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);
  assert.match(read("formal/InquiryCalculus.lean").toString(),
    /^import InquiryCalculus\.Legacy\.V20\.QuestionRouteOccurrence\r?$/mu);
  const document = read("formal-successor/PHASE_B_QUESTION_ROUTE_OCCURRENCE.md").toString();
  assert.match(document, /remains a named obligation/iu);

  const run = (args) => childProcess.spawnSync("lake", args, {
    cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true
  });
  if (process.argv.includes("--compile")) {
    const build = run(["build", moduleName, "--wfail"]);
    assert.equal(build.status, 0, build.stdout + build.stderr);
    const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "ic-question-route-"));
    for (const [index, proof] of [
      "pureCoordinatesAreNonactual", "Countermodel.equalSemanticProjection",
      "Countermodel.distinctQuestionRouteOccurrences",
      "Countermodel.supportedAnswerRemovalIsOccurrenceSpecific",
      "Countermodel.questionRemovalRetainsDistinctCheckedOccurrences",
      "Countermodel.eachProtectedCoordinateSeparatesRightRoute"
    ].entries()) {
      const auditFile = path.join(temporary, `audit-${index}.lean`);
      fs.writeFileSync(auditFile, `import ${moduleName}\n#print axioms ${moduleName}.${proof}\n`);
      const audit = childProcess.spawnSync("lake", ["env", "lean", auditFile], {
        cwd: path.join(root, "formal"), encoding: "utf8", windowsHide: true
      });
      assert.equal(audit.status, 0, audit.stdout + audit.stderr);
      assert.match(audit.stdout + audit.stderr, /does not depend on any axioms/u, proof);
    }
  }
  console.log(`PASS eight exact route sources, finite collision, removal fibers, and regeneration boundary; module sha256 ${digest(lean)}`);
}

main();
