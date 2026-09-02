#!/usr/bin/env node
"use strict";

// Rebuild a preformal residual index from stable obligation seeds, the one live
// Frontier selection, and append-only trace occurrences. The result is a
// deletable projection: it is neither successor semantics nor another history.

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const obligationIndex = require("./ic-obligation-index.js");

function fail(message) {
  throw new Error(message);
}

function sha256(bytes) {
  return crypto.createHash("sha256").update(bytes).digest("hex");
}

function stableId(value) {
  return /^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)+$/u.test(value);
}

function requireString(object, field, owner) {
  if (typeof object[field] !== "string" || object[field].trim() === "") {
    fail(`${owner} requires nonempty ${field}`);
  }
}

function requireStringArray(object, field, owner, allowEmpty = false) {
  if (!Array.isArray(object[field]) || object[field].some(
    (entry) => typeof entry !== "string" || entry.trim() === "",
  )) {
    fail(`${owner} requires a string array ${field}`);
  }
  if (!allowEmpty && object[field].length === 0) fail(`${owner} requires nonempty ${field}`);
  if (new Set(object[field]).size !== object[field].length) {
    fail(`${owner} ${field} must not contain duplicates`);
  }
}

// The active residual is DERIVED from the construction obligation field, not read
// from IMPLEMENTATION_FRONTIER.md. The Markdown is a generated projection of this
// selection, so editing it changes no live obligation.
function derivedSelection(root) {
  const { index, selected, digest } = obligationIndex.build(root);
  const retained = selected ?? { id: "FORMAL-CONSTRUCTION-NO-EXECUTABLE", gate: "C",
    statement: "Live construction obligations remain retained; no executable occurrence is currently available.",
    depends_on: [], coverage: "operational interruption, not semantic closure", provenance: "derived obligation field" };
  return {
    id: retained.id,
    digest,
    relative: "derived:.claude/hooks/ic-obligation-index.js",
    obligation: retained,
    index,
  };
}

function csv(value) {
  if (typeof value !== "string" || value === "" || value === "none") return [];
  return [...new Set(value.split(",").map((item) => item.trim()).filter(Boolean))];
}

function conditionKeys(value) {
  if (typeof value !== "string" || value === "" || value === "none") return [];
  const pattern = /^[A-Za-z0-9._-]+@[A-Za-z0-9._-]+@[A-Za-z0-9._-]+@[A-Za-z0-9._-]+@[A-Za-z0-9._-]+@(forward|reverse|neutral)$/u;
  const keys = value.split(";");
  if (keys.some((key) => !pattern.test(key))) {
    fail("condition_keys must use schema@roles@scope@applicability@grain@orientation");
  }
  return [...new Set(keys)];
}

