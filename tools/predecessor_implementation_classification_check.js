#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const root = path.resolve(__dirname, "..");
const schemaPath = path.join(root, "formal-successor", "PREDECESSOR_IMPLEMENTATION_CLASSIFICATION_SCHEMA.json");
const inventoryPath = path.join(root, "formal-successor", "PREDECESSOR_INVENTORY.json");
const texClassificationPath = path.join(root, "formal-successor", "PREDECESSOR_TEX_CLASSIFICATION.json");
const classificationPath = path.join(root, "formal-successor", "PREDECESSOR_IMPLEMENTATION_CLASSIFICATION.json");
const blobCache = new Map();

function digest(value) {
  return crypto.createHash("sha256").update(value).digest("hex");
}

function normalizeNewlines(value) {
  return value.replace(/\r\n?/gu, "\n");
}

function normalizeUnit(value) {
  return normalizeNewlines(value)
    .split("\n")
    .map((line) => line.replace(/\s+$/u, ""))
    .join("\n")
    .trim();
}

function stable(value) {
  return JSON.stringify(value);
}

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function gitBlob(revision, relativePath) {
  const cacheKey = `${revision}\0${relativePath}`;
  if (blobCache.has(cacheKey)) return blobCache.get(cacheKey);
  const result = spawnSync("git", ["show", `${revision}:${relativePath}`], {
    cwd: root,
    encoding: "buffer",
    maxBuffer: 128 * 1024 * 1024,
    windowsHide: true,
  });
  if (result.status !== 0) throw new Error(`cannot read ${relativePath} at ${revision}`);
  blobCache.set(cacheKey, result.stdout);
  return result.stdout;
}

function eligible(name) {
  return typeof name === "string" && /^[A-Z][A-Za-z0-9]{5,}$/u.test(name);
}

function textFor(lines, record) {
  return lines.slice(record.source.start_line - 1, record.source.end_line).join("\n");
}

function exactForms(name, text) {
  if (!eligible(name)) return [];
  return schema.edge_grammar.admitted_tex_forms
    .map((template) => template.replace("NAME", name))
    .filter((form) => text.includes(form));
}

function directEdges(item) {
  if (item.source_class !== "rust.public_item" || !eligible(item.context?.name)) return [];
  const result = [];
  for (const target of texRecords) {
    const forms = exactForms(item.context.name, texTexts.get(target.source_id));
    if (forms.length === 0) continue;
    result.push({
      target_id: target.source_id,
      target_disposition: target.disposition,
      edge_kind: schema.edge_grammar.direct_edge_kind,
      authority: schema.edge_grammar.authority,
      matched_forms: forms,
      symbols: [item.context.name],
      via_public_source_ids: [item.id],
    });
  }
  return result.sort((left, right) => left.target_id.localeCompare(right.target_id));
}

function moduleEdges(moduleItem) {
  const grouped = new Map();
  for (const item of publicItems.filter((candidate) => candidate.source.path === moduleItem.source.path)) {
    for (const edge of expectedDirect.get(item.id)) {
      const value = grouped.get(edge.target_id) || { symbols: new Set(), via: new Set(), forms: new Set() };
      edge.symbols.forEach((symbol) => value.symbols.add(symbol));
      edge.via_public_source_ids.forEach((sourceId) => value.via.add(sourceId));
      edge.matched_forms.forEach((form) => value.forms.add(form));
      grouped.set(edge.target_id, value);
    }
  }
  return [...grouped.entries()].sort(([left], [right]) => left.localeCompare(right)).map(([targetId, value]) => ({
    target_id: targetId,
    target_disposition: texById.get(targetId).disposition,
    edge_kind: schema.edge_grammar.module_edge_kind,
    authority: schema.edge_grammar.authority,
    matched_forms: [...value.forms].sort(),
    symbols: [...value.symbols].sort(),
    via_public_source_ids: [...value.via].sort(),
  }));
}

function expectedRole(item, hasEdges) {
  if (item.source_class === "rust.semantic_module_candidate") return hasEdges ? "semantic_module_correspondence_candidate" : "semantic_module_implementation_only";
  if (item.source_class === "rust.public_item") return hasEdges ? "public_symbol_correspondence_candidate" : "public_implementation_surface";
  if (item.source_class === "schema.fixture") return "wire_fixture";
  return "storage_schema";
}

