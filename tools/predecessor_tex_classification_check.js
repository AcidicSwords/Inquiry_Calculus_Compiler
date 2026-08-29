#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const schemaPath = path.join(root, "formal-successor", "PREDECESSOR_TEX_CLASSIFICATION_SCHEMA.json");
const inventoryPath = path.join(root, "formal-successor", "PREDECESSOR_INVENTORY.json");
const classificationPath = path.join(root, "formal-successor", "PREDECESSOR_TEX_CLASSIFICATION.json");

function digest(value) {
  return crypto.createHash("sha256").update(value).digest("hex");
}

function normalize(value) {
  return value.replace(/\r\n?/gu, "\n");
}

function normalizeUnit(value) {
  return normalize(value)
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

function slug(value) {
  const result = value
    .replace(/\\[A-Za-z@]+\*?(?:\[[^\]]*\])?/gu, " ")
    .replace(/[{}$\\]/gu, " ")
    .replace(/[^A-Za-z0-9]+/gu, "-")
    .replace(/^-+|-+$/gu, "")
    .toLowerCase();
  return result || "document";
}

function refs(excerpt) {
  const result = new Set();
  for (const match of excerpt.matchAll(/\\(?:c|C|eq|auto|page)?ref\{([^}]+)\}/gu)) {
    for (const label of match[1].split(",")) {
      if (label.trim()) result.add(label.trim());
    }
  }
  return [...result].sort();
}

function countBy(values, key) {
  const counts = {};
  for (const value of values) counts[key(value)] = (counts[key(value)] || 0) + 1;
  return Object.fromEntries(Object.entries(counts).sort(([left], [right]) => left.localeCompare(right)));
}

const schemaBytes = fs.readFileSync(schemaPath);
const inventoryBytes = fs.readFileSync(inventoryPath);
const classificationBytes = fs.readFileSync(classificationPath);
const schema = JSON.parse(schemaBytes.toString("utf8"));
const inventory = JSON.parse(inventoryBytes.toString("utf8"));
const classification = JSON.parse(classificationBytes.toString("utf8"));
const texBytes = fs.readFileSync(path.join(root, schema.inventory.canonical_tex_path));
const texLines = normalize(texBytes.toString("utf8")).split("\n");
const admitted = new Set(schema.inventory.admitted_source_classes);
const sourceItems = inventory.items.filter((item) => admitted.has(item.source_class));
const sourceById = new Map(sourceItems.map((item) => [item.id, item]));

function excerpt(item) {
  return texLines.slice(item.source.start_line - 1, item.source.end_line).join("\n");
}

function expectedDisposition(item, text) {
  const normalized = normalizeUnit(text);
  if (schema.rules.exact_canonical_context.includes(normalized)) return ["CanonicalProseOnly", null];
  if (
    item.source_class === "tex.declaration" &&
    schema.rules.binding_headings.includes(item.context.heading) &&
    new Set(["theorem", "proposition", "corollary"]).has(item.category)
  ) return ["BindingTheorem", null];
  if (item.source_class === "tex.declaration") {
    return [
      schema.rules.declaration_dispositions[item.category],
      schema.rules.legacy_declaration_status[item.category] || null,
    ];
  }
  const signals = item.context?.normative_signals || [];
  return [
    schema.rules.nondeclaration_disposition,
    signals.length > 0
      ? schema.rules.nondeclaration_status_by_signal.normative_signal_present
      : schema.rules.nondeclaration_status_by_signal.no_normative_signal,
  ];
}

function expectedRole(item, disposition) {
  if (disposition === "CanonicalProseOnly") return "canonical_context";
  if (item.source_class === "tex.declaration") return disposition === "BindingTheorem" ? "binding_declaration" : "standalone_declaration";
  if (item.source_class === "tex.list_item") return "list_component";
  if (item.source_class === "tex.display") return "display_component";
  return "compound_claim_boundary";
}

