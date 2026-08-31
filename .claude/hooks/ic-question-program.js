#!/usr/bin/env node
"use strict";

// Validation and rendering for the compact inquiry-spine contract. This file
// has no rhythm, scheduler, residual-class dispatch, or historical policy path.

const crypto = require("node:crypto");
const path = require("node:path");
const contractLoader = require("./ic-contract.js");
const instances = require("./ic-question-instance.js");
const recursiveGenerator = require("./ic-recursive-generator.js");

const ASK_FIELDS = [
  "question_form", "rendering", "source_lines", "generator_ids",
  "reciprocal_relations", "context", "source_digest", "program_manifest_digest",
];

function sha256(bytes) { return crypto.createHash("sha256").update(bytes).digest("hex"); }
function requireString(record, field) {
  if (typeof record[field] !== "string" || !record[field].trim()) {
    throw new Error(`inquiry-spine record requires nonempty ${field}`);
  }
}
function parseCsv(value, label, allowNone = false) {
  if (allowNone && value === "none") return [];
  const entries = value.split(",");
  if (entries.some((entry) => !entry || entry.trim() !== entry) || new Set(entries).size !== entries.length) {
    throw new Error(`${label} must be a unique unpadded CSV list`);
  }
  return entries;
}
function parseLines(value) {
  if (!/^\d+(?:,\d+)*$/u.test(value)) throw new Error("source_lines must be positive CSV integers");
  const lines = value.split(",").map(Number);
  if (new Set(lines).size !== lines.length) throw new Error("source_lines must be distinct");
  return lines;
}
function exactCsv(record, field, expected, allowNone = false) {
  const actual = parseCsv(record[field], field, allowNone);
  if (JSON.stringify(actual) !== JSON.stringify(expected)) {
    throw new Error(`${field} must exactly report ${expected.join(",") || "none"}`);
  }
}

function questionProgramCheck(record) {
  return sha256(Buffer.from(`ic-inquiry-spine-v5\0${ASK_FIELDS.map((key) => `${key}=${record[key]}`).join("\0")}`));
}

function activeInputs(root) {
  const loaded = contractLoader.read(root);
  return {
    sourceBytes: loaded.corpusBytes,
    sourceDigest: loaded.corpusDigest,
    manifestDigest: loaded.contractDigest,
    manifest: loaded.contract,
    contract: loaded.contract,
  };
}

function validatePolicy(record) {
  if (record.question_program_schema !== "5") throw new Error("only inquiry-spine policy schema 5 is active");
  for (const field of ["source_digest", "program_manifest_digest"]) {
    requireString(record, field);
    if (!/^[0-9a-f]{64}$/u.test(record[field])) throw new Error(`${field} must be SHA-256`);
  }
  if (record.fold_evidence_schema !== undefined && record.fold_evidence_schema !== "2") {
    throw new Error("inquiry spine requires fold evidence schema 2");
  }
}

function validateStoredQuestion(record, policy) {
  validatePolicy(policy);
  for (const field of ASK_FIELDS) requireString(record, field);
  if (record.source_digest !== policy.source_digest ||
      record.program_manifest_digest !== policy.program_manifest_digest) {
    throw new Error("Ask is detached from its pinned inquiry-spine policy");
  }
  parseLines(record.source_lines);
  if (record.question_program_check !== questionProgramCheck(record)) {
    throw new Error("Ask inquiry-spine validation digest is inconsistent");
  }
}

function validateQuestionProgram(record, root) {
  const { sourceBytes, sourceDigest, manifestDigest, contract } = activeInputs(root);
  for (const field of ASK_FIELDS) requireString(record, field);
  if (record.source_digest !== sourceDigest || record.program_manifest_digest !== manifestDigest) {
    throw new Error("Ask does not pin the current corpus and inquiry-spine contract");
  }
  const forms = contractLoader.formMap(contract);
  const form = forms.get(record.question_form);
  if (!form) throw new Error(`undeclared question form ${record.question_form}`);
  if (record.rendering !== `RENDER-${record.question_form}` &&
      !/^RI-[0-9a-f]{64}$/u.test(record.rendering) && !/^RG-[0-9a-f]{64}$/u.test(record.rendering)) {
    throw new Error("rendering does not preserve form/occurrence identity");
  }
  exactCsv(record, "source_lines", form.source_lines.map(String));
  const corpus = sourceBytes.toString("utf8").split(/\r?\n/u);
  for (const line of form.source_lines) {
    if (!corpus[line - 1]?.endsWith("?")) throw new Error(`corpus anchor ${line} is not a question`);
  }
  const generators = contract.generator_registry
    .filter((generator) => generator.question_forms.includes(record.question_form)).map((generator) => generator.id);
  exactCsv(record, "generator_ids", generators);
  parseCsv(record.reciprocal_relations, "reciprocal_relations", true);
  return questionProgramCheck(record);
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
  })));
}

function validateStoredField(record) {
  for (const field of ["field_id", "members", "basis", "coverage", "regenerated_from", "dispositions", "removal_evidence"]) {
    requireString(record, field);
  }
  let members, dispositions, removalEvidence;
  try {
    members = JSON.parse(record.members);
    dispositions = JSON.parse(record.dispositions);
    removalEvidence = JSON.parse(record.removal_evidence);
  } catch (error) { throw new Error(`field JSON is invalid: ${error.message}`); }
  if (!Array.isArray(members)) throw new Error("field members must be an array");
  for (const value of [dispositions, removalEvidence]) {
    if (!value || Array.isArray(value) || typeof value !== "object") throw new Error("field maps must be objects");
  }
  if (record.field_check !== fieldRecordCheck(record)) throw new Error("field digest is inconsistent");
  return { members, dispositions, removalEvidence };
}

