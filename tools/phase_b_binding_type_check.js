#!/usr/bin/env node
"use strict";
const childProcess = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const root = path.resolve(__dirname, "..");
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");
const clone = (value) => JSON.parse(JSON.stringify(value));
const normalize = (value) => value.replace(/\r\n?/gu, "\n").trim();
function readJson(relative) { const bytes = fs.readFileSync(path.join(root, relative)); return { bytes, value: JSON.parse(bytes) }; }
function excerpt(tex, source) { return normalize(tex.replace(/\r\n?/gu, "\n").split("\n").slice(source.start_line - 1, source.end_line).join("\n")); }
function sourceDigest(tex, source) { return hash(Buffer.from(excerpt(tex, source), "utf8")); }
function noComments(text) { return text.replace(/\/-[\s\S]*?-\//gu, "").replace(/--[^\n]*/gu, ""); }
function inspect(surface, schema, texOverlay, texBytes, schemaBytes, spineBytes, bindingText, typesText) {
  const errors = []; const tex = fs.readFileSync(path.join(root, schema.inputs.canonical_tex_path), "utf8");
  const expectedIds = [...schema.binding_sources, ...schema.type_sources]; const records = [...(surface.source_layers?.binding || []), ...(surface.source_layers?.type_grammar || [])];
  const byId = new Map(texOverlay.records.map((record) => [record.source_id, record]));
  if (surface.schema !== 1 || surface.status !== "generated_phase_b_binding_type_surface_not_successor_semantics") errors.push("surface status is invalid");
  if (surface.generated_from?.schema_sha256 !== hash(schemaBytes) || surface.generated_from?.tex_classification_sha256 !== hash(texBytes) || surface.generated_from?.spine_sha256 !== hash(spineBytes)) errors.push("surface input ancestry is detached");
  if (surface.generated_from?.binding_module_sha256 !== hash(Buffer.from(bindingText)) || surface.generated_from?.types_module_sha256 !== hash(Buffer.from(typesText))) errors.push("surface is detached from Lean modules");
  if (JSON.stringify(records.map((record) => record.source_id)) !== JSON.stringify(expectedIds) || new Set(records.map((record) => record.source_id)).size !== expectedIds.length) errors.push("source layers are incomplete, reordered, or duplicate");
  for (const record of records) { const classified = byId.get(record.source_id); if (!classified) { errors.push(`${record.source_id}: absent from classification`); continue; } if (JSON.stringify(record.source) !== JSON.stringify(classified.source) || record.disposition !== classified.disposition || record.destination !== classified.destination || record.obligation_status !== (classified.legacy_obligation?.status || null)) errors.push(`${record.source_id}: classification ancestry changed`); if (classified.source.sha256 !== sourceDigest(tex, classified.source) || classified.source_excerpt_sha256 !== sourceDigest(tex, classified.source)) errors.push(`${record.source_id}: canonical TeX identity changed`); }
  const formal = records.filter((record) => record.disposition === "FormalDefinition").length; const obligations = records.filter((record) => record.disposition === "LegacyObligation");
  if (surface.retained_surface?.binding_slot_count !== 10 || surface.retained_surface?.type_constructor_count !== 15 || surface.retained_surface?.explicit_definition_count !== formal || surface.retained_surface?.explicit_obligation_count !== obligations.length || JSON.stringify(surface.retained_surface?.obligation_statuses) !== JSON.stringify(obligations.map((record) => record.obligation_status))) errors.push("retained binding/type counts or obligations differ");
  if (surface.law !== schema.noncollapse_law || surface.next_residual !== schema.next_residual || surface.formal_gate_b?.status !== "PENDING") errors.push("surface erases noncollapse law, residual, or Gate B boundary");
  const binding = noComments(bindingText); const types = noComments(typesText);
  for (const name of schema.binding_slots) if (!new RegExp(`\\|\\s+${name}\\b`, "u").test(binding)) errors.push(`binding slot ${name} missing`);
  for (const name of schema.type_constructors) if (!new RegExp(`\\|\\s+${name}\\b`, "u").test(types)) errors.push(`type constructor ${name} missing`);
  for (const name of schema.obligations) if (!new RegExp(`\\|\\s+${name}\\b`, "u").test(types)) errors.push(`explicit grammar obligation ${name} missing`);
  if (!/inductive\s+TypeCode\s+\(B\s*:\s*Binding\)/u.test(types) || !/structure\s+TypeInterpretation\s+\(B\s*:\s*Binding\)/u.test(types) || !/admissible\s*:\s*TypeCode B\s*→\s*Prop/u.test(types)) errors.push("binding-indexed partial interpretation boundary is absent");
  if (/\b(?:String|axiom|sorry|admit|unsafe|opaque|abbrev)\b/u.test(`${binding}\n${types}`)) errors.push("Lean surface has a string/axiom/gap/escape hatch");
  if (/\b(?:def|abbrev)\s+TypeCode\b/u.test(types) || /Rust|ic_core|ic_runtime/u.test(`${binding}\n${types}`)) errors.push("TypeCode collapses to an alias or imports Rust meaning");
  return errors;
}
function reject(name, values) { if (inspect(...values).length === 0) throw new Error(`mutation breaker escaped: ${name}`); }
function main() {
  const compile = process.argv.slice(2).includes("--compile"); if (process.argv.slice(2).some((arg) => arg !== "--compile")) throw new Error("unknown argument");
  const loaded = { surface: readJson("formal-successor/PHASE_B_BINDING_TYPE_SURFACE.json"), schema: readJson("formal-successor/PHASE_B_BINDING_TYPE_SCHEMA.json"), tex: readJson("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"), spine: readJson("formal-successor/PHASE_B_PREDECESSOR_SPINE.json") };
  const bindingText = fs.readFileSync(path.join(root, loaded.schema.value.inputs.binding_module), "utf8"); const typesText = fs.readFileSync(path.join(root, loaded.schema.value.inputs.types_module), "utf8");
  const base = [loaded.surface.value, loaded.schema.value, loaded.tex.value, loaded.tex.bytes, loaded.schema.bytes, loaded.spine.bytes, bindingText, typesText]; const errors = inspect(...base); if (errors.length) throw new Error(errors.join("\n"));
  if (compile) childProcess.execFileSync("lake", ["env", "lean", "InquiryCalculus/Legacy/V20/Types.lean"], { cwd: path.join(root, "formal"), stdio: "pipe" });
  const mutations = []; function mutate(name, index, change) { const values = base.map((value, position) => position <= 2 ? clone(value) : value); change(values[index], values); mutations.push([name, values]); }
  mutate("binding source omission", 0, (v) => { v.source_layers.binding.pop(); }); mutate("type source omission", 0, (v) => { v.source_layers.type_grammar.pop(); }); mutate("source duplication", 0, (v) => { v.source_layers.type_grammar.push(clone(v.source_layers.binding[0])); }); mutate("foreign source", 0, (v) => { v.source_layers.binding[0].source_id = "FOREIGN"; }); mutate("obligation promotion", 0, (v) => { v.source_layers.type_grammar[0].disposition = "FormalDefinition"; }); mutate("Gate B promotion", 0, (v) => { v.formal_gate_b.status = "PASS"; }); mutate("detached module digest", 0, (v) => { v.generated_from.types_module_sha256 = "0".repeat(64); }); mutate("missing binding slot", 6, (_v, all) => { all[6] = all[6].replace("  | resources", "  | resourceRemoved"); }); mutate("missing type constructor", 7, (_v, all) => { all[7] = all[7].replace("  | code :", "  | codeRemoved :"); }); mutate("missing obligation", 7, (_v, all) => { all[7] = all[7].replace("  | nativeBindingQualification", "  | nativeBindingRemoved"); }); mutate("String tag", 7, (_v, all) => { all[7] += "\ndef illicit : String := \"tag\"\n"; }); mutate("TypeCode alias", 7, (_v, all) => { all[7] = all[7].replace("inductive TypeCode", "def TypeCode"); }); mutate("axiom", 6, (_v, all) => { all[6] += "\naxiom illicit : Prop\n"; }); mutate("lost admissibility", 7, (_v, all) => { all[7] = all[7].replace("admissible : TypeCode B → Prop", "admittedRemoved : TypeCode B → Prop"); });
  for (const [name, values] of mutations) reject(name, values); process.stdout.write(`independent Phase B binding/type checks passed (7 sources; 15 constructors; ${mutations.length}/${mutations.length} mutation breakers; compile ${compile ? "checked" : "delegated"}; Gate B PENDING)\n`);
}
try { main(); } catch (error) { process.stderr.write(`Phase B binding/type check: ${error.message}\n`); process.exit(1); }
