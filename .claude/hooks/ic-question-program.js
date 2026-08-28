#!/usr/bin/env node
"use strict";

// Validate and render the derived engineering-question program attached to a
// trace question. This is process evidence only: roots, spans, and recurrence
// positions compile to ordinary questions and introduce no calculus primitive.

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const V1_QUESTION_FIELDS = [
  "program",
  "coding_questions",
  "reciprocal_applicability",
  "reciprocal_pairs",
  "reciprocal_reason",
  "source_digest",
  "program_manifest_digest",
];

const V2_QUESTION_FIELDS = [
  "program",
  "rhythm",
  "coding_questions",
  "relation_spans",
  "root_spans",
  "recurrence_positions",
  "reciprocal_applicability",
  "reciprocal_pairs",
  "reciprocal_dimensions",
  "reciprocal_reason",
  "source_digest",
  "program_manifest_digest",
];

const REQUIRED_QUESTION_FIELDS = [
  "program",
  "rhythm",
  "residual_class",
  "compiled_questions",
  "question_families",
  "coding_questions",
  "coverage_dimensions",
  "root_spans",
  "rhythm_positions",
  "reciprocal_status",
  "reciprocal_challenges",
  "blocked_reciprocals",
  "reciprocal_pairs",
  "reciprocal_axes",
  "reciprocal_reason",
  "parent_residual",
  "condition_ids",
  "breaker_ids",
  "reciprocal_obligation",
  "question_disposition",
  "residual_shape",
  "method_frontier",
  "condition_keys",
  "source_digest",
  "program_manifest_digest",
];

function sha256(bytes) {
  return crypto.createHash("sha256").update(bytes).digest("hex");
}

function readBytes(root, relative) {
  return fs.readFileSync(path.join(root, ...relative.split("/")));
}

function requireString(record, field) {
  if (typeof record[field] !== "string" || record[field].trim() === "") {
    throw new Error(`question-program record requires nonempty ${field}`);
  }
}

function schemaFields(schema) {
  if (schema === "1") return V1_QUESTION_FIELDS;
  if (schema === "2") return V2_QUESTION_FIELDS;
  if (schema === "3") return REQUIRED_QUESTION_FIELDS;
  throw new Error(`unsupported question-program schema ${schema}`);
}

function parseLineList(value, label) {
  if (!/^\d+(?:,\d+)*$/u.test(value)) {
    throw new Error(`${label} must be a comma-separated list of positive source lines`);
  }
  const lines = value.split(",").map(Number);
  if (new Set(lines).size !== lines.length) {
    throw new Error(`${label} must not repeat a source line`);
  }
  return lines;
}

function parsePairs(value) {
  if (value === "none") return [];
  if (!/^\d+:\d+(?:;\d+:\d+)*$/u.test(value)) {
    throw new Error(
      "reciprocal_pairs must contain declared two-orientation pairs as left:right separated by semicolons",
    );
  }
  const pairs = value.split(";").map((pair) => pair.split(":").map(Number));
  const identities = pairs.map(([left, right]) => `${left}:${right}`);
  if (new Set(identities).size !== identities.length) {
    throw new Error("reciprocal_pairs must not repeat a pair");
  }
  return pairs;
}

function parseCsv(value, label, allowNone = false) {
  if (allowNone && value === "none") return [];
  const entries = value.split(",");
  if (entries.some((entry) => entry.length === 0 || entry.trim() !== entry)) {
    throw new Error(`${label} must be a comma-separated list with no empty or padded entries`);
  }
  if (new Set(entries).size !== entries.length) {
    throw new Error(`${label} must not contain duplicate entries`);
  }
  return entries;
}

function unique(values) {
  return [...new Set(values)];
}

function requireExactList(record, field, expected, allowNone = false) {
  const actual = parseCsv(record[field], field, allowNone);
  if (actual.join(",") !== expected.join(",")) {
    throw new Error(
      `${field} must exactly report ${expected.length === 0 ? "none" : expected.join(",")}`,
    );
  }
}

