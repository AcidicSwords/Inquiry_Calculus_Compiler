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

function inspect(surface, schema, classification, schemaBytes, classificationBytes, relations) {
  const errors = [];
  const classified = new Map(classification.records.map((record) => [record.source_id, record]));
  if (surface.status !== "generated_phase_b_relations_surface_not_successor_semantics") errors.push("status");
  if (
    surface.generated_from?.schema_sha256 !== digest(schemaBytes) ||
    surface.generated_from?.tex_classification_sha256 !== digest(classificationBytes) ||
    surface.generated_from?.relations_module_sha256 !== digest(relations)
  ) errors.push("digest");
  if (JSON.stringify(surface.records?.map((record) => record.source_id)) !== JSON.stringify(schema.sources)) {
    errors.push("sources");
  }
  if (new Set(surface.records?.map((record) => record.source_id)).size !== schema.sources.length) errors.push("duplicate-source");
  for (const record of surface.records ?? []) {
    const source = classified.get(record.source_id);
    if (!source || record.disposition !== source.disposition || JSON.stringify(record.source) !== JSON.stringify(source.source)) {
      errors.push(`ancestry:${record.source_id}`);
    }
  }
  if (
    surface.coverage?.explicit_definitions !== 7 || surface.coverage?.obligations !== 0 ||
    surface.next_residual !== "FORMAL-B-COARSE-RELATION-REFINEMENT" || surface.formal_gate_b?.status !== "PENDING"
  ) errors.push("boundary");
  for (const declaration of schema.required_declarations) {
    if (!new RegExp(`\\b(?:structure|def|inductive)\\s+${declaration}\\b`).test(relations)) errors.push(`missing:${declaration}`);
  }
  if (!/structure\s+Relation\s+\(B\s*:\s*Binding\)/.test(relations) ||
      !/domain codomain\s*:\s*AdmittedType B I/.test(relations) ||
      !/structure\s+RelationSchema/.test(relations) ||
      !/structure\s+ConverseBoundary/.test(relations) ||
      !/defined\s*:\s*\{domain codomain/.test(relations)) errors.push("typed-partial-boundary");
  const executable = relations.replace(/\/-[\s\S]*?-\//g, "");
  if (/\b(?:String|axiom|sorry|admit|unsafe|abbrev)\b/.test(executable) ||
      /\b(?:Formula|Question|Fiber|Probe|Program|Standing)\b/.test(executable)) errors.push("escape");
  return errors;
}

function mustReject(label, values) {
  assert.notEqual(inspect(...values).length, 0, `${label} escaped the independent relations checker`);
}

try {
  const compile = process.argv.includes("--compile");
  const schema = readJson("formal-successor/PHASE_B_RELATIONS_SCHEMA.json");
  const surface = readJson("formal-successor/PHASE_B_RELATIONS_SURFACE.json");
  const classification = readJson("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json");
  const relations = fs.readFileSync(path.join(root, schema.value.inputs.relations_module), "utf8");
  const base = [surface.value, schema.value, classification.value, schema.bytes, classification.bytes, relations];
  const errors = inspect(...base);
  if (errors.length) throw new Error(errors.join(", "));
  if (compile) childProcess.execFileSync("lake", ["env", "lean", "InquiryCalculus/Legacy/V20/Relations.lean"], {
    cwd: path.join(root, "formal"), stdio: "pipe",
  });
  const mutations = [];
  const mutate = (label, index, action) => {
    const values = base.map((value, current) => current < 3 ? copy(value) : value);
    action(values[index], values);
    mutations.push([label, values]);
  };
  mutate("source loss", 0, (value) => value.records.pop());
  mutate("source promotion", 0, (value) => value.records[0].disposition = "LegacyObligation");
  mutate("Gate B promotion", 0, (value) => value.formal_gate_b.status = "PASS");
  mutate("relation loss", 5, (_, values) => values[5] = values[5].replace("structure Relation", "structure RelationRemoved"));
  mutate("endpoint erasure", 5, (_, values) => values[5] = values[5].replace("domain codomain : AdmittedType B I", "domain codomain : Nat"));
  mutate("schema loss", 5, (_, values) => values[5] = values[5].replace("structure RelationSchema", "structure RelationSchemaRemoved"));
  mutate("function collapse", 5, (_, values) => values[5] = values[5].replace("totalSingleValued : relation.isFunctional", "totalSingleValued : True"));
  mutate("converse totalization", 5, (_, values) => values[5] = values[5].replace("defined : {domain codomain", "defined : {domain codomain" ).replace("→ Relation B I domain codomain → Prop", "→ Relation B I domain codomain → True"));
  mutate("formula leakage", 5, (_, values) => values[5] += "\nstructure FormulaLeak where token : Nat\n");
  mutate("string collapse", 5, (_, values) => values[5] += "\ndef relationTag : String := \"x\"\n");
  mutate("axiom completion", 5, (_, values) => values[5] += "\naxiom relationGap : Prop\n");
  for (const [label, values] of mutations) mustReject(label, values);
  process.stdout.write(`independent Phase B relations checks passed (${mutations.length}/${mutations.length}; compile ${compile ? "checked" : "delegated"})\n`);
} catch (error) {
  process.stderr.write(`Phase B relations check: ${error.message}\n`);
  process.exit(1);
}
