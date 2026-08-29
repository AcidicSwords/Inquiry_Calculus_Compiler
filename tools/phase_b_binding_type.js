#!/usr/bin/env node
"use strict";
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const root = path.resolve(__dirname, "..");
const digest = (bytes) => crypto.createHash("sha256").update(bytes).digest("hex");
const schemaPath = path.join(root, "formal-successor", "PHASE_B_BINDING_TYPE_SCHEMA.json");
const outputPath = path.join(root, "formal-successor", "PHASE_B_BINDING_TYPE_SURFACE.json");
function json(relative) { const bytes = fs.readFileSync(path.join(root, relative)); return { bytes, value: JSON.parse(bytes) }; }
function sourceRecord(records, id) { const record = records.find((entry) => entry.source_id === id); if (!record) throw new Error(`missing classified source ${id}`); return record; }
function generate() {
  const schemaLoaded = json("formal-successor/PHASE_B_BINDING_TYPE_SCHEMA.json"); const schema = schemaLoaded.value;
  const texLoaded = json(schema.inputs.tex_classification_path); const spineLoaded = json(schema.inputs.spine_path);
  const ordered = [...schema.binding_sources, ...schema.type_sources];
  if (new Set(ordered).size !== ordered.length) throw new Error("source identities are repeated across binding/type surface");
  const records = ordered.map((id) => { const record = sourceRecord(texLoaded.value.records, id); return { source_id: id, disposition: record.disposition, destination: record.destination, source: record.source, obligation_status: record.legacy_obligation?.status || null }; });
  const formal = records.filter((record) => record.disposition === "FormalDefinition");
  const obligations = records.filter((record) => record.disposition === "LegacyObligation");
  return {
    schema: 1, status: "generated_phase_b_binding_type_surface_not_successor_semantics",
    generated_from: {
      schema_sha256: digest(schemaLoaded.bytes), tex_classification_sha256: digest(texLoaded.bytes), spine_sha256: digest(spineLoaded.bytes), canonical_tex_sha256: schema.inputs.canonical_tex_sha256,
      binding_module_sha256: digest(fs.readFileSync(path.join(root, schema.inputs.binding_module))), types_module_sha256: digest(fs.readFileSync(path.join(root, schema.inputs.types_module)))
    },
    source_layers: { binding: records.slice(0, schema.binding_sources.length), type_grammar: records.slice(schema.binding_sources.length) },
    retained_surface: { binding_slot_count: schema.binding_slots.length, type_constructor_count: schema.type_constructors.length, explicit_definition_count: formal.length, explicit_obligation_count: obligations.length, obligation_statuses: obligations.map((record) => record.obligation_status) },
    law: schema.noncollapse_law, next_residual: schema.next_residual, formal_gate_b: schema.gate_b
  };
}
function main() { const command = process.argv[2] || "check"; const expected = generate(); if (command === "generate") { fs.writeFileSync(outputPath, `${JSON.stringify(expected, null, 2)}\n`); process.stdout.write("generated Phase B binding/type surface (7 sources; 15 constructors; Gate B PENDING)\n"); return; } if (command !== "check") throw new Error(`unknown command ${command}`); const actual = JSON.parse(fs.readFileSync(outputPath, "utf8")); if (`${JSON.stringify(actual, null, 2)}\n` !== `${JSON.stringify(expected, null, 2)}\n`) throw new Error("binding/type surface does not exactly regenerate"); process.stdout.write("Phase B binding/type surface regenerates exactly (7 sources; Gate B PENDING)\n"); }
if (require.main === module) { try { main(); } catch (error) { process.stderr.write(`Phase B binding/type surface: ${error.message}\n`); process.exit(1); } }
module.exports = { generate };