function questionProgramCheck(record, schema = "2") {
  const fields = schemaFields(schema);
  const normalized = fields.map((field) => `${field}=${record[field]}`).join("\0");
  return sha256(Buffer.from(`ic-question-program-v${schema}\0${normalized}`, "utf8"));
}

function validatePolicy(record) {
  for (const field of ["question_program_schema", "source_digest", "program_manifest_digest"]) {
    requireString(record, field);
  }
  schemaFields(record.question_program_schema);
  for (const field of ["source_digest", "program_manifest_digest"]) {
    if (!/^[0-9a-f]{64}$/u.test(record[field])) {
      throw new Error(`${field} must be a lowercase SHA-256 digest`);
    }
  }
}

function validatePolicyTransition(record, predecessor) {
  validatePolicy(record);
  for (const field of [
    "predecessor_source_digest",
    "predecessor_program_manifest_digest",
    "authority",
    "reason",
  ]) {
    requireString(record, field);
  }
  if (
    record.predecessor_source_digest !== predecessor.source_digest ||
    record.predecessor_program_manifest_digest !== predecessor.program_manifest_digest
  ) {
    throw new Error("question-program policy transition does not name its exact predecessor");
  }
}

function validateStoredQuestion(record, policy) {
  const schema = policy.question_program_schema;
  for (const field of schemaFields(schema)) requireString(record, field);
  if (
    record.source_digest !== policy.source_digest ||
    record.program_manifest_digest !== policy.program_manifest_digest
  ) {
    throw new Error("question-program record is detached from the active trace policy");
  }
  parseLineList(record.coding_questions, "coding_questions");
  if (schema === "3") {
    for (const field of [
      "compiled_questions",
      "question_families",
      "coverage_dimensions",
      "root_spans",
      "rhythm_positions",
      "reciprocal_challenges",
      "blocked_reciprocals",
      "reciprocal_axes",
      "condition_ids",
      "breaker_ids",
      "method_frontier",
    ]) {
      parseCsv(
        record[field],
        field,
        new Set([
          "reciprocal_challenges", "blocked_reciprocals", "reciprocal_axes",
          "condition_ids", "breaker_ids", "method_frontier",
        ]).has(field),
      );
    }
    if (!new Set(["represented", "partially_blocked", "blocked"]).has(record.reciprocal_status)) {
      throw new Error("reciprocal_status must be represented, partially_blocked, or blocked");
    }
    parsePairs(record.reciprocal_pairs);
  } else {
    if (!new Set(["applicable", "inapplicable"]).has(record.reciprocal_applicability)) {
      throw new Error("reciprocal_applicability must be applicable or inapplicable");
    }
    if (record.reciprocal_applicability === "applicable") parsePairs(record.reciprocal_pairs);
    if (record.reciprocal_applicability === "inapplicable" && record.reciprocal_pairs !== "none") {
      throw new Error("an inapplicable reciprocal direction must record reciprocal_pairs=none");
    }
  }
  if (record.question_program_check !== questionProgramCheck(record, schema)) {
    throw new Error("question-program validation digest is missing or inconsistent");
  }
}

function collectLines(program, field) {
  const direct = program[field] ?? [];
  return field === "paired_sequence" ? direct.flat() : direct;
}

function requireExactCoverage(record, field, order, selected, required = order) {
  const missing = required.filter((entry) => !selected.has(entry));
  if (missing.length > 0) {
    throw new Error(`${field} is missing required calculus-rhythm coverage: ${missing.join(",")}`);
  }
  const actual = order.filter((entry) => selected.has(entry));
  const declared = record[field] === "none" ? [] : record[field].split(",");
  if (declared.join(",") !== actual.join(",")) {
    throw new Error(`${field} must exactly report derived coverage ${actual.join(",") || "none"}`);
  }
}

