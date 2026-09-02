#!/usr/bin/env node
"use strict";

// Operational invocation with accountable Raw capture. Never interprets, checks or accepts a result.
const fs = require("node:fs");
const crypto = require("node:crypto");
const path = require("node:path");
const cp = require("node:child_process");
const spine = require("./ic-spine.js");

function requireSealedProbe(root, packet, attemptLimit) {
  const occurrence = packet.occurrence;
  const statePath = path.join(root, ".claude/trace/.state");
  if (!fs.existsSync(statePath)) throw new Error("no active lifecycle trace; local return must be a sealed Probe");
  const tracePath = path.join(root, ".claude/trace", fs.readFileSync(statePath, "utf8").trim());
  const records = fs.readFileSync(tracePath, "utf8").trim().split(/\r?\n/u).filter(Boolean).map(JSON.parse);
  const ask = [...records].reverse().find((record) => record.kind === "ask" && record.occurrence === occurrence);
  if (!ask || ask.mode !== "Probe") throw new Error(`selected occurrence ${occurrence} is not an active Probe`);
  const seal = records.find((record) => record.kind === "seal" && record.ask_occurrence === occurrence && record.seq > ask.seq);
  if (!seal) throw new Error(`selected Probe ${occurrence} has no prospective seal`);
  if (records.some((record) => record.kind === "answer" && record.ask_occurrence === occurrence)) {
    throw new Error(`selected Probe ${occurrence} has already returned an Answer`);
  }
  if (ask.obligation_identity !== packet.obligation_identity || ask.obligation_fingerprint !== packet.obligation_fingerprint ||
      ask.packet_digest !== spine.digest(packet) || seal.packet_digest !== ask.packet_digest) {
    throw new Error("sealed packet differs from the selected construction obligation");
  }
  const laterAsk = records.find((record) => record.kind === "ask" && record.seq > ask.seq && record.occurrence !== occurrence);
  if (laterAsk) throw new Error(`selected Probe ${occurrence} is stale after ${laterAsk.occurrence}`);
  const attempts = records.filter((record) => record.kind === "raw" && record.ask_occurrence === occurrence &&
    record.seq > seal.seq && /ic-local-attempt\.js/u.test(record.cmd ?? "")).length;
  if (attempts >= attemptLimit) throw new Error(`selected Probe ${occurrence} exhausted its ${attemptLimit} local attempts`);
  return { trace: path.basename(tracePath), ask_seq: ask.seq, seal_seq: seal.seq, attempts };
}

function preserveRaw(root, packet, lifecycle, raw, rawFile) {
  const digest = crypto.createHash("sha256").update(raw).digest("hex");
  const reference = `.claude/trace/raw/${digest}`;
  const target = path.join(root, reference);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  if (fs.existsSync(target)) {
    if (!fs.readFileSync(target).equals(Buffer.from(raw))) throw new Error("raw digest collision");
  } else fs.writeFileSync(target, raw, { flag: "wx", mode: 0o600 });
  const appended = cp.spawnSync(process.execPath, [path.join(root, ".claude/hooks/ic-append.js"), "append",
    path.join(root, ".claude/trace", lifecycle.trace)], {
    cwd: root, encoding: "utf8", windowsHide: true,
    input: JSON.stringify({ ts: new Date().toISOString(), kind: "raw", ask_occurrence: packet.occurrence,
      cmd: "node .claude/hooks/ic-local-attempt.js", digest, raw_ref: reference, sensitive: "false" }) + "\n"
  });
  if (appended.status !== 0) throw new Error(`Raw preserved at ${reference}, but trace append failed: ${appended.stderr}`);
  // An optional caller copy never replaces the authoritative captured return.
  if (rawFile) fs.writeFileSync(rawFile, raw, { flag: "wx", mode: 0o600 });
  return { path: reference, sha256: digest };
}

async function invokeSelected(root, packet, backend, { rawFile = null, fetcher = fetch } = {}) {
  if (packet.backend !== backend.id || packet.execution_hold) throw new Error("local backend is not selected");
  const lockPath = path.join(root, ".claude/trace/.local-attempt.lock");
  // A crashed invocation leaves an explicit operational blocker; never silently reset its budget.
  const lock = fs.openSync(lockPath, "wx");
  let timer;
  try {
    fs.writeFileSync(lock, JSON.stringify({ pid: process.pid, occurrence: packet.occurrence }));
    const lifecycle = requireSealedProbe(root, packet, backend.limits.attempts_per_occurrence);
    const controller = new AbortController();
    timer = setTimeout(() => controller.abort(), backend.limits.wall_clock_seconds * 1000);
    const request = { model: backend.model, stream: false, format: "json", prompt: JSON.stringify(packet),
      options: { num_ctx: backend.limits.context_tokens, num_predict: backend.limits.output_tokens }, keep_alive: "10m" };
    let response, raw;
    try {
      response = await fetcher(backend.endpoint, { method: "POST", headers: { "content-type": "application/json" },
        body: JSON.stringify(request), signal: controller.signal });
      raw = await response.text();
    } catch (error) {
      // This is the transport error's actual return, not a fabricated model response.
      const evidence = preserveRaw(root, packet, lifecycle,
        JSON.stringify({ kind: "transport_failure", name: error.name, message: error.message }), rawFile);
      throw new Error(`local transport failed; Raw ${evidence.sha256}: ${error.message}`);
    }
    const evidence = preserveRaw(root, packet, lifecycle, raw, rawFile);
    try {
      if (!response.ok) throw new Error(`Ollama HTTP ${response.status}`);
      const envelope = JSON.parse(raw), candidate = JSON.parse(envelope.response);
      const expected = [...packet.output_contract.exact_fields].sort(), actual = Object.keys(candidate).sort();
      if (JSON.stringify(actual) !== JSON.stringify(expected)) throw new Error(`CandidateReturn fields differ: ${actual.join(",")}`);
      if (!packet.output_contract.allowed_dispositions.includes(candidate.disposition)) throw new Error(`CandidateReturn disposition is not allowed: ${candidate.disposition}`);
      return { backend: backend.id, question_occurrence: packet.occurrence, request, raw_return: evidence,
        candidate_return: candidate, lifecycle, metrics: { total_duration_ns: envelope.total_duration,
          load_duration_ns: envelope.load_duration, prompt_tokens: envelope.prompt_eval_count,
          generated_tokens: envelope.eval_count }, acceptance: "unperformed" };
    } catch (error) { throw new Error(`Raw ${evidence.sha256} retained before rejection: ${error.message}`); }
  } finally { clearTimeout(timer); fs.closeSync(lock); fs.unlinkSync(lockPath); }
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
  if (packet.execution_hold) throw new Error(packet.execution_hold.reason);
  if (packet.backend !== backend.id) throw new Error(`selected backend is ${packet.backend}; local allocation is unavailable or exhausted`);
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
  process.stdout.write(`${JSON.stringify(await invokeSelected(root, packet, backend, { rawFile }), null, 2)}\n`);
}

module.exports = { requireSealedProbe, invokeSelected };
if (require.main === module) main().catch((error) => { process.stderr.write(`ic-local-attempt: ${error.message}\n`); process.exitCode = 1; });
