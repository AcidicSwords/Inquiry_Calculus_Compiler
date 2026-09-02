#!/usr/bin/env node
"use strict";

// No model calls: adversarial transport returns pass through the real sealed lifecycle.
const assert = require("node:assert/strict"), fs = require("node:fs"), os = require("node:os"), path = require("node:path");
const spine = require("../.claude/hooks/ic-spine.js");
const adapter = require("../.claude/hooks/ic-local-attempt.js");
const root = path.resolve(__dirname, "..");

async function main() {
  const base = fs.mkdtempSync(path.join(os.tmpdir(), "ic-local-return-"));
  try {
    for (const relative of ["formal-successor", "formal/InquiryCalculus", ".claude/hooks"]) {
      fs.cpSync(path.join(root, relative), path.join(base, relative), { recursive: true,
        filter: (source) => !/CONSTRUCTION_OBLIGATION_INDEX\.json|PROTECTED_CONSTRUCTION_SURFACE\.json/u.test(source) });
    }
    const memoryPath = path.join(base, "formal-successor/REGENERATIVE_SPINE.json");
    const memory = JSON.parse(fs.readFileSync(memoryPath, "utf8"));
    for (const capability of memory.protected_predecessor_capabilities) {
      for (const item of capability.remaining_correspondence ?? []) item.discharged_by = null;
    }
    fs.writeFileSync(memoryPath, JSON.stringify(memory));
    spine.begin(base, { should_change: "capture candidate actuality only", invariants: "no acceptance",
      discriminator: "malformed and transport failures remain Raw and consume attempts", wrong_impl: "lost failure resets budget",
      coverage: "two bounded mock transports through actual lifecycle" });
    const packet = spine.build(base).question_packet;
    const backend = JSON.parse(fs.readFileSync(path.join(base, "formal-successor/BACKEND_PROFILES.json")))
      .backends.find((entry) => entry.id === packet.backend);
    assert.equal(backend.id, "local-qwen-candidate");
    let release;
    const pending = adapter.invokeSelected(base, packet, backend, { fetcher: () => new Promise((resolve) => { release = resolve; }) });
    await assert.rejects(adapter.invokeSelected(base, packet, backend, { fetcher: async () => { throw Error("must not run"); } }), /EEXIST/u);
    release({ ok: true, text: async () => "not valid JSON" });
    await assert.rejects(pending, /retained before rejection/u);
    const readTrace = () => fs.readFileSync(path.join(base, ".claude/trace",
      fs.readFileSync(path.join(base, ".claude/trace/.state"), "utf8").trim()), "utf8").trim().split(/\r?\n/u).map(JSON.parse);
    const first = readTrace().filter((entry) => entry.kind === "raw");
    assert.equal(first.length, 1);
    assert.equal(fs.readFileSync(path.join(base, first[0].raw_ref), "utf8"), "not valid JSON");
    await assert.rejects(adapter.invokeSelected(base, packet, backend, {
      fetcher: async () => { throw Error("connection deliberately unavailable"); }
    }), /local transport failed; Raw/u);
    const returns = readTrace().filter((entry) => entry.kind === "raw");
    assert.equal(returns.length, 2);
    assert.equal(JSON.parse(fs.readFileSync(path.join(base, returns[1].raw_ref), "utf8")).kind, "transport_failure");
    await assert.rejects(adapter.invokeSelected(base, packet, backend, {
      fetcher: async () => { throw Error("must not run after exhaustion"); }
    }), /exhausted its/u);
    const rebuilt = spine.build(base);
    assert.equal(rebuilt.question_packet.backend, "frontier-review");
    assert.equal(rebuilt.closure_certificate.established, false);
    assert.equal(readTrace().filter((entry) => entry.kind === "answer").length, 0);
    console.log("PASS local return capture: concurrent call blocked; malformed and transport failures preserved; two attempts exhausted; frontier handoff; no acceptance or closure");
  } finally {
    const resolved = path.resolve(base);
    assert.equal(path.dirname(resolved), path.resolve(os.tmpdir()));
    assert.ok(path.basename(resolved).startsWith("ic-local-return-"));
    fs.rmSync(resolved, { recursive: true, force: true });
  }
}
main().catch((error) => { console.error(error); process.exitCode = 1; });
