#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const schemaPath = path.join(root, "formal-successor", "PREDECESSOR_TEX_CLASSIFICATION_SCHEMA.json");
const inventoryPath = path.join(root, "formal-successor", "PREDECESSOR_INVENTORY.json");
const outputPath = path.join(root, "formal-successor", "PREDECESSOR_TEX_CLASSIFICATION.json");

function sha256(value) {
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

function slug(value) {
  const result = value
    .replace(/\\[A-Za-z@]+\*?(?:\[[^\]]*\])?/gu, " ")
    .replace(/[{}$\\]/gu, " ")
    .replace(/[^A-Za-z0-9]+/gu, "-")
    .replace(/^-+|-+$/gu, "")
    .toLowerCase();
  return result || "document";
}

function exactExcerpt(lines, source) {
  return lines.slice(source.start_line - 1, source.end_line).join("\n");
}

function referencedLabels(excerpt) {
  const labels = new Set();
  const pattern = /\\(?:c|C|eq|auto|page)?ref\{([^}]+)\}/gu;
  for (const match of excerpt.matchAll(pattern)) {
    for (const label of match[1].split(",")) {
      const trimmed = label.trim();
      if (trimmed) labels.add(trimmed);
    }
  }
  return [...labels].sort();
}

function sourceRole(item, disposition) {
  if (disposition === "CanonicalProseOnly") return "canonical_context";
  if (item.source_class === "tex.declaration") {
    return disposition === "BindingTheorem" ? "binding_declaration" : "standalone_declaration";
  }
  if (item.source_class === "tex.list_item") return "list_component";
  if (item.source_class === "tex.display") return "display_component";
  return "compound_claim_boundary";
}

function classification(item, excerpt, schema) {
  const normalized = normalizeUnit(excerpt);
  let disposition;
  let legacyStatus = null;
  let rationale;

  if (schema.rules.exact_canonical_context.includes(normalized)) {
    disposition = "CanonicalProseOnly";
    rationale = "The exact source unit is document-rendering context and states no predecessor proposition.";
  } else if (
    item.source_class === "tex.declaration" &&
    schema.rules.binding_headings.includes(item.context.heading) &&
    new Set(["theorem", "proposition", "corollary"]).has(item.category)
  ) {
    disposition = "BindingTheorem";
    rationale = "An explicit theorem-like declaration occurs under an exact admitted binding heading; Phase B must retain the binding in its theorem type.";
  } else if (item.source_class === "tex.declaration") {
    disposition = schema.rules.declaration_dispositions[item.category];
    if (!disposition) throw new Error(`${item.id}: declaration category ${item.category} has no classification rule`);
    legacyStatus = schema.rules.legacy_declaration_status[item.category] || null;
    if (disposition === "FormalDefinition") {
      rationale = "The source is an explicit v2.0 definition environment and therefore has a definition formalization destination.";
    } else if (disposition === "FormalTheorem") {
      rationale = "The source is an explicit theorem-like v2.0 declaration outside an admitted binding-specific heading.";
    } else {
      rationale = `The explicit ${item.category} declaration asserts consequential content but has not yet been elaborated and checked; it remains a LegacyObligation.`;
    }
  } else {
    disposition = schema.rules.nondeclaration_disposition;
    const normativeSignals = item.context?.normative_signals || [];
    legacyStatus = normativeSignals.length > 0
      ? schema.rules.nondeclaration_status_by_signal.normative_signal_present
      : schema.rules.nondeclaration_status_by_signal.no_normative_signal;
    const role = item.source_class === "tex.list_item"
      ? "list component"
      : item.source_class === "tex.display"
        ? "mathematical display component"
        : "prose boundary";
    rationale = normativeSignals.length > 0
      ? `The extracted ${role} carries explicit normative signal(s) (${normativeSignals.join(", ")}) but its standalone proposition boundary is not established by extraction. It remains an explicit unproved LegacyObligation.`
      : `The extracted ${role} may state, qualify, or compose a predecessor claim, but its standalone proposition boundary is not established by extraction and keyword absence cannot discharge it. It remains an explicit ambiguous LegacyObligation.`;
  }

  const heading = item.context?.heading || "Document";
  const destinationRoot = schema.destination_roots[disposition];
  const destination = destinationRoot === null ? null : `${destinationRoot}/${slug(heading)}/${item.id}`;
  const record = {
    source_id: item.id,
    source_class: item.source_class,
    source: item.source,
    context: item.context || {},
    source_role: sourceRole(item, disposition),
    disposition,
    destination,
    review_status: "reviewed",
    review_basis: "exact_source_role_policy_v1",
    rationale,
    source_excerpt_sha256: sha256(Buffer.from(normalized, "utf8")),
  };

  if (disposition === "LegacyObligation") {
    record.legacy_obligation = {
      candidate_proposition: {
        language: "unelaborated-source-claim/v1",
        expression: `SourceClaim(${JSON.stringify(item.id)})`,
        elaboration_status: "unelaborated",
      },
      scope: {
        heading,
        source_role: record.source_role,
      },
      dependencies: referencedLabels(excerpt),
      status: legacyStatus || "Ambiguous",
      known_breaker: null,
      breaker_status: "not_yet_established",
    };
  }

  return record;
}

