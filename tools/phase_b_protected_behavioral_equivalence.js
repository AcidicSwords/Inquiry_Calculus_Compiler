#!/usr/bin/env node
"use strict";

const fs = require("node:fs");
const path = require("node:path");
const crypto = require("node:crypto");
const root = path.resolve(__dirname, "..");
const output = path.join(root, "formal-successor", "PHASE_B_PROTECTED_BEHAVIORAL_EQUIVALENCE_SURFACE.json");
const hash = (bytes) => crypto.createHash("sha256").update(bytes).digest("hex");

function generate() {
  const schemaBytes = fs.readFileSync(path.join(root, "formal-successor", "PHASE_B_PROTECTED_BEHAVIORAL_EQUIVALENCE_SCHEMA.json"));
  const schema = JSON.parse(schemaBytes);
  const classificationBytes = fs.readFileSync(path.join(root, schema.inputs.tex_classification_path));
  const classification = JSON.parse(classificationBytes);
  const texBytes = fs.readFileSync(path.join(root, schema.inputs.canonical_tex_path));
  if (hash(texBytes) !== schema.inputs.canonical_tex_sha256) throw new Error("canonical TeX changed");
  if (new Set(schema.sources).size !== schema.sources.length) throw new Error("duplicate source identity");
  const lines = texBytes.toString("utf8").replace(/\r\n?/gu, "\n").split("\n");
  const records = schema.sources.map((id) => {
    const matches = classification.records.filter((record) => record.source_id === id);
    if (matches.length !== 1) throw new Error(`missing or duplicate source: ${id}`);
    const record = matches[0];
    const excerpt = lines.slice(record.source.start_line - 1, record.source.end_line)
      .map((line) => line.trimEnd()).join("\n").trim();
    if (hash(excerpt) !== record.source.sha256) throw new Error(`source excerpt changed: ${id}`);
    return {
      source_id: id, disposition: record.disposition, destination: record.destination,
      source: record.source, legacy_status: record.legacy_obligation?.status ?? null,
    };
  });
  return {
    schema: 1,
    status: "generated_phase_b_protected_behavioral_equivalence_surface_not_successor_semantics",
    generated_from: {
      schema_sha256: hash(schemaBytes),
      tex_classification_sha256: hash(classificationBytes),
      canonical_tex_sha256: hash(texBytes),
      module_sha256: hash(fs.readFileSync(path.join(root, schema.inputs.module))),
    },
    records,
    next_residual: schema.next_residual,
    formal_gate_b: schema.gate_b,
  };
}

if (require.main === module) {
  try {
    const expected = `${JSON.stringify(generate(), null, 2)}\n`;
    const command = process.argv[2] ?? "check";
    if (command === "generate") fs.writeFileSync(output, expected);
    else if (command !== "check" || fs.readFileSync(output, "utf8") !== expected) {
      throw new Error("protected-equivalence surface does not exactly regenerate");
    }
    console.log(`protected-behavioral-equivalence surface ${command === "generate" ? "generated" : "regenerates exactly"}`);
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}

module.exports = { generate };
