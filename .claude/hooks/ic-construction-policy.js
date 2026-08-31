#!/usr/bin/env node
"use strict";

// Executable reference operations for the provisional construction harness.
// They check process behavior and do not define Inquiry Calculus semantics.

function requireArray(value, label) {
  if (!Array.isArray(value)) throw new Error(`${label} must be an array`);
  return value;
}

function selectWideContrast(candidates, compare) {
  const admissible = requireArray(candidates, "candidates").filter(
    (candidate) => candidate.admissible && candidate.decisive,
  );
  if (admissible.length === 0) return { status: "Unknown", candidate: null };
  if (typeof compare !== "function") {
    return { status: "incomparable_frontier", candidates: structuredClone(admissible) };
  }
  const ordered = admissible.toSorted(compare);
  return { status: "selected", candidate: ordered[0] };
}

function narrowRatchet(generatedRelations, supportedRelationIds) {
  const supported = new Set(supportedRelationIds);
  return requireArray(generatedRelations, "generatedRelations").filter(
    (relation) => supported.has(relation.id) && relation.independent_check === true,
  );
}

function closeLocalResidual(residuals, closedId, coverage) {
  return requireArray(residuals, "residuals").map((residual) =>
    residual.id === closedId
      ? { ...residual, state: "locally_closed", closure_coverage: coverage }
      : structuredClone(residual),
  );
}

function residualsSharingCoordinate(residuals, coordinateKind, coordinate) {
  return requireArray(residuals, "residuals").filter(
    (residual) => (residual[coordinateKind] ?? []).includes(coordinate),
  ).map((residual) => residual.id);
}

function localizeContradiction(left, right) {
  if (left.claim !== right.claim || left.polarity === right.polarity) {
    return { status: "not_a_collision" };
  }
  const candidateDimensions = ["applicability", "premise", "role", "path", "representation"];
  const differing = candidateDimensions.filter((dimension) =>
    JSON.stringify(left[dimension]) !== JSON.stringify(right[dimension]),
  );
  return differing.length > 0
    ? { status: "separator_required", dimensions: differing }
    : { status: "Unknown", dimensions: [], reason: "same-typed opposed returns require a new discriminator" };
}

function revisitFold(fold, continuation) {
  const separates = continuation.partition.some(
    (cell) => cell.includes(fold.left) !== cell.includes(fold.right),
  );
  return separates
    ? { ...fold, state: "reopened", breaker: continuation.id }
    : { ...fold, state: fold.state };
}

function findJointBreaker(coordinates, evaluate) {
  requireArray(coordinates, "coordinates");
  for (let size = 1; size <= coordinates.length; size += 1) {
    const frontier = [];
    function enumerate(start, selected) {
      if (selected.length === size) {
        frontier.push([...selected]);
        return;
      }
      for (let index = start; index < coordinates.length; index += 1) {
        selected.push(coordinates[index]);
        enumerate(index + 1, selected);
        selected.pop();
      }
    }
    enumerate(0, []);
    const breaker = frontier.find((combination) => evaluate(combination) === "break");
    if (breaker) return { status: "breaker", coordinates: breaker };
  }
  return { status: "Unknown", coordinates: [] };
}

function closureFromSearch({ breakerFound, coverageComplete }) {
  if (breakerFound) return "Negative";
  return coverageComplete ? "NoBreakerUnderDeclaredCompleteCoverage" : "Unknown";
}

function factorMethod(pathOccurrences) {
  const successful = requireArray(pathOccurrences, "pathOccurrences").filter(
    (occurrence) => occurrence.status === "supported",
  );
  if (successful.length < 2) return { status: "insufficient_recurrence" };
  const signature = successful[0].typed_path.join("->");
  if (!successful.every((occurrence) => occurrence.typed_path.join("->") === signature)) {
    return { status: "different_typed_paths" };
  }
  return {
    status: "candidate_method_not_warranted",
    typed_path: successful[0].typed_path,
    expansion: successful.map((occurrence) => occurrence.id),
    failure_exits: [...new Set(successful.flatMap((occurrence) => occurrence.failure_exits))],
  };
}