function countBy(records, key) {
  const counts = {};
  for (const record of records) counts[key(record)] = (counts[key(record)] || 0) + 1;
  return Object.fromEntries(Object.entries(counts).sort(([left], [right]) => left.localeCompare(right)));
}

const schemaBytes = fs.readFileSync(schemaPath);
const inventoryBytes = fs.readFileSync(inventoryPath);
const texClassificationBytes = fs.readFileSync(texClassificationPath);
const classificationBytes = fs.readFileSync(classificationPath);
const schema = JSON.parse(schemaBytes.toString("utf8"));
const inventory = JSON.parse(inventoryBytes.toString("utf8"));
const texClassification = JSON.parse(texClassificationBytes.toString("utf8"));
const classification = JSON.parse(classificationBytes.toString("utf8"));
const texBytes = fs.readFileSync(path.join(root, schema.inputs.canonical_tex_path));
const texLines = normalizeNewlines(texBytes.toString("utf8")).split("\n");
const texRecords = [...texClassification.records].sort((left, right) => left.source_id.localeCompare(right.source_id));
const texById = new Map(texRecords.map((record) => [record.source_id, record]));
const texTexts = new Map(texRecords.map((record) => [record.source_id, textFor(texLines, record)]));
const admitted = new Set(schema.inputs.admitted_source_classes);
const sourceItems = inventory.items.filter((item) => admitted.has(item.source_class));
const sourceById = new Map(sourceItems.map((item) => [item.id, item]));
const publicItems = sourceItems.filter((item) => item.source_class === "rust.public_item");
const expectedDirect = new Map(publicItems.map((item) => [item.id, directEdges(item)]));

function expectedEdges(item) {
  if (item.source_class === "rust.semantic_module_candidate") return moduleEdges(item);
  return expectedDirect.get(item.id) || [];
}

function validateSource(item, errors) {
  const bytes = gitBlob(item.source.revision, item.source.path);
  if (item.source_class === "rust.public_item") {
    const lines = normalizeNewlines(bytes.toString("utf8")).split("\n");
    const content = lines.slice(item.source.start_line - 1, item.source.end_line).join("\n");
    if (digest(Buffer.from(normalizeUnit(content), "utf8")) !== item.source.sha256) errors.push(`${item.id}: public source digest is detached`);
  } else if (digest(bytes) !== item.source.sha256) {
    errors.push(`${item.id}: repository blob digest is detached`);
  }
}

