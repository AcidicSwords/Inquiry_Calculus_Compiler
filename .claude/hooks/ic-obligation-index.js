#!/usr/bin/env node
"use strict";

// Derived construction obligation field.
//
//   ProtSurface -> ReqObl -> Live -> generated/covered -> executable -> selection
//
// The obligation index is generated, deterministic, deletable, reconstructible and
// non-authoritative. It is construction metainfrastructure, not Inquiry Calculus
// semantics. Nothing here warrants a claim: an obligation is a question, and a
// disposition records evidence obtained elsewhere.
//
// The reference live field is computed WITHOUT consulting the question generator so
// that a generator failure is visible as a gap instead of silently emptying inquiry.

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const cp = require("node:child_process");
const constructionSurface = require("./ic-construction-surface.js");
const evidence = require("./ic-construction-evidence.js");

const sha256 = (bytes) => crypto.createHash("sha256").update(bytes).digest("hex");

// Terminal dispositions end an obligation. Every other disposition keeps it live or
// explicitly nonterminal; silence is never a disposition.
const TERMINAL = new Set(["Proved", "Broken", "Inapplicable"]);
const NONTERMINAL_OPERATIONAL = new Set([
  "Blocked", "ResourceBounded", "RepresentationGap", "GeneratorGap", "OperationalUnknown",
]);

function fail(message) {
  throw new Error(message);
}

function obligation(fields) {
  for (const key of ["id", "kind", "statement", "disposition", "provenance"]) {
    if (typeof fields[key] !== "string" || !fields[key].trim()) {
      fail(`obligation requires nonempty ${key} (${fields.id ?? "<unknown>"})`);
    }
  }
  return {
    depends_on: [],
    subjects: [],
    protected_consequence: "declared by subject ancestry",
    horizon: "declared by subject ancestry",
    coverage: "declared by subject ancestry",
    breakers: [],
    activation: null,
    executable: false,
    blocked_reason: null,
    ...fields,
  };
}

// ---------------------------------------------------------------------------
// Obligation generators. Applicability is checked; a schema is never filled for
// its own sake, and an inapplicable obligation records why it does not apply.
// ---------------------------------------------------------------------------

function correspondenceObligations(items) {
  const output = [];
  const capabilities = items.filter((entry) => entry.kind === "protected_capability")
    .sort((left, right) => (left.dependency_order ?? 0) - (right.dependency_order ?? 0));
  for (const capability of capabilities) {
    if (capability.protection_loss) output.push(obligation({
      id: `FORMAL-RESTORE-${capability.id}`, kind: "protected_deletion",
      statement: `Restore or explicitly correct the accepted protected target ${capability.title}; its disappearance has no continuity witness.`,
      disposition: "Required", provenance: "Git accepted predecessor epoch", subjects: [capability.id],
      capability_order: 0, correspondence_order: 0, gate: "C",
      breakers: ["Deleting a protected target must not delete its obligations."],
    }));
    const records = (capability.remaining_correspondence ?? [])
      .filter((entry) => entry && typeof entry === "object");
    for (const record of records.sort((left, right) => (left.order ?? 0) - (right.order ?? 0))) {
      // Self-warrant exclusion: a correspondence record may not discharge itself.
      // Discharge must name independent checked evidence outside the record.
      const discharge = typeof record.discharged_by === "string" ? record.discharged_by.trim() : null;
      if (discharge && (discharge === record.id || /^(?:self|none|generated|assumed|candidate)\b/iu.test(discharge))) {
        fail(`correspondence ${record.id} claims self-warrant through discharged_by=${discharge}`);
      }
      output.push(obligation({
        id: record.id,
        kind: "protected_correspondence",
        statement: record.goal,
        // A file reference is a search hint, never a proof or a discharge.
        disposition: "Required",
        provenance: `${capability.provenance}/${capability.title}/remaining_correspondence`,
        subjects: [capability.id],
        depends_on: record.depends_on ?? [],
        protected_consequence: record.protected_difference ?? capability.title,
        horizon: record.horizon ?? capability.horizon,
        coverage: capability.checked_partial_result ?? capability.coverage,
        breakers: [record.discriminator, record.if_fail].filter(Boolean),
        gate: record.gate ?? "C",
        capability_order: capability.dependency_order ?? 0,
        correspondence_order: record.order ?? 0,
        if_pass: record.if_pass ?? "",
        if_fail: record.if_fail ?? "",
        relevant_decisions: record.relevant_decisions ?? [],
        relevant_failures: record.relevant_failures ?? [],
        discharged_by: discharge,
        checked_partial_artifact: capability.checked_partial_artifact ?? null,
        prior_occurrences: record.prior_occurrences ?? [],
      }));
    }
    // A capability with no typed correspondence records still owes regeneration.
    // Its obligation is live but blocked behind every lower dependency order.
    if (records.length === 0) {
      output.push(obligation({
        id: `REGEN-${capability.successor_target ?? capability.id}`,
        kind: "protected_regeneration",
        statement: `Regenerate the protected predecessor capability ${capability.title} in the successor with an exact correspondence witness.`,
        disposition: "Required",
        provenance: capability.provenance,
        subjects: [capability.id],
        protected_consequence: `Loss of ${capability.title} is a protected capability loss, not a simplification.`,
        capability_order: capability.dependency_order ?? 0,
        correspondence_order: 0,
        gate: "C",
        ambient_requirements: capability.ambient_requirements ?? [],
      }));
    }
  }
  for (const entry of output) {
    entry.depends_on = [...new Set([...entry.depends_on, ...output.filter((prior) =>
      prior.capability_order < entry.capability_order &&
      ["protected_correspondence", "protected_regeneration", "protected_deletion"].includes(prior.kind)).map((prior) => prior.id)])];
  }
  return output;
}