function validate(value) {
  const errors = [];
  if (schema.schema !== 1 || schema.status !== "phase_a_tex_classification_contract_not_successor_semantics") {
    errors.push("classification schema authority/status is invalid");
  }
  if (digest(texBytes) !== schema.inventory.canonical_tex_sha256) errors.push("canonical TeX is detached from the schema");
  if (value.schema !== 1 || value.status !== "reviewed_phase_a_tex_classification_not_successor_semantics") {
    errors.push("classification artifact status is invalid");
  }
  if (value.generated_from?.schema_sha256 !== digest(schemaBytes)) errors.push("classification schema digest is detached");
  if (value.generated_from?.inventory_sha256 !== digest(inventoryBytes)) errors.push("classification inventory digest is detached");
  if (value.generated_from?.canonical_tex_sha256 !== digest(texBytes)) errors.push("classification TeX digest is detached");
  if (value.local_gate?.id !== "FORMAL-A-TEX-INVENTORY" || value.local_gate?.status !== "READY_FOR_INDEPENDENT_CHECK") {
    errors.push("TeX-local gate state is invalid");
  }
  if (value.formal_gate_a?.status !== "PENDING") errors.push("classification self-promoted Formal Gate A");
  if (!Array.isArray(value.records)) return [...errors, "classification records are not an array"];

  const recordsById = new Map();
  let previous = "";
  for (const [index, record] of value.records.entries()) {
    for (const field of schema.required_record_fields) {
      if (!Object.hasOwn(record, field)) errors.push(`record ${index} is missing ${field}`);
    }
    if (recordsById.has(record.source_id)) errors.push(`duplicate classification ${record.source_id}`);
    recordsById.set(record.source_id, record);
    if (previous && previous.localeCompare(record.source_id) > 0) errors.push(`classification records are unsorted at ${record.source_id}`);
    previous = record.source_id;
    const item = sourceById.get(record.source_id);
    if (!item) {
      errors.push(`foreign classification identity ${record.source_id}`);
      continue;
    }
    const text = excerpt(item);
    const [expected, legacyStatus] = expectedDisposition(item, text);
    const role = expectedRole(item, expected);
    const destinationRoot = schema.destination_roots[expected];
    const destination = destinationRoot === null ? null : `${destinationRoot}/${slug(item.context?.heading || "Document")}/${item.id}`;
    if (record.source_class !== item.source_class) errors.push(`${record.source_id}: source class changed`);
    if (stable(record.source) !== stable(item.source)) errors.push(`${record.source_id}: source coordinate changed`);
    if (stable(record.context) !== stable(item.context || {})) errors.push(`${record.source_id}: source context changed`);
    const sourceDigest = digest(Buffer.from(normalizeUnit(text), "utf8"));
    if (sourceDigest !== item.source.sha256 || record.source_excerpt_sha256 !== sourceDigest) {
      errors.push(`${record.source_id}: source excerpt digest is detached`);
    }
    if (record.disposition !== expected) errors.push(`${record.source_id}: expected disposition ${expected}, got ${record.disposition}`);
    if (record.source_role !== role) errors.push(`${record.source_id}: expected source role ${role}, got ${record.source_role}`);
    if (stable(record.destination) !== stable(destination)) errors.push(`${record.source_id}: destination is not regenerated by its disposition`);
    if (record.review_status !== "reviewed" || record.review_basis !== "exact_source_role_policy_v1") {
      errors.push(`${record.source_id}: review closure/basis is invalid`);
    }
    if (typeof record.rationale !== "string" || record.rationale.length < 40) errors.push(`${record.source_id}: review rationale is absent`);

    if (expected === "LegacyObligation") {
      const obligation = record.legacy_obligation;
      if (!obligation) {
        errors.push(`${record.source_id}: missing LegacyObligation payload`);
      } else {
        if (obligation.candidate_proposition?.language !== "unelaborated-source-claim/v1" ||
            obligation.candidate_proposition?.expression !== `SourceClaim(${JSON.stringify(item.id)})` ||
            obligation.candidate_proposition?.elaboration_status !== "unelaborated") {
          errors.push(`${record.source_id}: candidate proposition is not exact and source-bound`);
        }
        if (obligation.scope?.heading !== (item.context?.heading || "Document") || obligation.scope?.source_role !== role) {
          errors.push(`${record.source_id}: obligation scope is detached`);
        }
        if (stable(obligation.dependencies) !== stable(refs(text))) errors.push(`${record.source_id}: dependency references are incomplete`);
        if (obligation.status !== (legacyStatus || "Ambiguous") || !schema.legacy_statuses.includes(obligation.status)) {
          errors.push(`${record.source_id}: LegacyObligation status is invalid`);
        }
        if (obligation.known_breaker !== null || obligation.breaker_status !== "not_yet_established") {
          errors.push(`${record.source_id}: Phase A fabricated or erased breaker status`);
        }
      }
    } else if (record.legacy_obligation !== undefined) {
      errors.push(`${record.source_id}: non-legacy disposition has an obligation payload`);
    }
  }

  for (const item of sourceItems) if (!recordsById.has(item.id)) errors.push(`missing classification ${item.id}`);
  const records = value.records.filter((record) => sourceById.has(record.source_id));
  if (value.coverage?.classified_source_items !== sourceItems.length || value.coverage?.unclassified_source_items !== 0) {
    errors.push("classification coverage totals are incomplete");
  }
  if (stable(value.coverage?.source_class_counts) !== stable(countBy(sourceItems, (item) => item.source_class))) {
    errors.push("source-class coverage counts are incorrect");
  }
  if (stable(value.coverage?.disposition_counts) !== stable(countBy(records, (record) => record.disposition))) {
    errors.push("disposition counts are incorrect");
  }
  if (stable(value.coverage?.source_role_counts) !== stable(countBy(records, (record) => record.source_role))) {
    errors.push("source-role counts are incorrect");
  }
  const legacy = records.filter((record) => record.legacy_obligation);
  if (stable(value.coverage?.legacy_status_counts) !== stable(countBy(legacy, (record) => record.legacy_obligation.status))) {
    errors.push("legacy-status counts are incorrect");
  }
  const withoutSignals = sourceItems.filter((item) =>
    new Set(["tex.narrative", "tex.list_item"]).has(item.source_class) &&
    (item.context?.normative_signals || []).length === 0 &&
    !schema.rules.exact_canonical_context.includes(normalizeUnit(excerpt(item))));
  if (withoutSignals.length < 1 || withoutSignals.some((item) => recordsById.get(item.id)?.disposition !== "LegacyObligation")) {
    errors.push("keyword absence incorrectly discharges a prose/list claim boundary");
  }
  if (new Set(records.map((record) => record.disposition)).size < 4) errors.push("classification collapsed distinct source roles into too few dispositions");
  if (new Set(records.map((record) => record.source_role)).size < 5) errors.push("classification collapsed distinct source roles");
  return errors;
}

