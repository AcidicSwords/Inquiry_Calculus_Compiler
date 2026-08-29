#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const root = path.resolve(__dirname, "..");
const schemaPath = path.join(root, "formal-successor", "PHASE_A_COVERAGE_SCHEMA.json");
const outputPath = path.join(root, "formal-successor", "PHASE_A_COVERAGE_CERTIFICATE.json");
const digest = (value) => crypto.createHash("sha256").update(value).digest("hex");
function load(relativePath) { const bytes = fs.readFileSync(path.join(root, relativePath)); return { bytes, value: JSON.parse(bytes.toString("utf8")) }; }
function countEdges(records, field) { return records.reduce((sum, record) => sum + (record[field] || []).length, 0); }

function generate() {
  const schemaLoaded = load("formal-successor/PHASE_A_COVERAGE_SCHEMA.json"); const schema = schemaLoaded.value;
  const inventoryLoaded = load(schema.inputs.inventory_path); const inventory = inventoryLoaded.value;
  const texLoaded = load(schema.inputs.tex_classification_path); const tex = texLoaded.value;
  const implementationLoaded = load(schema.inputs.implementation_classification_path); const implementation = implementationLoaded.value;
  const fixtureLoaded = load(schema.inputs.fixture_classification_path); const fixture = fixtureLoaded.value;
  const ownerRecords = {
    tex_overlay: tex.records,
    implementation_overlay: implementation.records,
    fixture_overlay: fixture.records,
    inventory_build_classification: inventory.items.filter((item) => schema.owners.inventory_build_classification.includes(item.source_class)),
  };
  const sourceOwnerCounts = new Map(inventory.items.map((item) => [item.id, 0])); const foreign = [];
  for (const [owner, records] of Object.entries(ownerRecords)) for (const record of records) {
    if (!sourceOwnerCounts.has(record.source_id || record.id)) foreign.push({ owner, source_id: record.source_id || record.id });
    else sourceOwnerCounts.set(record.source_id || record.id, sourceOwnerCounts.get(record.source_id || record.id) + 1);
  }
  const intersections = [...sourceOwnerCounts].filter(([, count]) => count > 1).map(([id, count]) => ({ source_id: id, owner_count: count }));
  const unowned = [...sourceOwnerCounts].filter(([, count]) => count === 0).map(([id]) => id);
  const texIds = new Set(tex.records.map((record) => record.source_id)); const implIds = new Set(implementation.records.map((record) => record.source_id)); const fixtureIds = new Set(fixture.records.map((record) => record.source_id));
  const schemaFixtureIds = new Set(implementation.records.filter((record) => record.source_class === "schema.fixture").map((record) => record.source_id));
  const conformanceModuleIds = new Set(fixture.records.filter((record) => record.source_class === "rust.conformance_module").map((record) => record.source_id));
  const semanticModuleIds = new Set(implementation.records.filter((record) => record.source_class === "rust.semantic_module_candidate").map((record) => record.source_id));
  const conformanceFixtureIds = new Set(fixture.records.filter((record) => record.source_class === "conformance.fixture").map((record) => record.source_id));
  const invalid = [];
  for (const record of implementation.records) for (const edge of record.claim_edges || []) if (!texIds.has(edge.target_id)) invalid.push({ source_id: record.source_id, field: "claim_edges", target_id: edge.target_id });
  for (const record of fixture.records) {
    for (const edge of record.claim_edges || []) if (!texIds.has(edge.target_id)) invalid.push({ source_id: record.source_id, field: "claim_edges", target_id: edge.target_id });
    for (const edge of record.implementation_targets || []) if (!implIds.has(edge.target_id)) invalid.push({ source_id: record.source_id, field: "implementation_targets", target_id: edge.target_id });
    for (const edge of record.witness_file_edges || []) if (!schemaFixtureIds.has(edge.target_id)) invalid.push({ source_id: record.source_id, field: "witness_file_edges", target_id: edge.target_id });
    for (const route of record.execution_routes || []) if (!conformanceModuleIds.has(route.module_source_id) && !semanticModuleIds.has(route.module_source_id)) invalid.push({ source_id: record.source_id, field: "execution_routes", target_id: route.module_source_id });
    for (const edge of record.fixture_edges || []) if (!conformanceFixtureIds.has(edge.fixture_source_id)) invalid.push({ source_id: record.source_id, field: "fixture_edges", target_id: edge.fixture_source_id });
  }
  const observed = {
    inventory_items: inventory.items.length,
    tex_owned: ownerRecords.tex_overlay.length,
    implementation_owned: ownerRecords.implementation_overlay.length,
    fixture_owned: ownerRecords.fixture_overlay.length,
    manifest_owned: ownerRecords.inventory_build_classification.length,
    owner_intersections: intersections.length,
    unowned_items: unowned.length,
    foreign_owner_records: foreign.length,
    implementation_claim_edges: countEdges(implementation.records, "claim_edges"),
    fixture_claim_edges: countEdges(fixture.records, "claim_edges"),
    fixture_implementation_targets: countEdges(fixture.records.filter((record) => record.source_class === "conformance.fixture"), "implementation_targets"),
    fixture_witness_file_edges: countEdges(fixture.records, "witness_file_edges"),
    fixture_execution_routes: countEdges(fixture.records, "execution_routes"),
    module_fixture_edges: countEdges(fixture.records, "fixture_edges"),
    invalid_edge_targets: invalid.length,
    fixture_unknown_standings: fixture.records.filter((record) => record.successor_standing === "Unknown").length,
    manifest_classified_items: ownerRecords.inventory_build_classification.filter((item) => item.review_status === "classified").length,
  };
  return {
    schema: 1,
    status: "generated_phase_a_joined_coverage_candidate_not_self_warrant",
    generated_from: {
      schema_path: "formal-successor/PHASE_A_COVERAGE_SCHEMA.json", schema_sha256: digest(schemaLoaded.bytes),
      inventory_path: schema.inputs.inventory_path, inventory_sha256: digest(inventoryLoaded.bytes),
      tex_classification_path: schema.inputs.tex_classification_path, tex_classification_sha256: digest(texLoaded.bytes),
      implementation_classification_path: schema.inputs.implementation_classification_path, implementation_classification_sha256: digest(implementationLoaded.bytes),
      fixture_classification_path: schema.inputs.fixture_classification_path, fixture_classification_sha256: digest(fixtureLoaded.bytes),
      predecessor_commit: schema.inputs.predecessor_commit,
    },
    observed_boundary: observed,
    ownership: {
      owners: Object.fromEntries(Object.entries(ownerRecords).map(([owner, records]) => [owner, { source_classes: schema.owners[owner], record_count: records.length, ordered_source_id_sha256: digest(records.map((record) => record.source_id || record.id).sort().join("\n")) }])),
      intersections,
      unowned_source_ids: unowned,
      foreign_owner_records: foreign,
    },
    edge_integrity: { invalid_targets: invalid },
    retained_boundaries: {
      tex_dispositions: tex.coverage.disposition_counts,
      implementation_dispositions: implementation.coverage.disposition_counts,
      fixture_dispositions: fixture.coverage.disposition_counts,
      fixture_successor_standing: fixture.coverage.successor_standing_counts,
      law: schema.noncollapse_law,
    },
    gate_a_candidate: {
      status: "READY_FOR_INDEPENDENT_CHECK",
      condition: "Every pinned predecessor item has exactly one source-bound destination or explicit classification and every admitted cross-overlay target is valid.",
      prohibited_self_warrant: "This generated certificate cannot pass Gate A; tools/phase_a_coverage_check.js must independently reconstruct and attack the joined relation.",
    },
  };
}
function validate(value, schema) {
  const errors = [];
  if (value.schema !== 1 || value.gate_a_candidate?.status !== "READY_FOR_INDEPENDENT_CHECK") errors.push("coverage artifact is not a non-self-warranting Gate A candidate");
  if (JSON.stringify(value.observed_boundary) !== JSON.stringify(schema.expected_boundary_at_pinned_inputs)) errors.push("observed joined boundary differs from schema");
  if (value.ownership?.intersections?.length || value.ownership?.unowned_source_ids?.length || value.ownership?.foreign_owner_records?.length || value.edge_integrity?.invalid_targets?.length) errors.push("joined ownership or target integrity is not closed");
  return errors;
}
function main() {
  const command = process.argv[2] || "check"; const schema = JSON.parse(fs.readFileSync(schemaPath, "utf8"));
  if (command === "generate") { const value = generate(); const errors = validate(value, schema); if (errors.length) throw new Error(errors.join("\n")); fs.writeFileSync(outputPath, `${JSON.stringify(value, null, 2)}\n`); process.stdout.write(`generated Phase A coverage candidate over ${value.observed_boundary.inventory_items} identities; independent check required\n`); return; }
  if (command === "check") { const actual = JSON.parse(fs.readFileSync(outputPath, "utf8")); const expected = generate(); const errors = validate(actual, schema); if (`${JSON.stringify(actual, null, 2)}\n` !== `${JSON.stringify(expected, null, 2)}\n`) errors.push("committed coverage candidate does not exactly regenerate"); if (errors.length) throw new Error(errors.join("\n")); process.stdout.write(`Phase A coverage candidate regenerates exactly (${actual.observed_boundary.inventory_items} identities; independent check required)\n`); return; }
  throw new Error(`unknown command ${command}; expected generate or check`);
}
module.exports = { generate, validate };
if (require.main === module) { try { main(); } catch (error) { process.stderr.write(`Phase A coverage: ${error.message}\n`); process.exit(1); } }