function readTraceOccurrences(root) {
  const traceDirectory = path.join(root, ".claude", "trace");
  if (!fs.existsSync(traceDirectory)) return { occurrences: [], sources: [], lifecycle: {} };
  const occurrences = [];
  const sources = [];
  const lifecycle = {
    fields: [], question_occurrences: [], answers: [], provisional_relations: [],
    dependencies: [], invalidations: [], folds: [], reopenings: [], checkpoints: [], closures: [],
  };
  const files = fs.readdirSync(traceDirectory).filter((name) => name.endsWith(".jsonl")).sort();
  for (const name of files) {
    const absolute = path.join(traceDirectory, name);
    const bytes = fs.readFileSync(absolute);
    const relevantKind = /"kind":"(?:field|ask|answer|reify|invalidate|fold|reopen|checkpoint|closure)"|"kind":"residual"[^\n]*"active_residual"/u;
    const candidateLines = bytes.toString("utf8").split(/\r?\n/u)
      .map((line, index) => ({ line, lineNumber: index + 1 }))
      .filter((entry) => relevantKind.test(entry.line));
    const records = candidateLines.map(({ line, lineNumber }) => {
      try {
        return JSON.parse(line);
      } catch (error) {
        fail(`indexed trace occurrence ${name}:${lineNumber} is not JSON: ${error.message}`);
      }
    });
    const relevant = records.filter(
      (record) => record.kind === "residual" && typeof record.active_residual === "string",
    );
    const lifecycleRecords = records.filter((record) => new Set([
      "field", "ask", "answer", "reify", "invalidate", "fold", "reopen", "checkpoint", "closure",
    ]).has(record.kind));
    if (relevant.length === 0 && lifecycleRecords.length === 0) continue;
    sources.push({ path: `.claude/trace/${name}`, sha256: sha256(bytes) });
    for (const record of relevant) {
      if (!stableId(record.active_residual)) {
        fail(`trace ${name}:${record.seq ?? "?"} has invalid active_residual`);
      }
      occurrences.push({
        trace: `.claude/trace/${name}`,
        seq: record.seq,
        residual_id: record.active_residual,
        parent_residual: record.parent_residual ?? "none",
        open_relation: record.open_relation ?? "Unknown",
        condition_ids: csv(record.condition_ids),
        condition_keys: conditionKeys(record.condition_keys),
        blocker_ids: csv(record.blocker_ids),
        breaker_ids: csv(record.breaker_ids),
        separator_ids: csv(record.separator_ids),
        survived_contrast_ids: csv(record.survived_contrast_ids),
        conflict_ids: csv(record.conflict_ids),
        gap_ids: csv(record.gap_ids),
        failed_fold_ids: csv(record.failed_fold_ids),
        reopen_condition_ids: csv(record.reopen_condition_ids),
        overlap_ids: csv(record.overlap_ids),
        coverage: record.coverage ?? "Unknown",
        resolution_class: record.resolution_class ?? "Unknown",
        residual_shape: record.residual_shape ?? "Generic",
        method_frontier: csv(record.method_frontier),
        next_question_family: record.next_question_family ?? "Unknown",
        residual_class: record.class ?? "unknown",
        next: record.next ?? "Unknown",
      });
    }
    for (const record of lifecycleRecords) {
      const ancestry = { trace: `.claude/trace/${name}`, seq: record.seq };
      if (record.kind === "field") {
        lifecycle.fields.push({ ...ancestry, field_id: record.field_id, regenerated_from: record.regenerated_from, field_check: record.field_check });
      } else if (record.kind === "ask") {
        lifecycle.question_occurrences.push({ ...ancestry, occurrence: record.occurrence, field_id: record.field_id, question_form: record.question_form, rendering: record.rendering, path: record.path, mode: record.mode });
      } else if (record.kind === "answer") {
        lifecycle.answers.push({ ...ancestry, occurrence: record.occurrence, ask_occurrence: record.ask_occurrence, resolution_class: record.resolution_class, status: record.status });
      } else if (record.kind === "reify") {
        let products = [];
        try { products = JSON.parse(record.products); } catch { products = []; }
        for (const product of products) {
          lifecycle.provisional_relations.push({ ...ancestry, answer_occurrence: record.answer_occurrence, ...product });
          for (const dependency of product.dependencies ?? []) {
            lifecycle.dependencies.push({ from: product.id, to: dependency, ...ancestry });
          }
        }
      } else if (record.kind === "invalidate") {
        lifecycle.invalidations.push({ ...ancestry, product_ids: record.product_ids, cause: record.cause });
      } else if (record.kind === "fold") {
        lifecycle.folds.push({ ...ancestry, fold_id: record.fold_id, members: record.members, representative: record.representative, reopen_condition: record.reopen_condition });
      } else if (record.kind === "reopen") {
        lifecycle.reopenings.push({ ...ancestry, fold_id: record.fold_id, restored_members: record.restored_members, discriminator: record.discriminator });
      } else if (record.kind === "checkpoint") {
        lifecycle.checkpoints.push({ ...ancestry, field_id: record.field_id, remains_open: record.remains_open });
      } else if (record.kind === "closure") {
        lifecycle.closures.push({ ...ancestry, field_id: record.field_id, scope: record.scope, warrant: record.warrant });
      }
    }
  }
  return { occurrences, sources, lifecycle };
}

