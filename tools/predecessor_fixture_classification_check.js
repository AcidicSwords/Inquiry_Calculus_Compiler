#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const root = path.resolve(__dirname, "..");
const schemaPath = path.join(root, "formal-successor", "PREDECESSOR_FIXTURE_CLASSIFICATION_SCHEMA.json");
const classificationPath = path.join(root, "formal-successor", "PREDECESSOR_FIXTURE_CLASSIFICATION.json");
const digest = (value) => crypto.createHash("sha256").update(value).digest("hex");
const normalize = (value) => value.replace(/\r\n?/gu, "\n");
function blob(revision, relativePath) {
  const result = spawnSync("git", ["show", `${revision}:${relativePath}`], { cwd: root, encoding: "buffer", maxBuffer: 128 * 1024 * 1024, windowsHide: true });
  if (result.status !== 0) throw new Error(`cannot read pinned ${relativePath}`);
  return result.stdout;
}
function token(text, value) { return new RegExp(`(?<![A-Za-z0-9_])${value.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}(?![A-Za-z0-9_])`, "u").test(text); }
function mask(text) {
  const out = [...text]; let i = 0; const erase = (a, b) => { for (let j = a; j < b; j += 1) if (out[j] !== "\n") out[j] = " "; };
  while (i < text.length) {
    if (text.startsWith("//", i)) { const e = text.indexOf("\n", i); erase(i, e < 0 ? text.length : e); i = e < 0 ? text.length : e; continue; }
    if (text.startsWith("/*", i)) { let d = 1; let j = i + 2; while (j < text.length && d) { if (text.startsWith("/*", j)) { d += 1; j += 2; } else if (text.startsWith("*/", j)) { d -= 1; j += 2; } else j += 1; } erase(i, j); i = j; continue; }
    if (text[i] === "r") { const m = text.slice(i).match(/^r(#{0,16})"/u); if (m) { const close = `"${m[1]}`; const found = text.indexOf(close, i + m[0].length); const e = found < 0 ? text.length : found + close.length; erase(i, e); i = e; continue; } }
    if (text[i] === '"') { let j = i + 1; while (j < text.length) { if (text[j] === "\\") j += 2; else if (text[j] === '"') { j += 1; break; } else j += 1; } erase(i, j); i = j; continue; }
    if (text[i] === "'") { const m = text.slice(i).match(/^'(?:\\.|[^'\\\n])'/u); if (m) { erase(i, i + m[0].length); i += m[0].length; continue; } }
    i += 1;
  }
  return out.join("");
}
function lineAt(text, offset) { return normalize(text.slice(0, offset)).split("\n").length; }
function closeBrace(code, open) { let depth = 0; for (let i = open; i < code.length; i += 1) { if (code[i] === "{") depth += 1; else if (code[i] === "}" && --depth === 0) return i; } throw new Error("unclosed test body"); }
function parseTests(relativePath, bytes, moduleSourceId) {
  const text = normalize(bytes.toString("utf8")); const code = mask(text); const result = [];
  for (const attribute of code.matchAll(/#\s*\[\s*(?:tokio::)?test(?:\s*\([^\]]*\))?\s*\]/gu)) {
    const offset = attribute.index + attribute[0].length; const match = code.slice(offset).match(/^(?:\s*#\s*\[[^\]]*\])*\s*(?:(?:pub(?:\([^)]*\))?|unsafe|async|const)\s+)*fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^>{}]*>)?\s*\([^)]*\)[^{;]*\{/u);
    if (!match) continue; const fnStart = offset + match.index + match[0].indexOf("fn "); const open = offset + match.index + match[0].lastIndexOf("{"); const close = closeBrace(code, open); const sourceText = text.slice(fnStart, close + 1);
    result.push({ test_function: match[1], test_path: relativePath, declaration_line: lineAt(text, fnStart), end_line: lineAt(text, close), source_sha256: digest(Buffer.from(sourceText)), body: code.slice(open + 1, close), module_source_id: moduleSourceId });
  }
  return result;
}
function clone(value) { return JSON.parse(JSON.stringify(value)); }
function keyRoute(value) { return `${value.test_path}\0${value.test_function}`; }
function keyEdge(value) { return value.target_id; }
function sameKeys(actual, expected, key) { return JSON.stringify(actual.map(key).sort()) === JSON.stringify(expected.map(key).sort()); }

function inspect(classification, schema, inventory, tex, implementation, inputBytes) {
  const errors = []; const revision = schema.inputs.predecessor_commit;
  if (classification.formal_gate_a?.status !== "PENDING") errors.push("fixture overlay self-promotes Formal Gate A");
  if (classification.generated_from?.schema_sha256 !== digest(inputBytes.schema)) errors.push("fixture schema digest is detached");
  if (classification.generated_from?.inventory_sha256 !== digest(inputBytes.inventory)) errors.push("inventory digest is detached");
  if (classification.generated_from?.tex_classification_sha256 !== digest(inputBytes.tex)) errors.push("TeX overlay digest is detached");
  if (classification.generated_from?.implementation_classification_sha256 !== digest(inputBytes.implementation)) errors.push("implementation overlay digest is detached");
  const admittedItems = inventory.items.filter((item) => schema.inputs.admitted_source_classes.includes(item.source_class));
  const bySource = new Map((classification.records || []).map((record) => [record.source_id, record]));
  if (bySource.size !== admittedItems.length || (classification.records || []).length !== admittedItems.length) errors.push("classification does not contain one record per admitted source identity");
  for (const item of admittedItems) { const record = bySource.get(item.id); if (!record) errors.push(`missing source identity ${item.id}`); else if (JSON.stringify(record.source) !== JSON.stringify(item.source)) errors.push(`${item.id}: source identity differs from inventory`); }

  const rustModules = inventory.items.filter((item) => item.category === "repository_file" && item.source.path.endsWith(".rs"));
  const tests = rustModules.flatMap((item) => parseTests(item.source.path, blob(revision, item.source.path), item.id));
  const testByName = new Map(); for (const test of tests) { const a = testByName.get(test.test_function) || []; a.push(test); testByName.set(test.test_function, a); }
  const statusLines = normalize(blob(revision, "CONFORMANCE_STATUS.md").toString("utf8")).split("\n");
  const texLines = normalize(fs.readFileSync(path.join(root, schema.inputs.canonical_tex_path), "utf8")).split("\n");
  const texText = new Map(tex.records.map((record) => [record.source_id, texLines.slice(record.source.start_line - 1, record.source.end_line).join("\n")]));
  const publicSymbols = implementation.records.filter((record) => record.source_class === "rust.public_item" && /^[A-Z][A-Za-z0-9]{5,}$/u.test(record.context?.name || ""));
  const fixtureFiles = inventory.items.filter((item) => item.source_class === "schema.fixture");
  const fixtureRecords = (classification.records || []).filter((record) => record.source_class === "conformance.fixture");
  for (const record of fixtureRecords) {
    if (record.review_status !== "reviewed" || record.disposition !== "LegacyObligation") errors.push(`${record.source_id}: fixture role is not reviewed LegacyObligation`);
    if (record.successor_standing !== "Unknown") errors.push(`${record.source_id}: status or route promoted successor standing`);
    const row = record.context.status_row ? statusLines[record.context.status_row.line - 1] : "";
    const expectedStatus = record.context.status_row?.status_label || null;
    if (record.predecessor_status_assertion !== expectedStatus) errors.push(`${record.source_id}: predecessor status differs from source`);
    const expectedRoutes = new Map();
    for (const [name, matches] of testByName) if (row && token(row, name)) {
      if (matches.length !== 1) errors.push(`${record.source_id}: exact test name ${name} is ambiguous`);
      else expectedRoutes.set(keyRoute(matches[0]), { test: matches[0], origins: ["status_exact_test_name"] });
    }
    if (record.context.registry_entry) {
      const reg = record.context.registry_entry; const matches = tests.filter((test) => test.test_path === reg.test_path && test.test_function === reg.test_function);
      if (matches.length !== 1) errors.push(`${record.source_id}: registry route is not one pinned test`);
      else { const k = keyRoute(matches[0]); const entry = expectedRoutes.get(k) || { test: matches[0], origins: [] }; entry.origins.push("registry_exact_test_route"); expectedRoutes.set(k, entry); }
    }
    if (!sameKeys(record.execution_routes || [], [...expectedRoutes.values()].map((x) => x.test), keyRoute)) errors.push(`${record.source_id}: execution-route field differs from exact source incidence`);
    for (const route of record.execution_routes || []) {
      const expected = expectedRoutes.get(keyRoute(route));
      if (!expected) continue;
      for (const field of ["declaration_line", "end_line", "source_sha256", "module_source_id"]) if (route[field] !== expected.test[field]) errors.push(`${record.source_id}: route ${route.test_function} has detached ${field}`);
      if (JSON.stringify([...new Set(route.route_origins)].sort()) !== JSON.stringify([...new Set(expected.origins)].sort())) errors.push(`${record.source_id}: route origins differ`);
      const expectedTargets = publicSymbols.filter((target) => token(expected.test.body, target.context.name)).map((target) => target.source_id);
      if (!sameKeys(route.implementation_targets || [], expectedTargets.map((target_id) => ({ target_id })), keyEdge)) errors.push(`${record.source_id}: implementation targets differ for ${route.test_function}`);
    }
    const expectedClaims = tex.records.filter((target) => token(texText.get(target.source_id), record.context.fixture_id)).map((target) => target.source_id);
    if (!sameKeys(record.claim_edges || [], expectedClaims.map((target_id) => ({ target_id })), keyEdge)) errors.push(`${record.source_id}: TeX claim edges differ from exact fixture-label incidence`);
    const expectedFiles = fixtureFiles.filter((item) => row.includes(item.source.path)).map((item) => item.id);
    if (!sameKeys(record.witness_file_edges || [], expectedFiles.map((target_id) => ({ target_id })), keyEdge)) errors.push(`${record.source_id}: fixture-file edges differ from exact path incidence`);
    const expectedImpl = [...new Set((record.execution_routes || []).flatMap((route) => (route.implementation_targets || []).map((edge) => edge.target_id)))];
    if (!sameKeys(record.implementation_targets || [], expectedImpl.map((target_id) => ({ target_id })), keyEdge)) errors.push(`${record.source_id}: aggregate implementation targets differ`);
  }

  const moduleRecords = (classification.records || []).filter((record) => record.source_class === "rust.conformance_module");
  for (const record of moduleRecords) {
    if (record.review_status !== "reviewed" || record.successor_standing !== "Unknown") errors.push(`${record.source_id}: module review or standing is invalid`);
    const expectedTests = tests.filter((test) => test.test_path === record.source.path);
    if (!sameKeys(record.test_declarations || [], expectedTests, (value) => `${value.test_function}\0${value.declaration_line}\0${value.source_sha256}`)) errors.push(`${record.source_id}: test declarations differ from pinned module`);
    const expectedFixtureEdges = fixtureRecords.flatMap((fixture) => (fixture.execution_routes || []).filter((route) => route.test_path === record.source.path).map((route) => `${fixture.source_id}\0${route.test_function}`));
    if (JSON.stringify((record.fixture_edges || []).map((edge) => `${edge.fixture_source_id}\0${edge.test_function}`).sort()) !== JSON.stringify(expectedFixtureEdges.sort())) errors.push(`${record.source_id}: fixture-to-module routes differ`);
    const expectedRole = expectedFixtureEdges.length ? "exercised_conformance_module" : "unrouted_conformance_module";
    if (record.module_role !== expectedRole) errors.push(`${record.source_id}: module role differs from route field`);
  }

  const expected = schema.expected_boundary_at_pinned_inputs;
  const observed = {
    classified_source_items: (classification.records || []).length,
    conformance_modules: moduleRecords.length,
    fixture_row_occurrences: fixtureRecords.filter((record) => record.context.status_row).length,
    distinct_fixture_labels: new Set(fixtureRecords.map((record) => record.context.fixture_id)).size,
    repeated_label_extra_occurrences: fixtureRecords.length - new Set(fixtureRecords.map((record) => record.context.fixture_id)).size,
    predecessor_pass_rows: fixtureRecords.filter((record) => record.predecessor_status_assertion === "PASS").length,
    predecessor_pending_rows: fixtureRecords.filter((record) => record.predecessor_status_assertion === "PENDING").length,
    test_declarations_in_conformance_modules: moduleRecords.reduce((sum, record) => sum + record.test_declarations.length, 0),
    statused_executable_fixtures: fixtureRecords.filter((record) => record.fixture_role === "statused_executable_fixture").length,
    statused_nonfunction_witness_fixtures: fixtureRecords.filter((record) => record.fixture_role === "statused_nonfunction_witness_fixture").length,
    statused_unresolved_fixtures: fixtureRecords.filter((record) => record.fixture_role === "statused_unresolved_fixture").length,
    exact_execution_routes: fixtureRecords.reduce((sum, record) => sum + record.execution_routes.length, 0),
    registered_execution_routes: fixtureRecords.flatMap((record) => record.execution_routes).filter((route) => route.route_origins.includes("registry_exact_test_route")).length,
    exact_fixture_file_edges: fixtureRecords.reduce((sum, record) => sum + record.witness_file_edges.length, 0),
    exact_tex_claim_edges: fixtureRecords.reduce((sum, record) => sum + record.claim_edges.length, 0),
    exact_implementation_target_edges: fixtureRecords.reduce((sum, record) => sum + record.implementation_targets.length, 0),
    registry_without_status_rows: fixtureRecords.filter((record) => record.fixture_role === "registry_without_status_fixture").length,
    unknown_successor_standing_records: (classification.records || []).filter((record) => record.successor_standing === "Unknown").length,
  };
  if (JSON.stringify(observed) !== JSON.stringify(expected)) errors.push(`observed boundary differs from schema: ${JSON.stringify(observed)}`);
  if (classification.coverage?.classified_source_items !== observed.classified_source_items || classification.coverage?.unclassified_source_items !== 0) errors.push("reported classification coverage differs");
  return errors;
}
function requireRejected(name, candidate, context) { if (inspect(candidate, ...context).length === 0) throw new Error(`mutation breaker escaped: ${name}`); }
function main() {
  const inputBytes = {
    schema: fs.readFileSync(schemaPath),
    inventory: fs.readFileSync(path.join(root, "formal-successor", "PREDECESSOR_INVENTORY.json")),
    tex: fs.readFileSync(path.join(root, "formal-successor", "PREDECESSOR_TEX_CLASSIFICATION.json")),
    implementation: fs.readFileSync(path.join(root, "formal-successor", "PREDECESSOR_IMPLEMENTATION_CLASSIFICATION.json")),
  };
  const schema = JSON.parse(inputBytes.schema); const inventory = JSON.parse(inputBytes.inventory); const tex = JSON.parse(inputBytes.tex); const implementation = JSON.parse(inputBytes.implementation); const classification = JSON.parse(fs.readFileSync(classificationPath, "utf8")); const context = [schema, inventory, tex, implementation, inputBytes];
  const errors = inspect(classification, ...context); if (errors.length) throw new Error(errors.join("\n"));
  const mutations = [];
  const addMutation = (name, change) => { const value = clone(classification); change(value); mutations.push([name, value]); };
  addMutation("deleted fixture", (v) => v.records.splice(v.records.findIndex((r) => r.source_class === "conformance.fixture"), 1));
  addMutation("deleted module", (v) => v.records.splice(v.records.findIndex((r) => r.source_class === "rust.conformance_module"), 1));
  addMutation("collapsed repeated label", (v) => { const repeated = v.records.filter((r) => r.context?.fixture_id === "PROVIDER-001A"); v.records.splice(v.records.indexOf(repeated[1]), 1); });
  addMutation("redirected registry test", (v) => { const r = v.records.find((x) => x.execution_routes?.some((e) => e.route_origins.includes("registry_exact_test_route"))); r.execution_routes.find((e) => e.route_origins.includes("registry_exact_test_route")).test_function = "missing_test"; });
  addMutation("deleted execution route", (v) => { v.records.find((r) => r.execution_routes?.length).execution_routes.pop(); });
  addMutation("fabricated TeX edge", (v) => { v.records.find((r) => r.source_class === "conformance.fixture").claim_edges.push({ target_id: "FABRICATED" }); });
  addMutation("fabricated implementation edge", (v) => { v.records.find((r) => r.execution_routes?.length).execution_routes[0].implementation_targets.push({ target_id: "FABRICATED" }); });
  addMutation("promoted successor standing", (v) => { v.records[0].successor_standing = "PASS"; });
  addMutation("altered predecessor status", (v) => { v.records.find((r) => r.predecessor_status_assertion === "PENDING").predecessor_status_assertion = "PASS"; });
  addMutation("detached test digest", (v) => { v.records.find((r) => r.execution_routes?.length).execution_routes[0].source_sha256 = "0".repeat(64); });
  addMutation("erased route origin", (v) => { v.records.find((r) => r.execution_routes?.some((e) => e.route_origins.includes("registry_exact_test_route"))).execution_routes.find((e) => e.route_origins.includes("registry_exact_test_route")).route_origins = []; });
  addMutation("blanket implementation-only", (v) => { for (const r of v.records) r.disposition = "ImplementationOnly"; });
  addMutation("self-promoted Gate A", (v) => { v.formal_gate_a.status = "PASS"; });
  addMutation("detached inventory digest", (v) => { v.generated_from.inventory_sha256 = "f".repeat(64); });
  for (const [name, candidate] of mutations) requireRejected(name, candidate, context);
  process.stdout.write(`independent fixture classification checks passed (${classification.records.length} records; ${classification.coverage.exact_execution_routes} routes; ${mutations.length}/${mutations.length} mutation breakers)\n`);
}
try { main(); } catch (error) { process.stderr.write(`predecessor fixture classification check: ${error.message}\n`); process.exit(1); }
