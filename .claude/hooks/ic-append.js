#!/usr/bin/env node
"use strict";

// Validate and append one inquiry-trace record while holding an exclusive lock.
// Node is already the cross-platform launcher dependency for this harness.

const fs = require("node:fs");
const path = require("node:path");
const crypto = require("node:crypto");
const {
  validatePolicy,
  validateFieldRecord,
  validateReifiedSeeds,
  validateQuestionProgram,
  validateStoredField,
  validateStoredQuestion,
} = require("./ic-question-program.js");
const instances = require("./ic-question-instance.js");
const recursiveGenerator = require("./ic-recursive-generator.js");
const foldEvidence = require("./ic-fold-evidence.js");
const contractLoader = require("./ic-contract.js");
const repositoryRoot = path.resolve(__dirname, "../..");
const loadContract = () => contractLoader.read(repositoryRoot).contract;

const [operation, tracePath, fuelPath] = process.argv.slice(2);
if (!new Set(["validate", "state", "append"]).has(operation) || !tracePath) {
  process.stderr.write("ic-append: expected validate|state|append TRACE_FILE\n");
  process.exit(2);
}

const absolute = path.resolve(tracePath);
const lockPath = `${absolute}.lock`;

function fail(message) {
  throw new Error(message);
}

function sleep(milliseconds) {
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);
}

function processExists(pid) {
  if (!Number.isSafeInteger(pid) || pid <= 0) {
    return false;
  }
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return error.code === "EPERM";
  }
}

function removeStaleLock() {
  let stat;
  try {
    stat = fs.statSync(lockPath);
  } catch {
    return false;
  }
  const oldEnough = Date.now() - stat.mtimeMs > 30_000;
  if (!oldEnough) {
    return false;
  }
  let owner = null;
  try {
    owner = JSON.parse(fs.readFileSync(lockPath, "utf8"));
  } catch {
    // A process may die between exclusive creation and owner serialization.
    // Once old enough, such an unowned/corrupt lock is stale by construction.
  }
  if (owner !== null && processExists(owner.pid)) return false;
  try {
    fs.unlinkSync(lockPath);
    return true;
  } catch (error) {
    return error.code === "ENOENT";
  }
}

function acquireLock() {
  const deadline = Date.now() + 5000;
  while (true) {
    let fd;
    try {
      fd = fs.openSync(lockPath, "wx", 0o600);
      try {
        fs.writeFileSync(
          fd,
          `${JSON.stringify({ pid: process.pid, created: new Date().toISOString() })}\n`,
        );
        fs.fsyncSync(fd);
      } catch (error) {
        fs.closeSync(fd);
        fd = undefined;
        try {
          fs.unlinkSync(lockPath);
        } catch {
          // Preserve the original initialization failure.
        }
        throw error;
      }
      return fd;
    } catch (error) {
      if (error.code === "EEXIST" && removeStaleLock()) {
        continue;
      }
      if (error.code !== "EEXIST" || Date.now() >= deadline) {
        fail(`cannot acquire trace lock: ${error.message}`);
      }
      sleep(25);
    }
  }
}

function releaseLock(fd) {
  try {
    fs.closeSync(fd);
  } finally {
    try {
      fs.unlinkSync(lockPath);
    } catch (error) {
      if (error.code !== "ENOENT") {
        process.stderr.write(`ic-append: cannot remove lock: ${error.message}\n`);
      }
    }
  }
}

let validatedTraceDigest;
function validatedRecords() {
  let text;
  try {
    text = fs.readFileSync(absolute, "utf8");
  } catch (error) {
    fail(`cannot read trace: ${error.message}`);
  }
  if (text.length > 0 && !text.endsWith("\n")) {
    fail("trace does not end at a record boundary");
  }
  const lines = text.length === 0 ? [] : text.slice(0, -1).split("\n");
  const records = lines.map((line, index) => {
    let record;
    try {
      record = JSON.parse(line);
    } catch (error) {
      fail(`line ${index + 1} is not valid JSON: ${error.message}`);
    }
    const expectedSeq = index + 1;
    const expectedParent = index;
    if (
      record === null ||
      Array.isArray(record) ||
      typeof record !== "object" ||
      record.seq !== expectedSeq ||
      record.parent !== expectedParent ||
      typeof record.kind !== "string" ||
      record.kind.length === 0
    ) {
      fail(
        `line ${expectedSeq} must be an object with seq=${expectedSeq}, ` +
          `parent=${expectedParent}, and a nonempty kind`,
      );
    }
    return record;
  });
  const policies = records.filter((record) => record.kind === "policy");
  if (records.length > 0 && (policies.length !== 1 || policies[0].seq !== 1)) {
    fail("the schema-5 inquiry-spine policy must occur exactly once at the first record");
  }
  if (policies.length === 1) {
    const policy = policies[0];
    validatePolicy(policy);
    for (const record of records) {
      if (record.kind === "policy_transition" || record.kind === "question") {
        fail(`line ${record.seq} uses a retired controller record kind`);
      }
      if (record.kind === "ask") validateStoredQuestion(record, policy);
      if (record.kind === "field") validateStoredField(record);
    }
  }
  if (records.length > 0) validateStateMachine(records);
  validatedTraceDigest = crypto.createHash("sha256").update(text).digest("hex");
  return records;
}

