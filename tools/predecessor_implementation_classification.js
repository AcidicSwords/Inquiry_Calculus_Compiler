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
const outputPath = path.join(root, "formal-successor", "PREDECESSOR_IMPLEMENTATION_CLASSIFICATION.json");

function sha256(value) {
  return crypto.createHash("sha256").update(value).digest("hex");
}

function normalizeNewlines(value) {
  return value.replace(/\r\n?/gu, "\n");
}

function slug(value) {
  const result = value
    .replace(/[^A-Za-z0-9]+/gu, "-")
    .replace(/^-+|-+$/gu, "")
    .toLowerCase();
  return result || "source";
}

function gitBlob(revision, relativePath) {
  const result = spawnSync("git", ["show", `${revision}:${relativePath}`], {
    cwd: root,
    encoding: "buffer",
    maxBuffer: 128 * 1024 * 1024,
    windowsHide: true,
  });
  if (result.status !== 0) throw new Error(`cannot read ${relativePath} at ${revision}: ${result.stderr.toString("utf8").trim()}`);
  return result.stdout;
}

function texExcerpt(lines, record) {
  return lines.slice(record.source.start_line - 1, record.source.end_line).join("\n");
}

function eligiblePublicSymbol(name) {
  return typeof name === "string" && /^[A-Z][A-Za-z0-9]{5,}$/u.test(name);
}

function exactForms(name, text, admittedForms) {
  if (!eligiblePublicSymbol(name)) return [];
  return admittedForms
    .map((template) => template.replace("NAME", name))
    .filter((form) => text.includes(form));
}

function directEdges(item, texRecords, texTexts, schema) {
  if (item.source_class !== "rust.public_item" || !eligiblePublicSymbol(item.context?.name)) return [];
  const edges = [];
  for (const target of texRecords) {
    const forms = exactForms(item.context.name, texTexts.get(target.source_id), schema.edge_grammar.admitted_tex_forms);
    if (forms.length === 0) continue;
    edges.push({
      target_id: target.source_id,
      target_disposition: target.disposition,
      edge_kind: schema.edge_grammar.direct_edge_kind,
      authority: schema.edge_grammar.authority,
      matched_forms: forms,
      symbols: [item.context.name],
      via_public_source_ids: [item.id],
    });
  }
  return edges.sort((left, right) => left.target_id.localeCompare(right.target_id));
}

function aggregateModuleEdges(moduleItem, publicItems, publicEdges, texById, schema) {
  const grouped = new Map();
  for (const item of publicItems.filter((candidate) => candidate.source.path === moduleItem.source.path)) {
    for (const edge of publicEdges.get(item.id) || []) {
      const value = grouped.get(edge.target_id) || { symbols: new Set(), via: new Set(), forms: new Set() };
      for (const symbol of edge.symbols) value.symbols.add(symbol);
      for (const sourceId of edge.via_public_source_ids) value.via.add(sourceId);
      for (const form of edge.matched_forms) value.forms.add(form);
      grouped.set(edge.target_id, value);
    }
  }
  return [...grouped.entries()]
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([targetId, value]) => ({
      target_id: targetId,
      target_disposition: texById.get(targetId).disposition,
      edge_kind: schema.edge_grammar.module_edge_kind,
      authority: schema.edge_grammar.authority,
      matched_forms: [...value.forms].sort(),
      symbols: [...value.symbols].sort(),
      via_public_source_ids: [...value.via].sort(),
    }));
}

function roleFor(item, hasEdges) {
  if (item.source_class === "rust.semantic_module_candidate") {
    return hasEdges ? "semantic_module_correspondence_candidate" : "semantic_module_implementation_only";
  }
  if (item.source_class === "rust.public_item") {
    return hasEdges ? "public_symbol_correspondence_candidate" : "public_implementation_surface";
  }
  if (item.source_class === "schema.fixture") return "wire_fixture";
  if (item.source_class === "schema.migration") return "storage_schema";
  throw new Error(`${item.id}: unsupported implementation source class ${item.source_class}`);
}

