#!/usr/bin/env node
"use strict";

// Breaker tests for the autonomous construction field.
//
// The governing invariant under test is:
//
//   THE FRONTIER IS AN OUTPUT OF THE CONSTRUCTION FIELD.
//
// Each test mutates a real copy of repository state and requires the derived field
// to behave correctly. These are mutation breakers, not fixtures: the copied root
// carries the actual protected capabilities, theorem seeds, and Lean declarations.

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const cp = require("node:child_process");

const root = path.resolve(__dirname, "..");
const hooks = path.join(root, ".claude", "hooks");
const constructionSurface = require(path.join(hooks, "ic-construction-surface.js"));
const obligationIndex = require(path.join(hooks, "ic-obligation-index.js"));
const frontierGenerate = require(path.join(hooks, "ic-frontier-generate.js"));
const spine = require(path.join(hooks, "ic-spine.js"));

// Everything the derived field reads. Copying exactly this set also proves the
// field needs no trace, cache, or frontier document to reconstruct itself.
const FIELD_INPUTS = [
  "formal-successor/REGENERATIVE_SPINE.json",
  "formal-successor/INTEGRATED_THEOREM_OBLIGATIONS.json",
  "formal-successor/RESIDUAL_OBLIGATIONS.json",
  "formal-successor/INQUIRY_SPINE_CONTRACT.json",
  "formal-successor/Questions.txt",
  "formal-successor/NORMALIZATION_CONTINUITY.json",
  "formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md",
  "formal-successor/BACKEND_PROFILES.json",
  "formal-successor/DECISIONS.jsonl",
];
const FIELD_TREES = [
  "formal/InquiryCalculus/Successor",
  "formal/InquiryCalculus/Legacy/V20",
];

let temporaryRoots = [];

function copyTree(from, to) {
  fs.mkdirSync(to, { recursive: true });
  for (const entry of fs.readdirSync(from, { withFileTypes: true })) {
    const source = path.join(from, entry.name);
    const target = path.join(to, entry.name);
    if (entry.isDirectory()) copyTree(source, target);
    else fs.copyFileSync(source, target);
  }
}

function scratchRoot() {
  const base = fs.mkdtempSync(path.join(os.tmpdir(), "ic-construction-field-"));
  temporaryRoots.push(base);
  for (const relative of FIELD_INPUTS) {
    const target = path.join(base, ...relative.split("/"));
    fs.mkdirSync(path.dirname(target), { recursive: true });
    fs.copyFileSync(path.join(root, ...relative.split("/")), target);
  }
  for (const relative of FIELD_TREES) {
    const from = path.join(root, ...relative.split("/"));
    if (fs.existsSync(from)) copyTree(from, path.join(base, ...relative.split("/")));
  }
  const git = (...args) => cp.execFileSync("git", ["-C", base, ...args], { encoding: "utf8", windowsHide: true, stdio: ["ignore", "pipe", "pipe"] });
  const ledgerPath = path.join(base, "formal-successor/DECISIONS.jsonl");
  const workingLedger = fs.readFileSync(ledgerPath, "utf8");
  const acceptedLedger = cp.execFileSync("git", ["-C", root, "show", "HEAD:formal-successor/DECISIONS.jsonl"],
    { encoding: "utf8", windowsHide: true });
  // Cold reconstruction preserves the admitted epoch, not pending candidate receipts.
  fs.writeFileSync(ledgerPath, acceptedLedger);
  for (const record of acceptedLedger.split(/\r?\n/u).filter(Boolean).map(JSON.parse)) {
    const driver = record.construction_return?.check?.driver;
    if (driver && /^tools\/[a-z0-9_]+\.(?:js|py)$/u.test(driver) && fs.existsSync(path.join(root, driver))) {
      fs.mkdirSync(path.join(base, "tools"), { recursive: true });
      fs.copyFileSync(path.join(root, driver), path.join(base, driver));
    }
  }
  git("init", "--quiet");
  git("add", "formal-successor/REGENERATIVE_SPINE.json", "formal-successor/DECISIONS.jsonl");
  git("-c", "user.name=ConstructionFixture", "-c", "user.email=fixture@example.invalid", "-c", "commit.gpgsign=false", "commit", "--quiet", "-m", "Accepted fixture epoch");
  fs.writeFileSync(ledgerPath, workingLedger);
  return base;
}