// Total continuity: every protected predecessor item requires a disposition.
// Absence from a later document is never a valid disposition.
function continuityObligations(items) {
  return items.filter((entry) => entry.kind === "protected_capability").map((capability) => obligation({
    id: `CONT-${capability.id}`,
    kind: "continuity_disposition",
    statement: `Record an explicit continuity disposition for protected predecessor capability ${capability.title}.`,
    disposition: capability.checked_partial_artifact ? "Open" : "Required",
    provenance: capability.provenance,
    subjects: [capability.id],
    protected_consequence: "retained, derived, strengthened, split, rebound, binding-conditional, broken, reopened, operational-only, or still-open must be stated",
    capability_order: capability.dependency_order ?? 0,
    correspondence_order: 99,
    gate: "C",
  }));
}

function seedObligations(items) {
  const output = [];
  for (const seed of items.filter((entry) => entry.kind === "theorem_seed")) {
    output.push(obligation({
      id: `THMOBL-${seed.registry_id}`,
      kind: "registered_theorem_seed",
      statement: seed.statement || seed.title,
      disposition: "Open",
      provenance: seed.provenance,
      subjects: [seed.id],
      depends_on: (seed.dependencies ?? []).map((id) => `THMOBL-${id.replace(/^THM-/u, "")}`),
      breakers: seed.decisive_check ? [seed.decisive_check] : [],
      gate: seed.gate,
      capability_order: 50,
      correspondence_order: 0,
      registry_id: seed.registry_id,
    }));
  }
  return output;
}

