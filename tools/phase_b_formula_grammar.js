#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const output = path.join(root, "formal-successor", "PHASE_B_FORMULA_GRAMMAR_SURFACE.json");
const digest = (value) => crypto.createHash("sha256").update(value).digest("hex");
const read = (relative) => {
  const bytes = fs.readFileSync(path.join(root, relative));
  return { bytes, value: JSON.parse(bytes) };
};

function surface() {
  const schema = read("formal-successor/PHASE_B_FORMULA_GRAMMAR_SCHEMA.json");
  const classification = read(schema.value.inputs.tex_classification_path);
  const indexed = new Map(classification.value.records.map((record) => [record.source_id, record]));
  const records = schema.value.sources.map((sourceId) => {
    const record = indexed.get(sourceId);
    if (!record) throw new Error(`missing classified source ${sourceId}`);
    return {
      source_id: sourceId,
      disposition: record.disposition,
      destination: record.destination,
      source: record.source,
      obligation_status: record.legacy_obligation?.status ?? null,
    };
  });
  return {
    schema: 1,
    status: "generated_phase_b_formula_grammar_surface_not_successor_semantics",
    generated_from: {
      schema_sha256: digest(schema.bytes),
      tex_classification_sha256: digest(classification.bytes),
      formula_module_sha256: digest(fs.readFileSync(path.join(root, schema.value.inputs.formula_module))),
    },
    records,
    coverage: {
      explicit_definitions: records.filter((record) => record.disposition === "FormalDefinition").length,
      obligations: records.filter((record) => record.disposition === "LegacyObligation").length,
      obligation_statuses: records.map((record) => record.obligation_status),
    },
    next_residual: schema.value.next_residual,
    formal_gate_b: schema.value.gate_b,
  };
}

try {
  const expected = `${JSON.stringify(surface(), null, 2)}\n`;
  if (process.argv[2] === "generate") {
    fs.writeFileSync(output, expected);
    process.stdout.write("generated Phase B formula grammar surface\n");
  } else if (process.argv[2] === "check" && fs.readFileSync(output, "utf8") === expected) {
    process.stdout.write("Phase B formula grammar surface regenerates exactly\n");
  } else {
    throw new Error("formula grammar surface does not exactly regenerate");
  }
} catch (error) {
  process.stderr.write(`Phase B formula grammar: ${error.message}\n`);
  process.exit(1);
}