function readSpine(base) {
  return JSON.parse(fs.readFileSync(path.join(base, "formal-successor/REGENERATIVE_SPINE.json"), "utf8"));
}

function writeSpine(base, value) {
  fs.writeFileSync(path.join(base, "formal-successor/REGENERATIVE_SPINE.json"), `${JSON.stringify(value, null, 2)}\n`);
}

// Resolve correspondence records by stable obligation id. Addressing them by array
// position couples the breakers to the current shape of remaining_correspondence, so a
// structural refinement could silently disarm a breaker instead of failing it.
function correspondenceRecords(value) {
  return (value.protected_predecessor_capabilities ?? [])
    .flatMap((capability) => (capability.remaining_correspondence ?? [])
      .filter((entry) => entry && typeof entry === "object"));
}

function correspondenceById(value, id) {
  const record = correspondenceRecords(value).find((entry) => entry.id === id);
  assert.notEqual(record, undefined, `correspondence record ${id} is not represented`);
  return record;
}

function cleanup() {
  for (const base of temporaryRoots) {
    assert.equal(path.dirname(path.resolve(base)), path.resolve(os.tmpdir()));
    assert.ok(path.basename(base).startsWith("ic-construction-field-"));
    fs.rmSync(base, { recursive: true, force: true });
  }
  temporaryRoots = [];
}

const results = [];
function test(name, body) {
  body();
  results.push(name);
}

