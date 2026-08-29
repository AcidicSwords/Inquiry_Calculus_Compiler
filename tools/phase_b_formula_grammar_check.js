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

function inspect(surface, schema, classification, schemaBytes, classificationBytes, formula) {
  const errors = [];
  const indexed = new Map(classification.records.map((record) => [record.source_id, record]));
  if (surface.status !== "generated_phase_b_formula_grammar_surface_not_successor_semantics") errors.push("status");
  if (
    surface.generated_from?.schema_sha256 !== digest(schemaBytes) ||
    surface.generated_from?.tex_classification_sha256 !== digest(classificationBytes) ||
    surface.generated_from?.formula_module_sha256 !== digest(formula)
  ) errors.push("digest");
  if (JSON.stringify(surface.records?.map((record) => record.source_id)) !== JSON.stringify(schema.sources)) errors.push("sources");
  if (new Set(surface.records?.map((record) => record.source_id)).size !== schema.sources.length) errors.push("duplicate-source");
  for (const record of surface.records ?? []) {
    const source = indexed.get(record.source_id);
    if (!source || record.disposition !== "LegacyObligation" ||
        record.obligation_status !== source.legacy_obligation?.status || JSON.stringify(record.source) !== JSON.stringify(source.source)) {
      errors.push(`ancestry:${record.source_id}`);
    }
  }
  if (
    surface.coverage?.explicit_definitions !== 0 || surface.coverage?.obligations !== 6 ||
    surface.coverage?.obligation_statuses?.filter((status) => status === "Ambiguous").length !== 5 ||
    surface.coverage?.obligation_statuses?.filter((status) => status === "Unproved").length !== 1 ||
    surface.next_residual !== "FORMAL-B-MINIMAL-LOGICAL-BASIS" || surface.formal_gate_b?.status !== "PENDING"
  ) errors.push("boundary");
  for (const declaration of schema.required_declarations) {
    if (!new RegExp(`\\b(?:structure|def|inductive)\\s+${declaration}\\b`).test(formula)) errors.push(`missing:${declaration}`);
  }
  for (const obligation of schema.obligations) {
    if (!new RegExp(`\\|\\s+${obligation}\\b`).test(formula)) errors.push(`obligation:${obligation}`);
  }
  if (!/inductive\s+CandidateFormulaSyntax/.test(formula) ||
      !/\|\s+logicalNot\s*:/.test(formula) || !/\|\s+exists\s*:/.test(formula) ||
      !/\|\s+forall\s*:/.test(formula) || !/B\.relationToken\s*→\s*List/.test(formula) ||
      !/CandidateFormulaSyntax B I\)\s*:\s*phi = phi/.test(formula)) errors.push("candidate-boundary");
  const executable = formula.replace(/\/-[\s\S]*?-\//g, "");
  if (/\b(?:String|axiom|sorry|admit|unsafe|abbrev)\b/.test(executable) ||
      /\b(?:NegationUse|Coverage|Departure|Question|Probe|Program|Standing|Rust)\b/.test(executable)) errors.push("escape");
  return errors;
}

function mustReject(label, values) {
  assert.notEqual(inspect(...values).length, 0, `${label} escaped the independent formula grammar checker`);
}

try {
  const compile = process.argv.includes("--compile");
  const schema = readJson("formal-successor/PHASE_B_FORMULA_GRAMMAR_SCHEMA.json");
  const surface = readJson("formal-successor/PHASE_B_FORMULA_GRAMMAR_SURFACE.json");
  const classification = readJson("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json");
  const formula = fs.readFileSync(path.join(root, schema.value.inputs.formula_module), "utf8");
  const base = [surface.value, schema.value, classification.value, schema.bytes, classification.bytes, formula];
  const errors = inspect(...base);
  if (errors.length) throw new Error(errors.join(", "));
  if (compile) childProcess.execFileSync("lake", ["env", "lean", "InquiryCalculus/Legacy/V20/FormulaGrammar.lean"], {
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
  mutate("Unproved erasure", 0, (value) => value.records[3].obligation_status = "Ambiguous");
  mutate("Gate B promotion", 0, (value) => value.formal_gate_b.status = "PASS");
  mutate("candidate syntax loss", 5, (_, values) => values[5] = values[5].replace("inductive CandidateFormulaSyntax", "inductive CandidateFormulaSyntaxRemoved"));
  mutate("negation loss", 5, (_, values) => values[5] = values[5].replace("| logicalNot", "| logicalNotRemoved"));
  mutate("quantifier loss", 5, (_, values) => values[5] = values[5].replace("| forall", "| forallRemoved"));
  mutate("relation-argument loss", 5, (_, values) => values[5] = values[5].replace("B.relationToken → List (CandidateTerm B I)", "B.relationToken"));
  mutate("obligation loss", 5, (_, values) => values[5] = values[5].replace("| logicalNegationSeparation", "| logicalNegationSeparationRemoved"));
  mutate("host Prop collapse", 5, (_, values) => values[5] += "\ndef formulaMeaning : CandidateFormulaSyntax B I → Prop := fun _ => True\n");
  mutate("string syntax", 5, (_, values) => values[5] += "\ndef formulaTag : String := \"x\"\n");
  mutate("negation-use leakage", 5, (_, values) => values[5] += "\nstructure NegationUseLeak where token : Nat\n");
  mutate("axiom completion", 5, (_, values) => values[5] += "\naxiom formulaGap : Prop\n");
  for (const [label, values] of mutations) mustReject(label, values);
  process.stdout.write(`independent Phase B formula grammar checks passed (${mutations.length}/${mutations.length}; compile ${compile ? "checked" : "delegated"})\n`);
} catch (error) {
  process.stderr.write(`Phase B formula grammar check: ${error.message}\n`);
  process.exit(1);
}
