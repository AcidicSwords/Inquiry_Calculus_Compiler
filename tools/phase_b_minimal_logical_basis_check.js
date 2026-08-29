#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const digest = (value) => crypto.createHash("sha256").update(value).digest("hex");
const readJson = (relative) => {
  const bytes = fs.readFileSync(path.join(root, relative));
  return { bytes, value: JSON.parse(bytes) };
};
const copy = (value) => JSON.parse(JSON.stringify(value));

function inspect(surface, schema, classification, schemaBytes, classificationBytes, basis) {
  const errors = [];
  const indexed = new Map(classification.records.map((record) => [record.source_id, record]));
  if (surface.status !== "generated_phase_b_minimal_logical_basis_surface_not_successor_semantics") errors.push("status");
  if (
    surface.generated_from?.schema_sha256 !== digest(schemaBytes) ||
    surface.generated_from?.tex_classification_sha256 !== digest(classificationBytes) ||
    surface.generated_from?.basis_module_sha256 !== digest(basis)
  ) errors.push("digest");
  if (JSON.stringify(surface.records?.map((record) => record.source_id)) !== JSON.stringify(schema.sources)) errors.push("sources");
  if (new Set(surface.records?.map((record) => record.source_id)).size !== schema.sources.length) errors.push("duplicate-source");
  for (const record of surface.records ?? []) {
    const source = indexed.get(record.source_id);
    if (!source || record.disposition !== "LegacyObligation" || record.obligation_status !== "Ambiguous" ||
        JSON.stringify(record.source) !== JSON.stringify(source.source)) errors.push(`ancestry:${record.source_id}`);
  }
  if (
    surface.coverage?.explicit_definitions !== 0 || surface.coverage?.obligations !== 7 ||
    surface.coverage?.obligation_statuses?.every((status) => status === "Ambiguous") !== true ||
    surface.next_residual !== "FORMAL-B-RELATION-EXPRESSION-IR" || surface.formal_gate_b?.status !== "PENDING"
  ) errors.push("boundary");
  for (const declaration of schema.required_declarations) {
    if (!new RegExp(`\\b(?:structure|def|inductive)\\s+${declaration}\\b`).test(basis)) errors.push(`missing:${declaration}`);
  }
  for (const obligation of schema.obligations) {
    if (!new RegExp(`\\|\\s+${obligation}\\b`).test(basis)) errors.push(`obligation:${obligation}`);
  }
  if (!/\[\.truth, \.equality, \.conjunction, \.existential, \.logicalNot\]/.test(basis) ||
      !/structure\s+NativeComplementBoundary/.test(basis) || !/\.logicalNot \(\.and/.test(basis) ||
      !/\.logicalNot \(\.exists/.test(basis) || !/deriveOr \(\.logicalNot phi\) psi/.test(basis)) errors.push("candidate-boundary");
  const executable = basis.replace(/\/-[\s\S]*?-\//g, "");
  if (/\b(?:String|axiom|sorry|admit|unsafe|abbrev)\b/.test(executable) ||
      /\b(?:NegationUse|Coverage|Departure|Question|Probe|Program|Standing|Rust)\b/.test(executable)) errors.push("escape");
  return errors;
}

function mustReject(label, values) {
  assert.notEqual(inspect(...values).length, 0, `${label} escaped the independent minimal logical basis checker`);
}

try {
  const compile = process.argv.includes("--compile");
  const schema = readJson("formal-successor/PHASE_B_MINIMAL_LOGICAL_BASIS_SCHEMA.json");
  const surface = readJson("formal-successor/PHASE_B_MINIMAL_LOGICAL_BASIS_SURFACE.json");
  const classification = readJson("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json");
  const basis = fs.readFileSync(path.join(root, schema.value.inputs.basis_module), "utf8");
  const base = [surface.value, schema.value, classification.value, schema.bytes, classification.bytes, basis];
  const errors = inspect(...base);
  if (errors.length) throw new Error(errors.join(", "));
  if (compile) childProcess.execFileSync("lake", ["env", "lean", "InquiryCalculus/Legacy/V20/MinimalLogicalBasis.lean"], {
    cwd: path.join(root, "formal"), stdio: "pipe",
  });
  const mutations = [];
  const mutate = (label, index, action) => {
    const values = base.map((value, current) => current < 3 ? copy(value) : value);
    action(values[index], values);
    mutations.push([label, values]);
  };
  mutate("source loss", 0, (value) => value.records.pop());
  mutate("source promotion", 0, (value) => value.records[0].disposition = "FormalDefinition");
  mutate("ambiguity erasure", 0, (value) => value.records[0].obligation_status = "Unproved");
  mutate("Gate B promotion", 0, (value) => value.formal_gate_b.status = "PASS");
  mutate("basis loss", 5, (_, values) => values[5] = values[5].replace("inductive ReferenceLogicalBasisToken", "inductive ReferenceLogicalBasisTokenRemoved"));
  mutate("basis member loss", 5, (_, values) => values[5] = values[5].replace(".logicalNot]", "]"));
  mutate("native complement boundary loss", 5, (_, values) => values[5] = values[5].replace("structure NativeComplementBoundary", "structure NativeComplementBoundaryRemoved"));
  mutate("disjunction derivation loss", 5, (_, values) => values[5] = values[5].replace("def deriveOr", "def deriveOrRemoved"));
  mutate("universal derivation loss", 5, (_, values) => values[5] = values[5].replace("def deriveForall", "def deriveForallRemoved"));
  mutate("implication derivation loss", 5, (_, values) => values[5] = values[5].replace("def deriveImplies", "def deriveImpliesRemoved"));
  mutate("obligation loss", 5, (_, values) => values[5] = values[5].replace("| nativeComplementLimitation", "| nativeComplementLimitationRemoved"));
  mutate("global complement", 5, (_, values) => values[5] += "\ndef everyBindingHasComplement : Prop := True\n");
  mutate("oriented negation leakage", 5, (_, values) => values[5] += "\nstructure NegationUseLeak where token : Nat\n");
  mutate("axiom completion", 5, (_, values) => values[5] += "\naxiom basisGap : Prop\n");
  for (const [label, values] of mutations) mustReject(label, values);
  process.stdout.write(`independent Phase B minimal logical basis checks passed (${mutations.length}/${mutations.length}; compile ${compile ? "checked" : "delegated"})\n`);
} catch (error) {
  process.stderr.write(`Phase B minimal logical basis check: ${error.message}\n`);
  process.exit(1);
}