function activeInputs(root) {
  const sourcePath = "formal-successor/Questions.txt";
  const manifestPath = "formal-successor/ENGINEERING_QUESTION_PROGRAMS.json";
  const harnessPath = "formal-successor/PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS.md";
  const asymmetryPath = "formal-successor/PREFORMAL_SEARCH_ASYMMETRY.md";
  const consolidatedPath = "formal-successor/SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md";
  const explorationPath = "formal-successor/QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md";
  const sourceBytes = readBytes(root, sourcePath);
  const manifestBytes = readBytes(root, manifestPath);
  const harnessBytes = readBytes(root, harnessPath);
  const asymmetryBytes = readBytes(root, asymmetryPath);
  const consolidatedBytes = readBytes(root, consolidatedPath);
  const explorationBytes = readBytes(root, explorationPath);
  const sourceDigest = sha256(sourceBytes);
  const manifestDigest = sha256(manifestBytes);
  const manifest = JSON.parse(manifestBytes.toString("utf8"));
  if (manifest.source_sha256 !== sourceDigest) {
    throw new Error("question-program manifest is detached from Questions.txt");
  }
  if (String(manifest.schema) !== "3") {
    throw new Error("active question-program manifest must use schema 3");
  }
  if (
    manifest.preformal_harness?.source !== harnessPath ||
    manifest.preformal_harness?.source_sha256 !== sha256(harnessBytes)
  ) {
    throw new Error("question-program manifest is detached from the preformal harness input");
  }
  if (
    manifest.preformal_harness?.search_asymmetry_source !== asymmetryPath ||
    manifest.preformal_harness?.search_asymmetry_sha256 !== sha256(asymmetryBytes)
  ) {
    throw new Error("question-program manifest is detached from the search/commit asymmetry input");
  }
  if (
    manifest.preformal_harness?.consolidated_spec_source !== consolidatedPath ||
    manifest.preformal_harness?.consolidated_spec_sha256 !== sha256(consolidatedBytes)
  ) {
    throw new Error("question-program manifest is detached from the consolidated harness specification");
  }
  if (
    manifest.preformal_harness?.exploration_algorithm_source !== explorationPath ||
    manifest.preformal_harness?.exploration_algorithm_sha256 !== sha256(explorationBytes)
  ) {
    throw new Error("question-program manifest is detached from the question-bank-derived exploration algorithm");
  }
  return { sourceBytes, sourceDigest, manifestDigest, manifest };
}