const baseErrors = validate(classification);
if (baseErrors.length > 0) {
  throw new Error(`independent classification validation failed:\n${baseErrors.join("\n")}`);
}

const breakers = [];
function mustReject(name, mutate) {
  const candidate = clone(classification);
  mutate(candidate);
  const errors = validate(candidate);
  if (errors.length === 0) throw new Error(`mutation breaker escaped: ${name}`);
  breakers.push(name);
}

mustReject("whole-section deletion", (value) => {
  const heading = "Minimum conformance suite";
  value.records = value.records.filter((record) => record.context.heading !== heading);
});
mustReject("single identity omission", (value) => value.records.splice(0, 1));
mustReject("duplicate identity", (value) => value.records.push(clone(value.records[0])));
mustReject("foreign identity", (value) => { value.records[0].source_id = "PRED-TEX-FOREIGN"; });
mustReject("keyword-only prose classification", (value) => {
  for (const record of value.records) {
    const item = sourceById.get(record.source_id);
    if (item && new Set(["tex.narrative", "tex.list_item"]).has(item.source_class) &&
        (item.context?.normative_signals || []).length === 0) {
      record.disposition = "CanonicalProseOnly";
      record.destination = null;
      delete record.legacy_obligation;
    }
  }
});
mustReject("blanket theorem promotion", (value) => {
  for (const record of value.records) {
    record.disposition = "FormalTheorem";
    record.destination = `phase:B/Legacy/V20/Claims/all/${record.source_id}`;
    record.source_role = "standalone_declaration";
    delete record.legacy_obligation;
  }
});
mustReject("invalid legacy status", (value) => {
  value.records.find((record) => record.legacy_obligation).legacy_obligation.status = "Accepted";
});
mustReject("lost display boundary", (value) => {
  value.records.find((record) => record.source_class === "tex.display").source_role = "standalone_declaration";
});
mustReject("detached source digest", (value) => { value.records[0].source_excerpt_sha256 = "0".repeat(64); });
mustReject("binding theorem collapse", (value) => {
  value.records.find((record) => record.disposition === "BindingTheorem").disposition = "FormalTheorem";
});
mustReject("Formal Gate A self-promotion", (value) => { value.formal_gate_a.status = "PASS"; });

process.stdout.write(
  `independent TeX classification checks passed (${classification.records.length} reviewed; ${breakers.length}/${breakers.length} mutation breakers; Gate A PENDING)\n`,
);
