#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const cp = require("node:child_process");
const root = path.resolve(__dirname, "..");
const contractLoader = require("../.claude/hooks/ic-contract.js");
const spine = require("../.claude/hooks/ic-spine.js");
const generator = require("../.claude/hooks/ic-recursive-generator.js");

const loaded = contractLoader.read(root);
const contract = loaded.contract;
assert.equal(contract.schema, 5);
assert.deepEqual(contract.model_recurrence, ["RELATE", "OPEN", "TURN", "RETURN", "DISTINGUISH", "FOLD", "CARRY"]);
assert.equal(new Set(contract.question_forms.map((form) => form.id)).size, contract.question_forms.length);
assert.ok(!Object.hasOwn(contract, "rhythm"));
assert.ok(!Object.hasOwn(contract, "inner_phases"));
assert.ok(!Object.hasOwn(contract, "method_dispatch"));

function product(id, relations, discriminators = []) {
  return {
    id, status: "checked", kind: "relation_surface", dependencies: [], invalidated: false,
    horizon: "fixture-horizon", coverage: "fixture-coverage",
    inquiry_generator_surface: { relations, questions: [], discriminators },
  };
}
const relation = (id, source, target, extra = {}) => ({ id, source, target, path: `fixture/${id}`, dependencies: [], ...extra });
const discriminator = (id, domain) => ({ id, domain, codomain: "Bool", path: `fixture/${id}`, dependencies: [] });

const surface = {
  products: [
    product("SURFACE", [relation("R", "A", "B"), relation("S", "B", "C"), relation("T", "X", "Y")], [discriminator("K", "C")]),
  ],
};
const paths = spine.derivePaths(surface);
assert.ok(paths.paths.some((item) => item.relation_ids.join(",") === "R,S"));
assert.ok(!paths.paths.some((item) => item.relation_ids.join(",") === "R,T"));
assert.ok(paths.paths.every((item) => item.represented_not_actual));
assert.ok(!paths.paths.some((item) => item.typed_source === "B" && item.typed_target === "A"));

const carried = spine.transport(surface, paths);
const compositeCarry = carried.find((item) => item.composition === "K∘S∘R");
assert.ok(compositeCarry, "later discriminator must transport through typed ancestry");
assert.equal(compositeCarry.typed_source, "A");
assert.equal(compositeCarry.standing, false);
assert.equal(compositeCarry.status, "Generated");

const contextualSurface = {
  products: [
    product("LEFT", [{ ...relation("R-left", "A", "B"), path: "left/R" }]),
    product("RIGHT", [{ ...relation("R-right", "A", "B"), path: "right/R" }]),
  ],
};
const contextual = spine.derivePaths(contextualSurface).paths;
assert.notEqual(contextual[0].path_id, contextual[1].path_id, "context/path ancestry must remain in identity");

const required = { occurrence: "Q-B", disposition: "Required", executable: true, dependencies: [] };
const requiredEarlier = { occurrence: "Q-A", disposition: "Required", executable: true, dependencies: [] };
const productive = { occurrence: "Q-0", disposition: "Productive", executable: true, dependencies: [] };
const unknown = { occurrence: "Q-X", disposition: "Unknown", executable: true, dependencies: [] };
const selection = spine.selectExecutable([required, productive, requiredEarlier, unknown], []);
assert.equal(selection.selected.occurrence, "Q-A");
assert.equal(selection.executable_frontier.length, 3);
assert.equal(spine.selectExecutable([requiredEarlier], ["dep"]).selected?.occurrence, "Q-A");
assert.equal(spine.selectExecutable([{ ...requiredEarlier, dependencies: ["dep"] }], ["dep"]).selected, null);

const closureBase = { field: {}, unresolved_ask: null, answer_awaiting_reification: null, surface_dirty: false, folds: [] };
assert.equal(spine.evaluateClosure(closureBase, [], []).admissible, true);
assert.equal(spine.evaluateClosure({ ...closureBase, unresolved_ask: {} }, [], []).admissible, false);
assert.equal(spine.evaluateClosure(closureBase, [required], []).admissible, false);
assert.equal(spine.evaluateClosure(closureBase, [], carried).admissible, false);
assert.equal(spine.evaluateClosure({ ...closureBase, folds: [{ reopen_required: true }] }, [], []).admissible, false);

const generated = generator.generate({
  relations: [relation("R", "A", "B"), relation("S", "B", "C")],
  questions: [{ occurrence: "Q1", path: "q/1", context: "ctx-1", exchangeable: false, dependencies: [] }],
  discriminators: [discriminator("K", "C")],
}, contract);
assert.ok(generated.some((question) => question.derivation.family === "COMPOSE"));
assert.ok(generated.some((question) => question.derivation.family === "TRANSPORT"));
assert.ok(generated.every((question) => question.context && question.path));
assert.ok(generated.every((question) => question.disposition === "Unknown" && question.executable === false));

const retired = [
  "AUTONOMOUS_ITERATION.md", "QUESTION_RHYTHM.md", "ENGINEERING_QUESTION_PROGRAMS.json",
  "PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS.md", "PREFORMAL_SEARCH_ASYMMETRY.md",
  "SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md", "QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md",
];
for (const name of retired) assert.equal(fs.existsSync(path.join(root, "formal-successor", name)), false);
const injection = fs.readFileSync(path.join(root, ".claude/hooks/ic-inject"), "utf8");
assert.match(injection, /ic-spine\.js/);
assert.doesNotMatch(injection, /SPECIFY\s*->|FIELD\s*->\s*ASK/);
const policy = fs.readFileSync(path.join(root, ".claude/hooks/ic-construction-policy.js"), "utf8");
assert.doesNotMatch(policy, /LEGACY_METHOD_DISPATCH|chooseMethodFrontier|deduplicateQuestions|groupQuestionEquivalenceCandidates/);
const append = fs.readFileSync(path.join(root, ".claude/hooks/ic-append.js"), "utf8");
assert.doesNotMatch(append, /validateLegacyStateMachine|validateV4StateMachine|foldEvidenceSchema !== "0"/);
const trace = fs.readFileSync(path.join(root, ".claude/hooks/ic-trace"), "utf8");
assert.doesNotMatch(trace, /active_schema|q_program_context|\$cmd" = "question/);
const questionProgram = fs.readFileSync(path.join(root, ".claude/hooks/ic-question-program.js"), "utf8");
assert.doesNotMatch(questionProgram, /validatePolicyTransition/);

const before = spine.derivePaths(surface);
const after = spine.derivePaths(JSON.parse(JSON.stringify(surface)));
assert.deepEqual(after, before, "deleting and rebuilding a path projection must be exact");

const context = cp.execFileSync(process.execPath, [path.join(root, ".claude/hooks/ic-spine.js"), "context", root], { encoding: "utf8" });
assert.equal((context.match(/"kind": "QuestionPacket"/g) ?? []).length, 1);
assert.doesNotMatch(context, /recurrence:|live questions:|selected executable occurrence:/);
const packet = spine.questionPacket(root, requiredEarlier);
assert.equal(packet.occurrence, requiredEarlier.occurrence);
assert.equal(packet.output_contract.authority, "candidate_only; independent checks and frontier review required");

console.log("inquiry spine checks passed (one controller recurrence, one QuestionPacket, typed paths, CARRY, context identity, selection, closure, regeneration)");