function generateClassification() {
  const schemaBytes = fs.readFileSync(schemaPath);
  const inventoryBytes = fs.readFileSync(inventoryPath);
  const schema = JSON.parse(schemaBytes.toString("utf8"));
  const inventory = JSON.parse(inventoryBytes.toString("utf8"));
  const texBytes = fs.readFileSync(path.join(root, schema.inventory.canonical_tex_path));
  if (sha256(texBytes) !== schema.inventory.canonical_tex_sha256) {
    throw new Error("canonical TeX digest differs from the classification schema");
  }
  const lines = normalizeNewlines(texBytes.toString("utf8")).split("\n");
  const admitted = new Set(schema.inventory.admitted_source_classes);
  const sourceItems = inventory.items
    .filter((item) => admitted.has(item.source_class))
    .sort((left, right) => left.id.localeCompare(right.id));
  const records = sourceItems.map((item) => classification(item, exactExcerpt(lines, item.source), schema));
  const dispositionCounts = {};
  const roleCounts = {};
  const legacyStatusCounts = {};
  for (const record of records) {
    dispositionCounts[record.disposition] = (dispositionCounts[record.disposition] || 0) + 1;
    roleCounts[record.source_role] = (roleCounts[record.source_role] || 0) + 1;
    if (record.legacy_obligation) {
      const status = record.legacy_obligation.status;
      legacyStatusCounts[status] = (legacyStatusCounts[status] || 0) + 1;
    }
  }
  const sorted = (value) => Object.fromEntries(Object.entries(value).sort(([left], [right]) => left.localeCompare(right)));
  return {
    schema: 1,
    status: "reviewed_phase_a_tex_classification_not_successor_semantics",
    generated_from: {
      schema_path: path.relative(root, schemaPath).replace(/\\/gu, "/"),
      schema_sha256: sha256(schemaBytes),
      inventory_path: path.relative(root, inventoryPath).replace(/\\/gu, "/"),
      inventory_sha256: sha256(inventoryBytes),
      canonical_tex_path: schema.inventory.canonical_tex_path,
      canonical_tex_sha256: schema.inventory.canonical_tex_sha256,
    },
    coverage: {
      classified_source_items: records.length,
      source_class_counts: sorted(sourceItems.reduce((counts, item) => {
        counts[item.source_class] = (counts[item.source_class] || 0) + 1;
        return counts;
      }, {})),
      disposition_counts: sorted(dispositionCounts),
      source_role_counts: sorted(roleCounts),
      legacy_status_counts: sorted(legacyStatusCounts),
      unclassified_source_items: 0,
    },
    local_gate: {
      id: "FORMAL-A-TEX-INVENTORY",
      status: "READY_FOR_INDEPENDENT_CHECK",
      reason: "Every generated TeX candidate has one conservative construction-specification disposition; semantic elaboration remains Phase B work.",
    },
    formal_gate_a: {
      status: "PENDING",
      reason: "Rust/schema/fixture classification and claim-edge coverage remain open outside this TeX-local overlay.",
    },
    records,
  };
}

function validateShape(value, schema) {
  const errors = [];
  if (value.schema !== 1) errors.push("classification schema must be 1");
  if (value.formal_gate_a?.status !== "PENDING") errors.push("TeX classification must not self-promote Formal Gate A");
  if (!Array.isArray(value.records)) errors.push("records must be an array");
  const ids = new Set();
  let previous = "";
  for (const [index, record] of (value.records || []).entries()) {
    for (const field of schema.required_record_fields) {
      if (!Object.hasOwn(record, field)) errors.push(`record ${index} is missing ${field}`);
    }
    if (ids.has(record.source_id)) errors.push(`duplicate source_id ${record.source_id}`);
    ids.add(record.source_id);
    if (previous && previous.localeCompare(record.source_id) > 0) errors.push(`records are not sorted at ${record.source_id}`);
    previous = record.source_id;
    if (!schema.construction_specification_dispositions.includes(record.disposition)) errors.push(`${record.source_id}: invalid disposition`);
    if (!schema.source_roles.includes(record.source_role)) errors.push(`${record.source_id}: invalid source role`);
    if (record.review_status !== "reviewed") errors.push(`${record.source_id}: review is not closed`);
    if (record.disposition === "LegacyObligation") {
      if (!record.legacy_obligation) errors.push(`${record.source_id}: missing LegacyObligation payload`);
      else if (!schema.legacy_statuses.includes(record.legacy_obligation.status)) errors.push(`${record.source_id}: invalid legacy status`);
    } else if (record.legacy_obligation) {
      errors.push(`${record.source_id}: non-legacy record has a LegacyObligation payload`);
    }
    if (record.disposition === "CanonicalProseOnly" ? record.destination !== null : typeof record.destination !== "string") {
      errors.push(`${record.source_id}: disposition has an invalid destination`);
    }
  }
  if (value.coverage?.classified_source_items !== (value.records || []).length) errors.push("classified count differs from records");
  if (value.coverage?.unclassified_source_items !== 0) errors.push("classification reports unclassified TeX candidates");
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
    process.stdout.write(`generated ${value.records.length} reviewed TeX classifications; Formal Gate A remains ${value.formal_gate_a.status}\n`);
    return;
  }
  if (command === "check") {
    const actual = JSON.parse(fs.readFileSync(outputPath, "utf8"));
    const expected = generateClassification();
    const errors = validateShape(actual, schema);
    if (`${JSON.stringify(actual, null, 2)}\n` !== `${JSON.stringify(expected, null, 2)}\n`) {
      errors.push("committed TeX classification does not exactly regenerate from the pinned inventory, source, and policy");
    }
    if (errors.length > 0) throw new Error(errors.join("\n"));
    process.stdout.write(`predecessor TeX classification regenerates exactly (${actual.records.length} reviewed; Gate A ${actual.formal_gate_a.status})\n`);
    return;
  }
  throw new Error(`unknown command ${command}; expected generate or check`);
}

module.exports = { classification, generateClassification, normalizeUnit, referencedLabels, sha256, validateShape };

if (require.main === module) {
  try {
    main();
  } catch (error) {
    process.stderr.write(`predecessor TeX classification: ${error.message}\n`);
    process.exit(1);
  }
}
