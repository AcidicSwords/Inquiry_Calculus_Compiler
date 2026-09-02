#!/usr/bin/env node
"use strict";

// Operational candidate invocation only. It does not append lifecycle evidence or accept a result.
const fs = require("node:fs");
const crypto = require("node:crypto");
const path = require("node:path");
const spine = require("./ic-spine.js");

function requireSealedProbe(root, occurrence, attemptLimit) {
  const statePath = path.join(root, ".claude/trace/.state");
  if (!fs.existsSync(statePath)) throw new Error("no active lifecycle trace; local return must be a sealed Probe");
  const tracePath = path.join(root, ".claude/trace", fs.readFileSync(statePath, "utf8").trim());
  const records = fs.readFileSync(tracePath, "utf8").trim().split(/\r?\n/u).filter(Boolean).map(JSON.parse);
  const ask = [...records].reverse().find((record) => record.kind === "ask" && record.occurrence === occurrence);
  if (!ask || ask.mode !== "Probe") throw new Error(`selected occurrence ${occurrence} is not an active Probe`);
  const seal = records.find((record) => record.kind === "seal" && record.ask_occurrence === occurrence && record.seq > ask.seq);
  if (!seal) throw new Error(`selected Probe ${occurrence} has no prospective seal`);
  const laterAsk = records.find((record) => record.kind === "ask" && record.seq > ask.seq && record.occurrence !== occurrence);
  if (laterAsk) throw new Error(`selected Probe ${occurrence} is stale after ${laterAsk.occurrence}`);
  const attempts = records.filter((record) => record.kind === "raw" && record.ask_occurrence === occurrence &&
    record.seq > seal.seq && /ic-local-attempt\.js/u.test(record.cmd ?? "")).length;
  if (attempts >= attemptLimit) throw new Error(`selected Probe ${occurrence} exhausted its ${attemptLimit} local attempts`);
  return { trace: path.basename(tracePath), ask_seq: ask.seq, seal_seq: seal.seq, attempts };
}

async function main() {
  const args = process.argv.slice(2);
  const dryRun = args.includes("--dry-run");
  const rawFileArg = args.find((arg) => arg.startsWith("--raw-file="));
  const rawFile = rawFileArg ? path.resolve(rawFileArg.slice("--raw-file=".length)) : null;
  const suppliedRoot = args.find((arg) => !arg.startsWith("--"));
  const root = path.resolve(suppliedRoot ?? path.resolve(__dirname, "../.."));
  const config = JSON.parse(fs.readFileSync(path.join(root, "formal-successor/BACKEND_PROFILES.json"), "utf8"));
  const backend = config.backends.find((entry) => entry.id === config.allocation.default_candidate_backend);
  if (!backend || backend.provider !== "ollama") throw new Error("configured local Ollama backend is missing");
  if (backend.trace_mode !== "Probe") throw new Error("local external returns must be configured as Probe actuality");
  const built = spine.build(root);
  const packet = built.question_packet;
  if (!packet) throw new Error("no selected executable occurrence; no candidate invocation is lawful");
  const request = {
    model: backend.model,
    stream: false,
    format: "json",
    prompt: JSON.stringify(packet),
    options: { num_ctx: backend.limits.context_tokens, num_predict: backend.limits.output_tokens },
    keep_alive: "10m"
  };
  if (dryRun) {
    process.stdout.write(`${JSON.stringify({ backend: backend.id, request, acceptance: "unperformed" }, null, 2)}\n`);
    return;
  }
  const lifecycle = requireSealedProbe(root, packet.occurrence, backend.limits.attempts_per_occurrence);
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), backend.limits.wall_clock_seconds * 1000);
  try {
    const response = await fetch(backend.endpoint, {
      method: "POST", headers: { "content-type": "application/json" },
      body: JSON.stringify(request), signal: controller.signal
    });
    const raw = await response.text();
    if (!response.ok) throw new Error(`Ollama ${response.status}: ${raw.slice(0, 500)}`);
    const envelope = JSON.parse(raw);
    const candidate = JSON.parse(envelope.response);
    const expected = [...packet.output_contract.exact_fields].sort();
    const actual = Object.keys(candidate).sort();
    if (JSON.stringify(actual) !== JSON.stringify(expected)) {
      throw new Error(`CandidateReturn fields differ: ${actual.join(",")}`);
    }
    if (!packet.output_contract.allowed_dispositions.includes(candidate.disposition)) {
      throw new Error(`CandidateReturn disposition is not allowed: ${candidate.disposition}`);
    }
    if (rawFile) fs.writeFileSync(rawFile, raw);
    process.stdout.write(`${JSON.stringify({
      backend: backend.id, question_occurrence: packet.occurrence, request,
      raw_return: rawFile ? { path: rawFile, sha256: crypto.createHash("sha256").update(raw).digest("hex") } : raw,
      candidate_return: candidate,
      lifecycle,
      metrics: { total_duration_ns: envelope.total_duration, load_duration_ns: envelope.load_duration,
        prompt_tokens: envelope.prompt_eval_count, generated_tokens: envelope.eval_count },
      acceptance: "unperformed"
    }, null, 2)}\n`);
  } finally { clearTimeout(timeout); }
}

main().catch((error) => { process.stderr.write(`ic-local-attempt: ${error.message}\n`); process.exitCode = 1; });