function recordFor(item, claimEdges, schema) {
  const hasEdges = claimEdges.length > 0;
  const disposition = hasEdges ? "LegacyObligation" : "ImplementationOnly";
  const implementationRole = roleFor(item, hasEdges);
  const destinationRoot = schema.destination_roots[disposition];
  const record = {
    source_id: item.id,
    source_class: item.source_class,
    source: item.source,
    context: item.context || {},
    implementation_role: implementationRole,
    disposition,
    destination: `${destinationRoot}/${item.source.path}/${item.id}`,
    claim_edges: claimEdges,
    edge_status: hasEdges ? "candidate_unproved" : "none_observed_under_exact_symbol_grammar",
    review_status: "reviewed",
    review_basis: "exact_tex_symbol_incidence_policy_v1",
    rationale: hasEdges
      ? "Exact wrapped TeX semantic-symbol incidence preserves a source-bound correspondence candidate, but supplies no semantic or preservation warrant."
      : "No admitted exact wrapped TeX semantic-symbol incidence was found; this predecessor artifact remains ImplementationOnly at Phase A and future correspondence stays Unknown.",
  };
  if (hasEdges) {
    const targets = claimEdges.map((edge) => edge.target_id);
    record.legacy_obligation = {
      candidate_proposition: {
        language: schema.legacy_obligation.candidate_language,
        expression: `ImplementsCandidate(${JSON.stringify(item.id)},${JSON.stringify(targets)})`,
        elaboration_status: "unelaborated",
      },
      scope: {
        source_path: item.source.path,
        implementation_role: implementationRole,
      },
      dependencies: targets,
      status: schema.legacy_obligation.status,
      known_breaker: null,
      breaker_status: schema.legacy_obligation.breaker_status,
    };
  } else {
    record.implementation_only = {
      reason: "no_admitted_exact_tex_symbol_edge",
      future_correspondence_status: "Unknown",
    };
  }
  return record;
}

function generateClassification() {
  const schemaBytes = fs.readFileSync(schemaPath);
  const inventoryBytes = fs.readFileSync(inventoryPath);
  const texClassificationBytes = fs.readFileSync(texClassificationPath);
  const schema = JSON.parse(schemaBytes.toString("utf8"));
  const inventory = JSON.parse(inventoryBytes.toString("utf8"));
  const texClassification = JSON.parse(texClassificationBytes.toString("utf8"));
  const texBytes = fs.readFileSync(path.join(root, schema.inputs.canonical_tex_path));
  if (sha256(texBytes) !== schema.inputs.canonical_tex_sha256) throw new Error("canonical TeX digest differs from implementation classification schema");
  const texLines = normalizeNewlines(texBytes.toString("utf8")).split("\n");
  const texRecords = [...texClassification.records].sort((left, right) => left.source_id.localeCompare(right.source_id));
  const texById = new Map(texRecords.map((record) => [record.source_id, record]));
  const texTexts = new Map(texRecords.map((record) => [record.source_id, texExcerpt(texLines, record)]));
  const admitted = new Set(schema.inputs.admitted_source_classes);
  const items = inventory.items.filter((item) => admitted.has(item.source_class));
  const publicItems = items.filter((item) => item.source_class === "rust.public_item");
  const publicEdges = new Map(publicItems.map((item) => [item.id, directEdges(item, texRecords, texTexts, schema)]));
  const records = items.map((item) => {
    const edges = item.source_class === "rust.semantic_module_candidate"
      ? aggregateModuleEdges(item, publicItems, publicEdges, texById, schema)
      : publicEdges.get(item.id) || [];
    return recordFor(item, edges, schema);
  }).sort((left, right) => left.source_id.localeCompare(right.source_id));

  const countBy = (key) => Object.fromEntries(Object.entries(records.reduce((counts, record) => {
    const value = key(record);
    counts[value] = (counts[value] || 0) + 1;
    return counts;
  }, {})).sort(([left], [right]) => left.localeCompare(right)));
  const directEdgeCount = records
    .flatMap((record) => record.claim_edges)
    .filter((edge) => edge.edge_kind === schema.edge_grammar.direct_edge_kind).length;
  const moduleEdgeCount = records
    .flatMap((record) => record.claim_edges)
    .filter((edge) => edge.edge_kind === schema.edge_grammar.module_edge_kind).length;
  return {
    schema: 1,
    status: "reviewed_phase_a_implementation_classification_not_semantic_authority",
    generated_from: {
      schema_path: path.relative(root, schemaPath).replace(/\\/gu, "/"),
      schema_sha256: sha256(schemaBytes),
      inventory_path: path.relative(root, inventoryPath).replace(/\\/gu, "/"),
      inventory_sha256: sha256(inventoryBytes),
      tex_classification_path: path.relative(root, texClassificationPath).replace(/\\/gu, "/"),
      tex_classification_sha256: sha256(texClassificationBytes),
      canonical_tex_path: schema.inputs.canonical_tex_path,
      canonical_tex_sha256: schema.inputs.canonical_tex_sha256,
      predecessor_commit: schema.inputs.predecessor_commit,
    },
    coverage: {
      classified_source_items: records.length,
      source_class_counts: countBy((record) => record.source_class),
      disposition_counts: countBy((record) => record.disposition),
      implementation_role_counts: countBy((record) => record.implementation_role),
      direct_public_symbols: records.filter((record) => record.implementation_role === "public_symbol_correspondence_candidate").length,
      direct_exact_symbol_edges: directEdgeCount,
      modules_with_aggregated_edges: records.filter((record) => record.implementation_role === "semantic_module_correspondence_candidate").length,
      aggregated_module_edges: moduleEdgeCount,
      unclassified_source_items: 0,
    },
    local_gate: {
      id: "FORMAL-A-RUST-SURFACE-INVENTORY",
      status: "READY_FOR_INDEPENDENT_CHECK",
      reason: "Every admitted Rust semantic/public/schema identity has an authority-separated role and every retained claim edge is exact, source-bound, and unproved.",
    },
    formal_gate_a: {
      status: "PENDING",
      reason: "Rust conformance modules, fixture identities, and fixture-to-claim edge coverage remain open.",
    },
    records,
  };
}