function readStdin() {
  return fs.readFileSync(0, "utf8");
}

function parseJsonObject(record, field) {
  let value;
  try {
    value = JSON.parse(record[field]);
  } catch (error) {
    fail(`line ${record.seq} has invalid ${field} JSON: ${error.message}`);
  }
  if (value === null || Array.isArray(value) || typeof value !== "object") {
    fail(`line ${record.seq} requires ${field} to be a JSON object`);
  }
  return value;
}

function parseJsonArray(record, field) {
  let value;
  try {
    value = JSON.parse(record[field]);
  } catch (error) {
    fail(`line ${record.seq} has invalid ${field} JSON: ${error.message}`);
  }
  if (!Array.isArray(value)) fail(`line ${record.seq} requires ${field} to be a JSON array`);
  return value;
}

function requireRecordString(record, field) {
  if (typeof record[field] !== "string" || record[field].trim() === "") {
    fail(`line ${record.seq} requires nonempty ${field}`);
  }
}

function requireIndependentEvidence(record, field) {
  requireRecordString(record, field);
  if (/^(?:none|self|generated|assumed)(?:$|[:/_-])/iu.test(record[field].replace(/\s+/gu, ""))) {
    fail(`line ${record.seq} ${field} is not independent evidence`);
  }
}

function validateFieldTransition(previous, next, state, record) {
  const nextByOccurrence = new Map(next.members.map((member) => [member.occurrence, member]));
  for (const member of next.members) {
    if (Object.hasOwn(member, "relational_instance")) instances.validateMember(member, state);
    if (Object.hasOwn(member, "derivation")) recursiveGenerator.validateMember(member, state, loadContract());
    // Readiness can change; an occurrence's question and derivation cannot.
    // A changed question/path needs a new occurrence, retaining the old ancestry.
    const identity = JSON.stringify([
      member.question_form, member.rendering, member.prompt, member.source_lines,
      member.generator_ids, member.context, member.path, member.dependencies,
      ...(Object.hasOwn(member, "relational_instance") ? [instances.canonical(member.relational_instance)] : []),
      ...(Object.hasOwn(member, "derivation") ? [recursiveGenerator.canonical(member.derivation)] : []),
    ]);
    const priorIdentity = state.questionIdentities.get(member.occurrence);
    if (priorIdentity !== undefined && priorIdentity !== identity) {
      fail(`line ${record.seq} changes question identity or path for occurrence ${member.occurrence}`);
    }
    state.questionIdentities.set(member.occurrence, identity);
    state.questions.set(member.occurrence, member);
  }
  const representedSeeds = new Set([...state.questions.values()].map((member) => member.relational_instance?.seed_product));
  const representedGeneratorSurfaces = new Set([...state.questions.values()].map((member) => member.derivation?.surface_product));
  for (const product of state.products.values()) {
    if (product.inquiry_seed && !representedSeeds.has(product.id)) {
      fail(`line ${record.seq} fails to materialize reified inquiry seed ${product.id}`);
    }
    if (product.inquiry_generator_surface && !representedGeneratorSurfaces.has(product.id)) {
      fail(`line ${record.seq} fails to materialize reified inquiry generator surface ${product.id}`);
    }
  }
  if (state.requiredRestore.size > 0) {
    for (const occurrence of state.requiredRestore) {
      if (!nextByOccurrence.has(occurrence)) {
        fail(`line ${record.seq} fails to restore reopened question occurrence ${occurrence}`);
      }
    }
    state.requiredRestore.clear();
  }
  for (const fold of state.folds.values()) {
    if (fold.state !== "folded") continue;
    for (const occurrence of fold.members) {
      if (occurrence !== fold.representative && nextByOccurrence.has(occurrence)) {
        fail(`line ${record.seq} restores folded occurrence ${occurrence} without a reopen event`);
      }
    }
  }
  if (previous === null) return;
  for (const member of previous.members) {
    if (nextByOccurrence.has(member.occurrence)) continue;
    const disposition = next.dispositions[member.occurrence];
    const evidence = next.removalEvidence[member.occurrence];
    const folded = [...state.folds.values()].some((fold) =>
      fold.state === "folded" && fold.representative !== member.occurrence &&
      nextByOccurrence.has(fold.representative) && fold.members.includes(member.occurrence));
    if (folded) continue;
    if (!disposition || !evidence) {
      fail(`line ${record.seq} silently removes live question ${member.occurrence}`);
    }
    if (new Set(["Unknown", "Partial", "Productive", "Required", "Blocked", "ResourceBounded"]).has(disposition)) {
      fail(`line ${record.seq} removes unresolved question ${member.occurrence} as ${disposition}`);
    }
    if (disposition === "Redundant") {
      fail(`line ${record.seq} destructively deduplicates ${member.occurrence} without an evidenced fold`);
    }
    if (disposition !== "Answered") {
      fail(`line ${record.seq} has unsupported retirement disposition ${disposition}; retain the occurrence`);
    }
    const answer = state.answers.get(evidence);
    if (!answer || answer.ask.occurrence !== member.occurrence) {
      fail(`line ${record.seq} retirement evidence is not a matching Answer for ${member.occurrence}`);
    }
    if (!new Set(["Supported", "Plural", "ExactEmpty"]).has(answer.resolution)) {
      fail(`line ${record.seq} retires unresolved ${answer.resolution} Answer as complete`);
    }
  }
}