// Declaration-generated obligations. Applicability is checked per declaration:
// a claim declaration owes breaker coverage and proof ancestry, a carrier
// declaration owes formation/typing, and denotation applies only where the
// owning module actually declares an interpretation.
function declarationObligations(items, denotingModules) {
  const output = [];
  for (const declaration of items.filter((entry) => entry.kind === "successor_declaration")) {
    if (declaration.claim) {
      output.push(obligation({
        id: `DECLBRK-${declaration.id.replace(/^DECL-/u, "")}`,
        kind: "declaration_breaker_coverage",
        statement: `Establish that ${declaration.title} has proof or countermodel coverage appropriate to the claim it makes.`,
        disposition: "Open",
        provenance: declaration.provenance,
        subjects: [declaration.id],
        capability_order: 60,
        correspondence_order: 0,
        gate: "C",
      }));
      output.push(obligation({
        id: `DECLPRE-${declaration.id.replace(/^DECL-/u, "")}`,
        kind: "proof_presupposition",
        statement: `Reconstruct the proof ancestry of ${declaration.title} until every leaf terminates in an admitted foundation.`,
        disposition: "Open",
        provenance: declaration.provenance,
        subjects: [declaration.id],
        capability_order: 61,
        correspondence_order: 0,
        gate: "C",
      }));
      continue;
    }
    output.push(obligation({
      id: `DECLTYP-${declaration.id.replace(/^DECL-/u, "")}`,
      kind: "declaration_typing",
      statement: `Establish that carrier declaration ${declaration.title} is well typed and its formation laws are exact.`,
      disposition: "Open",
      provenance: declaration.provenance,
      subjects: [declaration.id],
      capability_order: 62,
      correspondence_order: 0,
      gate: "C",
    }));
    if (denotingModules.has(declaration.module)) {
      output.push(obligation({
        id: `DECLDEN-${declaration.id.replace(/^DECL-/u, "")}`,
        kind: "declaration_denotation",
        statement: `Establish the denotation of ${declaration.title} and its stability under reindexing.`,
        disposition: "Open",
        provenance: declaration.provenance,
        subjects: [declaration.id],
        capability_order: 63,
        correspondence_order: 0,
        gate: "C",
      }));
    }
    // Implementation correspondence is genuinely inapplicable before Gate F.
    output.push(obligation({
      id: `DECLIMP-${declaration.id.replace(/^DECL-/u, "")}`,
      kind: "implementation_correspondence",
      statement: `Bind ${declaration.title} to its Rust implementation through the checked successor-to-Rust delta.`,
      disposition: "Blocked",
      provenance: declaration.provenance,
      subjects: [declaration.id],
      blocked_reason: "Formal Gate F is not recorded as passed; Rust remains downstream and frozen.",
      capability_order: 90,
      correspondence_order: 0,
      gate: "F",
    }));
  }
  return output;
}

// Ablation is mandatory in both directions: discover missing structure and remove
// unnecessary structure. Singleton ablation never certifies joint independence.
function ablationObligations(items) {
  const candidates = items.filter((entry) => entry.kind === "candidate_capability");
  const output = candidates.map((candidate) => obligation({
    id: `ABL-${candidate.id.replace(/^CAND-/u, "")}`,
    kind: "capability_ablation",
    statement: `Delete ${candidate.title} from the candidate basis and determine which protected capability, if any, fails to regenerate.`,
    disposition: "Open",
    provenance: candidate.provenance,
    subjects: [candidate.id],
    protected_consequence: "A capability is retained only when a protected predecessor distinction fails to regenerate without it.",
    breakers: ["Failed search for a derivation is not proof of independence."],
    // Ablation is a premise of basis minimality, not blocked on its conclusion.
    depends_on: [],
    capability_order: 40,
    correspondence_order: 0,
    gate: "C",
  }));
  if (candidates.length > 1) {
    output.push(obligation({
      id: "ABL-JOINT-CANDIDATE-BASIS",
      kind: "joint_ablation",
      statement: "Search joint removals over the candidate basis, since singleton ablation cannot expose mutual redundancy.",
      disposition: "Open",
      provenance: "formal-successor/REGENERATIVE_SPINE.json#current_semantic_kernel.primitive_candidates",
      subjects: candidates.map((candidate) => candidate.id),
      depends_on: candidates.map((candidate) => `ABL-${candidate.id.replace(/^CAND-/u, "")}`),
      capability_order: 41,
      correspondence_order: 0,
      gate: "C",
    }));
  }
  return output;
}

