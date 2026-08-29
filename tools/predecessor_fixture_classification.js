#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const root = path.resolve(__dirname, "..");
const schemaPath = path.join(root, "formal-successor", "PREDECESSOR_FIXTURE_CLASSIFICATION_SCHEMA.json");
const outputPath = path.join(root, "formal-successor", "PREDECESSOR_FIXTURE_CLASSIFICATION.json");

function sha256(value) { return crypto.createHash("sha256").update(value).digest("hex"); }
function normalizeNewlines(value) { return value.replace(/\r\n?/gu, "\n"); }
function lineNumber(text, offset) { return normalizeNewlines(text.slice(0, offset)).split("\n").length; }
function gitBlob(revision, relativePath) {
  const result = spawnSync("git", ["show", `${revision}:${relativePath}`], { cwd: root, encoding: "buffer", maxBuffer: 128 * 1024 * 1024, windowsHide: true });
  if (result.status !== 0) throw new Error(`cannot read ${relativePath} at ${revision}: ${result.stderr.toString("utf8").trim()}`);
  return result.stdout;
}
function exactToken(text, token) {
  return new RegExp(`(?<![A-Za-z0-9_])${token.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}(?![A-Za-z0-9_])`, "u").test(text);
}
function maskRust(text) {
  const out = [...text];
  let i = 0;
  const blank = (from, to) => { for (let j = from; j < to; j += 1) if (out[j] !== "\n") out[j] = " "; };
  while (i < text.length) {
    if (text.startsWith("//", i)) { const end = text.indexOf("\n", i); blank(i, end < 0 ? text.length : end); i = end < 0 ? text.length : end; continue; }
    if (text.startsWith("/*", i)) { let depth = 1; let j = i + 2; while (j < text.length && depth > 0) { if (text.startsWith("/*", j)) { depth += 1; j += 2; } else if (text.startsWith("*/", j)) { depth -= 1; j += 2; } else j += 1; } blank(i, j); i = j; continue; }
    if (text[i] === "r") {
      const raw = text.slice(i).match(/^r(#{0,16})"/u);
      if (raw) { const close = `"${raw[1]}`; const start = i + raw[0].length; const found = text.indexOf(close, start); const end = found < 0 ? text.length : found + close.length; blank(i, end); i = end; continue; }
    }
    if (text[i] === '"') { let j = i + 1; while (j < text.length) { if (text[j] === "\\") j += 2; else if (text[j] === '"') { j += 1; break; } else j += 1; } blank(i, j); i = j; continue; }
    if (text[i] === "'" && /^'(?:\\.|[^'\\\n])'/u.test(text.slice(i))) { const match = text.slice(i).match(/^'(?:\\.|[^'\\\n])'/u)[0]; blank(i, i + match.length); i += match.length; continue; }
    i += 1;
  }
  return out.join("");
}
function matchingBrace(masked, open) {
  let depth = 0;
  for (let index = open; index < masked.length; index += 1) {
    if (masked[index] === "{") depth += 1;
    else if (masked[index] === "}") { depth -= 1; if (depth === 0) return index; }
  }
  throw new Error(`unclosed Rust function body at offset ${open}`);
}
function testDeclarations(relativePath, bytes, moduleSourceId) {
  const text = normalizeNewlines(bytes.toString("utf8"));
  const masked = maskRust(text);
  const attribute = /#\s*\[\s*(?:tokio::)?test(?:\s*\([^\]]*\))?\s*\]/gu;
  const records = [];
  for (const match of masked.matchAll(attribute)) {
    const tail = masked.slice(match.index + match[0].length);
    const fn = tail.match(/^(?:\s*#\s*\[[^\]]*\])*\s*(?:(?:pub(?:\([^)]*\))?|unsafe|async|const)\s+)*fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^>{}]*>)?\s*\([^)]*\)[^{;]*\{/u);
    if (!fn) continue;
    const fnOffset = match.index + match[0].length + fn.index + fn[0].indexOf("fn ");
    const open = match.index + match[0].length + fn.index + fn[0].lastIndexOf("{");
    const close = matchingBrace(masked, open);
    const sourceText = text.slice(fnOffset, close + 1);
    records.push({
      test_function: fn[1],
      test_path: relativePath,
      declaration_line: lineNumber(text, fnOffset),
      end_line: lineNumber(text, close),
      source_sha256: sha256(Buffer.from(sourceText, "utf8")),
      body_code: masked.slice(open + 1, close),
      module_source_id: moduleSourceId,
    });
  }
  return records;
}
function loadJson(relativePath) { const bytes = fs.readFileSync(path.join(root, relativePath)); return { bytes, value: JSON.parse(bytes.toString("utf8")) }; }
function excerpt(lines, source) { return lines.slice(source.start_line - 1, source.end_line).join("\n"); }
function eligibleSymbol(name) { return typeof name === "string" && /^[A-Z][A-Za-z0-9]{5,}$/u.test(name); }
function countBy(records, key) {
  const counts = {}; for (const record of records) { const value = key(record); counts[value] = (counts[value] || 0) + 1; }
  return Object.fromEntries(Object.entries(counts).sort(([a], [b]) => a.localeCompare(b)));
}

function generateClassification() {
  const schemaLoaded = loadJson("formal-successor/PREDECESSOR_FIXTURE_CLASSIFICATION_SCHEMA.json");
  const inventoryLoaded = loadJson("formal-successor/PREDECESSOR_INVENTORY.json");
  const texLoaded = loadJson("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json");
  const implementationLoaded = loadJson("formal-successor/PREDECESSOR_IMPLEMENTATION_CLASSIFICATION.json");
  const schema = schemaLoaded.value; const inventory = inventoryLoaded.value; const tex = texLoaded.value; const implementation = implementationLoaded.value;
  if (inventory.generated_from.predecessor_commit !== schema.inputs.predecessor_commit) throw new Error("inventory predecessor coordinate differs from fixture schema");
  const revision = schema.inputs.predecessor_commit;
  const moduleItems = inventory.items.filter((item) => item.source_class === "rust.conformance_module");
  const fixtureItems = inventory.items.filter((item) => item.source_class === "conformance.fixture");
  const repositoryRustItems = inventory.items.filter((item) => item.category === "repository_file" && item.source.path.endsWith(".rs"));
  const sourceItemByPath = new Map(repositoryRustItems.map((item) => [item.source.path, item]));
  const tests = repositoryRustItems.flatMap((item) => testDeclarations(item.source.path, gitBlob(revision, item.source.path), item.id));
  const testsByName = new Map();
  for (const test of tests) { const values = testsByName.get(test.test_function) || []; values.push(test); testsByName.set(test.test_function, values); }
  const statusBytes = gitBlob(revision, "CONFORMANCE_STATUS.md");
  const statusLines = normalizeNewlines(statusBytes.toString("utf8")).split("\n");
  const texBytes = fs.readFileSync(path.join(root, schema.inputs.canonical_tex_path));
  const texLines = normalizeNewlines(texBytes.toString("utf8")).split("\n");
  const texTexts = new Map(tex.records.map((record) => [record.source_id, excerpt(texLines, record.source)]));
  const implementationById = new Map(implementation.records.map((record) => [record.source_id, record]));
  const publicSymbols = implementation.records.filter((record) => record.source_class === "rust.public_item" && eligibleSymbol(record.context?.name));
  const fixtureFileItems = inventory.items.filter((item) => item.source_class === "schema.fixture");

  function executionRoute(test, origins) {
    const implementationTargets = publicSymbols.filter((record) => exactToken(test.body_code, record.context.name)).map((record) => ({
      target_id: record.source_id,
      symbol: record.context.name,
      target_path: record.source.path,
      edge_kind: "ExactRustTestBodyPublicSymbolIncidence",
      authority: schema.edge_grammar.authority,
    })).sort((a, b) => a.target_id.localeCompare(b.target_id));
    return {
      test_path: test.test_path,
      test_function: test.test_function,
      declaration_line: test.declaration_line,
      end_line: test.end_line,
      source_sha256: test.source_sha256,
      module_source_id: test.module_source_id,
      route_origins: [...origins].sort(),
      implementation_targets: implementationTargets,
    };
  }

  const fixtureRecords = fixtureItems.map((item) => {
    const fixtureId = item.context.fixture_id;
    const rowText = item.context.status_row ? statusLines[item.context.status_row.line - 1] : "";
    const routes = new Map();
    for (const [name, matches] of testsByName) {
      if (!rowText || !exactToken(rowText, name)) continue;
      if (matches.length !== 1) throw new Error(`${item.id}: status witness ${name} resolves to ${matches.length} tests`);
      routes.set(`${matches[0].test_path}\0${name}`, { test: matches[0], origins: new Set(["status_exact_test_name"]) });
    }
    const registry = item.context.registry_entry;
    if (registry) {
      const matches = tests.filter((test) => test.test_path === registry.test_path && test.test_function === registry.test_function);
      if (matches.length !== 1) throw new Error(`${item.id}: registry route resolves to ${matches.length} tests`);
      const key = `${registry.test_path}\0${registry.test_function}`;
      const route = routes.get(key) || { test: matches[0], origins: new Set() };
      route.origins.add("registry_exact_test_route"); routes.set(key, route);
    }
    const executionRoutes = [...routes.values()].map(({ test, origins }) => executionRoute(test, origins)).sort((a, b) => `${a.test_path}\0${a.test_function}`.localeCompare(`${b.test_path}\0${b.test_function}`));
    const witnessFiles = fixtureFileItems.filter((candidate) => rowText.includes(candidate.source.path)).map((candidate) => ({ target_id: candidate.id, target_path: candidate.source.path, edge_kind: "ExactFixturePathOccurrence", authority: schema.edge_grammar.authority })).sort((a, b) => a.target_id.localeCompare(b.target_id));
    const claimEdges = tex.records.filter((target) => exactToken(texTexts.get(target.source_id), fixtureId)).map((target) => ({ target_id: target.source_id, target_disposition: target.disposition, edge_kind: "ExactTeXFixtureLabelOccurrence", authority: schema.edge_grammar.authority })).sort((a, b) => a.target_id.localeCompare(b.target_id));
    const implementationTargets = [...new Map(executionRoutes.flatMap((route) => route.implementation_targets).map((edge) => [edge.target_id, edge])).values()].sort((a, b) => a.target_id.localeCompare(b.target_id));
    const status = item.context.status_row?.status_label || null;
    const fixtureRole = executionRoutes.length > 0 ? "statused_executable_fixture" : witnessFiles.length > 0 ? "statused_nonfunction_witness_fixture" : item.context.status_row ? "statused_unresolved_fixture" : "registry_without_status_fixture";
    return {
      source_id: item.id,
      source_class: item.source_class,
      source: item.source,
      context: item.context,
      fixture_role: fixtureRole,
      disposition: "LegacyObligation",
      destination: `${schema.destination_roots.LegacyObligation}/${fixtureId}/${item.id}`,
      predecessor_status_assertion: status,
      execution_routes: executionRoutes,
      witness_file_edges: witnessFiles,
      claim_edges: claimEdges,
      implementation_targets: implementationTargets,
      relation_status: executionRoutes.length > 0 ? "exact_executable_route_candidate" : witnessFiles.length > 0 ? "exact_nonfunction_witness_candidate" : "no_exact_executable_or_file_witness_observed",
      successor_standing: schema.standing_law.successor_standing,
      review_status: "reviewed",
      review_basis: "exact_fixture_occurrence_and_pinned_execution_route_policy_v1",
      rationale: "The exact predecessor status occurrence and any source-bound witness routes are retained without converting status, execution, or incidence into successor warrant.",
    };
  }).sort((a, b) => a.source_id.localeCompare(b.source_id));

  const fixtureByTest = new Map();
  for (const fixture of fixtureRecords) for (const route of fixture.execution_routes) {
    const key = `${route.test_path}\0${route.test_function}`; const ids = fixtureByTest.get(key) || new Set(); ids.add(fixture.source_id); fixtureByTest.set(key, ids);
  }
  const moduleRecords = moduleItems.map((item) => {
    const declarations = tests.filter((test) => test.test_path === item.source.path).map((test) => ({ test_function: test.test_function, declaration_line: test.declaration_line, end_line: test.end_line, source_sha256: test.source_sha256 })).sort((a, b) => a.declaration_line - b.declaration_line);
    const fixtureEdges = declarations.flatMap((test) => [...(fixtureByTest.get(`${item.source.path}\0${test.test_function}`) || [])].map((sourceId) => ({ fixture_source_id: sourceId, test_function: test.test_function, edge_kind: "ExactFixtureTestRoute" }))).sort((a, b) => `${a.fixture_source_id}\0${a.test_function}`.localeCompare(`${b.fixture_source_id}\0${b.test_function}`));
    const claimEdges = [...new Map(fixtureEdges.flatMap((edge) => fixtureRecords.find((fixture) => fixture.source_id === edge.fixture_source_id).claim_edges).map((edge) => [edge.target_id, edge])).values()].sort((a, b) => a.target_id.localeCompare(b.target_id));
    const implementationTargets = [...new Map(declarations.flatMap((declaration) => {
      const test = tests.find((candidate) => candidate.test_path === item.source.path && candidate.test_function === declaration.test_function);
      return executionRoute(test, ["module_test_declaration"]).implementation_targets;
    }).map((edge) => [edge.target_id, edge])).values()].sort((a, b) => a.target_id.localeCompare(b.target_id));
    const exercised = fixtureEdges.length > 0;
    return {
      source_id: item.id,
      source_class: item.source_class,
      source: item.source,
      module_role: exercised ? "exercised_conformance_module" : "unrouted_conformance_module",
      disposition: exercised ? "LegacyObligation" : "ImplementationOnly",
      destination: `${schema.destination_roots[exercised ? "LegacyObligation" : "ImplementationOnly"]}/${item.source.path}/${item.id}`,
      test_declarations: declarations,
      fixture_edges: fixtureEdges,
      claim_edges: claimEdges,
      implementation_targets: implementationTargets,
      successor_standing: schema.standing_law.successor_standing,
      review_status: "reviewed",
      review_basis: "pinned_rust_test_declaration_and_fixture_route_policy_v1",
      rationale: exercised ? "Exact fixture routes exercise declarations in this conformance module; the resulting claim and implementation incidences remain candidate evidence only." : "The module and all of its test declarations are retained, but no exact status-row or registry route was observed; future fixture relation remains Unknown.",
    };
  }).sort((a, b) => a.source_id.localeCompare(b.source_id));
  for (const record of [...fixtureRecords, ...moduleRecords]) for (const edge of record.implementation_targets) if (!implementationById.has(edge.target_id)) throw new Error(`${record.source_id}: unknown implementation target ${edge.target_id}`);
  const records = [...fixtureRecords, ...moduleRecords].sort((a, b) => a.source_id.localeCompare(b.source_id));
  return {
    schema: 1,
    status: "reviewed_phase_a_fixture_classification_not_semantic_authority",
    generated_from: {
      schema_path: path.relative(root, schemaPath).replace(/\\/gu, "/"), schema_sha256: sha256(schemaLoaded.bytes),
      inventory_path: schema.inputs.inventory_path, inventory_sha256: sha256(inventoryLoaded.bytes),
      tex_classification_path: schema.inputs.tex_classification_path, tex_classification_sha256: sha256(texLoaded.bytes),
      implementation_classification_path: schema.inputs.implementation_classification_path, implementation_classification_sha256: sha256(implementationLoaded.bytes),
      predecessor_commit: revision,
    },
    coverage: {
      classified_source_items: records.length,
      source_class_counts: countBy(records, (record) => record.source_class),
      disposition_counts: countBy(records, (record) => record.disposition),
      fixture_role_counts: countBy(fixtureRecords, (record) => record.fixture_role),
      module_role_counts: countBy(moduleRecords, (record) => record.module_role),
      predecessor_status_counts: countBy(fixtureRecords, (record) => record.predecessor_status_assertion || "NONE"),
      test_declarations_in_conformance_modules: moduleRecords.reduce((sum, record) => sum + record.test_declarations.length, 0),
      exact_execution_routes: fixtureRecords.reduce((sum, record) => sum + record.execution_routes.length, 0),
      registered_execution_routes: fixtureRecords.flatMap((record) => record.execution_routes).filter((route) => route.route_origins.includes("registry_exact_test_route")).length,
      exact_fixture_file_edges: fixtureRecords.reduce((sum, record) => sum + record.witness_file_edges.length, 0),
      exact_tex_claim_edges: fixtureRecords.reduce((sum, record) => sum + record.claim_edges.length, 0),
      exact_implementation_target_edges: fixtureRecords.reduce((sum, record) => sum + record.implementation_targets.length, 0),
      registry_without_status_rows: fixtureRecords.filter((record) => record.fixture_role === "registry_without_status_fixture").length,
      successor_standing_counts: countBy(records, (record) => record.successor_standing),
      unclassified_source_items: 0,
    },
    local_gate: { id: "FORMAL-A-FIXTURE-INVENTORY", status: "READY_FOR_INDEPENDENT_CHECK", reason: "Every admitted fixture-row occurrence and conformance module has a reviewed authority-separated role, with exact source routes and explicit Unknown successor standing." },
    formal_gate_a: { status: "PENDING", reason: "The joined TeX, implementation, and fixture coverage relation still requires an independent global check." },
    records,
  };
}

function validateShape(value, schema) {
  const errors = []; const ids = new Set(); let prior = "";
  if (value.schema !== 1) errors.push("fixture classification schema must be 1");
  if (value.formal_gate_a?.status !== "PENDING") errors.push("fixture classification must not self-promote Formal Gate A");
  for (const record of value.records || []) {
    if (ids.has(record.source_id)) errors.push(`duplicate source_id ${record.source_id}`); ids.add(record.source_id);
    if (prior && prior.localeCompare(record.source_id) > 0) errors.push(`records unsorted at ${record.source_id}`); prior = record.source_id;
    if (!schema.used_dispositions.includes(record.disposition)) errors.push(`${record.source_id}: invalid disposition`);
    if (record.review_status !== "reviewed") errors.push(`${record.source_id}: review remains open`);
    if (record.successor_standing !== "Unknown") errors.push(`${record.source_id}: successor standing was promoted`);
    if (record.source_class === "conformance.fixture" && !schema.fixture_roles.includes(record.fixture_role)) errors.push(`${record.source_id}: invalid fixture role`);
    if (record.source_class === "rust.conformance_module" && !schema.module_roles.includes(record.module_role)) errors.push(`${record.source_id}: invalid module role`);
  }
  if (value.coverage?.classified_source_items !== (value.records || []).length || value.coverage?.unclassified_source_items !== 0) errors.push("classification coverage differs from records");
  return errors;
}
function main() {
  const command = process.argv[2] || "check"; const schema = JSON.parse(fs.readFileSync(schemaPath, "utf8"));
  if (command === "generate") { const value = generateClassification(); const errors = validateShape(value, schema); if (errors.length) throw new Error(errors.join("\n")); fs.writeFileSync(outputPath, `${JSON.stringify(value, null, 2)}\n`); process.stdout.write(`generated ${value.records.length} fixture/module classifications; ${value.coverage.exact_execution_routes} exact routes; Formal Gate A PENDING\n`); return; }
  if (command === "check") { const actual = JSON.parse(fs.readFileSync(outputPath, "utf8")); const expected = generateClassification(); const errors = validateShape(actual, schema); if (`${JSON.stringify(actual, null, 2)}\n` !== `${JSON.stringify(expected, null, 2)}\n`) errors.push("committed fixture classification does not exactly regenerate"); if (errors.length) throw new Error(errors.join("\n")); process.stdout.write(`fixture classification regenerates exactly (${actual.records.length} records; Formal Gate A ${actual.formal_gate_a.status})\n`); return; }
  throw new Error(`unknown command ${command}; expected generate or check`);
}
module.exports = { exactToken, generateClassification, maskRust, testDeclarations, validateShape };
if (require.main === module) { try { main(); } catch (error) { process.stderr.write(`predecessor fixture classification: ${error.message}\n`); process.exit(1); } }