function validateQuestionProgram(record, root) {
  for (const field of REQUIRED_QUESTION_FIELDS) requireString(record, field);

  const { sourceBytes, sourceDigest, manifestDigest, manifest } = activeInputs(root);
  if (record.source_digest !== sourceDigest) {
    throw new Error("source_digest does not match the active Questions.txt bytes");
  }
  if (record.program_manifest_digest !== manifestDigest) {
    throw new Error("program_manifest_digest does not match the active question-program manifest");
  }
  if (record.parent_residual !== "none" && !/^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)+$/u.test(record.parent_residual)) {
    throw new Error("parent_residual must be none or a stable residual id");
  }
  parseCsv(record.condition_ids, "condition_ids", true);
  parseCsv(record.breaker_ids, "breaker_ids", true);
  if (!new Set(["represented", "blocked", "not_applicable"]).has(record.reciprocal_obligation)) {
    throw new Error("reciprocal_obligation must be represented, blocked, or not_applicable");
  }
  const algorithm = manifest.preformal_harness?.exploration_algorithm;
  if (!(algorithm?.question_dispositions ?? []).includes(record.question_disposition)) {
    throw new Error(`unknown question_disposition ${record.question_disposition}`);
  }
  if (!(algorithm?.residual_shapes ?? []).includes(record.residual_shape)) {
    throw new Error(`unknown residual_shape ${record.residual_shape}`);
  }
  const expectedMethods = algorithm?.method_dispatch?.[record.residual_shape] ?? [];
  requireExactList(record, "method_frontier", expectedMethods);
  if (record.condition_keys !== "none" && !/^[A-Za-z0-9._-]+@[A-Za-z0-9._-]+@[A-Za-z0-9._-]+@[A-Za-z0-9._-]+@[A-Za-z0-9._-]+@(forward|reverse|neutral)(?:;[A-Za-z0-9._-]+@[A-Za-z0-9._-]+@[A-Za-z0-9._-]+@[A-Za-z0-9._-]+@[A-Za-z0-9._-]+@(forward|reverse|neutral))*$/u.test(record.condition_keys)) {
    throw new Error("condition_keys must be none or normalized schema@roles@scope@applicability@grain@orientation entries");
  }
  if (record.program !== manifest.composition?.id) {
    throw new Error(`program must be the declared composition ${manifest.composition?.id ?? "<missing>"}`);
  }
  const harness = manifest.preformal_harness;
  const scheduledRhythm = harness.residual_schedule?.[record.residual_class];
  if (!scheduledRhythm) {
    throw new Error(`residual_class ${record.residual_class} has no declared rhythm schedule`);
  }
  if (record.rhythm !== scheduledRhythm) {
    throw new Error(`residual ${record.residual_class} requires rhythm ${scheduledRhythm}`);
  }
  const rhythm = (harness.principal_rhythms ?? []).find(
    (candidate) => candidate.id === scheduledRhythm,
  );
  if (!rhythm) throw new Error(`scheduled rhythm ${scheduledRhythm} is undeclared`);

  const questionContracts = new Map(
    (harness.compiled_questions ?? []).map((question) => [question.id, question]),
  );
  const selectedQuestionIds = parseCsv(record.compiled_questions, "compiled_questions");
  if (selectedQuestionIds.join(",") !== rhythm.required_questions.join(",")) {
    throw new Error(
      `compiled_questions must exactly realize residual rhythm ${rhythm.id}: ` +
        rhythm.required_questions.join(","),
    );
  }
  const selectedQuestions = selectedQuestionIds.map((id) => {
    const question = questionContracts.get(id);
    if (!question) throw new Error(`compiled question ${id} is undeclared`);
    return question;
  });
  const questions = sourceBytes.toString("utf8").split(/\r?\n/u);
  const questionAt = (line) => questions[line - 1] ?? "";
  const expectedCodingLines = unique(selectedQuestions.flatMap((question) => question.source_lines));
  requireExactList(record, "coding_questions", expectedCodingLines.map(String));
  for (const line of expectedCodingLines) {
    if (
      line < manifest.sections.coding.first_question_line ||
      line > manifest.sections.coding.last_question_line ||
      !questionAt(line).endsWith("?")
    ) {
      throw new Error(`compiled coding source line ${line} is outside the pinned coding corpus`);
    }
  }
  const expectedFamilies = unique(selectedQuestions.map((question) => question.family));
  requireExactList(record, "question_families", expectedFamilies);
  const expectedPositions = unique(selectedQuestions.map((question) => question.position));
  requireExactList(record, "rhythm_positions", expectedPositions);
  const selectedDimensions = new Set(
    selectedQuestions.flatMap((question) => question.dimensions ?? []),
  );
  const missingDimensions = rhythm.required_dimensions.filter(
    (dimension) => !selectedDimensions.has(dimension),
  );
  if (missingDimensions.length > 0) {
    throw new Error(
      `compiled question subset is missing relational-span coverage: ${missingDimensions.join(",")}`,
    );
  }
  const expectedDimensions = harness.coverage_dimensions.filter((dimension) =>
    selectedDimensions.has(dimension),
  );
  requireExactList(record, "coverage_dimensions", expectedDimensions);

  const challengeContracts = new Map(
    (harness.reciprocal_challenges ?? []).map((challenge) => [challenge.id, challenge]),
  );
  const selectedChallengeIds = parseCsv(
    record.reciprocal_challenges,
    "reciprocal_challenges",
    true,
  );
  const blockedChallengeIds = parseCsv(
    record.blocked_reciprocals,
    "blocked_reciprocals",
    true,
  );
  const overlap = selectedChallengeIds.filter((id) => blockedChallengeIds.includes(id));
  if (overlap.length > 0) {
    throw new Error(`reciprocal challenges cannot be both represented and blocked: ${overlap.join(",")}`);
  }
  const accountedChallenges = [...selectedChallengeIds, ...blockedChallengeIds];
  if (
    accountedChallenges.length !== rhythm.required_reciprocals.length ||
    rhythm.required_reciprocals.some((id) => !accountedChallenges.includes(id)) ||
    accountedChallenges.some((id) => !rhythm.required_reciprocals.includes(id))
  ) {
    throw new Error(
      `required reciprocal closure is incomplete for ${rhythm.id}: ` +
        rhythm.required_reciprocals.join(","),
    );
  }
  const expectedStatus =
    blockedChallengeIds.length === 0
      ? "represented"
      : selectedChallengeIds.length === 0
        ? "blocked"
        : "partially_blocked";
  if (record.reciprocal_status !== expectedStatus) {
    throw new Error(`reciprocal_status must be ${expectedStatus} for the represented/blocked split`);
  }
  const selectedChallenges = selectedChallengeIds.map((id) => {
    const challenge = challengeContracts.get(id);
    if (!challenge) throw new Error(`reciprocal challenge ${id} is undeclared`);
    return challenge;
  });
  for (const id of blockedChallengeIds) {
    if (!challengeContracts.has(id)) throw new Error(`blocked reciprocal challenge ${id} is undeclared`);
  }
  const expectedPairs = selectedChallenges.map(
    (challenge) => `${challenge.pair[0]}:${challenge.pair[1]}`,
  );
  const actualPairs = parsePairs(record.reciprocal_pairs).map(
    ([left, right]) => `${left}:${right}`,
  );
  if (actualPairs.join(";") !== expectedPairs.join(";")) {
    throw new Error(`reciprocal_pairs must exactly realize represented challenges ${expectedPairs.join(";") || "none"}`);
  }
  for (const challenge of selectedChallenges) {
    const [left, right] = challenge.pair;
    if (
      left < manifest.sections.reciprocal_why.first_question_line ||
      left > manifest.sections.reciprocal_why.last_question_line ||
      right < manifest.sections.reciprocal_why.first_question_line ||
      right > manifest.sections.reciprocal_why.last_question_line ||
      !questionAt(left).endsWith("?") ||
      !questionAt(right).endsWith("?")
    ) {
      throw new Error(`reciprocal challenge ${challenge.id} is outside the pinned Reciprocal why corpus`);
    }
  }
  const expectedAxes = harness.central_reciprocal_axes
    .map((axis) => axis.id)
    .filter((axis) => selectedChallenges.some((challenge) => challenge.axes.includes(axis)));
  requireExactList(record, "reciprocal_axes", expectedAxes, true);

  const selectedRoots = new Set([
    ...selectedQuestions.flatMap((question) => question.roots ?? []),
    ...selectedChallenges.flatMap((challenge) => challenge.roots ?? []),
  ]);
  const expectedRoots = harness.root_hypothesis.filter((rootName) => selectedRoots.has(rootName));
  requireExactList(record, "root_spans", expectedRoots);

  return questionProgramCheck(record, "3");
}

