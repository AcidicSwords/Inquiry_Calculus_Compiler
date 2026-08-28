#!/usr/bin/env node
"use strict";

// Validate and append one inquiry-trace record while holding an exclusive lock.
// Node is already the cross-platform launcher dependency for this harness.

const fs = require("node:fs");
const path = require("node:path");
const {
  validatePolicy,
  validateQuestionProgram,
  validateStoredQuestion,
} = require("./ic-question-program.js");

const [operation, tracePath, fuelPath] = process.argv.slice(2);
if (!new Set(["validate", "append"]).has(operation) || !tracePath) {
  process.stderr.write("ic-append: expected validate|append TRACE_FILE\n");
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
  if (policies.length > 1 || (policies.length === 1 && policies[0].seq !== 1)) {
    fail("question-program policy must occur exactly once at the first record");
  }
  if (policies.length === 1) {
    validatePolicy(policies[0]);
    for (const record of records) {
      if (record.kind === "question") validateStoredQuestion(record, policies[0]);
    }
  }
  validateStateMachine(records);
  return records;
}

function readStdin() {
  return fs.readFileSync(0, "utf8");
}

function validateStateMachine(records) {
  let cycle = null;
  let lastResidual = 0;
  let lastStop = 0;
  let questionProgramPolicy = false;

  for (const record of records) {
    switch (record.kind) {
      case "policy":
        questionProgramPolicy = true;
        break;
      case "question":
        if (
          questionProgramPolicy &&
          cycle !== null &&
          cycle.uncomposedReturns > 0
        ) {
          cycle.questions += 1;
          cycle.uncomposedReturns -= 1;
        }
        break;
      case "seal":
        if (cycle !== null) {
          fail(`line ${record.seq} opens a seal before the prior cycle closes`);
        }
        cycle = { raw: 0, check: 0, questions: 0, uncomposedReturns: 0 };
        break;
      case "raw":
        if (cycle === null) {
          fail(`line ${record.seq} records a raw return without an open seal`);
        }
        cycle.raw += 1;
        cycle.uncomposedReturns += 1;
        break;
      case "check":
        if (cycle === null || cycle.raw === 0) {
          fail(`line ${record.seq} checks before an actual raw return`);
        }
        cycle.check += 1;
        break;
      case "residual":
        if (
          cycle === null ||
          cycle.raw === 0 ||
          cycle.check === 0 ||
          (questionProgramPolicy &&
            (cycle.questions === 0 || cycle.uncomposedReturns !== 0))
        ) {
          fail(
            `line ${record.seq} closes a cycle before every raw return has a subsequent question program and check`,
          );
        }
        cycle = null;
        lastResidual = record.seq;
        break;
      case "stop": {
        if (cycle !== null) {
          fail(`line ${record.seq} stops while a sealed cycle is open`);
        }
        if (lastResidual === 0 || lastResidual <= lastStop) {
          fail(`line ${record.seq} has no new checked residual to stop on`);
        }
        if (typeof record.warrant !== "string" || record.warrant.trim() === "") {
          fail(`line ${record.seq} stop requires a nonempty warrant`);
        }
        if (record.state === "Satisfied") {
          const warrant = record.warrant.toLowerCase().replace(/\s+/g, "");
          if (/^(?:none|self|agent|generated)(?:$|[:/_-])/u.test(warrant)) {
            fail(`line ${record.seq} Satisfied stop has a self warrant`);
          }
        }
        lastStop = record.seq;
        break;
      }
      default:
        break;
    }
  }

  return { open: cycle !== null };
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

let lockFd;
try {
  lockFd = acquireLock();
  const records = validatedRecords();
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
    if (record.kind === "question") {
      if (
        typeof record.fp !== "string" ||
        record.fp.length === 0 ||
        typeof record.answer !== "string"
      ) {
        fail("question record requires nonempty fp and string answer");
      }
      record.question_program_check = validateQuestionProgram(
        record,
        path.resolve(__dirname, "../.."),
      );
      const policy = records.find((prior) => prior.kind === "policy");
      if (policy) validateStoredQuestion(record, policy);
      if (
        records.some(
          (prior) => prior.fp === record.fp && prior.answer === record.answer,
        )
      ) {
        fail(
          "repeated state: same occurrence, continuation, bindings, frontier, " +
            "horizon, coverage, repository actuality, and answer",
        );
      }
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
    if (record.kind === "question") {
      consumeQuestionFuel();
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
