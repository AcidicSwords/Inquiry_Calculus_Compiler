#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const childProcess = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const spine = require("../.claude/hooks/ic-spine.js");

const root = path.resolve(__dirname, "..");
const read = (name) => fs.readFileSync(path.join(root, name));
const json = (name) => JSON.parse(read(name));
const digest = (value) => crypto.createHash("sha256").update(value).digest("hex");

function main() {
  assert.ok(process.argv.slice(2).every((argument) => argument === "--host"));
  const inputs = json("formal-successor/NORMALIZATION_INPUTS.json");
  const continuity = json("formal-successor/NORMALIZATION_CONTINUITY.json");
  const obligations = json("formal-successor/INTEGRATED_THEOREM_OBLIGATIONS.json");
  const profiles = json("formal-successor/BACKEND_PROFILES.json");
  const memory = json("formal-successor/REGENERATIVE_SPINE.json");
  assert.equal(inputs.status, "construction_pressure_not_semantic_authority");
  assert.deepEqual(inputs.inputs.map((entry) => entry.sha256), [
    "8ac49fc18cf9af8a650bf3dd10eac64560f01f14e23151da96269af7e7526458",
    "b47f3b4b44f52468c31b9845c222e39fac088e5663d92705e1bf622b8f22303e"
  ]);
  for (const entry of inputs.inputs) {
    if (fs.existsSync(entry.supplied_path)) assert.equal(digest(fs.readFileSync(entry.supplied_path)), entry.sha256);
  }
  assert.deepEqual(continuity.entries.map((entry) => entry.pass),
    Array.from({ length: 22 }, (_, index) => `P${index + 1}`));
  const obligationIds = new Set(obligations.obligations.map((entry) => entry.id));
  assert.ok(obligationIds.has("IC-THM-C-000"));
  for (const entry of continuity.entries) {
    assert.ok(entry.obligations.length > 0);
    for (const id of entry.obligations) assert.ok(obligationIds.has(id), `${entry.pass}: ${id}`);
  }
  assert.equal(memory.current_semantic_kernel.candidate_basis_obligation, "IC-THM-C-000");
  assert.equal(memory.current_semantic_kernel.primitive_candidates.length, 4);
  assert.equal(profiles.allocation.default_candidate_backend, "local-qwen-candidate");
  assert.equal(profiles.allocation.required_review_backend, "frontier-review");
  assert.ok(profiles.backends.every((entry) => entry.may_warrant === false));
  const local = profiles.backends.find((entry) => entry.id === "local-qwen-candidate");
  assert.equal(local.model, "inquiry-qwen3-coder:30b");
  assert.deepEqual(local.limits, {
    context_tokens: 8192, output_tokens: 2048, parallel_requests: 1,
    attempts_per_occurrence: 2, wall_clock_seconds: 900
  });
  const modelFile = read(".ollama/Modelfile.qwen3-coder-30b").toString();
  for (const expected of [
    "FROM qwen3-coder:30b-a3b-q4_K_M", "PARAMETER num_ctx 8192",
    "PARAMETER num_predict 2048", "never claim that you, a checker, or generated text warrants a theorem"
  ]) assert.ok(modelFile.includes(expected), expected);

  const context = childProcess.spawnSync(process.execPath, [".claude/hooks/ic-spine.js", "context", "."], {
    cwd: root, encoding: "utf8", windowsHide: true
  });
  assert.equal(context.status, 0, context.stdout + context.stderr);
  assert.equal((context.stdout.match(/"kind": "QuestionPacket"/gu) ?? []).length, 1);
  assert.doesNotMatch(context.stdout, /recurrence:|live questions:|selected executable occurrence:/u);
  const fixtureOccurrence = {
    occurrence: "OCC-AMBIENT-CAPABILITY-BASIS", prompt: "Can the alleged carrier be removed?",
    disposition: "Required", executable: true, dependencies: ["PROD-QUESTION-ROUTE-OCCURRENCE"]
  };
  const packet = spine.questionPacket(root, fixtureOccurrence);
  assert.equal(packet.occurrence, fixtureOccurrence.occurrence);
  assert.equal(packet.candidate_basis.length, 4);
  assert.deepEqual(Object.keys(packet.output_contract).sort(), ["allowed_dispositions", "authority", "exact_fields"]);
  assert.match(packet.output_contract.authority, /candidate_only/u);

  if (process.argv.includes("--host")) {
    const ollama = path.join(process.env.LOCALAPPDATA ?? "", "Programs", "Ollama", "ollama.exe");
    assert.ok(fs.existsSync(ollama), "Ollama executable is missing");
    const listed = childProcess.spawnSync(ollama, ["list"], { encoding: "utf8", windowsHide: true });
    assert.equal(listed.status, 0, listed.stdout + listed.stderr);
    assert.match(listed.stdout, /^inquiry-qwen3-coder:30b\s+/mu);
  }
  console.log("PASS total P1-P22 disposition, four-entry candidate basis, one QuestionPacket, bounded local Qwen candidate backend, and mandatory frontier review");
}

main();