function validateSpineStateMachine(records) {
  const state = {
    field: null,
    ask: null,
    actual: null,
    awaitingReify: null,
    reifiedAnswer: null,
    dirty: false,
    products: new Map(),
    productOrigins: new Map(),
    raws: [],
    checks: [],
    foldEvidenceSchema: "2",
    questionIdentities: new Map(),
    questions: new Map(),
    fieldIds: new Set(),
    askOccurrences: new Set(),
    answers: new Map(),
    invalidated: new Set(),
    folds: new Map(),
    requiredRestore: new Set(),
    fieldRefresh: false,
    control: null,
    lastCheckpoint: 0,
    lastCheckpointResume: 0,
    lastClosure: 0,
    closureOutcome: null,
    lastStop: 0,
  };
  const answerRanks = new Map([["provisional", 0], ["supported", 1], ["checked", 2], ["warranted", 3]]);
  const unresolved = () => Boolean(state.ask || state.awaitingReify || state.actual || state.dirty ||
    state.fieldRefresh || state.requiredRestore.size);
  for (const record of records) {
    // A closure certifies one state, not every later state in this trace.
    if (new Set(["field", "ask", "seal", "raw", "interpret", "check", "answer", "reify",
      "invalidate", "fold", "reopen", "route"]).has(record.kind)) state.lastClosure = 0;
    switch (record.kind) {
      case "policy":
        if (record.fold_evidence_schema !== "2") fail(`line ${record.seq} must use fold evidence schema 2`);
        break;
      case "control":
        for (const key of ["authority", "residual", "predecessor", "scope"]) requireRecordString(record, key);
        state.control = record;
        break;
      case "field": {
        if ([...state.folds.values()].some((fold) => fold.state === "folded" && fold.reopen_required)) {
          fail(`line ${record.seq} must reopen folds whose evidence or protected continuation coverage changed`);
        }
        if (state.fieldIds.has(record.field_id)) fail(`line ${record.seq} reuses field id ${record.field_id}`);
        const next = {
          id: record.field_id,
          regeneratedFrom: record.regenerated_from,
          verifiedAnswer: state.dirty && state.reifiedAnswer === record.regenerated_from ? state.reifiedAnswer : null,
          members: parseJsonArray(record, "members"),
          dispositions: parseJsonObject(record, "dispositions"),
          removalEvidence: parseJsonObject(record, "removal_evidence"),
        };
        if (state.ask || state.awaitingReify || state.actual) {
          fail(`line ${record.seq} regenerates a field while an Ask, Answer, or actual cycle is unresolved`);
        }
        if (state.dirty) {
          if (state.reifiedAnswer === null || record.regenerated_from !== state.reifiedAnswer) {
            fail(`line ${record.seq} regenerates a dirty surface without the consequential Answer reification`);
          }
        } else if (state.field === null && record.regenerated_from !== "bootstrap") {
          fail(`line ${record.seq} initial field must use regenerated_from=bootstrap`);
        }
        validateFieldTransition(state.field, next, state, record);
        state.fieldIds.add(next.id);
        state.field = next;
        state.dirty = false;
        state.fieldRefresh = false;
        state.reifiedAnswer = null;
        break;
      }
      case "ask": {
        for (const key of ["q", "mode", "occurrence", "field_id", "question_form", "rendering", "context", "path", "bindings", "horizon", "coverage", "authority", "evidence", "dependencies"]) {
          requireRecordString(record, key);
        }
        if (!new Set(["Pure", "Generate", "Probe", "Check", "Warrant"]).has(record.mode)) {
          fail(`line ${record.seq} has unknown Ask mode ${record.mode}`);
        }
        if (!state.field) fail(`line ${record.seq} asks without a current field`);
        if (state.askOccurrences.has(record.occurrence)) fail(`line ${record.seq} repeats Ask occurrence ${record.occurrence}`);
        if (unresolved()) {
          fail(`line ${record.seq} asks before the prior Answer is reified and the field regenerated`);
        }
        if (record.field_id !== state.field.id) fail(`line ${record.seq} asks from a stale field`);
        const member = state.field.members.find((candidate) => candidate.occurrence === record.occurrence);
        if (!member) fail(`line ${record.seq} Ask occurrence is not represented in the live field`);
        if (!member.executable) fail(`line ${record.seq} selects a non-executable question occurrence`);
        if (member.question_form !== record.question_form || member.rendering !== record.rendering ||
            member.prompt !== record.q || member.context !== record.context || member.path !== record.path) {
          fail(`line ${record.seq} Ask identity does not match its represented field occurrence`);
        }
        if (member.relational_instance) {
          instances.validateMember(member, state);
          const expected = {
            bindings: instances.canonical(member.relational_instance.bindings),
            horizon: member.relational_instance.horizon,
            coverage: member.relational_instance.coverage,
            dependencies: member.dependencies.join(",") || "none",
          };
          for (const [key, value] of Object.entries(expected)) {
            if (record[key] !== value) fail(`line ${record.seq} instance Ask ${key} differs from its represented field`);
          }
        }
        state.ask = { occurrence: record.occurrence, mode: record.mode, member, seq: record.seq };
        state.askOccurrences.add(record.occurrence);
        break;
      }
      case "seal":
        for (const key of ["ask_occurrence", "should_change", "invariants", "discriminator", "wrong_impl", "coverage"]) requireRecordString(record, key);
        if (!state.ask) fail(`line ${record.seq} seals without a prior Ask`);
        if (state.ask.mode !== "Probe") fail(`line ${record.seq} seals a non-Probe Ask`);
        if (record.ask_occurrence !== state.ask.occurrence) fail(`line ${record.seq} seal targets another Ask`);
        if (state.actual) fail(`line ${record.seq} opens a second actual cycle`);
        state.actual = { ask: state.ask.occurrence, raw: 0, rawDigests: new Set(), interpret: 0, check: 0 };
        break;
      case "raw":
        for (const key of ["ask_occurrence", "cmd", "digest", "raw_ref", "sensitive"]) requireRecordString(record, key);
        if (!state.ask || state.ask.mode !== "Probe" || !state.actual ||
            record.ask_occurrence !== state.ask.occurrence) {
          fail(`line ${record.seq} records Actual Raw without the matching sealed Probe Ask`);
        }
        state.actual.raw += 1;
        state.actual.rawDigests.add(record.digest);
        state.raws.push(record);
        break;
      case "interpret":
        if (!state.actual || state.actual.raw === 0 || record.ask_occurrence !== state.actual.ask) {
          fail(`line ${record.seq} interprets without the matching immutable Raw return`);
        }
        requireRecordString(record, "raw_digest");
        requireRecordString(record, "interpretation");
        requireRecordString(record, "provenance");
        if (!state.actual.rawDigests.has(record.raw_digest)) {
          fail(`line ${record.seq} Interpretation does not name a Raw digest from its Ask`);
        }
        state.actual.interpret += 1;
        break;
      case "check":
        for (const key of ["ask_occurrence", "verdict", "coverage", "evidence"]) requireRecordString(record, key);
        if (!state.ask || record.ask_occurrence !== state.ask.occurrence) {
          fail(`line ${record.seq} Check has no matching Ask occurrence`);
        }
        if (state.ask.mode === "Probe") {
          if (!state.actual || state.actual.raw === 0 || state.actual.interpret === 0) {
            fail(`line ${record.seq} checks a Probe before Raw and Interpretation`);
          }
          state.actual.check += 1;
        } else if (state.ask.mode !== "Check") {
          fail(`line ${record.seq} Check is inapplicable to Ask mode ${state.ask.mode}`);
        }
        state.checks.push(record);
        break;
      case "answer": {
        for (const key of ["occurrence", "ask_occurrence", "answer", "resolution_class", "status", "polarity", "residual", "evidence", "coverage", "authority"]) requireRecordString(record, key);
        if (!state.ask || record.ask_occurrence !== state.ask.occurrence) {
          fail(`line ${record.seq} Answer has no matching Ask occurrence`);
        }
        if (state.ask.mode === "Probe" && (!state.actual || state.actual.raw === 0 || state.actual.interpret === 0 || state.actual.check === 0)) {
          fail(`line ${record.seq} resolves an effectful Ask before Raw, Interpretation, and Check`);
        }
        if (state.ask.mode === "Check" && !records.some((prior) =>
          prior.seq > state.ask.seq && prior.seq < record.seq && prior.kind === "check" && prior.ask_occurrence === state.ask.occurrence)) {
          fail(`line ${record.seq} resolves a Check Ask without an independent Check record`);
        }
        if (!answerRanks.has(record.status)) fail(`line ${record.seq} has unknown answer status ${record.status}`);
        if (state.ask.mode === "Generate" && record.status !== "provisional") {
          fail(`line ${record.seq} Generate Answer must remain provisional, not acquire ${record.status} authority`);
        }
        if (state.answers.has(record.occurrence) || state.questionIdentities.has(record.occurrence)) {
          fail(`line ${record.seq} reuses Answer occurrence ${record.occurrence}`);
        }
        if (!new Set(["Supported", "Partial", "Plural", "ExactEmpty", "Unsupported", "Unknown", "Blocked", "ResourceBounded"]).has(record.resolution_class)) {
          fail(`line ${record.seq} has unknown resolution class ${record.resolution_class}`);
        }
        if (!new Set(["Positive", "Negative", "Mixed", "None"]).has(record.polarity)) {
          fail(`line ${record.seq} has unknown answer polarity ${record.polarity}`);
        }
        if (record.resolution_class === "Unknown" && record.polarity !== "None") {
          fail(`line ${record.seq} collapses Unknown into a polarity`);
        }
        if (record.resolution_class === "Partial" && (!record.residual || record.residual === "none")) {
          fail(`line ${record.seq} treats Partial as complete without an explicit residual`);
        }
        state.awaitingReify = { occurrence: record.occurrence, ask: state.ask, status: record.status, resolution: record.resolution_class };
        state.awaitingReify.seq = record.seq;
        state.awaitingReify.record = record;
        state.answers.set(record.occurrence, state.awaitingReify);
        state.ask = null;
        state.actual = null;
        state.dirty = true;
        break;
      }
      case "reify": {
        for (const key of ["answer_occurrence", "status", "products", "new_questions", "coverage"]) requireRecordString(record, key);
        if (!state.awaitingReify || record.answer_occurrence !== state.awaitingReify.occurrence) {
          fail(`line ${record.seq} reifies without the matching consequential Answer`);
        }
        if (!answerRanks.has(record.status) || answerRanks.get(record.status) > answerRanks.get(state.awaitingReify.status)) {
          fail(`line ${record.seq} reification upgrades answer authority`);
        }
        const products = parseJsonArray(record, "products");
        for (const product of products) {
          for (const key of ["id", "kind", "status", "provenance", "coverage", "applicability", "horizon"]) {
            if (typeof product[key] !== "string" || product[key].length === 0) fail(`line ${record.seq} reified product requires ${key}`);
          }
          if (!Array.isArray(product.dependencies)) fail(`line ${record.seq} reified product requires dependencies`);
          if (product.status === "Standing" || !answerRanks.has(product.status) ||
              answerRanks.get(product.status) > answerRanks.get(state.awaitingReify.status)) {
            fail(`line ${record.seq} promotes a reified product beyond its Answer authority`);
          }
          if (state.awaitingReify.ask.mode === "Generate" && product.kind === "ActualEvent") {
            fail(`line ${record.seq} fabricates an ActualEvent from Generate`);
          }
          if (state.products.has(product.id)) fail(`line ${record.seq} reuses product id ${product.id}`);
          instances.validateProduct(product, state);
          recursiveGenerator.validateProduct(product, state, loadContract());
          state.products.set(product.id, product);
          state.productOrigins.set(product.id, state.awaitingReify.occurrence);
          if (product.inquiry_protection) foldEvidence.protection(product, state, repositoryRoot);
          if (product.fold_evidence) foldEvidence.certificate(product, state, repositoryRoot);
        }
        foldEvidence.refresh(state);
        state.reifiedAnswer = state.awaitingReify.occurrence;
        state.awaitingReify = null;
        break;
      }
      case "invalidate": {
        if (state.ask || state.awaitingReify || state.actual || state.dirty) fail(`line ${record.seq} invalidates before resolving the current lifecycle`);
        const ids = parseJsonArray(record, "product_ids");
        requireRecordString(record, "cause");
        requireRecordString(record, "evidence");
        for (const id of ids) {
          if (!state.products.has(id)) fail(`line ${record.seq} invalidates unknown product ${id}`);
          state.invalidated.add(id);
        }
        for (const product of state.products.values()) {
          if ((product.dependencies ?? []).some((dependency) => ids.includes(dependency)) &&
              !ids.includes(product.id) && !state.invalidated.has(product.id)) {
            fail(`line ${record.seq} leaves dependent product ${product.id} standing after invalidating its support`);
          }
        }
        state.fieldRefresh = true;
        foldEvidence.refresh(state);
        break;
      }
      case "fold": {
        const members = parseJsonArray(record, "members");
        if (unresolved()) fail(`line ${record.seq} folds an unresolved lifecycle`);
        if (!state.field || members.length < 2 || new Set(members).size !== members.length || members.some((id) => !state.field.members.some((member) => member.occurrence === id))) {
          fail(`line ${record.seq} fold members are not distinct live question occurrences`);
        }
        for (const key of ["fold_id", "representative", "protected_equivalence_evidence", "regeneration", "reopen_condition", "horizon", "coverage"]) {
          requireRecordString(record, key);
        }
        requireIndependentEvidence(record, "protected_equivalence_evidence");
        requireIndependentEvidence(record, "regeneration");
        if (!members.includes(record.representative)) fail(`line ${record.seq} fold representative is not a member`);
        if (state.folds.has(record.fold_id)) fail(`line ${record.seq} repeats fold id ${record.fold_id}`);
        const admission = foldEvidence.admitFold(record, state, repositoryRoot);
        state.folds.set(record.fold_id, { members, representative: record.representative, state: "folded", ...admission });
        state.fieldRefresh = true;
        break;
      }
      case "reopen": {
        if (state.ask || state.awaitingReify || state.actual) {
          fail(`line ${record.seq} reopens before the prior lifecycle is reified/regenerated`);
        }
        for (const key of ["fold_id", "restored_members", "discriminator", "evidence"]) requireRecordString(record, key);
        const fold = state.folds.get(record.fold_id);
        if (!fold || fold.state !== "folded") fail(`line ${record.seq} reopens no active fold`);
        const restored = parseJsonArray(record, "restored_members");
        if (restored.some((id) => !fold.members.includes(id))) fail(`line ${record.seq} restores a nonmember of the fold`);
        const omitted = fold.members.filter((id) => !state.field.members.some((member) => member.occurrence === id));
        if (restored.length === 0 || new Set(restored).size !== restored.length || omitted.some((id) => !restored.includes(id))) {
          fail(`line ${record.seq} reopening must restore every omitted fold member`);
        }
        requireRecordString(record, "discriminator");
        requireRecordString(record, "evidence");
        fold.state = "reopened";
        fold.reopen_required = false;
        for (const id of restored) state.requiredRestore.add(id);
        state.fieldRefresh = true;
        break;
      }
      case "checkpoint":
        for (const key of ["field_id", "established", "remains_open", "fold_changes", "reopen_changes", "coverage"]) requireRecordString(record, key);
        if (!state.field || unresolved()) {
          fail(`line ${record.seq} checkpoints an unresolved lifecycle`);
        }
        if (record.field_id !== state.field.id) fail(`line ${record.seq} checkpoints a stale field`);
        state.lastCheckpoint = record.seq;
        break;
      case "note":
        if (record.event === "checkpoint_resume") {
          for (const key of ["field_id", "checkpoint", "fuel_grant", "authority", "reason", "remaining_open", "text"]) {
            requireRecordString(record, key);
          }
          if (!state.field || unresolved()) fail(`line ${record.seq} resumes an unresolved lifecycle`);
          if (record.field_id !== state.field.id) fail(`line ${record.seq} resumes a stale field`);
          if (state.lastCheckpoint === 0 || Number(record.checkpoint) !== state.lastCheckpoint) {
            fail(`line ${record.seq} does not name the latest checkpoint`);
          }
          if (state.lastCheckpointResume >= state.lastCheckpoint) {
            fail(`line ${record.seq} repeats checkpoint fuel renewal without a newer checkpoint`);
          }
          if (!state.control || !/user|human/iu.test(state.control.authority) ||
              !state.control.scope.toLowerCase().split(/[,;\s]+/u).includes("harness") ||
              !/user|human/iu.test(record.authority)) {
            fail(`line ${record.seq} checkpoint continuation requires current user-authorized harness control`);
          }
          if (!state.field.members.some((member) => member.executable &&
              new Set(["Productive", "Required"]).has(member.disposition))) {
            fail(`line ${record.seq} checkpoint continuation has no live productive executable question`);
          }
          if (record.fuel_grant !== "24") fail(`line ${record.seq} checkpoint continuation must grant one canonical 24-Ask ratchet`);
          state.lastCheckpointResume = record.seq;
        }
        break;
      case "residual":
        if (!state.field || unresolved()) {
          fail(`line ${record.seq} records a residual before field regeneration`);
        }
        state.control = null;
        break;
      case "route":
        for (const key of ["source_occurrence", "answer", "successor_occurrence", "provenance"]) requireRecordString(record, key);
        if (record.order_exchange === "true") requireIndependentEvidence(record, "effect_proof");
        break;
      case "closure": {
        if (!state.field || unresolved()) {
          fail(`line ${record.seq} closes an unresolved lifecycle`);
        }
        for (const key of ["field_id", "scope", "warrant", "adversarial_question", "adversarial_answer", "coverage"]) {
          requireRecordString(record, key);
        }
        if (record.field_id !== state.field.id) fail(`line ${record.seq} closes a stale field`);
        if (state.field.members.some((member) => member.executable && new Set(["Productive", "Required"]).has(member.disposition))) {
          fail(`line ${record.seq} treats a live productive executable question as task closure`);
        }
        const challenge = state.answers.get(record.adversarial_answer);
        const partialOutcome = new Set(["Unknown", "Blocked", "ResourceBounded"]).has(record.state);
        const completeChallenge = challenge && new Set(["Probe", "Check"]).has(challenge.ask.mode) &&
          answerRanks.get(challenge.status) >= 2 && new Set(["Supported", "Plural", "ExactEmpty"]).has(challenge.resolution);
        const partialChallenge = challenge && partialOutcome &&
          (challenge.resolution === record.state || (record.state === "ResourceBounded" && challenge.resolution === "Partial"));
        if (!challenge || challenge.ask.occurrence !== record.adversarial_question ||
            (!completeChallenge && !partialChallenge) || state.field.verifiedAnswer !== challenge.occurrence) {
          fail(`line ${record.seq} closure requires the matching checked adversarial Answer (or explicitly retained partial outcome) and its regenerated field`);
        }
        state.lastClosure = record.seq;
        state.closureOutcome = record.state ?? null;
        break;
      }
      case "stop":
        if (unresolved()) fail(`line ${record.seq} Stop has an unresolved lifecycle`);
        if (state.lastClosure === 0 || state.lastClosure <= state.lastStop) {
          fail(`line ${record.seq} Stop requires a new task-level closure, not a checkpoint`);
        }
        requireRecordString(record, "warrant");
        if (!new Set(["Satisfied", "Equivalent", "Impossible", "Blocked", "Unknown", "ResourceBounded"]).has(record.state)) {
          fail(`line ${record.seq} Stop has invalid state ${record.state}`);
        }
        if (state.closureOutcome !== null && state.closureOutcome !== record.state) {
          fail(`line ${record.seq} Stop cannot upgrade or change the declared closure outcome`);
        }
        if (record.state === "Satisfied") {
          requireIndependentEvidence(record, "warrant");
          if (/^agent(?:$|[:/_-])/iu.test(record.warrant.replace(/\s+/gu, ""))) fail(`line ${record.seq} Satisfied stop has a self warrant`);
          if (state.field.members.length !== 0) fail(`line ${record.seq} Satisfied cannot erase unresolved field members`);
        }
        state.lastStop = record.seq;
        state.control = null;
        break;
      case "question":
        fail(`line ${record.seq} uses retired combined question/answer instead of the inquiry spine`);
      default:
        break;
    }
  }
  return { open: unresolved(), mutation_open: Boolean(state.actual),
    stop_pending: state.lastClosure === 0 || state.lastStop < state.lastClosure,
    can_initialize: !unresolved() && state.lastClosure > 0 && state.lastStop > state.lastClosure,
    field_id: state.field?.id ?? null, unresolved_ask: state.ask?.occurrence ?? null,
    answer_awaiting_reification: state.awaitingReify?.occurrence ?? null,
    surface_dirty: state.dirty || state.fieldRefresh || state.requiredRestore.size > 0,
    restore_required: [...state.requiredRestore], last_checkpoint: state.lastCheckpoint,
    last_checkpoint_resume: state.lastCheckpointResume, last_closure: state.lastClosure, last_stop: state.lastStop,
    control: state.control, fold_evidence_schema: state.foldEvidenceSchema,
    folds: [...state.folds].map(([id, fold]) => ({ fold_id: id, ...fold })) };
}

