#!/usr/bin/env node
"use strict";

// One public, derived inquiry-spine projection. Internal modules reconstruct
// evidence and relations; only this module composes them into model context.

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { read: readContract } = require("./ic-contract.js");
const relational = require("./ic-relational-surface.js");

const canonical = (value) => Array.isArray(value) ? `[${value.map(canonical).join(",")}]` :
  value && typeof value === "object" ? `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(",")}}` : JSON.stringify(value);
const digest = (value) => crypto.createHash("sha256").update(canonical(value)).digest("hex");

function directRelations(surface) {
  return surface.products.filter((product) => !product.invalidated && product.inquiry_generator_surface)
    .flatMap((product) => product.inquiry_generator_surface.relations.map((relation) => ({
      ...relation,
      dependencies: [...new Set([product.id, ...(relation.dependencies ?? []), ...(product.dependencies ?? [])])],
      horizon: product.horizon ?? "declared by relation ancestry",
      coverage: product.coverage ?? "declared by relation ancestry",
      provenance: `reified:${product.id}`,
    })));
}

function pathRecord(relations) {
  const source = relations[0].source;
  const target = relations.at(-1).target;
  const relationIds = relations.map((relation) => relation.id);
  const value = {
    ordered_occurrence_ids: relationIds,
    typed_source: source,
    typed_target: target,
    relation_ids: relationIds,
    dependencies: [...new Set(relations.flatMap((relation) => [relation.id, ...relation.dependencies]))],
    horizon: [...new Set(relations.map((relation) => relation.horizon))],
    coverage: [...new Set(relations.map((relation) => relation.coverage))],
    provenance: relations.map((relation) => relation.provenance),
    represented_not_actual: true,
  };
  return { path_id: `PATH-${digest(value)}`, ...value };
}

function derivePaths(surface, limit = 512) {
  const relations = directRelations(surface);
  const paths = relations.map((relation) => pathRecord([relation]));
  const queue = relations.map((relation) => [relation]);
  let bounded = false;
  while (queue.length && paths.length < limit) {
    const current = queue.shift();
    const used = new Set(current.map((relation) => relation.id));
    for (const next of relations) {
      if (current.at(-1).target !== next.source || used.has(next.id)) continue;
      const composed = [...current, next];
      paths.push(pathRecord(composed));
      queue.push(composed);
      if (paths.length >= limit) { bounded = queue.length > 0; break; }
    }
  }
  return { paths, bounded, limit };
}

function discriminators(surface) {
  return surface.products.filter((product) => !product.invalidated && product.inquiry_generator_surface)
    .flatMap((product) => product.inquiry_generator_surface.discriminators.map((item) => ({
      ...item,
      dependencies: [...new Set([product.id, ...(item.dependencies ?? []), ...(product.dependencies ?? [])])],
      horizon: product.horizon ?? "declared by discriminator ancestry",
      coverage: product.coverage ?? "declared by discriminator ancestry",
    })));
}

function transport(surface, pathProjection) {
  const output = [];
  for (const discriminator of discriminators(surface)) {
    for (const prior of pathProjection.paths) {
      if (prior.typed_target !== discriminator.domain) continue;
      const value = {
        discriminator_id: discriminator.id,
        path_id: prior.path_id,
        typed_source: prior.typed_source,
        typed_target: discriminator.codomain ?? `Answer(${discriminator.id})`,
        composition: `${discriminator.id}∘${[...prior.relation_ids].reverse().join("∘")}`,
        dependencies: [...new Set([...prior.dependencies, discriminator.id, ...discriminator.dependencies])],
        horizon: [prior.horizon, discriminator.horizon],
        coverage: [prior.coverage, discriminator.coverage],
        provenance: { discriminator: discriminator.path, path: prior.provenance },
        status: "Generated",
        standing: false,
      };
      output.push({ transport_id: `CARRY-${digest(value)}`, ...value });
    }
  }
  return output;
}

function liveQuestions(surface) {
  const byId = new Map();
  for (const member of [...(surface.field?.members ?? []), ...surface.generated_questions]) {
    if (!byId.has(member.occurrence)) byId.set(member.occurrence, member);
  }
  return [...byId.values()];
}

function selectExecutable(questions, invalidated) {
  const invalid = new Set(invalidated);
  const live = questions.filter((question) => question.executable &&
    !(question.dependencies ?? []).some((id) => invalid.has(id)) &&
    ["Required", "Productive"].includes(question.disposition));
  const rank = (question) => question.disposition === "Required" ? 0 : 1;
  live.sort((left, right) => rank(left) - rank(right) || left.occurrence.localeCompare(right.occurrence));
  return { selected: live[0] ?? null, executable_frontier: live, tie_break_is_operational_only: true };
}

function evaluateClosure(surface, questions, carried) {
  const reasons = [];
  if (!surface.field) reasons.push("no represented field");
  if (surface.unresolved_ask) reasons.push("unresolved Ask");
  if (surface.answer_awaiting_reification) reasons.push("Answer awaiting reification");
  if (surface.surface_dirty) reasons.push("dirty derived field");
  if (questions.some((question) => question.executable && ["Required", "Productive"].includes(question.disposition))) {
    reasons.push("live Required/Productive executable question");
  }
  if (surface.folds.some((fold) => fold.reopen_required)) reasons.push("fold requires reopening");
  if (carried.some((item) => item.status === "Generated")) reasons.push("transported discriminators await disposition");
  return { admissible: reasons.length === 0, reasons, coverage_relative: true };
}

