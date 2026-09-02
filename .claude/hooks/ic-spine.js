#!/usr/bin/env node
"use strict";

// One public, derived inquiry-spine projection. Internal modules reconstruct
// evidence and relations; only this module composes them into model context.

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const cp = require("node:child_process");
const { read: readContract } = require("./ic-contract.js");
const relational = require("./ic-relational-surface.js");
const obligationIndex = require("./ic-obligation-index.js");

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

function evaluateClosure(surface, questions, carried, construction = null) {
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
  if (construction?.index.live.length) reasons.push("reference-live construction obligations remain");
  if (construction?.coverage.generator_gaps.length) reasons.push("construction generator coverage gap");
  if (construction?.index.representation_gaps.length) reasons.push("construction representation gap");
  return { admissible: reasons.length === 0, reasons, coverage_relative: true };
}

// One ordinary partially-bound relation, specialized to each exact obligation.
// This is a construction-level rendering, not a new calculus question language.
function constructionQuestions(root, built, surface, contract) {
  const form = contract.question_forms.find((entry) => entry.id === "CQ-OPEN-POSITION");
  if (!form) return [];
  const profiles = JSON.parse(fs.readFileSync(path.join(root, "formal-successor/BACKEND_PROFILES.json"), "utf8"));
  const live = new Set(built.index.live);
  return built.index.obligations.filter((entry) => live.has(entry.id)).map((entry) => {
    const assigned = obligationIndex.allocation(entry, profiles, surface);
    return {
      occurrence: `OCC-CONSTRUCTION-${digest([entry.id, entry.evidence_fingerprint]).slice(0, 24)}-${assigned.completed_attempts}`,
      obligation_identity: entry.id, obligation_fingerprint: entry.evidence_fingerprint,
      question_form: form.id, rendering: `RENDER-${form.id}`, prompt: form.prompt,
      source_lines: form.source_lines,
      generator_ids: contract.generator_registry.filter((generator) => generator.question_forms.includes(form.id)).map((generator) => generator.id),
      open_relation: { label: "ConstructionDischarge", statement: entry.statement,
        roles: [{ name: "obligation", carrier: "ConstructionObligation" }, { name: "disposition_witness", carrier: `DispositionWitness(${entry.id})` }] },
      bound_roles: { obligation: { carrier: "ConstructionObligation", id: entry.id } },
      open_roles: [{ name: "disposition_witness", carrier: `DispositionWitness(${entry.id})` }],
      context: `construction:${entry.id}:${entry.evidence_fingerprint}`, path: `formal-successor/construction/${entry.id}`,
      dependencies: entry.depends_on, disposition: entry.executable ? "Required" : "Blocked",
      executable: entry.executable && Boolean(assigned.backend),
      backend: assigned.backend, resource_status: assigned.resource_status,
      local_attempts_used: assigned.local_attempts_used,
    };
  });
}

