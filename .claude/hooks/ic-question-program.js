#!/usr/bin/env node
"use strict";

// Validate and render the derived engineering-question program attached to a
// trace question. This is process evidence only: roots, spans, and recurrence
// positions compile to ordinary questions and introduce no calculus primitive.

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const instances = require("./ic-question-instance.js");
const recursiveGenerator = require("./ic-recursive-generator.js");

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

const V4_ASK_FIELDS = [
  "question_form",
  "rendering",
  "source_lines",
  "generator_ids",
  "reciprocal_relations",
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
  if (schema === "4") return V4_ASK_FIELDS;
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
  if (record.fold_evidence_schema !== undefined && !["0", "1", "2"].includes(record.fold_evidence_schema)) {
    throw new Error("unsupported fold evidence policy");
  }
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
  if (Number(record.fold_evidence_schema ?? 0) < Number(predecessor.fold_evidence_schema ?? 0)) {
    throw new Error("fold evidence policy cannot be downgraded");
  }
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
  if (schema === "4") parseLineList(record.source_lines, "source_lines");
  else parseLineList(record.coding_questions, "coding_questions");
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
  } else if (schema === "1" || schema === "2") {
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
  if (String(manifest.schema) !== "4") {
    throw new Error("active question-program manifest must use schema 4");
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
  const lifecycle = manifest.active_lifecycle;
  if (lifecycle?.trace_schema !== 4) {
    throw new Error("active question-program manifest lacks trace schema 4");
  }
  const forbidden = JSON.stringify(lifecycle).toLowerCase();
  for (const term of ["universal_semantic_next_question", "universal_question_score", "fixed_residual_to_rhythm_schedule"]) {
    if (!(lifecycle.forbidden_selection ?? []).includes(term)) {
      throw new Error(`active lifecycle does not forbid ${term}`);
    }
  }
  if (Object.hasOwn(lifecycle, "method_dispatch") || Object.hasOwn(lifecycle, "residual_schedule") ||
      Object.hasOwn(lifecycle, "universal_question_score") || Object.hasOwn(lifecycle, "next_question")) {
    throw new Error("active lifecycle must not install fixed method dispatch or residual scheduling");
  }
  const forms = new Set((manifest.preformal_harness?.compiled_questions ?? []).map((entry) => entry.id));
  const generators = lifecycle.generator_registry ?? [];
  if (new Set(generators.map((entry) => entry.id)).size !== generators.length || generators.length === 0) {
    throw new Error("active generator registry must contain unique generators");
  }
  const covered = new Set();
  for (const generator of generators) {
    if (typeof generator.relation !== "string" || generator.relation.length === 0) {
      throw new Error(`${generator.id}: generator lacks a relation contract`);
    }
    for (const form of generator.question_forms ?? []) {
      if (!forms.has(form)) throw new Error(`${generator.id}: undeclared question form ${form}`);
      covered.add(form);
    }
  }
  for (const form of forms) {
    if (!covered.has(form)) throw new Error(`question form ${form} has no active generator`);
  }
  void forbidden;
  return { sourceBytes, sourceDigest, manifestDigest, manifest };
}

function validateLegacyQuestionProgram(record, sourceBytes, sourceDigest, manifestDigest, manifest) {
  for (const field of REQUIRED_QUESTION_FIELDS) requireString(record, field);
  if (record.source_digest !== sourceDigest || record.program_manifest_digest !== manifestDigest) {
    throw new Error("legacy question program is detached from the active pinned inputs");
  }
  const harness = manifest.preformal_harness;
  const rhythmId = harness.legacy_residual_schedule?.[record.residual_class];
  if (!rhythmId) throw new Error(`residual_class ${record.residual_class} has no declared legacy rhythm schedule`);
  if (record.rhythm !== rhythmId) throw new Error(`residual ${record.residual_class} requires rhythm ${rhythmId}`);
  const rhythm = harness.principal_rhythms.find((candidate) => candidate.id === rhythmId);
  if (!rhythm) throw new Error(`scheduled legacy rhythm ${rhythmId} is undeclared`);
  if (record.program !== manifest.composition?.id) throw new Error(`program must be ${manifest.composition?.id}`);
  requireExactList(record, "compiled_questions", rhythm.required_questions);
  const formMap = new Map(harness.compiled_questions.map((entry) => [entry.id, entry]));
  const forms = rhythm.required_questions.map((id) => {
    const form = formMap.get(id);
    if (!form) throw new Error(`compiled question ${id} is undeclared`);
    return form;
  });
  requireExactList(record, "coding_questions", unique(forms.flatMap((form) => form.source_lines)).map(String));
  requireExactList(record, "question_families", unique(forms.map((form) => form.family)));
  requireExactList(record, "rhythm_positions", unique(forms.map((form) => form.position)));
  const dimensions = new Set(forms.flatMap((form) => form.dimensions));
  requireExactList(record, "coverage_dimensions", harness.coverage_dimensions.filter((item) => dimensions.has(item)));
  const represented = parseCsv(record.reciprocal_challenges, "reciprocal_challenges", true);
  const blocked = parseCsv(record.blocked_reciprocals, "blocked_reciprocals", true);
  if (represented.some((id) => blocked.includes(id))) throw new Error("reciprocal closure overlaps represented and blocked challenges");
  const accounted = [...represented, ...blocked];
  if (accounted.length !== rhythm.required_reciprocals.length ||
      rhythm.required_reciprocals.some((id) => !accounted.includes(id)) ||
      accounted.some((id) => !rhythm.required_reciprocals.includes(id))) {
    throw new Error(`required reciprocal closure is incomplete for ${rhythm.id}`);
  }
  const status = blocked.length === 0 ? "represented" : represented.length === 0 ? "blocked" : "partially_blocked";
  if (record.reciprocal_status !== status) throw new Error(`reciprocal_status must be ${status}`);
  const challengeMap = new Map(harness.reciprocal_challenges.map((entry) => [entry.id, entry]));
  const challenges = represented.map((id) => {
    const challenge = challengeMap.get(id);
    if (!challenge) throw new Error(`undeclared reciprocal challenge ${id}`);
    return challenge;
  });
  const expectedPairs = challenges.map((challenge) => `${challenge.pair[0]}:${challenge.pair[1]}`);
  const actualPairs = parsePairs(record.reciprocal_pairs).map((pair) => pair.join(":"));
  if (actualPairs.join(";") !== expectedPairs.join(";")) throw new Error(`reciprocal_pairs must exactly realize represented challenges ${expectedPairs.join(";") || "none"}`);
  const axes = harness.central_reciprocal_axes.map((axis) => axis.id)
    .filter((axis) => challenges.some((challenge) => challenge.axes.includes(axis)));
  requireExactList(record, "reciprocal_axes", axes, true);
  const roots = new Set([...forms.flatMap((form) => form.roots), ...challenges.flatMap((challenge) => challenge.roots)]);
  requireExactList(record, "root_spans", harness.root_hypothesis.filter((rootName) => roots.has(rootName)));
  const expectedMethods = harness.exploration_algorithm.legacy_method_dispatch?.[record.residual_shape] ?? [];
  requireExactList(record, "method_frontier", expectedMethods, true);
  const questions = sourceBytes.toString("utf8").split(/\r?\n/u);
  for (const line of unique(forms.flatMap((form) => form.source_lines))) {
    if (!questions[line - 1]?.endsWith("?")) throw new Error(`coding_questions source line ${line} is not a question`);
  }
  return questionProgramCheck(record, "3");
}

function validateQuestionProgram(record, root) {
  const { sourceBytes, sourceDigest, manifestDigest, manifest } = activeInputs(root);
  if (Object.hasOwn(record, "program")) {
    return validateLegacyQuestionProgram(record, sourceBytes, sourceDigest, manifestDigest, manifest);
  }
  for (const field of V4_ASK_FIELDS) requireString(record, field);
  if (record.source_digest !== sourceDigest) {
    throw new Error("source_digest does not match the active Questions.txt bytes");
  }
  if (record.program_manifest_digest !== manifestDigest) {
    throw new Error("program_manifest_digest does not match the active question-program manifest");
  }
  const harness = manifest.preformal_harness;
  const questionContracts = new Map((harness.compiled_questions ?? []).map((question) => [question.id, question]));
  const question = questionContracts.get(record.question_form);
  if (!question) throw new Error(`question form ${record.question_form} is undeclared`);
  if (record.rendering !== `RENDER-${record.question_form}` &&
      !(manifest.active_lifecycle.relational_instances?.schema === 1 && /^RI-[0-9a-f]{64}$/u.test(record.rendering)) &&
      !(manifest.active_lifecycle.recursive_generator_contract?.schema === 1 && /^RG-[0-9a-f]{64}$/u.test(record.rendering))) {
    throw new Error(`rendering must preserve question-form identity RENDER-${record.question_form}`);
  }
  const questions = sourceBytes.toString("utf8").split(/\r?\n/u);
  const questionAt = (line) => questions[line - 1] ?? "";
  requireExactList(record, "source_lines", question.source_lines.map(String));
  for (const line of question.source_lines) {
    if (
      line < manifest.sections.coding.first_question_line ||
      line > manifest.sections.coding.last_question_line ||
      !questionAt(line).endsWith("?")
    ) {
      throw new Error(`question-form source line ${line} is outside the pinned coding corpus`);
    }
  }
  const expectedGenerators = manifest.active_lifecycle.generator_registry
    .filter((generator) => generator.question_forms.includes(record.question_form))
    .map((generator) => generator.id);
  requireExactList(record, "generator_ids", expectedGenerators);
  const family = (harness.program_families ?? []).find((entry) => entry.id === question.family);
  requireExactList(record, "reciprocal_relations", family?.reciprocal_challenges ?? [], true);
  return questionProgramCheck(record, "4");
}

function fieldRecordCheck(record) {
  return sha256(Buffer.from(JSON.stringify({
    field_id: record.field_id,
    members: JSON.parse(record.members),
    basis: record.basis,
    coverage: record.coverage,
    regenerated_from: record.regenerated_from,
    dispositions: JSON.parse(record.dispositions),
    removal_evidence: JSON.parse(record.removal_evidence),
  }), "utf8"));
}

function validateReifiedSeeds(record, root, policy) {
  const seeds = JSON.parse(record.products).filter((product) => Object.hasOwn(product, "inquiry_seed"));
  if (seeds.length === 0) return;
  const { manifest, manifestDigest, sourceDigest } = activeInputs(root);
  if (policy?.program_manifest_digest !== manifestDigest || policy?.source_digest !== sourceDigest ||
      manifest.active_lifecycle.relational_instances?.schema !== 1) {
    throw new Error("inquiry seeds require the active pinned relation-instance policy");
  }
  const forms = new Set(manifest.preformal_harness.compiled_questions.map((form) => form.id));
  for (const product of seeds) {
    if (!forms.has(product.inquiry_seed?.question_form)) throw new Error("inquiry seed uses undeclared corpus form");
  }
}

function validateStoredField(record) {
  for (const field of ["field_id", "members", "basis", "coverage", "regenerated_from", "dispositions", "removal_evidence"]) {
    requireString(record, field);
  }
  let members;
  let dispositions;
  let removalEvidence;
  try {
    members = JSON.parse(record.members);
    dispositions = JSON.parse(record.dispositions);
    removalEvidence = JSON.parse(record.removal_evidence);
  } catch (error) {
    throw new Error(`field JSON is invalid: ${error.message}`);
  }
  if (!Array.isArray(members)) {
    throw new Error("field members must be a JSON array (possibly empty after evidenced retirement)");
  }
  if (dispositions === null || Array.isArray(dispositions) || typeof dispositions !== "object" ||
      removalEvidence === null || Array.isArray(removalEvidence) || typeof removalEvidence !== "object") {
    throw new Error("field dispositions and removal_evidence must be JSON objects");
  }
  if (record.field_check !== fieldRecordCheck(record)) {
    throw new Error("field validation digest is missing or inconsistent");
  }
  return { members, dispositions, removalEvidence };
}

function validateFieldRecord(record, root) {
  const { members } = validateStoredField({ ...record, field_check: fieldRecordCheck(record) });
  const { manifest } = activeInputs(root);
  const harness = manifest.preformal_harness;
  const forms = new Map(harness.compiled_questions.map((entry) => [entry.id, entry]));
  const generators = manifest.active_lifecycle.generator_registry;
  const occurrences = new Set();
  for (const member of members) {
    if (member === null || Array.isArray(member) || typeof member !== "object") {
      throw new Error("each field member must be an object");
    }
    for (const key of ["occurrence", "question_form", "rendering", "prompt", "path", "disposition"]) {
      if (typeof member[key] !== "string" || member[key].length === 0) {
        throw new Error(`field member requires ${key}`);
      }
    }
    if (occurrences.has(member.occurrence)) throw new Error(`duplicate field occurrence ${member.occurrence}`);
    occurrences.add(member.occurrence);
    const form = forms.get(member.question_form);
    if (!form) throw new Error(`field member uses undeclared question form ${member.question_form}`);
    if (Object.hasOwn(member, "relational_instance")) {
      if (manifest.active_lifecycle.relational_instances?.schema !== 1) throw new Error("relational instances are not enabled by active policy");
      const expected = `RI-${instances.renderingIdentity(member)}`;
      if (member.rendering !== expected || member.prompt !== instances.render(form.prompt, member.relational_instance)) {
        throw new Error(`${member.occurrence}: instance rendering identity or prompt changed`);
      }
      // Reference/role checks require trace ancestry and are enforced by the
      // shared append/replay state machine, not by this manifest-only check.
    } else if (Object.hasOwn(member, "derivation")) {
      recursiveGenerator.validateRendering(member, manifest);
    } else if (member.rendering !== `RENDER-${member.question_form}` || member.prompt !== form.prompt) {
      throw new Error(`${member.occurrence}: rendering identity or prompt changed`);
    }
    if (JSON.stringify(member.source_lines) !== JSON.stringify(form.source_lines)) {
      throw new Error(`${member.occurrence}: source-line provenance changed`);
    }
    const expectedGenerators = generators.filter((entry) => entry.question_forms.includes(member.question_form)).map((entry) => entry.id);
    if (JSON.stringify(member.generator_ids) !== JSON.stringify(expectedGenerators)) {
      throw new Error(`${member.occurrence}: generator registry identity changed`);
    }
    if (!Array.isArray(member.dependencies)) throw new Error(`${member.occurrence}: dependencies must be an array`);
    if (!manifest.active_lifecycle.live_dispositions.includes(member.disposition)) {
      throw new Error(`${member.occurrence}: disposition ${member.disposition} is not live`);
    }
    if (typeof member.executable !== "boolean") throw new Error(`${member.occurrence}: executable must be boolean`);
  }
  return fieldRecordCheck(record);
}

function renderQuestionProgram(root) {
  const { sourceBytes, sourceDigest, manifestDigest, manifest } = activeInputs(root);
  const questions = sourceBytes.toString("utf8").split(/\r?\n/u);
  const line = (number) => questions[number - 1] ?? "";
  const harness = manifest.preformal_harness;
  const lifecycle = manifest.active_lifecycle;
  const compiled = new Map(harness.compiled_questions.map((question) => [question.id, question]));
  const output = [
    "FORMAL-SUCCESSOR RELATIONAL QUESTION FIELD REGISTRY",
    `status: ${harness.status}`,
    `predecessor recurrence: ${harness.predecessor_recurrence}`,
    `two-scale law: search=${harness.two_scale_law.search}; commit=${harness.two_scale_law.commit}`,
    `source_sha256: ${sourceDigest}`,
    `program_manifest_sha256: ${manifestDigest}`,
    `central axes: ${harness.central_reciprocal_axes.map((axis) => `${axis.forward}<->${axis.reverse}`).join(", ")}`,
    `recurrence: ${lifecycle.recurrence.join(" -> ")}`,
    `effectful recurrence: ${lifecycle.effectful_recurrence.join(" -> ")}`,
    "Question generators (unordered):",
  ];
  for (const generator of lifecycle.generator_registry) {
    output.push(`  ${generator.id} [${generator.relation}]`);
    for (const id of generator.question_forms) {
      const question = compiled.get(id);
      output.push(`    ${id} RENDER-${id} [${question.family}; ${question.dimensions.join("+")}] ${question.prompt}`);
    }
  }
  output.push(`selection: ${lifecycle.selection_law}`);
  output.push(`reciprocal corpus remains pinned through ${harness.reciprocal_challenges.length} declared relations`);
  void line;
  return `${output.join("\n")}\n`;
}

function renderQuestionSummary(root) {
  const { sourceDigest, manifestDigest, manifest } = activeInputs(root);
  const harness = manifest.preformal_harness;
  const lifecycle = manifest.active_lifecycle;
  return [
    `QUESTION FIELD SCHEMA ${lifecycle.trace_schema}`,
    `families: ${harness.program_families.map((family) => family.code).join(",")}`,
    `compiled: ${harness.compiled_questions.length} questions; ${harness.reciprocal_challenges.length} opposed challenges`,
    `generators: ${lifecycle.generator_registry.map((generator) => generator.id).join(",")}`,
    `dispositions: ${lifecycle.live_dispositions.concat(lifecycle.terminal_dispositions).join(",")}`,
    "methods: matched by explicit applicability contracts; method failure does not close inquiry",
    `axes: ${harness.central_reciprocal_axes.map((axis) => `${axis.forward}<->${axis.reverse}`).join(", ")}`,
    "selection: no semantic next-question oracle; unchosen materialized occurrences remain live",
    `recurrence: ${lifecycle.recurrence.join(" -> ")}`,
    `two-scale: search=${harness.two_scale_law.search}; commit=${harness.two_scale_law.commit}`,
    `policy: Questions=${sourceDigest.slice(0, 16)} manifest=${manifestDigest.slice(0, 16)}`,
    "",
  ].join("\n");
}

module.exports = {
  REQUIRED_QUESTION_FIELDS,
  V4_ASK_FIELDS,
  fieldRecordCheck,
  questionProgramCheck,
  renderQuestionProgram,
  renderQuestionSummary,
  validatePolicy,
  validatePolicyTransition,
  validateFieldRecord,
  validateReifiedSeeds,
  validateQuestionProgram,
  validateStoredQuestion,
  validateStoredField,
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
