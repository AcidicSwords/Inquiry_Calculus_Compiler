"use strict";

// Process-level evidence admission, not semantic equivalence or self-warrant.
// Schema 2 additionally binds a deliberately small first-order observation
// program to exact question identities and re-executes it. This establishes only
// that finite engineering projection. Arbitrary tools, models, meanings and the
// sufficiency of a declared horizon remain independent obligations.
const fs = require("node:fs");
const path = require("node:path");
const crypto = require("node:crypto");
const { canonical } = require("./ic-question-instance.js");
const hash = (bytes) => crypto.createHash("sha256").update(bytes).digest("hex");
const same = (a, b) => canonical(a) === canonical(b);
function requireThat(ok, reason) { if (!ok) throw new Error(`fold evidence: ${reason}`); }
function strings(value, name) {
  requireThat(Array.isArray(value) && value.length > 0 && value.every((x) => typeof x === "string" && x.trim()) &&
    new Set(value).size === value.length, `${name} must be a nonempty unique string array`);
  return value;
}
function exact(value, keys, name) {
  requireThat(value && !Array.isArray(value) && typeof value === "object" &&
    Object.keys(value).length === keys.length && keys.every((key) => Object.hasOwn(value, key)),
  `${name} requires exactly ${keys.join(",")}`);
}
const identityKeys = new Set(["occurrence", "question_form", "rendering", "prompt", "source_lines",
  "generator_ids", "path", "dependencies", "relational_instance"]);
function identity(member) {
  const keys = ["occurrence", "question_form", "rendering", "prompt", "source_lines", "generator_ids", "path", "dependencies", "relational_instance"];
  return Object.fromEntries(keys.filter((k) => Object.hasOwn(member, k)).map((k) => [k, member[k]]));
}
function identityDigest(member) { return hash(canonical(identity(member))); }
function hasDigest(text, digest) { return typeof text === "string" && text.split(/[;,\s]+/u).includes(digest); }

function reportFor(product, digest, state, root, schema) {
  requireThat(product && !state.invalidated.has(product.id), "missing or invalidated product");
  requireThat(["checked", "warranted"].includes(product.status), "product is not checked");
  const origin = state.answers.get(state.productOrigins.get(product.id));
  requireThat(origin && origin.ask.mode === "Probe" && ["checked", "warranted"].includes(origin.status) &&
    ["Supported", "Plural"].includes(origin.resolution) && ["Positive", "Mixed"].includes(origin.record.polarity),
  "evidence needs a completed positively supported checked Probe Answer");
  requireThat(/^[0-9a-f]{64}$/u.test(digest), "invalid Raw digest");
  const raw = state.raws.find((r) => r.ask_occurrence === origin.ask.occurrence && r.digest === digest);
  requireThat(raw && raw.sensitive === "false", "Raw is missing, foreign, or not inspectable");
  requireThat(raw.raw_ref === `.claude/trace/raw/${digest}`, "Raw reference is not its canonical byte store");
  requireThat(hasDigest(origin.record.evidence, digest) && state.checks.some((r) =>
    r.ask_occurrence === origin.ask.occurrence && r.seq < origin.seq && hasDigest(r.evidence, digest)),
  "Answer and Check do not support the exact Raw report");
  const bytes = fs.readFileSync(path.join(root, raw.raw_ref));
  requireThat(hash(bytes) === digest, "Raw bytes do not match their digest");
  let report;
  try { report = JSON.parse(bytes.toString("utf8")); } catch { throw new Error("fold evidence: Raw is not a structured checker report"); }
  requireThat(report?.schema === schema, "report schema differs from its evidence descriptor");
  return report;
}

function execute(program, member) {
  exact(program, ["schema", "language", "field"], "observation program");
  requireThat(program.schema === 1 && program.language === "question_identity_projection",
    "unsupported observation program");
  requireThat(identityKeys.has(program.field), "observation program projects a non-identity field");
  const represented = identity(member);
  requireThat(Object.hasOwn(represented, program.field), `question identity has no ${program.field} field`);
  return represented[program.field];
}

function protection(product, state, root) {
  const p = product.inquiry_protection;
  const active = Number(state.foldEvidenceSchema);
  requireThat(p && p.schema === active && [1, 2].includes(p.schema) && typeof p.raw_digest === "string",
    "protected continuation descriptor differs from active evidence policy");
  strings(p.targets, "protected targets");
  for (const id of p.targets) requireThat(state.questions.has(id), `unrepresented protection target ${id}`);
  const report = reportFor(product, p.raw_digest, state, root, p.schema);
  if (p.schema === 1) {
    requireThat(report.kind === "protected_continuation" && same(report.targets, p.targets), "protection report targets differ");
  } else {
    exact(p, ["schema", "targets", "execution", "raw_digest"], "protected continuation descriptor");
    const claim = { targets: p.targets, execution: p.execution };
    requireThat(report.kind === "protected_continuation" && same(report.claim, claim), "protection report claim differs");
    requireThat(Array.isArray(report.observations) && report.observations.length === p.targets.length,
      "protected execution coverage is incomplete or duplicated");
    const seen = new Set();
    for (const cell of report.observations) {
      requireThat(p.targets.includes(cell.member) && cell.status === "Supported" && Object.hasOwn(cell, "value"),
        "uncovered or unsupported protected execution");
      requireThat(!seen.has(cell.member), "duplicate protected execution cell"); seen.add(cell.member);
      requireThat(same(cell.value, execute(p.execution, state.questions.get(cell.member))),
        "reported protected observation differs from first-order execution");
    }
  }
  // The immutable product id is the continuation identity. Labels are not identities.
  return p;
}