function validateStateMachine(records) {
  const initialPolicy = records.find((record) => record.kind === "policy");
  if (initialPolicy?.question_program_schema !== "5") {
    fail("only the consolidated schema-5 inquiry spine is live; use the offline Git ancestry for historical traces");
  }
  return validateSpineStateMachine(records);
}

function consumeQuestionFuel() {
  if (!fuelPath) {
    fail("question append requires a fuel file");
  }
  let text;
  try {
    text = fs.readFileSync(path.resolve(fuelPath), "utf8").trim();
  } catch (error) {
    fail(`cannot read question fuel: ${error.message}`);
  }
  if (!/^[0-9]+$/.test(text)) {
    fail("question fuel is not a nonnegative integer");
  }
  const remaining = Number(text);
  if (!Number.isSafeInteger(remaining) || remaining <= 0) {
    fail("fuel exhausted; stop ResourceBounded with partial result and frontier");
  }
  const fd = fs.openSync(path.resolve(fuelPath), "w", 0o600);
  try {
    fs.writeFileSync(fd, String(remaining - 1));
    fs.fsyncSync(fd);
  } finally {
    fs.closeSync(fd);
  }
}

function renewCheckpointFuel(grant) {
  if (!fuelPath) fail("checkpoint continuation requires a fuel file");
  let text;
  try {
    text = fs.readFileSync(path.resolve(fuelPath), "utf8").trim();
  } catch (error) {
    fail(`cannot read checkpoint fuel: ${error.message}`);
  }
  if (text !== "0") fail("checkpoint continuation requires exactly exhausted fuel");
  if (grant !== "24") fail("checkpoint continuation grant must be exactly 24");
  const fd = fs.openSync(path.resolve(fuelPath), "w", 0o600);
  try {
    fs.writeFileSync(fd, grant);
    fs.fsyncSync(fd);
  } finally {
    fs.closeSync(fd);
  }
}