function validate(value) {
  const errors = [];
  if (schema.schema !== 1 || schema.status !== "phase_a_implementation_classification_contract_not_semantic_authority") errors.push("classification schema authority/status is invalid");
  if (schema.inputs.predecessor_commit !== "4a18e2e308f359a64f19b7d056652f19fd9aaeae") errors.push("predecessor coordinate changed");
  if (digest(texBytes) !== schema.inputs.canonical_tex_sha256) errors.push("canonical TeX digest is detached");
  if (value.schema !== 1 || value.status !== "reviewed_phase_a_implementation_classification_not_semantic_authority") errors.push("classification artifact status is invalid");
  if (value.generated_from?.schema_sha256 !== digest(schemaBytes)) errors.push("schema digest is detached");
  if (value.generated_from?.inventory_sha256 !== digest(inventoryBytes)) errors.push("inventory digest is detached");
  if (value.generated_from?.tex_classification_sha256 !== digest(texClassificationBytes)) errors.push("TeX classification digest is detached");
  if (value.local_gate?.id !== "FORMAL-A-RUST-SURFACE-INVENTORY" || value.local_gate?.status !== "READY_FOR_INDEPENDENT_CHECK") errors.push("implementation-local gate is invalid");
  if (value.formal_gate_a?.status !== "PENDING") errors.push("classification self-promoted Formal Gate A");
  if (!Array.isArray(value.records)) return [...errors, "records are not an array"];

  const recordsById = new Map();
  let previous = "";
  for (const [index, record] of value.records.entries()) {
    for (const field of schema.required_record_fields) if (!Object.hasOwn(record, field)) errors.push(`record ${index} is missing ${field}`);
    if (recordsById.has(record.source_id)) errors.push(`duplicate classification ${record.source_id}`);
    recordsById.set(record.source_id, record);
    if (previous && previous.localeCompare(record.source_id) > 0) errors.push(`records are unsorted at ${record.source_id}`);
    previous = record.source_id;
    const item = sourceById.get(record.source_id);
    if (!item) {
      errors.push(`foreign implementation identity ${record.source_id}`);
      continue;
    }
    if (record.source_class !== item.source_class || stable(record.source) !== stable(item.source) || stable(record.context) !== stable(item.context || {})) errors.push(`${record.source_id}: source identity or context changed`);
    validateSource(item, errors);
    const edges = expectedEdges(item);
    const hasEdges = edges.length > 0;
    const disposition = hasEdges ? "LegacyObligation" : "ImplementationOnly";
    const role = expectedRole(item, hasEdges);
    const destination = `${schema.destination_roots[disposition]}/${item.source.path}/${item.id}`;
    if (record.disposition !== disposition) errors.push(`${record.source_id}: expected ${disposition}, got ${record.disposition}`);
    if (record.implementation_role !== role) errors.push(`${record.source_id}: expected role ${role}`);
    if (record.destination !== destination) errors.push(`${record.source_id}: destination is detached`);
    if (stable(record.claim_edges) !== stable(edges)) errors.push(`${record.source_id}: claim edges are not exact/regenerative`);
    if (record.review_status !== "reviewed" || record.review_basis !== "exact_tex_symbol_incidence_policy_v1") errors.push(`${record.source_id}: review basis is invalid`);
    if (typeof record.rationale !== "string" || record.rationale.length < 60) errors.push(`${record.source_id}: rationale is absent`);
    if (hasEdges) {
      const targets = edges.map((edge) => edge.target_id);
      const obligation = record.legacy_obligation;
      if (!obligation || record.implementation_only) errors.push(`${record.source_id}: correspondence obligation payload is missing`);
      else {
        if (obligation.candidate_proposition?.language !== schema.legacy_obligation.candidate_language ||
            obligation.candidate_proposition?.expression !== `ImplementsCandidate(${JSON.stringify(item.id)},${JSON.stringify(targets)})` ||
            obligation.candidate_proposition?.elaboration_status !== "unelaborated") errors.push(`${record.source_id}: candidate proposition is invalid`);
        if (stable(obligation.dependencies) !== stable(targets) || obligation.status !== "Unproved") errors.push(`${record.source_id}: obligation dependencies/status are invalid`);
        if (obligation.known_breaker !== null || obligation.breaker_status !== "not_yet_established") errors.push(`${record.source_id}: breaker state was fabricated`);
      }
      if (record.edge_status !== "candidate_unproved") errors.push(`${record.source_id}: exact incidence was promoted or erased`);
    } else {
      if (record.legacy_obligation || record.implementation_only?.reason !== "no_admitted_exact_tex_symbol_edge" || record.implementation_only?.future_correspondence_status !== "Unknown") errors.push(`${record.source_id}: ImplementationOnly payload collapses Unknown`);
      if (record.edge_status !== "none_observed_under_exact_symbol_grammar") errors.push(`${record.source_id}: absent exact edge status is invalid`);
    }
  }

  for (const item of sourceItems) if (!recordsById.has(item.id)) errors.push(`missing classification ${item.id}`);
  const recognized = value.records.filter((record) => sourceById.has(record.source_id));
  const direct = recognized.flatMap((record) => record.claim_edges).filter((edge) => edge.edge_kind === schema.edge_grammar.direct_edge_kind);
  const aggregated = recognized.flatMap((record) => record.claim_edges).filter((edge) => edge.edge_kind === schema.edge_grammar.module_edge_kind);
  const expectedCoverage = {
    classified_source_items: sourceItems.length,
    source_class_counts: countBy(sourceItems, (item) => item.source_class),
    disposition_counts: countBy(recognized, (record) => record.disposition),
    implementation_role_counts: countBy(recognized, (record) => record.implementation_role),
    direct_public_symbols: recognized.filter((record) => record.implementation_role === "public_symbol_correspondence_candidate").length,
    direct_exact_symbol_edges: direct.length,
    modules_with_aggregated_edges: recognized.filter((record) => record.implementation_role === "semantic_module_correspondence_candidate").length,
    aggregated_module_edges: aggregated.length,
    unclassified_source_items: 0,
  };
  if (stable(value.coverage) !== stable(expectedCoverage)) errors.push("coverage counts are not exact");
  const boundary = schema.expected_boundary_at_pinned_inputs;
  if (expectedCoverage.classified_source_items !== boundary.classified_source_items ||
      expectedCoverage.direct_public_symbols !== boundary.direct_public_symbols ||
      expectedCoverage.direct_exact_symbol_edges !== boundary.direct_exact_symbol_edges ||
      expectedCoverage.modules_with_aggregated_edges !== boundary.modules_with_aggregated_edges ||
      expectedCoverage.aggregated_module_edges !== boundary.aggregated_module_edges ||
      expectedCoverage.disposition_counts.LegacyObligation !== boundary.legacy_obligations ||
      expectedCoverage.disposition_counts.ImplementationOnly !== boundary.implementation_only) errors.push("pinned decisive boundary counts changed");
  return errors;
}