function inquiryObligations(items) {
  return items.filter((entry) => entry.kind === "candidate_inquiry").map((inquiry) => obligation({
    id: `INQOBL-${inquiry.id.replace(/^INQ-/u, "")}`,
    kind: "candidate_inquiry",
    statement: inquiry.title,
    disposition: "Open",
    provenance: inquiry.provenance,
    subjects: [inquiry.id],
    breakers: inquiry.breaker ? [inquiry.breaker] : [],
    missing_structure: inquiry.missing_structure ?? [],
    depends_on: inquiry.dependencies.map((id) => `THMOBL-${id.replace(/^THM-/u, "")}`),
    capability_order: 45,
    correspondence_order: 0,
    gate: "C",
    // A candidate inquiry whose missing structure is unrepresentable in the
    // current language is a representation gap, not a semantic absence.
    // Missing mathematics is a dependency, not evidence of language failure.
    representation_gap: false,
  }));
}

function breakerObligations(items) {
  return items.filter((entry) => entry.kind === "known_breaker").map((breaker) => obligation({
    id: `MODEL-${breaker.id.replace(/^BRK-/u, "")}`,
    kind: "permanent_breaker",
    statement: `Keep the checked breaker discriminating: ${breaker.title}`,
    disposition: "Open",
    provenance: breaker.provenance,
    subjects: [breaker.id],
    capability_order: 70,
    correspondence_order: 0,
    gate: "D",
  }));
}

function corpusObligations(items) {
  return items.filter((entry) => entry.kind === "corpus_obligation").map((form) => obligation({
    id: `CORPOBL-${form.form_id}`,
    kind: "corpus_disposition",
    statement: `Retain an explicit disposition for corpus form ${form.form_id} under the current formal question language.`,
    disposition: "Open",
    provenance: form.provenance,
    subjects: [form.id],
    capability_order: 80,
    correspondence_order: 0,
    gate: "I",
  }));
}

function languageObligations(items) {
  return items.filter((entry) => entry.kind === "proved_theorem").map((proved) => obligation({
    id: `LANG-${proved.id.replace(/^PROVED-/u, "")}`,
    kind: "language_correspondence",
    statement: `Establish exact formal/notation/prose correspondence for ${proved.title} without prose asserting stronger modality than the theorem.`,
    disposition: "Open",
    provenance: proved.provenance,
    subjects: [proved.id],
    protected_consequence: "prose must not turn relation into causation, converse into inverse, possibility into actuality, checking into warrant, or failed search into impossibility",
    capability_order: 85,
    correspondence_order: 0,
    gate: "J",
  }));
}