function protectedFor(members, state) {
  return [...state.products.values()].filter((p) => p.inquiry_protection && !state.invalidated.has(p.id) &&
    p.inquiry_protection.targets.some((id) => members.includes(id))).map((p) => p.id).sort();
}

function certificate(product, state, root) {
  const e = product?.fold_evidence;
  const active = Number(state.foldEvidenceSchema);
  requireThat(e?.schema === active && [1, 2].includes(e.schema) &&
    ["protected_equivalence", "regeneration"].includes(e.relation),
  "fold certificate differs from active evidence policy");
  strings(e.members, "members"); strings(e.continuations, "continuations");
  requireThat(e.members.length >= 2 && e.members.includes(e.representative), "invalid certificate representative");
  requireThat(typeof e.horizon === "string" && e.horizon.trim() && typeof e.coverage === "string" && e.coverage.trim(), "missing exact scope");
  const identities = Object.create(null);
  for (const id of e.members) {
    requireThat(state.questions.has(id), `unrepresented member ${id}`);
    identities[id] = identityDigest(state.questions.get(id));
  }
  requireThat(same(identities, e.member_identities), "member identity, rendering or path differs");
  const protections = new Map();
  for (const id of e.continuations) {
    requireThat(state.products.get(id)?.inquiry_protection && !state.invalidated.has(id), `continuation ${id} is not independently admitted`);
    const admitted = protection(state.products.get(id), state, root);
    if (e.schema === 2) {
      requireThat(e.members.every((member) => admitted.targets.includes(member)),
        `continuation ${id} is not applicable to every fold member`);
      protections.set(id, admitted);
    }
    requireThat(product.dependencies.includes(id), `continuation ${id} is absent from dependencies`);
  }
  const report = reportFor(product, e.raw_digest, state, root, e.schema);
  const { raw_digest: _digest, ...claim } = e;
  requireThat(report.kind === "fold_check" && same(report.claim, claim), "Raw report is about another fold claim");
  if (e.relation === "protected_equivalence") {
    requireThat(Array.isArray(report.observations) && report.observations.length === e.members.length * e.continuations.length,
      "observation coverage is incomplete or duplicated");
    const cells = new Map();
    for (const cell of report.observations) {
      requireThat(e.members.includes(cell.member) && e.continuations.includes(cell.continuation) &&
        cell.status === "Supported" && Object.hasOwn(cell, "value"), "uncovered or unsupported observation");
      const key = canonical([cell.member, cell.continuation]);
      requireThat(!cells.has(key), "duplicate observation cell"); cells.set(key, cell.value);
    }
    for (const c of e.continuations) for (const m of e.members) {
      if (e.schema === 2) requireThat(same(cells.get(canonical([m, c])),
        execute(protections.get(c).execution, state.questions.get(m))),
      "fold report differs from protected first-order execution");
      requireThat(same(cells.get(canonical([m, c])), cells.get(canonical([e.representative, c]))), "protected observations distinguish members");
    }
  } else {
    const expected = Object.fromEntries(e.members.map((id) => [id, identity(state.questions.get(id))]));
    requireThat(same(report.regenerated_members, expected), "regeneration loses exact occurrence ancestry");
  }
  return e;
}

function admitFold(record, state, root) {
  let members, continuations;
  try { members = JSON.parse(record.members); continuations = JSON.parse(record.protected_continuations); }
  catch { throw new Error("fold evidence: protected_continuations must be explicit JSON"); }
  strings(continuations, "protected_continuations");
  const eqProduct = state.products.get(record.protected_equivalence_evidence);
  const regenProduct = state.products.get(record.regeneration);
  const eq = certificate(eqProduct, state, root), regen = certificate(regenProduct, state, root);
  requireThat(eq.relation === "protected_equivalence" && regen.relation === "regeneration" && eq.raw_digest !== regen.raw_digest,
    "equivalence and regeneration require distinct matching reports");
  const expected = { members, representative: record.representative, horizon: record.horizon, coverage: record.coverage, continuations };
  for (const e of [eq, regen]) {
    for (const [key, value] of Object.entries(expected)) requireThat(same(e[key], value), `fold ${key} differs from its evidence`);
  }
  requireThat(same([...continuations].sort(), protectedFor(members, state)), "fold omits or invents a currently protected continuation");
  const dependencies = new Set([eqProduct.id, regenProduct.id, ...continuations]);
  // Resolve the complete transitive support field, not only direct dependencies.
  const pending = [...dependencies];
  while (pending.length) {
    const id = pending.pop(), p = state.products.get(id);
    requireThat(p && !state.invalidated.has(id), `missing or invalidated support ${id}`);
    for (const d of p.dependencies) if (!dependencies.has(d)) { dependencies.add(d); pending.push(d); }
  }
  return { evidence_schema: Number(state.foldEvidenceSchema), continuations: [...continuations], support: [...dependencies], reopen_required: false, reopen_reasons: [] };
}

function refresh(state) {
  for (const fold of state.folds.values()) {
    if (![1, 2].includes(fold.evidence_schema) || fold.state !== "folded") continue;
    const reasons = fold.support.filter((id) => state.invalidated.has(id)).map((id) => `invalidated:${id}`);
    for (const id of protectedFor(fold.members, state)) if (!fold.continuations.includes(id)) reasons.push(`new-continuation:${id}`);
    if (!reasons.length) continue;
    fold.reopen_required = true;
    fold.reopen_reasons = [...new Set([...fold.reopen_reasons, ...reasons])];
    state.fieldRefresh = true;
  }
}

module.exports = { identity, identityDigest, execute, protection, certificate, admitFold, refresh };