function renderQuestionProgram(root) {
  const { sourceBytes, sourceDigest, manifestDigest, manifest } = activeInputs(root);
  const questions = sourceBytes.toString("utf8").split(/\r?\n/u);
  const line = (number) => questions[number - 1] ?? "";
  const harness = manifest.preformal_harness;
  const rhythm = harness.principal_rhythms.find(
    (candidate) => candidate.id === harness.residual_schedule.default,
  );
  const compiled = new Map(harness.compiled_questions.map((question) => [question.id, question]));
  const challenges = new Map(
    harness.reciprocal_challenges.map((challenge) => [challenge.id, challenge]),
  );
  const output = [
    `PREFORMAL SUCCESSOR QUESTION RHYTHM ${rhythm.id}`,
    `status: ${harness.status}`,
    `predecessor recurrence: ${harness.predecessor_recurrence}`,
    `two-scale law: search=${harness.two_scale_law.search}; commit=${harness.two_scale_law.commit}`,
    `source_sha256: ${sourceDigest}`,
    `program_manifest_sha256: ${manifestDigest}`,
    `central axes: ${harness.central_reciprocal_axes.map((axis) => `${axis.forward}<->${axis.reverse}`).join(", ")}`,
    `coverage dimensions: ${rhythm.required_dimensions.join(", ")}`,
    "Default compiled questions:",
  ];
  for (const id of rhythm.required_questions) {
    const question = compiled.get(id);
    output.push(
      `  ${question.position} ${id} [${question.family}; ${question.dimensions.join("+")}] ${question.prompt}`,
    );
  }
  output.push("Required reciprocal challenges:");
  for (const id of rhythm.required_reciprocals) {
    const challenge = challenges.get(id);
    output.push(
      `  ${challenge.movement}: ${challenge.pair[0]} ${line(challenge.pair[0])} <-> ` +
        `${challenge.pair[1]} ${line(challenge.pair[1])}`,
    );
  }
  output.push("Residual schedule:");
  for (const [residual, selectedRhythm] of Object.entries(harness.residual_schedule)) {
    output.push(`  ${residual} -> ${selectedRhythm}`);
  }
  return `${output.join("\n")}\n`;
}

