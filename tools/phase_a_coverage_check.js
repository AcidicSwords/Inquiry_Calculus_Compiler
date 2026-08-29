#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const root = path.resolve(__dirname, "..");
const schemaPath = path.join(root, "formal-successor", "PHASE_A_COVERAGE_SCHEMA.json");
const certificatePath = path.join(root, "formal-successor", "PHASE_A_COVERAGE_CERTIFICATE.json");
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");
function readJson(relativePath) { const bytes = fs.readFileSync(path.join(root, relativePath)); return { bytes, value: JSON.parse(bytes.toString("utf8")) }; }
function clone(value) { return JSON.parse(JSON.stringify(value)); }
function countEdges(records, field) { return records.reduce((sum, record) => sum + (record[field] || []).length, 0); }
function sorted(values) { return [...values].sort((a, b) => a.localeCompare(b)); }
function sameSet(left, right) { return JSON.stringify(sorted(left)) === JSON.stringify(sorted(right)); }
function ownerDigest(records) { return hash(sorted(records.map((record) => record.source_id || record.id)).join("\n")); }

function inspect(certificate, schema, inventory, tex, implementation, fixture, bytes) {
  const errors = [];
  const inputs = schema.inputs;
  const digestChecks = [
    ["schema_sha256", bytes.schema], ["inventory_sha256", bytes.inventory],
    ["tex_classification_sha256", bytes.tex], ["implementation_classification_sha256", bytes.implementation],
    ["fixture_classification_sha256", bytes.fixture],
  ];
  for (const [field, source] of digestChecks) if (certificate.generated_from?.[field] !== hash(source)) errors.push(`coverage certificate has detached ${field}`);
  if (certificate.generated_from?.predecessor_commit !== inputs.predecessor_commit || inventory.generated_from?.predecessor_commit !== inputs.predecessor_commit) errors.push("coverage inputs differ from the pinned predecessor coordinate");
  if (certificate.gate_a_candidate?.status !== "READY_FOR_INDEPENDENT_CHECK") errors.push("generated certificate attempts to decide Gate A or is not ready");
  for (const overlay of [tex, implementation, fixture]) if (overlay.formal_gate_a?.status !== "PENDING") errors.push("a local overlay attempts to decide global Gate A");

  const owners = {
    tex_overlay: tex.records || [],
    implementation_overlay: implementation.records || [],
    fixture_overlay: fixture.records || [],
    inventory_build_classification: inventory.items.filter((item) => schema.owners.inventory_build_classification.includes(item.source_class)),
  };
  const inventoryById = new Map(inventory.items.map((item) => [item.id, item]));
  const sourceOwners = new Map(inventory.items.map((item) => [item.id, []])); const foreign = [];
  for (const [owner, records] of Object.entries(owners)) {
    const admitted = new Set(schema.owners[owner]);
    for (const record of records) {
      const id = record.source_id || record.id; const source = inventoryById.get(id);
      if (!source) { foreign.push(`${owner}:${id}`); continue; }
      if (!admitted.has(source.source_class) || record.source_class && record.source_class !== source.source_class) errors.push(`${owner}:${id} has a foreign source class`);
      sourceOwners.get(id).push(owner);
      if (record.source && JSON.stringify(record.source) !== JSON.stringify(source.source)) errors.push(`${owner}:${id} has detached source identity`);
    }
    const expected = inventory.items.filter((item) => admitted.has(item.source_class)).map((item) => item.id);
    if (!sameSet(records.map((record) => record.source_id || record.id), expected)) errors.push(`${owner} does not exactly own its admitted inventory subset`);
    const stated = certificate.ownership?.owners?.[owner];
    if (stated?.record_count !== records.length || stated?.ordered_source_id_sha256 !== ownerDigest(records) || !sameSet(stated?.source_classes || [], schema.owners[owner])) errors.push(`${owner} certificate summary differs from independently reconstructed ownership`);
  }
  const intersections = [...sourceOwners].filter(([, values]) => values.length > 1); const unowned = [...sourceOwners].filter(([, values]) => values.length === 0);
  if (intersections.length || unowned.length || foreign.length) errors.push("joined ownership is not an exact one-owner partition");
  if ((certificate.ownership?.intersections || []).length || (certificate.ownership?.unowned_source_ids || []).length || (certificate.ownership?.foreign_owner_records || []).length) errors.push("certificate reports an open ownership relation");

  const texIds = new Set(tex.records.map((record) => record.source_id)); const implementationIds = new Set(implementation.records.map((record) => record.source_id));
  const schemaFixtureIds = new Set(implementation.records.filter((record) => record.source_class === "schema.fixture").map((record) => record.source_id));
  const semanticModuleIds = new Set(implementation.records.filter((record) => record.source_class === "rust.semantic_module_candidate").map((record) => record.source_id));
  const fixtureIds = new Set(fixture.records.map((record) => record.source_id));
  const conformanceModuleIds = new Set(fixture.records.filter((record) => record.source_class === "rust.conformance_module").map((record) => record.source_id));
  const conformanceFixtureIds = new Set(fixture.records.filter((record) => record.source_class === "conformance.fixture").map((record) => record.source_id));
  let invalidTargets = 0;
  for (const record of implementation.records) for (const edge of record.claim_edges || []) if (!texIds.has(edge.target_id)) invalidTargets += 1;
  for (const record of fixture.records) {
    for (const edge of record.claim_edges || []) if (!texIds.has(edge.target_id)) invalidTargets += 1;
    for (const edge of record.implementation_targets || []) if (!implementationIds.has(edge.target_id)) invalidTargets += 1;
    for (const edge of record.witness_file_edges || []) if (!schemaFixtureIds.has(edge.target_id)) invalidTargets += 1;
    for (const route of record.execution_routes || []) if (!conformanceModuleIds.has(route.module_source_id) && !semanticModuleIds.has(route.module_source_id)) invalidTargets += 1;
    for (const edge of record.fixture_edges || []) if (!conformanceFixtureIds.has(edge.fixture_source_id)) invalidTargets += 1;
  }
  if (invalidTargets !== 0 || (certificate.edge_integrity?.invalid_targets || []).length !== 0) errors.push("a cross-overlay edge has an invalid target");
  for (const record of fixture.records) if (record.successor_standing !== "Unknown") errors.push(`${record.source_id} promoted fixture successor standing`);
  for (const item of owners.inventory_build_classification) if (item.review_status !== "classified" || item.disposition !== "predecessor_build_evidence") errors.push(`${item.id} manifest is not explicitly classified`);

  const observed = {
    inventory_items: inventory.items.length,
    tex_owned: owners.tex_overlay.length,
    implementation_owned: owners.implementation_overlay.length,
    fixture_owned: owners.fixture_overlay.length,
    manifest_owned: owners.inventory_build_classification.length,
    owner_intersections: intersections.length,
    unowned_items: unowned.length,
    foreign_owner_records: foreign.length,
    implementation_claim_edges: countEdges(implementation.records, "claim_edges"),
    fixture_claim_edges: countEdges(fixture.records, "claim_edges"),
    fixture_implementation_targets: countEdges(fixture.records.filter((record) => record.source_class === "conformance.fixture"), "implementation_targets"),
    fixture_witness_file_edges: countEdges(fixture.records, "witness_file_edges"),
    fixture_execution_routes: countEdges(fixture.records, "execution_routes"),
    module_fixture_edges: countEdges(fixture.records, "fixture_edges"),
    invalid_edge_targets: invalidTargets,
    fixture_unknown_standings: fixture.records.filter((record) => record.successor_standing === "Unknown").length,
    manifest_classified_items: owners.inventory_build_classification.filter((item) => item.review_status === "classified").length,
  };
  if (JSON.stringify(observed) !== JSON.stringify(schema.expected_boundary_at_pinned_inputs)) errors.push(`independent observed boundary differs from schema: ${JSON.stringify(observed)}`);
  if (JSON.stringify(certificate.observed_boundary) !== JSON.stringify(observed)) errors.push("coverage certificate boundary differs from independent reconstruction");
  const retained = certificate.retained_boundaries || {};
  if (JSON.stringify(retained.tex_dispositions) !== JSON.stringify(tex.coverage.disposition_counts) || JSON.stringify(retained.implementation_dispositions) !== JSON.stringify(implementation.coverage.disposition_counts) || JSON.stringify(retained.fixture_dispositions) !== JSON.stringify(fixture.coverage.disposition_counts) || JSON.stringify(retained.fixture_successor_standing) !== JSON.stringify(fixture.coverage.successor_standing_counts) || retained.law !== schema.noncollapse_law) errors.push("certificate erases or changes a retained disposition/standing boundary");

  if (tex.generated_from?.inventory_sha256 !== hash(bytes.inventory) || implementation.generated_from?.inventory_sha256 !== hash(bytes.inventory) || fixture.generated_from?.inventory_sha256 !== hash(bytes.inventory)) errors.push("a local overlay is detached from the corrected inventory");
  if (implementation.generated_from?.tex_classification_sha256 !== hash(bytes.tex) || fixture.generated_from?.tex_classification_sha256 !== hash(bytes.tex) || fixture.generated_from?.implementation_classification_sha256 !== hash(bytes.implementation)) errors.push("cross-overlay input ancestry is detached");
  return errors;
}
function reject(name, values) { if (inspect(...values).length === 0) throw new Error(`mutation breaker escaped: ${name}`); }
function main() {
  const loaded = {
    schema: readJson("formal-successor/PHASE_A_COVERAGE_SCHEMA.json"), certificate: readJson("formal-successor/PHASE_A_COVERAGE_CERTIFICATE.json"),
    inventory: readJson("formal-successor/PREDECESSOR_INVENTORY.json"), tex: readJson("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"),
    implementation: readJson("formal-successor/PREDECESSOR_IMPLEMENTATION_CLASSIFICATION.json"), fixture: readJson("formal-successor/PREDECESSOR_FIXTURE_CLASSIFICATION.json"),
  };
  const bytes = Object.fromEntries(Object.entries(loaded).filter(([key]) => key !== "certificate").map(([key, entry]) => [key, entry.bytes]));
  const base = [loaded.certificate.value, loaded.schema.value, loaded.inventory.value, loaded.tex.value, loaded.implementation.value, loaded.fixture.value, bytes];
  const errors = inspect(...base); if (errors.length) throw new Error(errors.join("\n"));
  const mutations = [];
  function mutation(name, index, edit) { const values = base.map((value, position) => position === 6 ? { ...value } : clone(value)); edit(values[index], values); mutations.push([name, values]); }
  mutation("whole TeX class omission", 3, (v) => { v.records = v.records.filter((record) => record.source_class !== "tex.display"); });
  mutation("implementation source omission", 4, (v) => { v.records.pop(); });
  mutation("fixture source omission", 5, (v) => { v.records.pop(); });
  mutation("cross-owner duplication", 5, (v, all) => { v.records.push(clone(all[4].records[0])); });
  mutation("foreign owner source", 3, (v) => { v.records[0].source_id = "FOREIGN"; });
  mutation("foreign implementation claim target", 4, (v) => { v.records.find((r) => r.claim_edges?.length).claim_edges[0].target_id = "FOREIGN"; });
  mutation("foreign fixture claim target", 5, (v) => { v.records.find((r) => r.claim_edges?.length).claim_edges[0].target_id = "FOREIGN"; });
  mutation("foreign implementation target", 5, (v) => { v.records.find((r) => r.implementation_targets?.length).implementation_targets[0].target_id = "FOREIGN"; });
  mutation("foreign witness file", 5, (v) => { v.records.find((r) => r.witness_file_edges?.length).witness_file_edges[0].target_id = "FOREIGN"; });
  mutation("foreign execution module", 5, (v) => { v.records.find((r) => r.execution_routes?.length).execution_routes[0].module_source_id = "FOREIGN"; });
  mutation("foreign fixture route", 5, (v) => { v.records.find((r) => r.fixture_edges?.length).fixture_edges[0].fixture_source_id = "FOREIGN"; });
  mutation("fixture standing promotion", 5, (v) => { v.records[0].successor_standing = "PASS"; });
  mutation("manifest declassification", 2, (v) => { v.items.find((item) => item.source_class === "rust.crate_manifest").review_status = "pending"; });
  mutation("local Gate A self-promotion", 3, (v) => { v.formal_gate_a.status = "PASS"; });
  mutation("certificate Gate A self-promotion", 0, (v) => { v.gate_a_candidate.status = "PASS"; });
  mutation("detached inventory digest", 0, (v) => { v.generated_from.inventory_sha256 = "0".repeat(64); });
  mutation("fabricated boundary count", 0, (v) => { v.observed_boundary.inventory_items += 1; });
  mutation("erased retained boundary", 0, (v) => { v.retained_boundaries.fixture_successor_standing = { PASS: 226 }; });
  mutation("detached owner digest", 0, (v) => { v.ownership.owners.tex_overlay.ordered_source_id_sha256 = "f".repeat(64); });
  for (const [name, values] of mutations) reject(name, values);
  process.stdout.write(`independent Phase A coverage checks passed (${loaded.inventory.value.items.length} identities; exact one-owner partition; ${mutations.length}/${mutations.length} mutation breakers; Gate A PASS at this coverage)\n`);
}
try { main(); } catch (error) { process.stderr.write(`Phase A coverage check: ${error.message}\n`); process.exit(1); }
