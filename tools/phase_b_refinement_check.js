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

function inspect(surface, schema, classification, schemaBytes, classificationBytes, refinement) {
  const errors = [];
  const indexed = new Map(classification.records.map((record) => [record.source_id, record]));
  if (surface.status !== "generated_phase_b_refinement_surface_not_successor_semantics") errors.push("status");
  if (
    surface.generated_from?.schema_sha256 !== digest(schemaBytes) ||
    surface.generated_from?.tex_classification_sha256 !== digest(classificationBytes) ||
    surface.generated_from?.refinement_module_sha256 !== digest(refinement)
  ) errors.push("digest");
  if (JSON.stringify(surface.records?.map((record) => record.source_id)) !== JSON.stringify(schema.sources)) errors.push("sources");
  if (new Set(surface.records?.map((record) => record.source_id)).size !== schema.sources.length) errors.push("duplicate-source");
  for (const record of surface.records ?? []) {
    const source = indexed.get(record.source_id);
    if (!source || record.disposition !== source.disposition || JSON.stringify(record.source) !== JSON.stringify(source.source)) {
      errors.push(`ancestry:${record.source_id}`);
    }
  }
  if (
    surface.coverage?.explicit_definitions !== 4 || surface.coverage?.obligations !== 1 ||
    surface.next_residual !== "FORMAL-B-FORMULA-GRAMMAR" || surface.formal_gate_b?.status !== "PENDING"
  ) errors.push("boundary");
  for (const declaration of schema.required_declarations) {
    if (!new RegExp(`\\b(?:structure|def|inductive)\\s+${declaration}\\b`).test(refinement)) errors.push(`missing:${declaration}`);
  }
  for (const obligation of schema.obligations) {
    if (!new RegExp(`\\|\\s+${obligation}\\b`).test(refinement)) errors.push(`obligation:${obligation}`);
  }
  if (!/holds _ _ := True/.test(refinement) || !/structure\s+ExistenceBoundary/.test(refinement) ||
      !/E\.actual x ∧ E\.actual y/.test(refinement) || !/S\.holds x y → R\.holds x y/.test(refinement) ||
      !/proper\s*:\s*∃ x y/.test(refinement)) errors.push("coarse-refinement-boundary");
  const executable = refinement.replace(/\/-[\s\S]*?-\//g, "");
  if (/\b(?:String|axiom|sorry|admit|unsafe|abbrev)\b/.test(executable) ||
      /\b(?:Formula|Question|Probe|Program|Rust|Standing)\b/.test(executable)) errors.push("escape");
  return errors;
}

function mustReject(label, values) {
  assert.notEqual(inspect(...values).length, 0, `${label} escaped the independent refinement checker`);
}

try {
  const compile = process.argv.includes("--compile");
  const schema = readJson("formal-successor/PHASE_B_REFINEMENT_SCHEMA.json");
  const surface = readJson("formal-successor/PHASE_B_REFINEMENT_SURFACE.json");
  const classification = readJson("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json");
  const refinement = fs.readFileSync(path.join(root, schema.value.inputs.refinement_module), "utf8");
  const base = [surface.value, schema.value, classification.value, schema.bytes, classification.bytes, refinement];
  const errors = inspect(...base);
  if (errors.length) throw new Error(errors.join(", "));
  if (compile) childProcess.execFileSync("lake", ["env", "lean", "InquiryCalculus/Legacy/V20/Refinement.lean"], {
    cwd: path.join(root, "formal"), stdio: "pipe",
  });
  const mutations = [];
  const mutate = (label, index, action) => {
    const values = base.map((value, current) => current < 3 ? copy(value) : value);
    action(values[index], values);
    mutations.push([label, values]);
  };
  mutate("source loss", 0, (value) => value.records.pop());
  mutate("source promotion", 0, (value) => value.records[4].disposition = "FormalDefinition");
  mutate("Gate B promotion", 0, (value) => value.formal_gate_b.status = "PASS");
  mutate("coarse carrier loss", 5, (_, values) => values[5] = values[5].replace("structure RepresentedRelation", "structure RepresentedRelationRemoved"));
  mutate("universal collapse", 5, (_, values) => values[5] = values[5].replace("holds _ _ := True", "holds x y := x = y"));
  mutate("existence boundary loss", 5, (_, values) => values[5] = values[5].replace("structure ExistenceBoundary", "structure ExistenceBoundaryRemoved"));
  mutate("global coexistence", 5, (_, values) => values[5] = values[5].replace("E.actual x ∧ E.actual y", "True"));
  mutate("order inversion", 5, (_, values) => values[5] = values[5].replace("S.holds x y → R.holds x y", "R.holds x y → S.holds x y"));
  mutate("properness loss", 5, (_, values) => values[5] = values[5].replace("proper : ∃ x y", "proper : True -- ∃ x y"));
  mutate("obligation promotion", 5, (_, values) => values[5] = values[5].replace("| noVacuityFromUniversalRelatedness", "def noVacuityFromUniversalRelatedness : Prop := True"));
  mutate("question leakage", 5, (_, values) => values[5] += "\nstructure QuestionLeak where token : Nat\n");
  mutate("axiom completion", 5, (_, values) => values[5] += "\naxiom refinementGap : Prop\n");
  for (const [label, values] of mutations) mustReject(label, values);
  process.stdout.write(`independent Phase B refinement checks passed (${mutations.length}/${mutations.length}; compile ${compile ? "checked" : "delegated"})\n`);
} catch (error) {
  process.stderr.write(`Phase B refinement check: ${error.message}\n`);
  process.exit(1);
}