function renderQuestionSummary(root) {
  const { sourceDigest, manifestDigest, manifest } = activeInputs(root);
  const harness = manifest.preformal_harness;
  const defaultRhythm = harness.residual_schedule.default;
  return [
    `QUESTION PROGRAM ${defaultRhythm}`,
    `families: ${harness.program_families.map((family) => family.code).join(",")}`,
    `compiled: ${harness.compiled_questions.length} questions; ${harness.reciprocal_challenges.length} opposed challenges`,
    `dispositions: ${harness.exploration_algorithm.question_dispositions.join(",")}`,
    `method dispatch: residual-shape selected; ${harness.exploration_algorithm.materialization_law}`,
    `axes: ${harness.central_reciprocal_axes.map((axis) => `${axis.forward}<->${axis.reverse}`).join(", ")}`,
    "selection: residual-selected; relationally nonredundant; answer-dependent; required reciprocals represented or typed blocked",
    `two-scale: search=${harness.two_scale_law.search}; commit=${harness.two_scale_law.commit}`,
    `policy: Questions=${sourceDigest.slice(0, 16)} manifest=${manifestDigest.slice(0, 16)}`,
    "",
  ].join("\n");
}

module.exports = {
  REQUIRED_QUESTION_FIELDS,
  questionProgramCheck,
  renderQuestionProgram,
  renderQuestionSummary,
  validatePolicy,
  validatePolicyTransition,
  validateQuestionProgram,
  validateStoredQuestion,
};

if (require.main === module) {
  try {
    const [command, root, encoded] = process.argv.slice(2);
    if (command === "projection" && root && !encoded) {
      process.stdout.write(renderQuestionProgram(path.resolve(root)));
    } else if (command === "summary" && root && !encoded) {
      process.stdout.write(renderQuestionSummary(path.resolve(root)));
    } else if (command === "validate" && root && encoded) {
      const record = JSON.parse(encoded);
      process.stdout.write(`${validateQuestionProgram(record, path.resolve(root))}\n`);
    } else {
      throw new Error(
        "usage: ic-question-program.js projection|summary ROOT | validate ROOT JSON_RECORD",
      );
    }
  } catch (error) {
    process.stderr.write(`ic-question-program: ${error.message}\n`);
    process.exitCode = 1;
  }
}