let lockFd;
try {
  lockFd = acquireLock();
  const records = validatedRecords();
  if (operation === "state") {
    const policy = records.find((record) => record.kind === "policy");
    process.stdout.write(`${JSON.stringify({ schema: policy?.question_program_schema ?? null,
      record_count: records.length, trace_sha256: validatedTraceDigest, ...validateStateMachine(records) })}\n`);
  }
  if (operation === "append") {
    const input = readStdin();
    if (!input.endsWith("\n") || input.slice(0, -1).includes("\n")) {
      fail("append input must be exactly one newline-terminated JSON record");
    }
    let record;
    try {
      record = JSON.parse(input);
    } catch (error) {
      fail(`append input is not valid JSON: ${error.message}`);
    }
    if (
      record === null ||
      Array.isArray(record) ||
      typeof record !== "object" ||
      Object.hasOwn(record, "seq") ||
      Object.hasOwn(record, "parent") ||
      typeof record.kind !== "string" ||
      record.kind.length === 0
    ) {
      fail("append input must be an object with no seq/parent and a nonempty kind");
    }
    if (record.kind === "question" || record.kind === "policy_transition") {
      fail("retired controller record kinds cannot enter a schema-5 trace");
    }
    if (record.kind === "field") {
      record.field_check = validateFieldRecord(record, path.resolve(__dirname, "../.."));
    }
    if (record.kind === "reify") {
      const policy = records.find((prior) => prior.kind === "policy");
      validateReifiedSeeds(record, path.resolve(__dirname, "../.."), policy);
    }
    if (record.kind === "ask") {
      if (typeof record.fp !== "string" || record.fp.length === 0) {
        fail("Ask record requires nonempty fp");
      }
      record.question_program_check = validateQuestionProgram(
        record,
        path.resolve(__dirname, "../.."),
      );
      const policy = [...records]
        .reverse()
        .find((prior) => prior.kind === "policy");
      if (policy) validateStoredQuestion(record, policy);
      if (records.some((prior) => prior.kind === "ask" && prior.fp === record.fp)) {
        fail("repeated state: same Ask occurrence and relational coordinates");
      }
    }
    if (record.kind === "policy") {
      const loaded = contractLoader.read(repositoryRoot);
      if (loaded.contractDigest !== record.program_manifest_digest) fail("new policy must pin the current inquiry-spine contract");
      const version = String(loaded.contract.fold?.evidence_schema ?? 0);
      if (record.fold_evidence_schema !== undefined && record.fold_evidence_schema !== version) fail("fold evidence policy differs from the contract");
      record.fold_evidence_schema = version;
    }
    if (record.kind === "policy") {
      if (records.length !== 0) fail("question-program policy must be the first record");
      validatePolicy(record);
    }
    const expectedSeq = records.length + 1;
    const stored = {
      seq: expectedSeq,
      parent: expectedSeq - 1,
      ...record,
    };
    validateStateMachine([...records, stored]);
    if (record.kind === "ask") {
      consumeQuestionFuel();
    }
    if (record.kind === "note" && record.event === "checkpoint_resume") {
      renewCheckpointFuel(record.fuel_grant);
    }
    const storedInput = `${JSON.stringify(stored)}\n`;
    const fd = fs.openSync(absolute, "a", 0o600);
    try {
      const buffer = Buffer.from(storedInput, "utf8");
      const written = fs.writeSync(fd, buffer, 0, buffer.length);
      if (written !== buffer.length) {
        fail(`short trace write: ${written} of ${buffer.length} bytes`);
      }
      fs.fsyncSync(fd);
    } finally {
      fs.closeSync(fd);
    }
    process.stdout.write(`${expectedSeq}\n`);
  }
} catch (error) {
  process.stderr.write(`ic-append: ${error.message}\n`);
  process.exitCode = 1;
} finally {
  if (lockFd !== undefined) {
    releaseLock(lockFd);
  }
}