const baseErrors = validate(classification);
if (baseErrors.length > 0) throw new Error(`independent implementation classification failed:\n${baseErrors.join("\n")}`);

const breakers = [];
function mustReject(name, mutate) {
  const candidate = clone(classification);
  mutate(candidate);
  if (validate(candidate).length === 0) throw new Error(`mutation breaker escaped: ${name}`);
  breakers.push(name);
}

mustReject("semantic module deletion", (value) => { value.records = value.records.filter((record) => record.source_class !== "rust.semantic_module_candidate" || record !== value.records.find((candidate) => candidate.source_class === "rust.semantic_module_candidate")); });
mustReject("public item deletion", (value) => { value.records.splice(value.records.findIndex((record) => record.source_class === "rust.public_item"), 1); });
mustReject("schema deletion", (value) => { value.records.splice(value.records.findIndex((record) => record.source_class.startsWith("schema.")), 1); });
mustReject("duplicate identity", (value) => { value.records.push(clone(value.records[0])); });
mustReject("foreign TeX target", (value) => { value.records.find((record) => record.claim_edges.length).claim_edges[0].target_id = "PRED-TEX-FOREIGN"; });
mustReject("generic fuzzy edge", (value) => { const record = value.records.find((candidate) => candidate.disposition === "ImplementationOnly" && candidate.source_class === "rust.public_item"); record.claim_edges.push(clone(value.records.find((candidate) => candidate.claim_edges.length).claim_edges[0])); });
mustReject("blanket ImplementationOnly", (value) => { for (const record of value.records) { record.disposition = "ImplementationOnly"; record.claim_edges = []; delete record.legacy_obligation; record.implementation_only = { reason: "no_admitted_exact_tex_symbol_edge", future_correspondence_status: "Unknown" }; } });
mustReject("blanket correspondence", (value) => { const edge = clone(value.records.find((record) => record.claim_edges.length).claim_edges[0]); for (const record of value.records) { record.disposition = "LegacyObligation"; record.claim_edges = [edge]; delete record.implementation_only; } });
mustReject("accepted edge authority", (value) => { value.records.find((record) => record.claim_edges.length).claim_edges[0].authority = "accepted_semantic_correspondence"; });
mustReject("erased module provenance", (value) => { value.records.find((record) => record.implementation_role === "semantic_module_correspondence_candidate").claim_edges[0].via_public_source_ids = []; });
mustReject("detached source digest", (value) => { value.records[0].source.sha256 = "0".repeat(64); });
mustReject("fixture role collapse", (value) => { value.records.find((record) => record.source_class === "schema.fixture").implementation_role = "storage_schema"; });
mustReject("Formal Gate A self-promotion", (value) => { value.formal_gate_a.status = "PASS"; });

process.stdout.write(`independent implementation classification checks passed (${classification.records.length} reviewed; ${breakers.length}/${breakers.length} mutation breakers; Gate A PENDING)\n`);
