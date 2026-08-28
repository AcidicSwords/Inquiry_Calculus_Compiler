#!/usr/bin/env node
"use strict";

// Validate the engineering-question program attached to a trace question.
// This is process evidence only: it introduces no calculus primitive or runtime mode.

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const REQUIRED_QUESTION_FIELDS = [
  "program",
  "coding_questions",
  "reciprocal_applicability",
  "reciprocal_pairs",
  "reciprocal_reason",
  "source_digest",
  "program_manifest_digest",
];

function sha256(bytes) {
  return crypto.createHash("sha256").update(bytes).digest("hex");
}

function readBytes(root, relative) {
  return fs.readFileSync(path.join(root, ...relative.split("/")));
}

function readJson(root, relative) {
  return JSON.parse(readBytes(root, relative).toString("utf8"));
}

function requireString(record, field) {
  if (typeof record[field] !== "string" || record[field].trim() === "") {
    throw new Error(`question-program record requires nonempty ${field}`);
  }
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

function questionProgramCheck(record) {
  const normalized = REQUIRED_QUESTION_FIELDS.map((field) => `${field}=${record[field]}`).join("\0");
  return sha256(Buffer.from(`ic-question-program-v1\0${normalized}`, "utf8"));
}

function validateStoredQuestion(record, policy) {
  for (const field of REQUIRED_QUESTION_FIELDS) requireString(record, field);
  if (
    record.source_digest !== policy.source_digest ||
    record.program_manifest_digest !== policy.program_manifest_digest
  ) {
    throw new Error("question-program record is detached from the trace policy");
  }
  parseLineList(record.coding_questions, "coding_questions");
  if (!new Set(["applicable", "inapplicable"]).has(record.reciprocal_applicability)) {
    throw new Error("reciprocal_applicability must be applicable or inapplicable");
  }
  if (record.reciprocal_applicability === "applicable") parsePairs(record.reciprocal_pairs);
  if (record.reciprocal_applicability === "inapplicable" && record.reciprocal_pairs !== "none") {
    throw new Error("an inapplicable reciprocal direction must record reciprocal_pairs=none");
  }
  if (record.question_program_check !== questionProgramCheck(record)) {
    throw new Error("question-program validation digest is missing or inconsistent");
  }
}

function collectLines(program, field) {
  const direct = program[field] ?? [];
  return field === "paired_sequence" ? direct.flat() : direct;
}

function validateQuestionProgram(record, root) {
  for (const field of REQUIRED_QUESTION_FIELDS) requireString(record, field);

  const sourcePath = "formal-successor/Questions.txt";
  const manifestPath = "formal-successor/ENGINEERING_QUESTION_PROGRAMS.json";
  const sourceBytes = readBytes(root, sourcePath);
  const manifestBytes = readBytes(root, manifestPath);
  const sourceDigest = sha256(sourceBytes);
  const manifestDigest = sha256(manifestBytes);
  if (record.source_digest !== sourceDigest) {
    throw new Error("source_digest does not match the active Questions.txt bytes");
  }
  if (record.program_manifest_digest !== manifestDigest) {
    throw new Error("program_manifest_digest does not match the active question-program manifest");
  }

  const manifest = JSON.parse(manifestBytes.toString("utf8"));
  if (manifest.source_sha256 !== sourceDigest) {
    throw new Error("question-program manifest is detached from Questions.txt");
  }
  if (record.program !== manifest.composition?.id) {
    throw new Error(`program must be the declared composition ${manifest.composition?.id ?? "<missing>"}`);
  }

  const questions = sourceBytes.toString("utf8").split(/\r?\n/u);
  const questionAt = (line) => questions[line - 1] ?? "";
  const codingPrograms = (manifest.programs ?? []).filter((program) =>
    program.id.startsWith("QP-CODING-"),
  );
  const allowedCoding = new Set(
    codingPrograms.flatMap((program) => [
      ...collectLines(program, "sequence"),
      ...collectLines(program, "paired_sequence"),
    ]),
  );
  for (const line of parseLineList(record.coding_questions, "coding_questions")) {
    if (!allowedCoding.has(line) || !questionAt(line).endsWith("?")) {
      throw new Error(`coding question line ${line} is not declared by a QP-CODING program`);
    }
  }

  if (!new Set(["applicable", "inapplicable"]).has(record.reciprocal_applicability)) {
    throw new Error("reciprocal_applicability must be applicable or inapplicable");
  }
  if (record.reciprocal_applicability === "inapplicable") {
    if (record.reciprocal_pairs !== "none") {
      throw new Error("an inapplicable reciprocal direction must record reciprocal_pairs=none");
    }
  } else {
    const reciprocalPrograms = (manifest.programs ?? []).filter((program) =>
      program.id.startsWith("QP-WHY-"),
    );
    const allowedPairs = new Set(
      reciprocalPrograms.flatMap((program) =>
        (program.paired_sequence ?? []).map(([left, right]) => `${left}:${right}`),
      ),
    );
    for (const [left, right] of parsePairs(record.reciprocal_pairs)) {
      const identity = `${left}:${right}`;
      if (!allowedPairs.has(identity)) {
        throw new Error(`reciprocal pair ${identity} is not a declared two-orientation pair`);
      }
      if (!questionAt(left).endsWith("?") || !questionAt(right).endsWith("?")) {
        throw new Error(`reciprocal pair ${identity} does not address two corpus questions`);
      }
    }
  }

  return questionProgramCheck(record);
}

function validatePolicy(record) {
  for (const field of ["question_program_schema", "source_digest", "program_manifest_digest"]) {
    requireString(record, field);
  }
  if (record.question_program_schema !== "1") {
    throw new Error("question_program_schema must be 1");
  }
  for (const field of ["source_digest", "program_manifest_digest"]) {
    if (!/^[0-9a-f]{64}$/u.test(record[field])) {
      throw new Error(`${field} must be a lowercase SHA-256 digest`);
    }
  }
}

module.exports = {
  REQUIRED_QUESTION_FIELDS,
  questionProgramCheck,
  validatePolicy,
  validateQuestionProgram,
  validateStoredQuestion,
};

if (require.main === module) {
  try {
    const [root, encoded] = process.argv.slice(2);
    if (!root || !encoded) throw new Error("usage: ic-question-program.js ROOT JSON_RECORD");
    const record = JSON.parse(encoded);
    process.stdout.write(`${validateQuestionProgram(record, path.resolve(root))}\n`);
  } catch (error) {
    process.stderr.write(`ic-question-program: ${error.message}\n`);
    process.exitCode = 1;
  }
}
