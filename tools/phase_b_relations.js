#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const artifact = path.join(root, "formal-successor", "PHASE_B_RELATIONS_SURFACE.json");
const digest = (value) => crypto.createHash("sha256").update(value).digest("hex");
const read = (relative) => {
  const bytes = fs.readFileSync(path.join(root, relative));
  return { bytes, value: JSON.parse(bytes) };
};

function generate() {
  const schema = read("formal-successor/PHASE_B_RELATIONS_SCHEMA.json");
  const classification = read(schema.value.inputs.tex_classification_path);
  const byId = new Map(classification.value.records.map((record) => [record.source_id, record]));
  const records = schema.value.sources.map((sourceId) => {
    const record = byId.get(sourceId);
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
    status: "generated_phase_b_relations_surface_not_successor_semantics",
    generated_from: {
      schema_sha256: digest(schema.bytes),
      tex_classification_sha256: digest(classification.bytes),
      relations_module_sha256: digest(fs.readFileSync(path.join(root, schema.value.inputs.relations_module))),
    },
    records,
    coverage: {
      explicit_definitions: records.filter((record) => record.disposition === "FormalDefinition").length,
      obligations: records.filter((record) => record.disposition === "LegacyObligation").length,
    },
    next_residual: schema.value.next_residual,
    formal_gate_b: schema.value.gate_b,
  };
}

try {
  const expected = `${JSON.stringify(generate(), null, 2)}\n`;
  if (process.argv[2] === "generate") {
    fs.writeFileSync(artifact, expected);
    process.stdout.write("generated Phase B relations surface\n");
  } else if (process.argv[2] === "check" && fs.readFileSync(artifact, "utf8") === expected) {
    process.stdout.write("Phase B relations surface regenerates exactly\n");
  } else {
    throw new Error("relations surface does not exactly regenerate");
  }
} catch (error) {
  process.stderr.write(`Phase B relations: ${error.message}\n`);
  process.exit(1);
}
