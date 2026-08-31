#!/usr/bin/env node
"use strict";

// Rebuild the compact active engineering surface from append-only trace ancestry.
// This is a deletable projection for inquiry context. It is not successor semantics,
// authoritative history, a semantic State primitive, or a next-question controller.

const fs = require("node:fs");
const path = require("node:path");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const { build: buildResidualTopology } = require("./ic-residual-topology.js");
const instances = require("./ic-question-instance.js");

function parseTrace(root) {
  const traceDirectory = path.join(root, ".claude", "trace");
  const statePath = path.join(traceDirectory, ".state");
  if (!fs.existsSync(statePath)) return { path: null, records: [], lifecycle: null };
  const name = fs.readFileSync(statePath, "utf8").trim();
  if (!/^[A-Za-z0-9._-]+$/u.test(name)) throw new Error("invalid active trace state filename");
  const relative = `.claude/trace/${name}`;
  const absolute = path.join(root, ...relative.split("/"));
  const lifecycle = JSON.parse(cp.execFileSync(process.execPath,
    [path.join(__dirname, "ic-append.js"), "state", absolute], { encoding: "utf8", windowsHide: true }));
  const text = fs.readFileSync(absolute, "utf8");
  if (crypto.createHash("sha256").update(text).digest("hex") !== lifecycle.trace_sha256) {
    throw new Error("trace changed during projection; rebuild from the new occurrence boundary");
  }
  const records = text.split(/\r?\n/u).filter(Boolean).map(JSON.parse);
  return { path: relative, records, lifecycle };
}

function parseJson(value, fallback) {
  if (typeof value !== "string") return fallback;
  try { return JSON.parse(value); } catch { return fallback; }
}

function build(root) {
  const trace = parseTrace(root);
  const manifest = JSON.parse(fs.readFileSync(path.join(root, "formal-successor", "ENGINEERING_QUESTION_PROGRAMS.json"), "utf8"));
  const topology = buildResidualTopology(root).topology;
  const fields = trace.records.filter((record) => record.kind === "field");
  const asks = trace.records.filter((record) => record.kind === "ask");
  const answers = trace.records.filter((record) => record.kind === "answer");
  const reifications = trace.records.filter((record) => record.kind === "reify");
  const latestField = fields.at(-1) ?? null;
  const answeredAsks = new Set(answers.map((record) => record.ask_occurrence));
  const unresolvedAsk = asks.findLast((record) => !answeredAsks.has(record.occurrence)) ?? null;
  const reifiedAnswers = new Set(reifications.map((record) => record.answer_occurrence));
  const awaitingReification = answers.findLast((record) => !reifiedAnswers.has(record.occurrence)) ?? null;
  const products = [];
  for (const record of reifications) {
    for (const product of parseJson(record.products, [])) {
      products.push({ ...product, answer_occurrence: record.answer_occurrence, trace_seq: record.seq });
    }
  }
  const invalidated = new Set(trace.records.filter((record) => record.kind === "invalidate")
    .flatMap((record) => parseJson(record.product_ids, [])));
  const folds = new Map();
  for (const record of trace.records) {
    if (record.kind === "fold") {
      folds.set(record.fold_id, {
        fold_id: record.fold_id,
        members: parseJson(record.members, []),
        representative: record.representative,
        reopen_condition: record.reopen_condition,
        state: "folded",
      });
    } else if (record.kind === "reopen" && folds.has(record.fold_id)) {
      folds.get(record.fold_id).state = "reopened";
      folds.get(record.fold_id).discriminator = record.discriminator;
      folds.get(record.fold_id).restored_members = parseJson(record.restored_members, []);
    }
  }
  const members = latestField ? parseJson(latestField.members, []) : [];
  const questions = new Map(fields.flatMap((record) => parseJson(record.members, [])).map((member) => [member.occurrence, member]));
  const context = { products: new Map(products.map((product) => [product.id, product])), questions, invalidated };
  const completed = new Set(answers.filter((answer) => ["Supported", "Plural", "ExactEmpty"].includes(answer.resolution_class))
    .map((answer) => answer.ask_occurrence));
  const generationFailures = [];
  const generatedQuestions = [];
  for (const product of products.filter((product) => product.inquiry_seed)) {
    try {
      const member = instances.materialize(product, context, manifest);
      if (member.dependencies.some((id) => invalidated.has(id))) member.disposition = "Blocked";
      if (!completed.has(member.occurrence)) generatedQuestions.push(member);
    } catch (error) {
      // A candidate seed with an unavailable corpus form must remain visible;
      // it must not erase the rest of the surface or pretend inquiry is empty.
      generationFailures.push({ seed_product: product.id, state: "Blocked", reason: error.message });
    }
  }
  const availableRelations = new Set([
    ...products.filter((product) => !invalidated.has(product.id)).map((product) => product.kind),
    ...members.flatMap((member) => member.dependencies ?? []),
  ]);
  const methods = manifest.active_lifecycle.method_contract_registry.map((contract) => {
    const missing = contract.applicable_when.filter((relation) => !availableRelations.has(relation));
    return { id: contract.id, applicable: missing.length === 0, missing };
  });
  return {
    schema: 1,
    status: "derived_rebuildable_engineering_surface_not_successor_semantics_or_history",
    trace: trace.path,
    policy_schema: trace.records.filter((record) => record.kind === "policy" || record.kind === "policy_transition").at(-1)?.question_program_schema ?? null,
    active_residual: topology.active_residual,
    field: latestField ? {
      field_id: latestField.field_id,
      regenerated_from: latestField.regenerated_from,
      basis: latestField.basis,
      coverage: latestField.coverage,
      members,
    } : null,
    unresolved_ask: unresolvedAsk && {
      occurrence: unresolvedAsk.occurrence,
      question_form: unresolvedAsk.question_form,
      mode: unresolvedAsk.mode,
      path: unresolvedAsk.path,
    },
    answer_awaiting_reification: awaitingReification?.occurrence ?? null,
    surface_dirty: trace.lifecycle?.surface_dirty ?? false,
    lifecycle: trace.lifecycle,
    products: products.map((product) => ({ ...product, invalidated: invalidated.has(product.id) })),
    generated_questions: generatedQuestions,
    generation_failures: generationFailures,
    folds: [...folds.values()].map((fold) => ({ ...fold,
      ...(trace.lifecycle?.folds ?? []).find((admission) => admission.fold_id === fold.fold_id),
    })),
    recent_checkpoint: trace.records.filter((record) => record.kind === "checkpoint").at(-1) ?? null,
    recent_closure: trace.records.filter((record) => record.kind === "closure").at(-1) ?? null,
    methods,
    residual_counts: Object.fromEntries(topology.states.map((state) => [state, topology.nodes.filter((node) => node.state === state).length])),
  };
}