function structuralObligations(items) {
  const output = [];
  for (const subject of items) {
    const kinds = {
      predecessor_declaration: "predecessor_continuity",
      normalization_continuity: "normalization_continuity",
      residual_seed: "residual_discharge",
      non_collapse_requirement: "non_collapse",
      declared_nonprimitive: "regeneration_after_removal",
      derived_definition_candidate: "derivation_or_missing_structure",
      unresolved_typed_obligation: "typed_residual",
      source_question: "source_corpus_disposition",
      construction_requirement: "requirement_applicability",
    };
    const kind = kinds[subject.kind];
    if (kind) output.push(obligation({
      id: `COVER-${subject.id}`, kind, statement: `Give the scoped checked disposition of ${subject.title}. Determine applicability explicitly; silence and an unproved prose assumption are not dispositions.`,
      disposition: "Open", provenance: subject.provenance, subjects: [subject.id],
      capability_order: 75, gate: "C",
    }));
    if (subject.kind !== "successor_declaration") continue;
    output.push(obligation({
      id: `CONTRACT-${subject.id}`, kind: "declaration_contract",
      statement: `For ${subject.title}, determine the applicable formation, typing, denotation, substitution, composition, protection, notation, prose, recovery and reopening laws. Exhibit exact types and hypotheses for each, or a checked inapplicability witness.`,
      disposition: "Open", provenance: subject.provenance, subjects: [subject.id], capability_order: 64, gate: "C",
    }));
    output.push(obligation({
      id: `DEFORM-${subject.id}`, kind: "deformation_applicability",
      statement: `Remove, replace, weaken, strengthen, reverse, transport or change the context/binding/representation of ${subject.title} where well typed. Find a protected loss or a smaller regeneration; localize a decisive breaker by subtraction. Identify joint removals when singleton deletion is insufficient.`,
      disposition: "Open", provenance: subject.provenance, subjects: [subject.id], capability_order: 65, gate: "C",
    }));
    // Actual import incidence, not pairwise linguistic analogy. This obligation
    // demands typed connection/separation before naming any commutation/adjunction.
    for (const imported of subject.imports ?? []) {
      output.push(obligation({
        id: `RELATE-${subject.id}-${constructionSurface.slug(imported)}`, kind: "relation_connectivity",
        statement: `Determine which declarations of ${imported} are consequential to ${subject.title}; give typed connection, separation, inapplicability or open dispositions, with exact composition/preservation/factorization hypotheses where applicable.`,
        disposition: "Open", provenance: subject.provenance, subjects: [subject.id], capability_order: 66, gate: "C",
      }));
    }
  }
  for (const [id, statement, gate] of [
    ["DOMAIN", "Discharge the independent domain bindings required by construction specification §§59 and 103 Phase K; remove source names, test in an alien binding, and recover the protected native relation without promoting binding assumptions.", "K"],
    ["LANGUAGE", "Discharge exact formal/notation/canonical-prose correspondence, all applicable round trips and modality non-collapse for the claimed successor surface.", "J"],
    ["RECONSTRUCTION", "Reconstruct the protected successor from the predecessor plus retained checked spine without historical explanatory narrative, and rerun primitive and joint elimination.", "L"],
    ["IMPACT", "Discharge conservative source-dependency impact coverage, including statements, proof presuppositions, bindings, protection horizons, questions, corpus, prose, conformance and implementation correspondence.", "C"],
  ]) output.push(obligation({ id: `FORMAL-CONSTRUCTION-${id}`, kind: id.toLowerCase(), statement,
    disposition: "Open", provenance: "formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md#117", capability_order: 90, gate }));
  return output;
}

function impactClosure(obligations, changed) {
  const affected = new Set(changed);
  let prior;
  do {
    prior = affected.size;
    for (const entry of obligations) if (entry.depends_on.some((id) => affected.has(id))) affected.add(entry.id);
  } while (prior !== affected.size);
  return [...affected].sort();
}

function runtimeState(root) {
  const pointer = path.join(root, ".claude/trace/.state");
  if (!fs.existsSync(pointer)) return {};
  const name = fs.readFileSync(pointer, "utf8").trim();
  if (!/^[A-Za-z0-9._-]+$/u.test(name)) fail("invalid active trace pointer");
  const records = fs.readFileSync(path.join(root, ".claude/trace", name), "utf8").split(/\r?\n/u).filter(Boolean).map(JSON.parse);
  return { actual_asks: records.filter((record) => record.kind === "ask"), actual_returns: records.filter((record) => record.kind === "raw"),
    actual_answers: records.filter((record) => record.kind === "answer") };
}

function allocation(entry, profiles, runtime) {
  const local = profiles.backends.find((backend) => backend.id === profiles.allocation.default_candidate_backend);
  const reviewer = profiles.backends.find((backend) => backend.id === profiles.allocation.required_review_backend);
  const aliases = new Set(entry.prior_occurrences ?? []);
  const asks = (runtime.actual_asks ?? []).filter((ask) => ask.obligation_identity === entry.id || aliases.has(ask.occurrence));
  const ids = new Set(asks.map((ask) => ask.occurrence));
  const used = (runtime.actual_returns ?? []).filter((raw) => ids.has(raw.ask_occurrence) && /ic-local-attempt\.js/u.test(raw.cmd ?? "")).length;
  const exhausted = used >= (local?.limits.attempts_per_occurrence ?? 0);
  const backend = exhausted || entry.reopened || entry.discharged_by ? reviewer : local;
  return { backend: backend?.id ?? null, local_attempts_used: used,
    resource_status: backend ? (exhausted ? "LocalResourceBounded_FrontierAvailable" : "Available") : "ResourceBounded",
    completed_attempts: (runtime.actual_answers ?? []).filter((answer) => ids.has(answer.ask_occurrence)).length };
}

