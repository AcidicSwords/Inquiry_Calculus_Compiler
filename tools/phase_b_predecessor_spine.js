#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const root = path.resolve(__dirname, "..");
const schemaPath = path.join(root, "formal-successor", "PHASE_B_PREDECESSOR_SPINE_SCHEMA.json");
const outputPath = path.join(root, "formal-successor", "PHASE_B_PREDECESSOR_SPINE.json");
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");
function load(relativePath) { const bytes = fs.readFileSync(path.join(root, relativePath)); return { bytes, value: JSON.parse(bytes.toString("utf8")) }; }
function visitDag(layers) {
  const byId = new Map(layers.map((layer) => [layer.id, layer])); const visiting = new Set(); const visited = new Set();
  function visit(id) { if (visiting.has(id)) throw new Error(`dependency cycle at ${id}`); if (visited.has(id)) return; const layer = byId.get(id); if (!layer) throw new Error(`unknown layer ${id}`); visiting.add(id); for (const dep of layer.depends_on) visit(dep); visiting.delete(id); visited.add(id); }
  for (const id of byId.keys()) visit(id);
}
function generate() {
  const schemaLoaded = load("formal-successor/PHASE_B_PREDECESSOR_SPINE_SCHEMA.json"); const schema = schemaLoaded.value;
  const texLoaded = load(schema.inputs.tex_classification_path); const tex = texLoaded.value; const byId = new Map(tex.records.map((record) => [record.source_id, record]));
  visitDag(schema.layers);
  const seen = new Set();
  const layers = schema.layers.map((layer, index) => ({
    id: layer.id,
    ordinal: index,
    depends_on: layer.depends_on,
    destination: layer.destination,
    state: layer.state,
    sources: layer.source_ids.map((sourceId) => {
      if (seen.has(sourceId)) throw new Error(`source ${sourceId} occurs in more than one Phase B layer`); seen.add(sourceId);
      const record = byId.get(sourceId); if (!record) throw new Error(`unknown TeX classification source ${sourceId}`);
      return { source_id: sourceId, disposition: record.disposition, destination: record.destination, source: record.source, obligation_status: record.legacy_obligation?.status || null };
    }),
  }));
  const openSources = layers.flatMap((layer) => layer.sources).filter((source) => source.disposition === "LegacyObligation");
  return {
    schema: 1,
    status: "generated_phase_b_predecessor_spine_not_semantic_completion",
    generated_from: {
      schema_path: "formal-successor/PHASE_B_PREDECESSOR_SPINE_SCHEMA.json", schema_sha256: hash(schemaLoaded.bytes),
      tex_classification_path: schema.inputs.tex_classification_path, tex_classification_sha256: hash(texLoaded.bytes),
      canonical_tex_sha256: schema.inputs.canonical_tex_sha256,
      gate_a_evidence: schema.inputs.gate_a_evidence,
      lean_ambient_module: schema.inputs.lean_ambient_module,
      lean_ambient_sha256: hash(fs.readFileSync(path.join(root, schema.inputs.lean_ambient_module))),
    },
    ambient_boundary: {
      supplied_features: schema.ambient_features,
      explicit_nonassumptions: schema.ambient_nonassumptions,
      classicality_status: "ExplicitPredecessorObligation",
      law: "Lean supplies the listed metalanguage forms directly; no Inquiry Calculus duplicate or global classical axiom is introduced.",
    },
    coverage: {
      layer_count: layers.length,
      selected_source_count: seen.size,
      selected_legacy_obligations: openSources.length,
      checked_boundary_layers: layers.filter((layer) => layer.state.startsWith("checked_boundary")).length,
      open_layers: layers.filter((layer) => layer.state === "open").length,
    },
    layers,
    next_residual: {
      id: "FORMAL-B-BINDING-TYPE-SURFACE",
      relation: "Elaborate the binding-indexed predecessor type universe and reference type grammar without confusing Lean Type with a calculus type code or assuming unsupported native structure.",
    },
    formal_gate_b: schema.gate_b,
  };
}
function validate(value, schema) {
  const errors = [];
  if (value.formal_gate_b?.status !== "PENDING") errors.push("Phase B spine self-promotes Gate B");
  if (value.coverage?.layer_count !== schema.required_layer_order.length || value.coverage?.checked_boundary_layers !== 1 || value.coverage?.open_layers !== schema.required_layer_order.length - 1) errors.push("Phase B layer coverage differs");
  if (JSON.stringify(value.layers?.map((layer) => layer.id)) !== JSON.stringify(schema.required_layer_order)) errors.push("Phase B layer order differs");
  if (value.ambient_boundary?.classicality_status !== "ExplicitPredecessorObligation") errors.push("predecessor classicality was silently assumed or erased");
  return errors;
}
function main() {
  const command = process.argv[2] || "check"; const schema = JSON.parse(fs.readFileSync(schemaPath, "utf8"));
  if (command === "generate") { const value = generate(); const errors = validate(value, schema); if (errors.length) throw new Error(errors.join("\n")); fs.writeFileSync(outputPath, `${JSON.stringify(value, null, 2)}\n`); process.stdout.write(`generated Phase B predecessor spine (${value.coverage.layer_count} layers; ${value.coverage.selected_source_count} exact sources; Gate B PENDING)\n`); return; }
  if (command === "check") { const actual = JSON.parse(fs.readFileSync(outputPath, "utf8")); const expected = generate(); const errors = validate(actual, schema); if (`${JSON.stringify(actual, null, 2)}\n` !== `${JSON.stringify(expected, null, 2)}\n`) errors.push("committed Phase B spine does not exactly regenerate"); if (errors.length) throw new Error(errors.join("\n")); process.stdout.write(`Phase B predecessor spine regenerates exactly (${actual.coverage.layer_count} layers; Gate B ${actual.formal_gate_b.status})\n`); return; }
  throw new Error(`unknown command ${command}`);
}
module.exports = { generate, validate, visitDag };
if (require.main === module) { try { main(); } catch (error) { process.stderr.write(`Phase B predecessor spine: ${error.message}\n`); process.exit(1); } }