function acceptanceAuthority({ baselineDigest, candidateDigest, authority }) {
  if (baselineDigest === candidateDigest) return "unchanged";
  return authority === "explicit_user_control_migration" ? "authorized_change" : "reject_self_warrant";
}

function matchApplicableMethods(methodContracts, availableRelations) {
  const available = new Set(requireArray(availableRelations, "availableRelations"));
  return requireArray(methodContracts, "methodContracts").filter((contract) =>
    requireArray(contract.applicable_when, `${contract.id}.applicable_when`)
      .every((relation) => available.has(relation)),
  ).map((contract) => structuredClone(contract));
}

function normalizeConditionKey(key) {
  const fields = ["schema", "bound_roles", "scope", "applicability", "grain", "orientation"];
  for (const field of fields) {
    if (typeof key[field] !== "string" || !/^[A-Za-z0-9._-]+$/u.test(key[field])) {
      throw new Error(`condition key requires normalized ${field}`);
    }
  }
  if (!["forward", "reverse", "neutral"].includes(key.orientation)) {
    throw new Error("condition orientation must be forward, reverse, or neutral");
  }
  return fields.map((field) => key[field]).join("@");
}

function buildReverseIncidence(residuals) {
  const index = new Map();
  for (const residual of requireArray(residuals, "residuals")) {
    for (const feature of residual.boundary_features ?? []) {
      const identity = `${feature.kind}:${feature.key}`;
      if (!index.has(identity)) index.set(identity, new Set());
      index.get(identity).add(residual.id);
    }
  }
  return index;
}

function classifyQuestion(question) {
  if (question.answer !== undefined) return { disposition: "Answered", answer: question.answer };
  if (question.inapplicable_reason) return { disposition: "Inapplicable", reason: question.inapplicable_reason };
  if (question.blocked_reason) return { disposition: "Blocked", reason: question.blocked_reason };
  if (question.redundant_via) {
    return { disposition: "Productive", equivalence_candidate_via: question.redundant_via };
  }
  if (question.required) return { disposition: "Required" };
  if (question.productive) return { disposition: "Productive" };
  return { disposition: "Unknown", coverage: question.coverage ?? "undeclared" };
}

function resolveField(field) {
  if (field.blocked) return { resolution: "Blocked", blocker: field.blocked };
  if (field.resource_bounded) return { resolution: "ResourceBounded", residual: field.residual };
  if (!field.coverage_complete) return { resolution: "Unknown", coverage: field.coverage };
  if (field.supported === false) return { resolution: "Unsupported", evidence: field.evidence };
  if (field.classes.length === 0) {
    if (!field.certificate) return { resolution: "Unknown", coverage: field.coverage };
    return { resolution: "ExactEmpty", certificate: field.certificate };
  }
  if (field.classes.length === 1) return { resolution: "Supported", classes: [...field.classes] };
  return { resolution: "Plural", classes: [...field.classes] };
}

function comparatorApplicability(term, binding) {
  const requirements = {
    expected: "probability_model",
    smallest: "preorder",
    largest: "preorder",
    strongest: "strength_order",
    best: "preference_relation",
    cheapest: "cost_model",
  };
  const requirement = requirements[term];
  if (!requirement) return { disposition: "Productive" };
  return binding?.[requirement]
    ? { disposition: "Productive", comparator: requirement }
    : { disposition: "Inapplicable", reason: `missing ${requirement}` };
}

function selectQuestionFrontier(questions, dominates) {
  const candidates = requireArray(questions, "questions");
  if (typeof dominates !== "function") return structuredClone(candidates);
  return candidates.filter((candidate) => !candidates.some(
    (other) => other !== candidate && dominates(other, candidate),
  ));
}

module.exports = {
  acceptanceAuthority,
  buildReverseIncidence,
  classifyQuestion,
  closeLocalResidual,
  closureFromSearch,
  comparatorApplicability,
  factorMethod,
  findJointBreaker,
  localizeContradiction,
  matchApplicableMethods,
  narrowRatchet,
  normalizeConditionKey,
  residualsSharingCoordinate,
  resolveField,
  revisitFold,
  selectWideContrast,
  selectQuestionFrontier,
};