// ---------------------------------------------------------------------------

function build(root, runtimeOverride = null) {
  const { surface, digest: surfaceDigest } = constructionSurface.build(root);
  const items = surface.items;
  const profiles = JSON.parse(fs.readFileSync(path.join(root, "formal-successor/BACKEND_PROFILES.json"), "utf8"));
  const runtime = runtimeOverride ?? runtimeState(root);
  let gateF = false;
  try {
    const accepted = cp.execFileSync("git", ["-C", root, "show", "HEAD:formal-successor/CONFORMANCE_STATUS.md"],
      { encoding: "utf8", windowsHide: true, stdio: ["ignore", "pipe", "pipe"] });
    gateF = /^\| FORMAL-GATE-F \| PASS \|/mu.test(accepted);
  } catch { /* No accepted gate witness. */ }

  // Modules that actually declare an interpretation, used for denotation applicability.
  const denotingModules = new Set(items
    .filter((entry) => entry.kind === "successor_declaration" && /denote|denotation/iu.test(entry.title))
    .map((entry) => entry.module));

  const generated = [
    ...correspondenceObligations(items),
    ...continuityObligations(items),
    ...seedObligations(items),
    ...declarationObligations(items, denotingModules),
    ...ablationObligations(items),
    ...inquiryObligations(items),
    ...breakerObligations(items),
    ...corpusObligations(items),
    ...languageObligations(items),
    ...structuralObligations(items),
  ];

  const byId = new Map();
  for (const entry of generated) {
    if (entry.kind === "implementation_correspondence" && gateF) {
      entry.disposition = "Open"; entry.blocked_reason = null;
    }
    if (byId.has(entry.id)) fail(`duplicate obligation identity ${entry.id}`);
    byId.set(entry.id, entry);
  }

  const returns = evidence.readReturns(root);
  const sourceInputs = surface.generated_from.filter((source) => ![
    "formal-successor/REGENERATIVE_SPINE.json", "formal-successor/RESIDUAL_OBLIGATIONS.json",
    "formal-successor/INTEGRATED_THEOREM_OBLIGATIONS.json",
  ].includes(source.path));
  // Least closure over independently admitted residuals. These are checked
  // products of inquiry, not a hand-maintained cursor or model-selected schedule.
  let grew;
  do {
    grew = false;
    for (const record of returns) {
      const parent = byId.get(record.obligation);
      if (!parent || !evidence.checkReturn(root, parent, sourceInputs, record).valid) continue;
      for (const residual of record.new_obligations ?? []) {
        if (!residual.id || !residual.statement || !residual.horizon || !residual.protected_consequence) {
          fail(`checked return ${record.decision} contains an untyped residual`);
        }
        if (byId.has(residual.id)) continue;
        byId.set(residual.id, obligation({ ...residual, disposition: "Open", executable: false,
          provenance: record.decision, kind: "derived_checked_residual", capability_order: 1 }));
        grew = true;
      }
    }
  } while (grew);
  for (const entry of byId.values()) {
    const record = returns.filter((candidate) => candidate.obligation === entry.id).at(-1);
    const result = evidence.checkReturn(root, entry, sourceInputs, record);
    entry.evidence_fingerprint = evidence.fingerprint(entry, sourceInputs);
    entry.evidence_status = result;
    if (result.valid) entry.disposition = result.disposition;
    else if (record) entry.reopened = result.reason;
  }
  for (const entry of [...byId.values()]) {
    if (entry.disposition !== "Broken") continue;
    const record = returns.filter((candidate) => candidate.obligation === entry.id).at(-1);
    const id = `CORRECT-${entry.id}`;
    byId.set(id, obligation({ id, kind: "breaker_correction", statement: record.corrective_residual,
      disposition: "Required", provenance: record.decision, subjects: entry.subjects,
      capability_order: 0, gate: entry.gate, breakers: entry.breakers }));
  }
  // A broken premise is terminal as an inquiry but cannot supply a positive
  // hypothesis. Reopen all transitive dependents of stale or broken evidence.
  const invalidated = [...byId.values()].filter((entry) => entry.reopened || entry.disposition === "Broken").map((entry) => entry.id);
  for (const id of impactClosure([...byId.values()], invalidated)) {
    const entry = byId.get(id);
    if (entry.disposition === "Proved" && !invalidated.includes(id)) {
      entry.disposition = "Open"; entry.reopened = "invalidated dependency ancestry";
    }
  }

  // Activation: an obligation is active when every dependency it names is terminal.
  // A named dependency that does not exist is a representation failure, not silence.
  for (const entry of byId.values()) {
    const missing = entry.depends_on.filter((id) => !byId.has(id));
    const unmet = entry.depends_on.filter((id) => byId.has(id) && byId.get(id).disposition !== "Proved");
    entry.activation = {
      active: missing.length === 0 && unmet.length === 0,
      unmet_dependencies: unmet,
      missing_dependencies: missing,
    };
    if (missing.length > 0) {
      entry.disposition = "OperationalUnknown";
      entry.blocked_reason = `names undeclared dependencies: ${missing.join(",")}`;
    } else if (unmet.length > 0 && !TERMINAL.has(entry.disposition)) {
      entry.blocked_reason = `awaiting ${unmet.join(",")}`;
    }
  }

  // Reference live field, computed independently of the question generator.
  // Dependency-blocked obligations remain reference-live. Readiness only affects
  // execution; it must not remove an outstanding protected requirement.
  const live = [...byId.values()].filter((entry) => !TERMINAL.has(entry.disposition));

  // Executable field. Live is not executable: a live obligation may remain blocked,
  // unsupported, resource bounded, or awaiting a dependency and still be represented.
  for (const entry of live) {
    Object.assign(entry, allocation(entry, profiles, runtime));
    entry.executable = entry.activation.active && !entry.representation_gap &&
      !NONTERMINAL_OPERATIONAL.has(entry.disposition) && entry.blocked_reason === null && Boolean(entry.backend);
  }
  const executable = live.filter((entry) => entry.executable);

  // Selection among live executable occurrences uses explicit relations only.
  // A stable identity tie-break allocates execution and asserts no optimality.
  const rank = (entry) => [
    entry.disposition === "Required" ? 0 : 1,
    entry.capability_order ?? 99,
    entry.correspondence_order ?? 99,
    entry.id,
  ];
  const ranked = [...executable].sort((left, right) => {
    const a = rank(left);
    const b = rank(right);
    for (let index = 0; index < a.length; index += 1) {
      if (a[index] === b[index]) continue;
      return a[index] < b[index] ? -1 : 1;
    }
    return 0;
  });
  const selected = ranked[0] ?? null;

  const index = {
    schema: 1,
    status: "derived_rebuildable_construction_obligation_index_not_semantics_authority_or_history",
    generated_from: [...surface.generated_from, { path: "derived", sha256: surfaceDigest }],
    counts: {
      required: byId.size,
      live: live.length,
      executable: executable.length,
      terminal: [...byId.values()].filter((entry) => TERMINAL.has(entry.disposition)).length,
    },
    dispositions: Object.fromEntries([...new Set([...byId.values()].map((entry) => entry.disposition))].sort()
      .map((disposition) => [disposition, [...byId.values()].filter((entry) => entry.disposition === disposition).length])),
    selection: {
      selected: selected?.id ?? null,
      ranked: ranked.map((entry) => entry.id),
      tie_break_is_operational_only: true,
      relations: ["required_discharge", "protected_impact", "correspondence_order", "stable_identity"],
    },
    live: live.map((entry) => entry.id),
    executable: executable.map((entry) => entry.id),
    representation_gaps: [...byId.values()]
      .filter((entry) => entry.disposition === "RepresentationGap" || entry.representation_gap)
      .map((entry) => ({ obligation: entry.id, reason: entry.blocked_reason ?? "declared missing structure" })),
    impact: { invalidated, affected: impactClosure([...byId.values()], invalidated),
      coverage: "conservative full governing-source and Lean-module byte envelope; not exact proof-term dependency minimality" },
    obligations: [...byId.values()].sort((left, right) => left.id.localeCompare(right.id)),
  };
  return { index, selected, digest: sha256(JSON.stringify(index)) };
}