function render(root) {
  const surface = build(root);
  const members = surface.field?.members ?? [];
  const openHoles = surface.products.filter((product) => !product.invalidated && /hole|gap|open/iu.test(product.kind));
  const recentProducts = surface.products.filter((product) => !product.invalidated).slice(-6);
  const methodText = surface.methods.filter((method) => method.applicable).map((method) => method.id).join(",") || "none established applicable";
  return [
    "ACTIVE RELATIONAL ENGINEERING SURFACE (derived, rebuildable, non-semantic)",
    `trace: ${surface.trace ?? "none"}; schema: ${surface.policy_schema ?? "none"}`,
    `frontier: ${surface.active_residual}`,
    `field: ${surface.field?.field_id ?? "none"}; live questions: ${members.length}`,
    ...members.slice(0, 10).map((member) => `  ${member.occurrence} [${member.disposition}; ${member.executable ? "executable" : "not executable"}; path=${member.path}] ${member.prompt}`),
    `unresolved Ask: ${surface.unresolved_ask?.occurrence ?? "none"}`,
    `answer awaiting reification: ${surface.answer_awaiting_reification ?? "none"}; dirty=${surface.surface_dirty}`,
    `open holes/gaps: ${openHoles.map((product) => product.id).join(",") || "none reified"}`,
    `recent products: ${recentProducts.map((product) => `${product.id}:${product.status}`).join(",") || "none"}`,
    `seed-generated questions: ${surface.generated_questions.length}; provisional formability only, no execution or selection implied`,
    `blocked seeds: ${surface.generation_failures.map((failure) => `${failure.seed_product}: ${failure.reason}`).join("; ") || "none"}`,
    `folds: ${surface.folds.map((fold) => `${fold.fold_id}:${fold.state}`).join(",") || "none"}`,
    `folds requiring reopening: ${surface.folds.filter((fold) => fold.reopen_required).map((fold) => `${fold.fold_id} (${fold.reopen_reasons.join(",")})`).join("; ") || "none"}`,
    `applicable methods by explicit contract: ${methodText}`,
    `residuals: ${Object.entries(surface.residual_counts).map(([key, value]) => `${key}=${value}`).join(" ")}`,
    "Reconstruct represented relations; select only a live executable occurrence using explicit execution constraints; preserve every unchosen occurrence.",
    "After Answer: reify consequential products and regenerate the field before another Ask.",
    "",
  ].join("\n");
}

module.exports = { build, render };

if (require.main === module) {
  try {
    const [command, root] = process.argv.slice(2);
    if (!root || !new Set(["json", "projection"]).has(command)) throw new Error("usage: ic-relational-surface.js json|projection ROOT");
    const resolved = path.resolve(root);
    process.stdout.write(command === "json" ? `${JSON.stringify(build(resolved), null, 2)}\n` : render(resolved));
  } catch (error) {
    process.stderr.write(`ic-relational-surface: ${error.message}\n`);
    process.exitCode = 1;
  }
}