function expectedGenerators(contract, formId) {
  return contract.generator_registry.filter((entry) => entry.question_forms.includes(formId)).map((entry) => entry.id);
}

function validateFieldRecord(record, root) {
  const { members } = validateStoredField({ ...record, field_check: fieldRecordCheck(record) });
  const { contract } = activeInputs(root);
  const forms = contractLoader.formMap(contract);
  const occurrences = new Set();
  for (const member of members) {
    if (!member || Array.isArray(member) || typeof member !== "object") throw new Error("field member must be an object");
    for (const key of ["occurrence", "question_form", "rendering", "prompt", "context", "path", "disposition"]) {
      if (typeof member[key] !== "string" || !member[key]) throw new Error(`field member requires ${key}`);
    }
    if (occurrences.has(member.occurrence)) throw new Error(`duplicate field occurrence ${member.occurrence}`);
    occurrences.add(member.occurrence);
    const form = forms.get(member.question_form);
    if (!form) throw new Error(`undeclared field form ${member.question_form}`);
    if (member.relational_instance) {
      if (member.rendering !== `RI-${instances.renderingIdentity(member)}` ||
          member.prompt !== instances.render(form.prompt, member.relational_instance)) {
        throw new Error(`${member.occurrence}: relational rendering changed`);
      }
    } else if (member.derivation) recursiveGenerator.validateRendering(member, contract);
    else if (member.rendering !== `RENDER-${member.question_form}` || member.prompt !== form.prompt) {
      throw new Error(`${member.occurrence}: direct rendering changed`);
    }
    if (JSON.stringify(member.source_lines) !== JSON.stringify(form.source_lines) ||
        JSON.stringify(member.generator_ids) !== JSON.stringify(expectedGenerators(contract, member.question_form))) {
      throw new Error(`${member.occurrence}: corpus or generator ancestry changed`);
    }
    if (!Array.isArray(member.dependencies) || typeof member.executable !== "boolean") {
      throw new Error(`${member.occurrence}: malformed execution dependencies`);
    }
    if (!contract.lifecycle.live_dispositions.includes(member.disposition)) {
      throw new Error(`${member.occurrence}: disposition is not live`);
    }
  }
  return fieldRecordCheck(record);
}

function validateReifiedSeeds(record, root, policy) {
  const products = JSON.parse(record.products);
  const relevant = products.filter((product) => product.inquiry_seed || product.inquiry_generator_surface);
  if (!relevant.length) return;
  const { contract, manifestDigest, sourceDigest } = activeInputs(root);
  if (policy?.question_program_schema !== "5" || policy.program_manifest_digest !== manifestDigest ||
      policy.source_digest !== sourceDigest) throw new Error("reified inquiry inputs require active schema-5 policy");
  const forms = new Set(contract.question_forms.map((form) => form.id));
  for (const product of relevant) {
    if (product.inquiry_seed && !forms.has(product.inquiry_seed.question_form)) {
      throw new Error("inquiry seed uses an undeclared corpus form");
    }
  }
}

function renderQuestionProgram(root) {
  const { sourceDigest, manifestDigest, contract } = activeInputs(root);
  const forms = contractLoader.formMap(contract);
  const lines = [
    "INQUIRY SPINE — DERIVED MACHINE CONTRACT",
    `recurrence: ${contract.model_recurrence.join(" -> ")} -> RELATE`,
    `corpus_sha256: ${sourceDigest}`,
    `contract_sha256: ${manifestDigest}`,
    "Generators derive ordinary questions from represented relations; they do not schedule them:",
  ];
  for (const generator of contract.generator_registry) {
    lines.push(`  ${generator.id} requires ${generator.requires.join(" + ")}`);
    for (const id of generator.question_forms) lines.push(`    ${id}: ${forms.get(id).prompt}`);
  }
  lines.push("Selection uses only represented executability/dependency/Frontier relations; every unchosen occurrence remains live.");
  return `${lines.join("\n")}\n`;
}

function renderQuestionSummary(root) {
  const { sourceDigest, manifestDigest, contract } = activeInputs(root);
  return [
    "ONE INQUIRY SPINE",
    `recurrence: ${contract.model_recurrence.join(" -> ")} -> RELATE`,
    `forms: ${contract.question_forms.length}; generators: ${contract.generator_registry.map((x) => x.id).join(",")}`,
    "lifecycle is evidence protocol beneath RETURN, not a second reasoning loop",
    "selection: represented executable frontier only; no fixed rhythm, semantic scheduler, or residual-to-method dispatch",
    `policy: Questions=${sourceDigest.slice(0, 16)} contract=${manifestDigest.slice(0, 16)}`,
    "",
  ].join("\n");
}

module.exports = {
  ASK_FIELDS, activeInputs, fieldRecordCheck, questionProgramCheck,
  renderQuestionProgram, renderQuestionSummary, validatePolicy,
  validateFieldRecord, validateReifiedSeeds,
  validateQuestionProgram, validateStoredQuestion, validateStoredField,
};

if (require.main === module) {
  try {
    const [command, root, encoded] = process.argv.slice(2);
    if (command === "projection" && root && !encoded) process.stdout.write(renderQuestionProgram(path.resolve(root)));
    else if (command === "summary" && root && !encoded) process.stdout.write(renderQuestionSummary(path.resolve(root)));
    else if (command === "validate" && root && encoded) {
      process.stdout.write(`${validateQuestionProgram(JSON.parse(encoded), path.resolve(root))}\n`);
    } else throw new Error("usage: ic-question-program.js projection|summary ROOT | validate ROOT JSON_RECORD");
  } catch (error) {
    process.stderr.write(`ic-question-program: ${error.message}\n`);
    process.exitCode = 1;
  }
}