// Generator adequacy. Reach is what the question machinery can actually
// materialize; a live obligation with no covering occurrence is a generator gap,
// which means "not currently generated", never "false", "impossible", or "closed".
// The coverage relation nu >= omega. An occurrence covers an obligation only when
// it names that obligation exactly or declares it in `covers`. Sharing a subject is
// NOT adequacy: two obligations over the same capability can protect different
// distinctions, and treating one as covering the other would hide a real gap.
function coverage(root, reachOccurrences) {
  const { index } = build(root);
  const obligations = new Map(index.obligations.map((entry) => [entry.id, entry]));
  const declared = new Map();
  for (const entry of reachOccurrences) {
    if (!entry || typeof entry !== "object" || typeof entry.occurrence !== "string") continue;
    const obligation = obligations.get(entry.obligation_identity);
    if (obligation && entry.obligation_fingerprint === obligation.evidence_fingerprint &&
        entry.open_relation?.label === "ConstructionDischarge" &&
        entry.open_relation.statement === obligation.statement &&
        entry.bound_roles?.obligation?.id === obligation.id &&
        entry.open_roles?.[0]?.carrier === `DispositionWitness(${obligation.id})`) {
      declared.set(entry.occurrence, [obligation.id]);
    }
  }
  const covered = [];
  const gaps = [];
  const witnesses = new Map([...declared].flatMap(([occurrence, coveredIds]) => coveredIds.map((id) => [id, occurrence])));
  for (const id of index.live) {
    const entry = obligations.get(id);
    const witness = witnesses.get(id);
    if (witness) covered.push({ obligation: id, occurrence: witness });
    else gaps.push({ obligation: id, kind: entry.kind, state: "GeneratorGap" });
  }
  return { generator_adequate: gaps.length === 0, covered, generator_gaps: gaps,
    coverage: "exact construction discharge contracts, not successor semantic question completeness" };
}