// The packet derives its mathematical content from the SELECTED OBLIGATION in the
// construction obligation field. It no longer parses IMPLEMENTATION_FRONTIER.md,
// which is a generated projection of that same selection.
function questionPacket(root, selected, supplied = null) {
  if (!selected) return null;
  const { index } = supplied ?? obligationIndex.build(root);
  const obligation = index.obligations.find((entry) => entry.id === selected.obligation_identity);
  if (!obligation || selected.obligation_fingerprint !== obligation.evidence_fingerprint) {
    throw new Error("QuestionPacket occurrence does not cover the exact current obligation");
  }
  const instance = selected.relational_instance ?? null;
  const memory = JSON.parse(fs.readFileSync(path.join(root, "formal-successor/REGENERATIVE_SPINE.json"), "utf8"));
  const candidateBasis = memory.current_semantic_kernel?.primitive_candidates ?? [];
  return {
    schema: 1,
    kind: "QuestionPacket",
    obligation_identity: obligation.id,
    occurrence: selected.occurrence,
    exact_question: selected.prompt,
    // A generated operational rendering is not canonical prose. Canonical prose
    // requires the Gate-J formal/notation/prose correspondence, which is not yet
    // established, so this field stays explicitly operational.
    construction_rendering:
      `${selected.prompt} Open relation: ${obligation.statement}`,
    canonical_prose: null,
    canonical_prose_status: "unavailable_until_gate_j_language_correspondence",
    answer_type: "CandidateReturn",
    bound_roles: selected.bound_roles ?? instance?.bindings ?? {},
    open_roles: selected.open_roles ?? instance?.open_roles ?? [],
    open_relation: selected.open_relation,
    obligation_fingerprint: selected.obligation_fingerprint,
    backend: selected.backend,
    resource_status: selected.resource_status,
    activation_witness: {
      disposition: obligation.disposition,
      executable: obligation.executable,
      obligation: obligation.id,
      active: obligation.activation.active,
      unmet_dependencies: obligation.activation.unmet_dependencies,
      derived_from: obligation.provenance,
    },
    protected_consequence: obligation.protected_consequence,
    dependency_context: [...new Set([...(selected.dependencies ?? []), ...obligation.depends_on])],
    candidate_basis: candidateBasis,
    live_goal: obligation.statement,
    decisive_breakers: obligation.breakers,
    required_discharge: obligation.disposition === "Required",
    horizon: obligation.horizon,
    coverage: obligation.coverage,
    field_accounting: {
      required: index.counts.required,
      live: index.counts.live,
      executable: index.counts.executable,
      retained_unchosen: index.selection.ranked.slice(1, 6),
      representation_gaps: index.representation_gaps.length,
    },
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
  const construction = obligationIndex.build(root);
  const questions = constructionQuestions(root, construction, surface, loaded.contract);
  const byObligation = new Map(questions.map((question) => [question.obligation_identity, question]));
  const executable = construction.index.selection.ranked.map((id) => byObligation.get(id)).filter((question) => question?.executable);
  const selection = { selected: executable[0] ?? null, executable_frontier: executable, tie_break_is_operational_only: true };
  const covered = obligationIndex.coverage(root, questions);
  const closure = evaluateClosure(surface, questions, carried, { ...construction, coverage: covered });
  const packet = questionPacket(root, selection.selected, construction);
  // An unresolved effect remains the sole active task. Projection can describe
  // the next question, but invoking it is forbidden until the active return closes.
  if (packet && surface.unresolved_ask && surface.unresolved_ask.occurrence !== packet.occurrence) packet.execution_hold = {
    occurrence: surface.unresolved_ask.occurrence,
    reason: "complete the existing sealed return before another Ask",
  };
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
    question_packet: packet,
    retained_trace_questions: liveQuestions(surface),
    construction: { counts: construction.index.counts, selection: construction.index.selection, coverage: covered,
      impact: construction.index.impact },
    closure_certificate: {
      kind: "SuccessorDefinitionClosureCert", established: closure.admissible,
      horizon: "all represented consequential construction requirements; applicability is itself an open obligation",
      criteria: ["type", "capability", "declaration", "relation", "theorem_completeness", "proof_presupposition",
        "breaker", "dependency", "continuity", "generator_adequacy", "service", "corpus", "language", "domain",
        "regeneration", "self_warrant_exclusion", "residual_accounting"],
      unresolved: construction.index.live,
      reasons: closure.reasons,
      authority: "construction accounting only; never successor semantics or self-warrant",
    },
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

function begin(root, seal) {
  for (const key of ["should_change", "invariants", "discriminator", "wrong_impl", "coverage"]) {
    if (typeof seal[key] !== "string" || !seal[key].trim()) throw new Error(`prospective seal requires ${key}`);
  }
  const built = build(root), packet = built.question_packet;
  if (!packet || packet.execution_hold || built.lifecycle?.unresolved_ask || built.lifecycle?.answer_awaiting_reification) throw new Error("no lawful next Ask while an effect is unresolved or no executable occurrence exists");
  const directory = path.join(root, ".claude/trace"), pointer = path.join(directory, ".state"), fuel = path.join(directory, ".fuel");
  fs.mkdirSync(directory, { recursive: true });
  let trace;
  const append = (record) => {
    const result = cp.spawnSync(process.execPath, [path.join(root, ".claude/hooks/ic-append.js"), "append", trace, fuel],
      { cwd: root, encoding: "utf8", windowsHide: true, input: `${JSON.stringify({ ts: new Date().toISOString(), ...record })}\n` });
    if (result.status !== 0) throw new Error(result.stderr || result.stdout);
  };
  if (fs.existsSync(pointer)) trace = path.join(directory, fs.readFileSync(pointer, "utf8").trim());
  else {
    if (fs.readdirSync(directory).some((name) => name.endsWith(".jsonl"))) throw new Error("recover the existing trace pointer before starting a new ancestry");
    trace = path.join(directory, `${Date.now()}-construction.jsonl`);
    fs.writeFileSync(trace, "", { flag: "wx" }); fs.writeFileSync(pointer, path.basename(trace)); fs.writeFileSync(fuel, "24");
    const loaded = readContract(root);
    append({ kind: "policy", question_program_schema: "5", source_digest: loaded.corpusDigest,
      program_manifest_digest: loaded.contractDigest });
  }
  // Preserve all historical alternatives, but only fresh generated occurrences
  // can carry the current construction allocation.
  const retained = built.retained_trace_questions.map((question) => ({ ...question, disposition: "Unknown", executable: false }));
  const members = new Map([...retained, ...built.questions].map((question) => [question.occurrence, question]));
  const fieldId = `FIELD-CONSTRUCTION-${digest([packet, built.lifecycle?.record_count ?? 0]).slice(0, 24)}`;
  const records = fs.readFileSync(trace, "utf8").split(/\r?\n/u).filter(Boolean).map(JSON.parse);
  append({ kind: "field", field_id: fieldId, members: JSON.stringify([...members.values()]),
    basis: "derived construction obligations and exact ConstructionDischarge coverage", coverage: "all retained construction obligations; operational allocation only",
    regenerated_from: built.lifecycle?.surface_dirty ? records.filter((record) => record.kind === "reify").at(-1).answer_occurrence :
      built.lifecycle?.field_id ? "derived-after-checked-return" : "bootstrap", dispositions: "{}", removal_evidence: "{}" });
  const selected = built.selection.selected, loaded = readContract(root);
  append({ kind: "ask", occurrence: selected.occurrence, field_id: fieldId, q: selected.prompt, mode: "Probe",
    question_form: selected.question_form, rendering: selected.rendering, source_lines: selected.source_lines.join(","), generator_ids: selected.generator_ids.join(","),
    reciprocal_relations: "obligation/disposition,witness/breaker,source/dependency", context: selected.context, path: selected.path,
    bindings: canonical(packet.bound_roles), horizon: packet.horizon, coverage: packet.coverage,
    authority: "construction specification and current explicit task authorization", evidence: "independent checked return at sealed coverage",
    dependencies: selected.dependencies.join(",") || "none", source_digest: loaded.corpusDigest, program_manifest_digest: loaded.contractDigest,
    obligation_identity: selected.obligation_identity, obligation_fingerprint: selected.obligation_fingerprint,
    packet_digest: digest(packet), backend: selected.backend, fp: digest([packet, fieldId]) });
  append({ kind: "seal", ask_occurrence: selected.occurrence, packet_digest: digest(packet), ...seal });
  return { occurrence: selected.occurrence, obligation: selected.obligation_identity, backend: selected.backend, field: fieldId };
}

module.exports = { build, begin, constructionQuestions, derivePaths, digest, evaluateClosure, pathRecord, questionPacket, render, selectExecutable, transport };

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
    else if (command === "begin") process.stdout.write(`${JSON.stringify(begin(root, JSON.parse(fs.readFileSync(0, "utf8"))))}\n`);
    else throw new Error("usage: ic-spine.js context|json|paths|questions|select|packet [ROOT]");
  } catch (error) {
    process.stderr.write(`ic-spine: ${error.message}\n`);
    process.exitCode = 1;
  }
}