function frontierRecord(root) {
  const text = fs.readFileSync(path.join(root, "IMPLEMENTATION_FRONTIER.md"), "utf8");
  const match = text.match(/<!-- LIVE_FRONTIER_BEGIN -->\s*([\s\S]*?)\s*<!-- LIVE_FRONTIER_END -->/u);
  if (!match) throw new Error("live frontier block is missing");
  return Object.fromEntries(match[1].trim().split(/\r?\n/u).map((line) => {
    const at = line.indexOf(":");
    return [line.slice(0, at).trim(), line.slice(at + 1).trim()];
  }));
}

function questionPacket(root, selected) {
  if (!selected) return null;
  const frontier = frontierRecord(root);
  const instance = selected.relational_instance ?? null;
  const memory = JSON.parse(fs.readFileSync(path.join(root, "formal-successor/REGENERATIVE_SPINE.json"), "utf8"));
  const candidateBasis = memory.current_semantic_kernel?.primitive_candidates ?? [];
  return {
    schema: 1,
    kind: "QuestionPacket",
    occurrence: selected.occurrence,
    exact_question: selected.prompt,
    canonical_prose: `${selected.prompt} Apply this independently to each entry of the candidate basis, then attempt to derive every alleged nonprimitive from the survivors.`,
    answer_type: "CandidateReturn",
    bound_roles: instance?.bindings ?? {},
    open_roles: instance?.open_roles ?? [],
    activation_witness: { disposition: selected.disposition, executable: selected.executable, frontier: frontier.id },
    protected_consequence: frontier.protected_difference,
    dependency_context: selected.dependencies ?? [],
    candidate_basis: candidateBasis,
    live_goal: frontier.goal,
    decisive_breakers: [frontier.discriminator, frontier.if_fail],
    horizon: selected.horizon ?? frontier.horizon,
    coverage: selected.coverage ?? "declared by occurrence ancestry",
    output_contract: {
      exact_fields: ["candidate", "exact_types", "hypotheses", "derivation_attempt", "breaker", "disposition", "propagation", "residuals"],
      allowed_dispositions: ["proved_candidate", "derivable_candidate", "binding_conditional", "broken", "inapplicable", "unresolved"],
      authority: "candidate_only; independent checks and frontier review required"
    }
  };
}

function build(root) {
  const loaded = readContract(root);
  const surface = relational.build(root);
  const pathProjection = derivePaths(surface);
  const carried = transport(surface, pathProjection);
  const questions = liveQuestions(surface);
  const invalidated = surface.products.filter((product) => product.invalidated).map((product) => product.id);
  const selection = selectExecutable(questions, invalidated);
  const closure = evaluateClosure(surface, questions, carried);
  return {
    schema: 1,
    authority: loaded.contract.authority,
    recurrence: loaded.contract.model_recurrence,
    frontier: surface.active_residual,
    lifecycle: surface.lifecycle,
    relations: directRelations(surface),
    questions,
    paths: pathProjection,
    transported_discriminators: carried,
    folds: surface.folds,
    selection,
    question_packet: questionPacket(root, selection.selected),
    closure,
    generation_failures: surface.generation_failures,
  };
}

function render(root) {
  const spine = build(root);
  const packet = spine.question_packet;
  return [
    "INQUIRY CALCULUS — ONE QUESTION PACKET",
    packet ? JSON.stringify(packet, null, 2) : JSON.stringify({ schema: 1, kind: "QuestionPacket", status: "no_executable_occurrence" }, null, 2),
    "Return exactly one CandidateReturn matching output_contract. Do not claim acceptance or warrant.",
    "",
  ].join("\n");
}

module.exports = { build, derivePaths, evaluateClosure, pathRecord, questionPacket, render, selectExecutable, transport };

if (require.main === module) {
  try {
    const [command, suppliedRoot] = process.argv.slice(2);
    const root = path.resolve(suppliedRoot ?? path.resolve(__dirname, "../.."));
    if (command === "context") process.stdout.write(render(root));
    else if (command === "json") process.stdout.write(`${JSON.stringify(build(root), null, 2)}\n`);
    else if (command === "paths") process.stdout.write(`${JSON.stringify(build(root).paths, null, 2)}\n`);
    else if (command === "questions") process.stdout.write(`${JSON.stringify(build(root).questions, null, 2)}\n`);
    else if (command === "select") process.stdout.write(`${JSON.stringify(build(root).selection, null, 2)}\n`);
    else if (command === "packet") process.stdout.write(`${JSON.stringify(build(root).question_packet, null, 2)}\n`);
    else throw new Error("usage: ic-spine.js context|json|paths|questions|select|packet [ROOT]");
  } catch (error) {
    process.stderr.write(`ic-spine: ${error.message}\n`);
    process.exitCode = 1;
  }
}