function validateShape(value, schema) {
  const errors = [];
  if (value.schema !== 1) errors.push("implementation classification schema must be 1");
  if (value.formal_gate_a?.status !== "PENDING") errors.push("implementation classification must not self-promote Formal Gate A");
  if (!Array.isArray(value.records)) errors.push("records must be an array");
  const ids = new Set();
  let previous = "";
  for (const [index, record] of (value.records || []).entries()) {
    for (const field of schema.required_record_fields) if (!Object.hasOwn(record, field)) errors.push(`record ${index} is missing ${field}`);
    if (ids.has(record.source_id)) errors.push(`duplicate source_id ${record.source_id}`);
    ids.add(record.source_id);
    if (previous && previous.localeCompare(record.source_id) > 0) errors.push(`records are unsorted at ${record.source_id}`);
    previous = record.source_id;
    if (!schema.used_dispositions.includes(record.disposition)) errors.push(`${record.source_id}: invalid implementation disposition`);
    if (!schema.implementation_roles.includes(record.implementation_role)) errors.push(`${record.source_id}: invalid implementation role`);
    if (record.review_status !== "reviewed") errors.push(`${record.source_id}: review is not closed`);
    if (record.disposition === "LegacyObligation") {
      if (!record.legacy_obligation || record.claim_edges.length === 0 || record.implementation_only) errors.push(`${record.source_id}: invalid correspondence obligation payload`);
    } else if (!record.implementation_only || record.claim_edges.length !== 0 || record.legacy_obligation) {
      errors.push(`${record.source_id}: invalid ImplementationOnly payload`);
    }
  }
  if (value.coverage?.classified_source_items !== (value.records || []).length) errors.push("classified count differs from records");
  if (value.coverage?.unclassified_source_items !== 0) errors.push("classification reports unclassified implementation items");
  return errors;
}

function main() {
  const command = process.argv[2] || "check";
  const schema = JSON.parse(fs.readFileSync(schemaPath, "utf8"));
  if (command === "generate") {
    const value = generateClassification();
    const errors = validateShape(value, schema);
    if (errors.length > 0) throw new Error(errors.join("\n"));
    fs.writeFileSync(outputPath, `${JSON.stringify(value, null, 2)}\n`);
    process.stdout.write(`generated ${value.records.length} implementation classifications; ${value.coverage.direct_exact_symbol_edges} exact public edges; Formal Gate A ${value.formal_gate_a.status}\n`);
    return;
  }
  if (command === "check") {
    const actual = JSON.parse(fs.readFileSync(outputPath, "utf8"));
    const expected = generateClassification();
    const errors = validateShape(actual, schema);
    if (`${JSON.stringify(actual, null, 2)}\n` !== `${JSON.stringify(expected, null, 2)}\n`) errors.push("committed implementation classification does not exactly regenerate");
    if (errors.length > 0) throw new Error(errors.join("\n"));
    process.stdout.write(`predecessor implementation classification regenerates exactly (${actual.records.length} reviewed; Gate A ${actual.formal_gate_a.status})\n`);
    return;
  }
  throw new Error(`unknown command ${command}; expected generate or check`);
}

module.exports = { directEdges, eligiblePublicSymbol, exactForms, generateClassification, sha256, validateShape };

if (require.main === module) {
  try {
    main();
  } catch (error) {
    process.stderr.write(`predecessor implementation classification: ${error.message}\n`);
    process.exit(1);
  }
}