function validateSeed(root) {
  const relative = "formal-successor/RESIDUAL_OBLIGATIONS.json";
  const bytes = fs.readFileSync(path.join(root, ...relative.split("/")));
  const seed = JSON.parse(bytes.toString("utf8"));
  if (seed.schema !== 1) fail("residual obligation seed schema must be 1");
  if (seed.status !== "explicit_project_obligation_seed_not_successor_semantics_or_moving_history") {
    fail("residual obligations must remain classified outside semantics and moving history");
  }
  // The seed supplies stable obligations and dependency edges. It must not name a
  // hand-maintained document as the mathematical selection source: selection is
  // derived from the construction obligation field.
  if (seed.selection_source !== "derived:.claude/hooks/ic-obligation-index.js") {
    fail("residual obligation selection source must be the derived construction obligation index");
  }
  if (JSON.stringify(seed.states) !== JSON.stringify(["active", "blocked", "latent", "reopened"])) {
    fail("residual states must be active, blocked, latent, reopened");
  }
  requireString(seed, "closure_law", "residual obligation seed");
  if (!/local result closes only its named obligation/iu.test(seed.closure_law)) {
    fail("residual obligation seed lacks the local-closure law");
  }
  if (!Array.isArray(seed.nodes) || seed.nodes.length === 0) fail("residual obligations require nodes");
  const nodes = new Map();
  for (const node of seed.nodes) {
    for (const field of ["id", "state", "phase", "obligation", "coverage"]) {
      requireString(node, field, `residual obligation ${node?.id ?? "<unknown>"}`);
    }
    if (!stableId(node.id)) fail(`residual obligation ${node.id} has an invalid stable id`);
    if (nodes.has(node.id)) fail(`duplicate residual obligation ${node.id}`);
    for (const field of ["depends_on", "conditions", "evidence", "reopen_when"]) {
      requireStringArray(node, field, `residual obligation ${node.id}`, field === "depends_on");
    }
    node.condition_keys = node.conditions.map(
      (condition) => `project.condition@${condition}@repository@applicable@obligation@neutral`,
    );
    nodes.set(node.id, structuredClone(node));
  }
  if (!nodes.has(seed.initial_selection)) fail("initial residual selection is not a seeded obligation");
  const initiallyActive = [...nodes.values()].filter((node) => node.state === "active");
  if (initiallyActive.length !== 1 || initiallyActive[0].id !== seed.initial_selection) {
    fail("obligation seed must have one explicit initial selection");
  }
  const conditions = new Map();
  if (!Array.isArray(seed.conditions)) fail("residual obligations require conditions");
  for (const condition of seed.conditions) {
    requireString(condition, "id", "residual condition");
    requireString(condition, "meaning", `residual condition ${condition.id}`);
    if (conditions.has(condition.id)) fail(`duplicate residual condition ${condition.id}`);
    conditions.set(condition.id, structuredClone(condition));
  }
  for (const node of nodes.values()) {
    for (const dependency of node.depends_on) {
      if (!nodes.has(dependency)) fail(`${node.id} depends on unknown residual ${dependency}`);
      if (dependency === node.id) fail(`${node.id} depends on itself`);
    }
    for (const condition of node.conditions) {
      if (!conditions.has(condition)) fail(`${node.id} cites unknown condition ${condition}`);
    }
  }
  const visiting = new Set();
  const visited = new Set();
  function visit(id) {
    if (visiting.has(id)) fail(`residual dependency cycle reaches ${id}`);
    if (visited.has(id)) return;
    visiting.add(id);
    for (const dependency of nodes.get(id).depends_on) visit(dependency);
    visiting.delete(id);
    visited.add(id);
  }
  for (const id of nodes.keys()) visit(id);
  return { seed, nodes, conditions, source: { path: relative, sha256: sha256(bytes) } };
}