function render(root) {
  const { index, selected } = build(root);
  return [
    `CONSTRUCTION OBLIGATION INDEX ${sha256(JSON.stringify(index)).slice(0, 16)}`,
    `required: ${index.counts.required}  live: ${index.counts.live}  executable: ${index.counts.executable}  terminal: ${index.counts.terminal}`,
    `dispositions: ${Object.entries(index.dispositions).map(([key, value]) => `${key}=${value}`).join(" ")}`,
    `selected: ${selected?.id ?? "none"}`,
    selected ? `  goal: ${selected.statement}` : "",
    `representation gaps: ${index.representation_gaps.length}`,
    "Live != Executable. NoGeneratedQuestion != NoLiveObligation. ResourceBounded != SemanticClosure.",
    "",
  ].filter(Boolean).join("\n");
}

module.exports = { build, coverage, render, TERMINAL, impactClosure, allocation, runtimeState };

if (require.main === module) {
  try {
    const [command, suppliedRoot, output] = process.argv.slice(2);
    const root = path.resolve(suppliedRoot ?? path.resolve(__dirname, "../.."));
    if (command === "json") process.stdout.write(`${JSON.stringify(build(root).index, null, 2)}\n`);
    else if (command === "projection") process.stdout.write(render(root));
    else if (command === "select") process.stdout.write(`${JSON.stringify(build(root).selected, null, 2)}\n`);
    else if (command === "digest") process.stdout.write(`${build(root).digest}\n`);
    else if (command === "build") {
      if (!output) fail("build requires an output path");
      fs.writeFileSync(path.resolve(output), `${JSON.stringify(build(root).index, null, 2)}\n`);
    } else fail("usage: ic-obligation-index.js json|projection|select|digest|build ROOT [OUTPUT]");
  } catch (error) {
    process.stderr.write(`ic-obligation-index: ${error.message}\n`);
    process.exitCode = 1;
  }
}
