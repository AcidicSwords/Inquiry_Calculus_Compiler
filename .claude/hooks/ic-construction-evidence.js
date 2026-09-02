"use strict";

// Evidence adapter, not a controller. Accepted construction returns live in the
// existing decision ledger; generated fields and model returns cannot admit them.
const fs = require("node:fs");
const path = require("node:path");
const crypto = require("node:crypto");
const cp = require("node:child_process");
const sha = (value) => crypto.createHash("sha256").update(value).digest("hex");
const canonical = (value) => Array.isArray(value) ? `[${value.map(canonical).join(",")}]` :
  value && typeof value === "object" ? `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(",")}}` : JSON.stringify(value);

function readReturns(root) {
  const file = path.join(root, "formal-successor/DECISIONS.jsonl");
  if (!fs.existsSync(file)) return [];
  // A working candidate cannot add its own acceptance authority. Only the frozen
  // Git acceptance epoch contributes admitted returns; pending checked records
  // are reviewed and committed as a green ratchet before becoming field evidence.
  let accepted;
  try {
    accepted = cp.execFileSync("git", ["-C", root, "show", "HEAD:formal-successor/DECISIONS.jsonl"],
      { encoding: "utf8", windowsHide: true, stdio: ["ignore", "pipe", "pipe"] });
  } catch { return []; }
  const admitted = new Set(accepted.split(/\r?\n/u).filter(Boolean).map((line) => canonical(JSON.parse(line))));
  return fs.readFileSync(file, "utf8").split(/\r?\n/u).filter(Boolean).map(JSON.parse)
    .filter((record) => admitted.has(canonical(record)))
    .filter((record) => ["EXACT", "CONDITIONAL"].includes(record.status) && record.construction_return)
    .map((record) => ({ decision: record.id, ...record.construction_return }));
}

// All governing source inputs are conservatively protected. Lean source bytes,
// not just declaration names, participate. Refinement of this dependency envelope
// itself needs a coverage proof; over-invalidation cannot silently retain a claim.
function fingerprint(entry, sources) {
  const { discharged_by, disposition, activation, executable, blocked_reason,
    evidence_fingerprint, evidence_status, reopened, backend, local_attempts_used,
    resource_status, completed_attempts, ...relation } = entry;
  return sha(canonical({ relation, inputs: sources }));
}

function checkReturn(root, entry, sources, record) {
  if (!record || record.obligation !== entry.id) return { valid: false, reason: "no independent checked return" };
  if (record.fingerprint !== fingerprint(entry, sources)) return { valid: false, reason: "source, horizon or dependency changed" };
  if (record.authority !== "frontier_review_of_independent_return" || !record.baseline_commit?.match(/^[0-9a-f]{40}$/u)) {
    return { valid: false, reason: "missing independent acceptance epoch" };
  }
  if (!["Proved", "Broken", "Inapplicable"].includes(record.disposition) || !record.coverage || !record.reopen_when) {
    return { valid: false, reason: "untyped disposition or absent coverage" };
  }
  if (!record.check || record.check.exit_code !== 0 || record.check.kind !== "independent" ||
      !record.check.driver?.match(/^tools\/[a-z0-9_]+\.(?:js|py)$/u) ||
      !record.check.raw || sha(record.check.raw) !== record.check.raw_sha256) {
    return { valid: false, reason: "absent independently checked raw return" };
  }
  const driver = path.join(root, record.check.driver);
  if (!fs.existsSync(driver) || sha(fs.readFileSync(driver)) !== record.check.driver_sha256) {
    return { valid: false, reason: "independent discriminator changed or missing" };
  }
  if (record.disposition === "Broken" && !record.corrective_residual) {
    return { valid: false, reason: "breaker lacks corrective residual" };
  }
  if (record.disposition === "Inapplicable" && !record.applicability_witness) {
    return { valid: false, reason: "inapplicability requires a scoped witness" };
  }
  return { valid: true, decision: record.decision, disposition: record.disposition };
}

module.exports = { sha, canonical, fingerprint, readReturns, checkReturn };