try {
  // -------------------------------------------------------------------------
  // 1. Frontier inversion: hand-editing the projection changes no live obligation.
  // -------------------------------------------------------------------------
  test("frontier inversion", () => {
    const base = scratchRoot();
    const before = obligationIndex.build(base);
    fs.writeFileSync(path.join(base, "IMPLEMENTATION_FRONTIER.md"), frontierGenerate.render(base));

    const tampered = frontierGenerate.render(base)
      .replace("FORMAL-C-REGULAR-RELATION-REPRESENTABILITY", "FORMAL-Z-OPERATOR-INVENTED-BY-HAND")
      .replace(/^goal: .*$/mu, "goal: a hand-written goal that no structure supports");
    fs.writeFileSync(path.join(base, "IMPLEMENTATION_FRONTIER.md"), tampered);

    const after = obligationIndex.build(base);
    assert.equal(after.selected.id, before.selected.id,
      "editing the frontier changed the derived selection");
    assert.equal(after.selected.statement, before.selected.statement,
      "editing the frontier changed the derived goal");
    assert.deepEqual(after.index.live, before.index.live,
      "editing the frontier changed the reference live field");

    // The drift itself must be detectable and rejected, not silently accepted.
    assert.equal(frontierGenerate.check(base).drifted, true,
      "hand-edited frontier drift was not detected");
  });

  // -------------------------------------------------------------------------
  // 2. Cold reconstruction: delete every rebuildable projection and regenerate.
  // -------------------------------------------------------------------------
  test("cold frontier reconstruction", () => {
    const base = scratchRoot();
    fs.writeFileSync(path.join(base, "IMPLEMENTATION_FRONTIER.md"), frontierGenerate.render(base));
    const before = obligationIndex.build(base);

    for (const projection of [
      "IMPLEMENTATION_FRONTIER.md",
      "formal-successor/PROTECTED_CONSTRUCTION_SURFACE.json",
      "formal-successor/CONSTRUCTION_OBLIGATION_INDEX.json",
    ]) {
      fs.rmSync(path.join(base, ...projection.split("/")), { force: true });
    }
    assert.equal(fs.existsSync(path.join(base, "IMPLEMENTATION_FRONTIER.md")), false);

    const after = obligationIndex.build(base);
    assert.equal(after.selected.id, before.selected.id,
      "cold reconstruction changed the selected obligation");
    assert.equal(after.digest, before.digest,
      "cold reconstruction changed the obligation field digest");
    fs.writeFileSync(path.join(base, "IMPLEMENTATION_FRONTIER.md"), frontierGenerate.render(base));
    assert.equal(frontierGenerate.check(base).drifted, false,
      "regenerated frontier does not match its own projection");
    const cold = spine.build(base);
    assert.ok(cold.question_packet, "cold checkout must produce a packet without trace state");
    assert.equal(cold.question_packet.obligation_identity, after.selected.id);
    assert.equal(cold.closure.admissible, false);
  });

  // -------------------------------------------------------------------------
  // 3. Real packet from actual repository state, not a synthetic fixture.
  // -------------------------------------------------------------------------
  test("real question packet", () => {
    const built = spine.build(root);
    const packet = built.question_packet;
    assert.notEqual(packet, null, "actual repository state produced no QuestionPacket");
    assert.equal(packet.obligation_identity, obligationIndex.build(root).selected.id);
    assert.ok(packet.live_goal.length > 0, "packet has no live goal");
    assert.ok(packet.decisive_breakers.length > 0, "packet has no decisive breaker");
    assert.ok(packet.open_roles.length > 0, "packet lost its typed open position");
    // Operational rendering must not be labelled canonical prose before Gate J.
    assert.equal(packet.canonical_prose, null,
      "operational rendering was labelled canonical prose");
    assert.match(packet.output_contract.authority, /candidate_only/u);
  });

  // -------------------------------------------------------------------------
  // 4. Protected-target deletion is detected.
  // -------------------------------------------------------------------------
  test("protected target deletion detected", () => {
    const base = scratchRoot();
    const before = obligationIndex.build(base);
    const beforeContinuity = before.index.obligations
      .filter((entry) => entry.kind === "continuity_disposition").map((entry) => entry.id);

    const value = readSpine(base);
    const removed = value.protected_predecessor_capabilities.pop();
    writeSpine(base, value);

    const after = obligationIndex.build(base);
    const afterContinuity = after.index.obligations
      .filter((entry) => entry.kind === "continuity_disposition").map((entry) => entry.id);

    assert.equal(beforeContinuity.length, afterContinuity.length,
      "deleting a target silently removed its continuity obligation");
    const lost = beforeContinuity.filter((id) => !afterContinuity.includes(id));
    assert.equal(lost.length, 0, "no protected continuity obligation may disappear");
    assert.equal(after.selected.id, `FORMAL-RESTORE-PCAP-${constructionSurface.slug(removed.id)}`);
  });

  // -------------------------------------------------------------------------
  // 5. A new consequential declaration generates obligations with no registry edit.
  // -------------------------------------------------------------------------
  test("new declaration generates obligations", () => {
    const base = scratchRoot();
    const before = obligationIndex.build(base);
    const registryBefore = fs.readFileSync(
      path.join(base, "formal-successor/INTEGRATED_THEOREM_OBLIGATIONS.json"), "utf8");

    const added = path.join(base, "formal/InquiryCalculus/Successor/Relational/ProbeAdded.lean");
    fs.writeFileSync(added, [
      "namespace InquiryCalculus.Successor.Relational",
      "",
      "structure ProbeAddedCarrier where",
      "  value : Nat",
      "",
      "theorem probeAddedClaim (c : ProbeAddedCarrier) : c.value = c.value := rfl",
      "",
      "end InquiryCalculus.Successor.Relational",
      "",
    ].join("\n"));

    const after = obligationIndex.build(base);
    const newIds = after.index.obligations.map((entry) => entry.id)
      .filter((id) => !before.index.obligations.some((entry) => entry.id === id));

    assert.ok(newIds.some((id) => id.startsWith("DECLTYP-")),
      "a new carrier declaration generated no typing obligation");
    assert.ok(newIds.some((id) => id.startsWith("DECLBRK-")),
      "a new claim declaration generated no breaker-coverage obligation");
    assert.ok(newIds.some((id) => id.startsWith("DECLPRE-")),
      "a new claim declaration generated no proof-presupposition obligation");
    assert.equal(
      fs.readFileSync(path.join(base, "formal-successor/INTEGRATED_THEOREM_OBLIGATIONS.json"), "utf8"),
      registryBefore,
      "obligations required a manual theorem-registry edit");
  });

  // -------------------------------------------------------------------------
  // 6 + 12. A suppressed reference-live obligation is a GeneratorGap, not closure.
  // -------------------------------------------------------------------------
  test("suppressed generation reports GeneratorGap", () => {
    const base = scratchRoot();
    const { index } = obligationIndex.build(base);

    // Full reach: every live obligation is covered by a matching occurrence.
    const fullReach = spine.constructionQuestions(base, obligationIndex.build(base), {},
      JSON.parse(fs.readFileSync(path.join(base, "formal-successor/INQUIRY_SPINE_CONTRACT.json"), "utf8")));
    const full = obligationIndex.coverage(base, fullReach);
    assert.equal(full.generator_adequate, true, "full reach was not adequate");
    assert.equal(full.generator_gaps.length, 0);

    // Suppress one occurrence: the obligation stays live and is reported as a gap.
    const suppressed = index.live[0];
    const partial = obligationIndex.coverage(base, fullReach.filter((entry) => entry.obligation_identity !== suppressed));
    assert.equal(partial.generator_adequate, false,
      "suppressing a live obligation still reported generator adequacy");
    assert.ok(partial.generator_gaps.some((gap) => gap.obligation === suppressed),
      "the suppressed obligation was not reported as a generator gap");
    assert.equal(partial.generator_gaps.every((gap) => gap.state === "GeneratorGap"), true);

    // No generated question is not no live obligation.
    const empty = obligationIndex.coverage(base, []);
    assert.equal(empty.generator_gaps.length, index.live.length,
      "an empty generator emptied the live field instead of reporting gaps");
    assert.ok(index.live.length > 0, "the reference live field collapsed with the generator");
    assert.equal(obligationIndex.coverage(base, index.live.map((id) => `QI-${id}`)).covered.length, 0,
      "matching names are not typed coverage witnesses");
  });

  // -------------------------------------------------------------------------
  // 7. Dependency mutation reopens and closes consequential descendants.
  // -------------------------------------------------------------------------
  test("dependency mutation propagates", () => {
    const base = scratchRoot();
    const before = obligationIndex.build(base);
    const dependent = before.index.obligations
      .find((entry) => entry.id === "FORMAL-C-TYPED-RELATIONAL-SURFACE-CLOSURE");
    assert.notEqual(dependent, undefined, "expected a dependent correspondence obligation");
    assert.equal(dependent.activation.active, false,
      "the dependent obligation was active before its dependency was discharged");

    // Claim file-reference discharge for every correspondence except the descendant.
    const value = readSpine(base);
    for (const record of correspondenceRecords(value)) {
      if (record.id === "FORMAL-C-TYPED-RELATIONAL-SURFACE-CLOSURE") continue;
      record.discharged_by = "formal/InquiryCalculus/Successor/Relational/TypedSurface.lean";
    }
    writeSpine(base, value);

    const after = obligationIndex.build(base);
    const activated = after.index.obligations
      .find((entry) => entry.id === "FORMAL-C-TYPED-RELATIONAL-SURFACE-CLOSURE");
    assert.equal(activated.activation.active, false,
      "unverified file references must not discharge a dependency");
    const provedIds = (built) => built.index.obligations.filter((entry) =>
      entry.kind === "protected_correspondence" && entry.disposition === "Proved").map((entry) => entry.id);
    assert.deepEqual(provedIds(after), provedIds(before),
      "file references cannot add a proof or erase an independently admitted proof");

    // Reverting one transitive dependency must reopen the descendant again.
    const reverted = readSpine(base);
    correspondenceById(reverted, "FORMAL-C-REGULAR-RELATION-REPRESENTABILITY").discharged_by = null;
    writeSpine(base, reverted);
    const reopened = obligationIndex.build(base).index.obligations
      .find((entry) => entry.id === "FORMAL-C-TYPED-RELATIONAL-SURFACE-CLOSURE");
    assert.equal(reopened.activation.active, false,
      "reverting an upstream discharge did not reopen its consequential descendant");
    const modulePath = path.join(base, "formal/InquiryCalculus/Successor/Relational/Representability.lean");
    const source = fs.readFileSync(modulePath, "utf8"), changed = source.replace("→ a = c", "→ a = a");
    assert.notEqual(changed, source);
    const original = obligationIndex.build(base);
    fs.writeFileSync(modulePath, changed);
    const modified = obligationIndex.build(base);
    assert.notEqual(original.digest, modified.digest, "same-name source edits were invisible");
    assert.notEqual(original.selected.evidence_fingerprint, modified.selected.evidence_fingerprint);
    assert.deepEqual(obligationIndex.impactClosure([{ id: "a", depends_on: [] }, { id: "b", depends_on: ["a"] }, { id: "c", depends_on: ["b"] }], ["a"]), ["a", "b", "c"]);
  });

  // -------------------------------------------------------------------------
  // 8. Horizon refinement reopens a previously settled position.
  // -------------------------------------------------------------------------
  test("horizon refinement reopens selection", () => {
    const base = scratchRoot();
    const before = obligationIndex.build(base);

    const value = readSpine(base);
    const capability = value.protected_predecessor_capabilities
      .find((entry) => entry.id === "typed-relational-surface");
    capability.remaining_correspondence.unshift({
      id: "FORMAL-C-REFINED-PROTECTED-DISCRIMINATOR",
      gate: "C",
      order: 0,
      depends_on: [],
      goal: "Discharge the newly admitted protected discriminator before the prior correspondence.",
      protected_difference: "A finer protected horizon can split a previously determined class.",
      discriminator: "Exhibit two members identified by the prior horizon and separated by the refinement.",
      horizon: "refinement only",
      if_pass: "re-derive the dependent correspondence under the refined horizon",
      if_fail: "retain the refinement as an unresolved protected discriminator",
      discharged_by: null,
    });
    writeSpine(base, value);

    const after = obligationIndex.build(base);
    assert.notEqual(after.selected.id, before.selected.id,
      "a newly admitted protected discriminator did not reopen the selected position");
    assert.equal(after.selected.id, "FORMAL-C-REFINED-PROTECTED-DISCRIMINATOR");
    assert.ok(after.index.live.includes(before.selected.id),
      "the previously selected obligation was dropped rather than retained");
  });

  // -------------------------------------------------------------------------
  // 11. A live obligation naming unrepresentable structure is a representation gap.
  // -------------------------------------------------------------------------
  test("representation failure is distinct from semantic absence", () => {
    const base = scratchRoot();
    const value = readSpine(base);
    correspondenceById(value, "FORMAL-C-TYPED-RELATIONAL-SURFACE-CLOSURE").depends_on =
      ["FORMAL-C-NOT-A-REPRESENTED-OBLIGATION"];
    writeSpine(base, value);

    const { index } = obligationIndex.build(base);
    const entry = index.obligations.find((item) => item.id === "FORMAL-C-TYPED-RELATIONAL-SURFACE-CLOSURE");
    assert.equal(entry.disposition, "OperationalUnknown",
      "an undeclared dependency is not proof that the question language cannot express it");
    assert.ok(index.live.includes(entry.id));
    // A representation gap is not a generator gap and not a closure.
    assert.notEqual(entry.disposition, "GeneratorGap");
    assert.equal(obligationIndex.TERMINAL.has(entry.disposition), false,
      "a representation gap was treated as terminal");
  });

  // -------------------------------------------------------------------------
  // 13. A candidate cannot participate in establishing its own acceptance.
  // -------------------------------------------------------------------------
  test("self-warrant attempt is rejected", () => {
    const base = scratchRoot();
    const target = "FORMAL-C-REGULAR-RELATION-REPRESENTABILITY";
    const value = readSpine(base);
    correspondenceById(value, target).discharged_by = target;
    writeSpine(base, value);
    assert.throws(() => obligationIndex.build(base), /self-warrant/u,
      "a correspondence discharged itself");

    for (const claimed of ["self", "none", "generated", "assumed", "candidate"]) {
      const attempt = readSpine(base);
      correspondenceById(attempt, target).discharged_by = claimed;
      writeSpine(base, attempt);
      assert.throws(() => obligationIndex.build(base), /self-warrant/u,
        `a discharge claiming ${claimed} was accepted`);
    }
  });

  // -------------------------------------------------------------------------
  // 14. A finite description is not a finite exact semantic closure certificate.
  // -------------------------------------------------------------------------
  test("finite presentation is not closure certification", () => {
    const { index } = obligationIndex.build(root);
    const built = spine.build(root);
    assert.equal(built.closure_certificate.established, false,
      "a finite index must not establish closure");
    assert.equal(built.closure_certificate.criteria.length, 17);
    assert.equal(spine.evaluateClosure({ field: {}, folds: [] }, [], [],
      { index, coverage: obligationIndex.coverage(root, []) }).admissible, false,
      "an empty executable trace must not hide live construction obligations");
    assert.ok(index.counts.live > 0,
      "a finite index summary was produced while claiming an empty live field");
    // Distinct nonterminal statuses must stay distinct in the vocabulary.
    for (const status of ["Blocked", "ResourceBounded", "RepresentationGap", "GeneratorGap"]) {
      assert.equal(obligationIndex.TERMINAL.has(status), false,
        `${status} was treated as a terminal disposition`);
    }
  });

  // -------------------------------------------------------------------------
  // 9 + 10. Typed nonterminal statuses stay separated from closure.
  // -------------------------------------------------------------------------
  test("nonterminal statuses are not closure", () => {
    const contract = JSON.parse(fs.readFileSync(
      path.join(root, "formal-successor/INQUIRY_SPINE_CONTRACT.json"), "utf8"));
    for (const status of ["Blocked", "ResourceBounded", "Unknown"]) {
      assert.ok(contract.lifecycle.resolution_classes.includes(status),
        `${status} is not a represented resolution class`);
      assert.equal(contract.lifecycle.terminal_dispositions.includes(status), false,
        `${status} was admitted as a terminal disposition`);
    }
    assert.ok(obligationIndex.TERMINAL.has("Proved"));
    assert.ok(obligationIndex.TERMINAL.has("Broken"));
    assert.ok(obligationIndex.TERMINAL.has("Inapplicable"));
    const built = obligationIndex.build(root), id = built.selected.id;
    const runtime = { actual_asks: [{ occurrence: "spent", obligation_identity: id }],
      actual_returns: [1, 2].map(() => ({ ask_occurrence: "spent", cmd: "node ic-local-attempt.js" })) };
    const question = spine.constructionQuestions(root, built, runtime, contract).find((entry) => entry.obligation_identity === id);
    assert.equal(question.backend, "frontier-review");
    assert.equal(question.resource_status, "LocalResourceBounded_FrontierAvailable");
    assert.equal(question.executable, true);
  });

  test("working candidate acceptance metadata is not an admitted return", () => {
    const base = scratchRoot();
    const file = path.join(base, "formal-successor/DECISIONS.jsonl");
    fs.appendFileSync(file, JSON.stringify({ id: "D-FAKE", status: "EXACT", construction_return: {
      authority: "frontier_review_of_independent_return", disposition: "Proved", obligation: "FORMAL-C-RELATION-CONTEXTUAL-TELESCOPE",
    } }) + "\n");
    const admitted = require(path.join(hooks, "ic-construction-evidence.js")).readReturns(base);
    assert.ok(!admitted.some((record) => record.decision === "D-FAKE"));
  });

  test("corpus and structural closure exceed manually registered theorem IDs", () => {
    const { index } = obligationIndex.build(root);
    for (const kind of ["source_corpus_disposition", "predecessor_continuity", "normalization_continuity", "relation_connectivity",
      "requirement_applicability", "deformation_applicability", "domain", "regeneration_after_removal"]) {
      assert.ok(index.obligations.some((entry) => entry.kind === kind), kind);
    }
  });

  test("sealed cold Ask returns, reifies and generates its next occurrence", () => {
    const base = scratchRoot();
    const seal = { should_change: "fixture return only", invariants: "no mathematics promoted",
      discriminator: "synthetic lifecycle trace closes exactly one occurrence", wrong_impl: "reuse the answered occurrence or empty the live field", coverage: "process fixture only" };
    const first = spine.begin(base, seal);
    const trace = path.join(base, ".claude/trace", fs.readFileSync(path.join(base, ".claude/trace/.state"), "utf8"));
    const append = (record) => {
      const result = cp.spawnSync(process.execPath, [path.join(hooks, "ic-append.js"), "append", trace, path.join(base, ".claude/trace/.fuel")],
        { encoding: "utf8", windowsHide: true, input: `${JSON.stringify(record)}\n`, maxBuffer: 32 * 1024 * 1024 });
      assert.equal(result.status, 0, result.stderr || result.stdout);
    };
    const hash = "a".repeat(64);
    append({ kind: "raw", ask_occurrence: first.occurrence, cmd: "synthetic lifecycle fixture", digest: hash, raw_ref: `.claude/trace/raw/${hash}`, sensitive: "false" });
    append({ kind: "interpret", ask_occurrence: first.occurrence, raw_digest: hash, interpretation: "fixture has no mathematical conclusion", provenance: "synthetic process test" });
    append({ kind: "check", ask_occurrence: first.occurrence, verdict: "PASS", evidence: "fixture ordering only", coverage: "process only" });
    append({ kind: "answer", ask_occurrence: first.occurrence, occurrence: "ANS-FIXTURE", answer: "no formal result", resolution_class: "Unsupported",
      status: "checked", polarity: "None", residual: first.obligation, evidence: "fixture ordering only", coverage: "process only", authority: "synthetic process test" });
    append({ kind: "reify", answer_occurrence: "ANS-FIXTURE", status: "checked", products: "[]", new_questions: "none", coverage: "process only" });
    const second = spine.begin(base, seal);
    assert.equal(second.obligation, first.obligation);
    assert.notEqual(second.occurrence, first.occurrence);
    const built = spine.build(base);
    assert.equal(built.question_packet.occurrence, second.occurrence);
    assert.equal(built.closure.admissible, false);
  });

  test("all unavailable backends retain a live ResourceBounded field", () => {
    const base = scratchRoot(), file = path.join(base, "formal-successor/BACKEND_PROFILES.json");
    const profiles = JSON.parse(fs.readFileSync(file, "utf8")); profiles.backends = [];
    fs.writeFileSync(file, JSON.stringify(profiles));
    const index = obligationIndex.build(base).index, built = spine.build(base);
    assert.ok(index.live.length > 0); assert.equal(index.executable.length, 0);
    assert.equal(built.question_packet, null); assert.equal(built.closure.admissible, false);
    assert.ok(index.obligations.some((entry) => entry.resource_status === "ResourceBounded"));
  });

  test("accepted fixture returns reopen on source change and breakers create corrective residuals", () => {
    // Synthetic accepted evidence tests accounting, not the mathematical truth of
    // either obligation. Real kernel evidence is checked by the formal CI gates.
    const base = scratchRoot(), ev = require(path.join(hooks, "ic-construction-evidence.js"));
    const driver = "tools/fixture_check.js", source = "// synthetic independent checker fixture\n";
    fs.mkdirSync(path.join(base, "tools"), { recursive: true }); fs.writeFileSync(path.join(base, driver), source);
    const git = (...args) => cp.execFileSync("git", ["-C", base, ...args], { encoding: "utf8", windowsHide: true, stdio: ["ignore", "pipe", "pipe"] });
    const baseline = git("rev-parse", "HEAD").trim(), raw = "PASS synthetic accounting only";
    const before = obligationIndex.build(base);
    const ids = ["FORMAL-C-RELATION-CONTEXTUAL-TELESCOPE", "FORMAL-C-RELATION-FAITHFUL-ENCODING"];
    const ledger = path.join(base, "formal-successor/DECISIONS.jsonl");
    for (const [i, id] of ids.entries()) {
      const entry = before.index.obligations.find((item) => item.id === id);
      fs.appendFileSync(ledger, JSON.stringify({ id: `D-FIXTURE-${i}`, status: "EXACT", construction_return: {
        obligation: id, fingerprint: entry.evidence_fingerprint, authority: "frontier_review_of_independent_return", baseline_commit: baseline,
        disposition: "Proved", coverage: "synthetic accounting only", reopen_when: "any input changes",
        check: { kind: "independent", exit_code: 0, driver, driver_sha256: ev.sha(source), raw, raw_sha256: ev.sha(raw) },
      } }) + "\n");
    }
    git("add", "formal-successor/DECISIONS.jsonl", driver);
    git("-c", "user.name=ConstructionFixture", "-c", "user.email=fixture@example.invalid", "-c", "commit.gpgsign=false", "commit", "--quiet", "-m", "Accepted synthetic accounting returns");
    const accepted = obligationIndex.build(base);
    for (const id of ids) assert.equal(accepted.index.obligations.find((item) => item.id === id).disposition, "Proved");
    const file = path.join(base, "formal/InquiryCalculus/Successor/Relational/Representability.lean");
    const old = fs.readFileSync(file, "utf8"); fs.writeFileSync(file, old.replace("→ a = c", "→ a = a"));
    const reopened = obligationIndex.build(base);
    for (const id of ids) assert.ok(reopened.index.obligations.find((item) => item.id === id).reopened);
    assert.equal(reopened.index.obligations.find((item) => item.id === ids[1]).activation.active, false);
    fs.writeFileSync(file, old);
    const lines = fs.readFileSync(ledger, "utf8").trim().split(/\r?\n/u).map(JSON.parse);
    const receipt = lines.find((line) => line.id === "D-FIXTURE-0").construction_return;
    receipt.disposition = "Broken"; receipt.corrective_residual = "Retain the exact weaker statement required by the fixture breaker.";
    fs.writeFileSync(ledger, lines.map(JSON.stringify).join("\n") + "\n");
    git("add", "formal-successor/DECISIONS.jsonl");
    git("-c", "user.name=ConstructionFixture", "-c", "user.email=fixture@example.invalid", "-c", "commit.gpgsign=false", "commit", "--quiet", "-m", "Accepted synthetic breaker");
    const broken = obligationIndex.build(base);
    assert.equal(broken.selected.kind, "breaker_correction");
    assert.equal(broken.index.obligations.find((item) => item.id === ids[1]).activation.active, false);
  });

  // -------------------------------------------------------------------------
  // Governing invariant: no runtime module may read the frontier for content.
  // -------------------------------------------------------------------------
  test("no module reads the frontier for mathematical content", () => {
    for (const name of ["ic-spine.js", "ic-residual-topology.js", "ic-relational-surface.js"]) {
      const text = fs.readFileSync(path.join(hooks, name), "utf8");
      assert.doesNotMatch(text, /readFileSync\([^)]*IMPLEMENTATION_FRONTIER/u,
        `${name} still reads IMPLEMENTATION_FRONTIER.md`);
      assert.doesNotMatch(text, /LIVE_FRONTIER_BEGIN/u,
        `${name} still parses the frontier live block`);
    }
    const seed = JSON.parse(fs.readFileSync(
      path.join(root, "formal-successor/RESIDUAL_OBLIGATIONS.json"), "utf8"));
    assert.equal(seed.selection_source, "derived:.claude/hooks/ic-obligation-index.js",
      "the residual seed still names a document as the selection source");
  });

  cleanup();
  console.log(`construction obligation field checks passed (${results.length} breakers):`);
  for (const name of results) console.log(`  - ${name}`);
} catch (error) {
  cleanup();
  process.stderr.write(`construction_obligation_check: ${error.message}\n`);
  process.exitCode = 1;
}