function build(root) {
  const { seed, nodes, conditions, source } = validateSeed(root);
  const selected = derivedSelection(root);
  const { occurrences, sources, lifecycle } = readTraceOccurrences(root);

  for (const occurrence of occurrences) {
    if (!nodes.has(occurrence.residual_id)) {
      nodes.set(occurrence.residual_id, {
        id: occurrence.residual_id,
        state: "latent",
        phase: "trace",
        obligation: occurrence.next,
        depends_on: stableId(occurrence.parent_residual) ? [occurrence.parent_residual] : [],
        conditions: [],
        coverage: occurrence.coverage,
        evidence: [`${occurrence.trace}#${occurrence.seq}`],
        reopen_when: ["a trace-declared breaker, blocker, overlap, or coverage condition changes"],
      });
    }
    const node = nodes.get(occurrence.residual_id);
    node.occurrences ??= [];
    node.occurrences.push(occurrence);
    node.conditions = [...new Set([...node.conditions, ...occurrence.condition_ids])];
    node.condition_keys = [...new Set([...(node.condition_keys ?? []), ...occurrence.condition_keys])];
    for (const id of occurrence.condition_ids) {
      if (!conditions.has(id)) conditions.set(id, { id, meaning: "trace-declared condition coordinate" });
    }
  }

  if (!nodes.has(selected.id)) {
    nodes.set(selected.id, {
      id: selected.id,
      state: "active",
      phase: selected.obligation.gate ?? "derived",
      obligation: selected.obligation.statement,
      depends_on: selected.obligation.depends_on.filter((id) => nodes.has(id)),
      conditions: [],
      coverage: selected.obligation.coverage,
      evidence: [selected.obligation.provenance],
      reopen_when: ["the derived construction obligation field changes its selection"],
    });
  }

  for (const node of nodes.values()) {
    const latest = node.occurrences?.at(-1);
    if (node.id === selected.id) node.state = "active";
    else if (latest?.residual_class === "regression") node.state = "reopened";
    else if (["env_failure", "resource"].includes(latest?.residual_class)) node.state = "blocked";
    else node.state = "latent";
    node.breaker_ids = [...new Set((node.occurrences ?? []).flatMap((item) => item.breaker_ids))];
    node.blocker_ids = [...new Set((node.occurrences ?? []).flatMap((item) => item.blocker_ids))];
    node.separator_ids = [...new Set((node.occurrences ?? []).flatMap((item) => item.separator_ids))];
    node.survived_contrast_ids = [...new Set((node.occurrences ?? []).flatMap((item) => item.survived_contrast_ids))];
    node.conflict_ids = [...new Set((node.occurrences ?? []).flatMap((item) => item.conflict_ids))];
    node.gap_ids = [...new Set((node.occurrences ?? []).flatMap((item) => item.gap_ids))];
    node.failed_fold_ids = [...new Set((node.occurrences ?? []).flatMap((item) => item.failed_fold_ids))];
    node.reopen_condition_ids = [...new Set((node.occurrences ?? []).flatMap((item) => item.reopen_condition_ids))];
    node.overlap_ids = [...new Set((node.occurrences ?? []).flatMap((item) => item.overlap_ids))];
  }

  const coordinateKinds = [
    "condition_keys", "separator_ids", "breaker_ids", "survived_contrast_ids",
    "conflict_ids", "blocker_ids", "gap_ids", "failed_fold_ids", "reopen_condition_ids",
  ];
  const basins = [];
  for (const coordinateKind of coordinateKinds) {
    const groups = new Map();
    for (const node of nodes.values()) {
      for (const coordinate of node[coordinateKind] ?? []) {
        if (!groups.has(coordinate)) groups.set(coordinate, []);
        groups.get(coordinate).push(node.id);
      }
    }
    for (const [coordinate, residualIds] of groups) {
      if (residualIds.length < 2) continue;
      basins.push({
        id: `BASIN-${coordinateKind.replace(/_IDS$/u, "").replace(/_/gu, "-").toUpperCase()}-${sha256(coordinate).slice(0, 10).toUpperCase()}`,
        coordinate_kind: coordinateKind,
        coordinate,
        residual_ids: residualIds.sort(),
        factor_status: "derived_overlap_candidate_not_yet_warranted",
      });
    }
  }
  basins.sort((left, right) => left.id.localeCompare(right.id));

  const topology = {
    schema: 2,
    status: "derived_rebuildable_preformal_projection_not_successor_semantics_or_history",
    selection_source: selected.relative,
    active_residual: selected.id,
    states: seed.states,
    closure_law: seed.closure_law,
    source_digests: [source, { path: selected.relative, sha256: selected.digest }, ...sources],
    conditions: [...conditions.values()].sort((left, right) => left.id.localeCompare(right.id)),
    nodes: [...nodes.values()].sort((left, right) => left.id.localeCompare(right.id)),
    basins,
    lifecycle,
  };
  const digest = sha256(JSON.stringify(topology));
  const counts = Object.fromEntries(topology.states.map((state) => [
    state,
    topology.nodes.filter((node) => node.state === state).length,
  ]));
  if (counts.active !== 1) fail(`derived residual index must have exactly one active node, found ${counts.active}`);
  return { topology, counts, digest };
}

function render(root) {
  const { topology, counts, digest } = build(root);
  return [
    `RESIDUAL INDEX ${digest}`,
    `active: ${topology.active_residual}`,
    `counts: active=${counts.active} blocked=${counts.blocked} latent=${counts.latent} reopened=${counts.reopened}`,
    `overlap basins: ${topology.basins.length} (derived candidates, not warranted methods)`,
    `lifecycle: fields=${topology.lifecycle.fields.length} asks=${topology.lifecycle.question_occurrences.length} answers=${topology.lifecycle.answers.length} provisional=${topology.lifecycle.provisional_relations.length} folds=${topology.lifecycle.folds.length} reopens=${topology.lifecycle.reopenings.length} checkpoints=${topology.lifecycle.checkpoints.length}`,
    "closure: local obligation/binding/horizon/coverage only; broader residuals persist",
    "",
  ].join("\n");
}

module.exports = { build, render, validate: build };

if (require.main === module) {
  try {
    const [command, root, output] = process.argv.slice(2);
    if (!root || !new Set(["validate", "projection", "context", "build"]).has(command)) {
      fail("usage: ic-residual-topology.js validate|projection|context|build ROOT [OUTPUT]");
    }
    const resolved = path.resolve(root);
    if (command === "projection") process.stdout.write(render(resolved));
    else if (command === "context") {
      const { topology, counts, digest } = build(resolved);
      process.stdout.write(
        `${digest}\t${topology.active_residual}\t` +
          `active:${counts.active},blocked:${counts.blocked},latent:${counts.latent},reopened:${counts.reopened}\n`,
      );
    } else if (command === "build") {
      if (!output) fail("build requires an output path");
      fs.writeFileSync(path.resolve(output), `${JSON.stringify(build(resolved).topology, null, 2)}\n`);
    } else process.stdout.write(`${build(resolved).digest}\n`);
  } catch (error) {
    process.stderr.write(`ic-residual-topology: ${error.message}\n`);
    process.exitCode = 1;
  }
}
