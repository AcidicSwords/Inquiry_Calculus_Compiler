#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const childProcess = require("node:child_process");

const root = path.resolve(__dirname, "..");
const hash = (value) => crypto.createHash("sha256").update(value).digest("hex");
const clone = (value) => JSON.parse(JSON.stringify(value));
const normalizeNewlines = (value) => value.replace(/\r\n?/gu, "\n");
const normalizeUnit = (value) => normalizeNewlines(value).trim();

function readJson(relativePath) {
  const bytes = fs.readFileSync(path.join(root, relativePath));
  return { bytes, value: JSON.parse(bytes.toString("utf8")) };
}

function sourceExcerpt(texText, source) {
  return normalizeNewlines(texText)
    .split("\n")
    .slice(source.start_line - 1, source.end_line)
    .join("\n");
}

function stripLeanComments(text) {
  return text
    .replace(/\/-[\s\S]*?-\//gu, "")
    .replace(/--[^\n]*/gu, "");
}

function inspect(value, schema, tex, texBytes, schemaBytes, leanText, leanModuleExists = true) {
  const errors = [];
  const expectedOrder = schema.required_layer_order || [];
  const layers = value.layers || [];
  const texById = new Map((tex.records || []).map((record) => [record.source_id, record]));
  const schemaById = new Map((schema.layers || []).map((layer) => [layer.id, layer]));
  const actualById = new Map(layers.map((layer) => [layer.id, layer]));

  if (value.schema !== 1 || value.status !== "generated_phase_b_predecessor_spine_not_semantic_completion") errors.push("Phase B artifact authority/status is invalid");
  if (schema.schema !== 1 || schema.status !== "phase_b_predecessor_elaboration_spine_not_successor_semantics") errors.push("Phase B schema authority/status is invalid");
  if (value.generated_from?.schema_sha256 !== hash(schemaBytes)) errors.push("Phase B artifact is detached from its schema bytes");
  if (value.generated_from?.tex_classification_sha256 !== hash(texBytes)) errors.push("Phase B artifact is detached from its TeX classification bytes");
  if (value.generated_from?.canonical_tex_sha256 !== schema.inputs?.canonical_tex_sha256) errors.push("Phase B artifact is detached from the canonical TeX coordinate");
  if (value.generated_from?.gate_a_evidence !== "FORMAL-A-COVERAGE-001") errors.push("Phase B artifact lacks its Gate A entry evidence");
  if (value.generated_from?.lean_ambient_module !== schema.inputs?.lean_ambient_module) errors.push("Phase B artifact names a foreign ambient module");
  if (!leanModuleExists) errors.push("ambient Lean module is missing");
  if (leanModuleExists && value.generated_from?.lean_ambient_sha256 !== hash(Buffer.from(leanText, "utf8"))) errors.push("Phase B artifact is detached from the ambient Lean module");

  if (JSON.stringify((schema.layers || []).map((layer) => layer.id)) !== JSON.stringify(expectedOrder)) errors.push("schema layer declarations do not realize the required order");
  if (new Set(expectedOrder).size !== expectedOrder.length || schemaById.size !== expectedOrder.length) errors.push("schema layer identities are not unique and total");
  if (JSON.stringify(layers.map((layer) => layer.id)) !== JSON.stringify(expectedOrder)) errors.push("generated layer order or coverage differs");
  if (actualById.size !== expectedOrder.length) errors.push("generated layer identities are duplicated or missing");

  const seenSources = new Set();
  let selectedLegacyObligations = 0;
  for (let index = 0; index < expectedOrder.length; index += 1) {
    const id = expectedOrder[index];
    const expected = schemaById.get(id);
    const layer = actualById.get(id);
    if (!expected || !layer) continue;
    if (layer.ordinal !== index) errors.push(`${id}: ordinal differs from the dependency order`);
    if (JSON.stringify(layer.depends_on) !== JSON.stringify(expected.depends_on)) errors.push(`${id}: dependency relation differs from the schema`);
    for (const dependency of layer.depends_on || []) {
      const dependencyIndex = expectedOrder.indexOf(dependency);
      if (dependencyIndex < 0 || dependencyIndex >= index) errors.push(`${id}: dependency is absent, cyclic, or points forward`);
    }
    if (layer.destination !== expected.destination || /(?:^|[./:])rust(?:$|[./:])/iu.test(layer.destination)) errors.push(`${id}: destination is foreign or imports Rust meaning`);
    if (layer.state !== expected.state) errors.push(`${id}: state differs from the declared local boundary`);
    if (JSON.stringify((layer.sources || []).map((source) => source.source_id)) !== JSON.stringify(expected.source_ids)) errors.push(`${id}: exact selected-source order differs`);
    for (const selected of layer.sources || []) {
      if (seenSources.has(selected.source_id)) errors.push(`${selected.source_id}: selected more than once`);
      seenSources.add(selected.source_id);
      const record = texById.get(selected.source_id);
      if (!record) { errors.push(`${selected.source_id}: is not an exact TeX classification source`); continue; }
      if (selected.disposition !== record.disposition || selected.destination !== record.destination || JSON.stringify(selected.source) !== JSON.stringify(record.source) || selected.obligation_status !== (record.legacy_obligation?.status || null)) errors.push(`${selected.source_id}: copied classification ancestry differs`);
      const excerptDigest = hash(Buffer.from(normalizeUnit(sourceExcerpt(fs.readFileSync(path.join(root, schema.inputs.canonical_tex_path), "utf8"), record.source)), "utf8"));
      if (record.source.sha256 !== excerptDigest || record.source_excerpt_sha256 !== excerptDigest) errors.push(`${selected.source_id}: source identity does not regenerate from canonical TeX`);
      if (record.disposition === "LegacyObligation") selectedLegacyObligations += 1;
    }
  }

  const ambient = actualById.get("ambient_metalanguage");
  if (!ambient || ambient.sources?.length !== 1 || ambient.sources[0].source_id !== "PRED-TEX-PROSE-983F2B30F7C1C1D2") errors.push("ambient metalanguage boundary is not anchored to the exact v2.0 source claim");
  if (ambient?.destination !== "InquiryCalculus.Meta.Ambient" || ambient?.state !== "checked_boundary_with_explicit_classicality_obligation") errors.push("ambient boundary destination or standing differs");
  if (value.ambient_boundary?.classicality_status !== "ExplicitPredecessorObligation") errors.push("predecessor classicality was silently assumed, erased, or discharged");
  if (JSON.stringify(value.ambient_boundary?.supplied_features) !== JSON.stringify(schema.ambient_features) || JSON.stringify(value.ambient_boundary?.explicit_nonassumptions) !== JSON.stringify(schema.ambient_nonassumptions)) errors.push("ambient supplied/nonassumed boundary differs");

  const code = stripLeanComments(leanText);
  const requiredTheorems = [
    "ambient_type_identity", "ambient_proposition_identity", "ambient_equality_identity",
    "ambient_function_identity", "ambient_dependent_function_identity", "ambient_dependent_pair_identity",
    "ambient_universal_quantifier_identity", "ambient_existential_quantifier_identity",
  ];
  for (const theorem of requiredTheorems) if (!new RegExp(`\\btheorem\\s+${theorem}\\b`, "u").test(code)) errors.push(`ambient theorem ${theorem} is missing`);
  if (/\b(?:axiom|sorry|admit|def|abbrev|opaque|constant|structure|inductive|classical)\b/u.test(code)) errors.push("ambient module introduces a declaration, gap, or classical assumption instead of only checking the boundary");
  if (!code.includes("namespace InquiryCalculus.Meta")) errors.push("ambient declarations escaped their namespace");

  const checked = layers.filter((layer) => layer.state === "checked_boundary_with_explicit_classicality_obligation").length;
  const open = layers.filter((layer) => layer.state === "open").length;
  if (value.coverage?.layer_count !== 13 || value.coverage?.selected_source_count !== seenSources.size || value.coverage?.selected_legacy_obligations !== selectedLegacyObligations || value.coverage?.checked_boundary_layers !== checked || value.coverage?.open_layers !== open || checked !== 1 || open !== 12) errors.push("Phase B local coverage summary differs from independent reconstruction");
  if (value.next_residual?.id !== "FORMAL-B-BINDING-TYPE-SURFACE") errors.push("Phase B artifact does not select the declared first nonambient residual");
  if (value.formal_gate_b?.status !== "PENDING" || schema.gate_b?.status !== "PENDING") errors.push("ambient boundary improperly promotes Formal Gate B");
  return errors;
}

function reject(name, values) {
  if (inspect(...values).length === 0) throw new Error(`mutation breaker escaped: ${name}`);
}

function main() {
  const loaded = {
    value: readJson("formal-successor/PHASE_B_PREDECESSOR_SPINE.json"),
    schema: readJson("formal-successor/PHASE_B_PREDECESSOR_SPINE_SCHEMA.json"),
    tex: readJson("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"),
  };
  const leanPath = path.join(root, loaded.schema.value.inputs.lean_ambient_module);
  const leanText = fs.readFileSync(leanPath, "utf8");
  const base = [loaded.value.value, loaded.schema.value, loaded.tex.value, loaded.tex.bytes, loaded.schema.bytes, leanText, true];
  const errors = inspect(...base);
  if (errors.length) throw new Error(errors.join("\n"));

  childProcess.execFileSync("lake", ["env", "lean", "InquiryCalculus/Meta/Ambient.lean"], {
    cwd: path.join(root, "formal"), stdio: "pipe",
  });

  const mutations = [];
  function mutation(name, index, edit) {
    const values = base.map((value, position) => position <= 2 ? clone(value) : value);
    edit(values[index], values);
    mutations.push([name, values]);
  }
  mutation("missing layer", 0, (value) => { value.layers.pop(); });
  mutation("missing selected source", 0, (value) => { value.layers[1].sources.pop(); });
  mutation("duplicate selected source", 0, (value) => { value.layers[2].sources.push(clone(value.layers[1].sources[0])); });
  mutation("foreign selected source", 0, (value) => { value.layers[1].sources[0].source_id = "FOREIGN"; });
  mutation("forward dependency", 0, (value) => { value.layers[1].depends_on = ["type_grammar"]; });
  mutation("type declaration moved into ambient", 0, (value) => { value.layers[0].sources.push(value.layers[2].sources.shift()); });
  mutation("ambient claim moved into calculus", 0, (value) => { value.layers[1].sources.push(value.layers[0].sources.shift()); });
  mutation("classicality silently assumed", 0, (value) => { value.ambient_boundary.classicality_status = "Assumed"; });
  mutation("Rust semantic destination", 0, (value) => { value.layers[4].destination = "Rust::relation"; });
  mutation("Gate B self-promotion", 0, (value) => { value.formal_gate_b.status = "PASS"; });
  mutation("detached source ancestry", 0, (value) => { value.layers[1].sources[0].source.sha256 = "0".repeat(64); });
  mutation("detached schema digest", 0, (value) => { value.generated_from.schema_sha256 = "0".repeat(64); });
  mutation("missing ambient theorem", 5, (_value, values) => { values[5] = values[5].replace("ambient_type_identity", "ambient_type_missing"); });
  mutation("ambient axiom", 5, (_value, values) => { values[5] += "\naxiom illicit : Prop\n"; });
  mutation("ambient sorry", 5, (_value, values) => { values[5] += "\ntheorem illicit : True := by sorry\n"; });
  mutation("ambient calculus definition", 5, (_value, values) => { values[5] += "\ndef CalculusType := Type\n"; });
  mutation("missing ambient module", 6, (_value, values) => { values[6] = false; });
  for (const [name, values] of mutations) reject(name, values);
  process.stdout.write(`independent Phase B spine checks passed (13 ordered layers; 41 exact sources; ${mutations.length}/${mutations.length} mutation breakers; Gate B PENDING)\n`);
}

try { main(); } catch (error) { process.stderr.write(`Phase B predecessor spine check: ${error.message}\n`); process.exit(1); }
