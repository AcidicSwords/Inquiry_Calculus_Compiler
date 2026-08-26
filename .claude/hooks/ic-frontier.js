#!/usr/bin/env node
"use strict";

// Single exact parser for the live implementation-frontier block. Both the
// runtime hooks and the repository documentation checker consume this result.

const crypto = require("node:crypto");
const fs = require("node:fs");

const frontierPath = process.argv[2];
const format = process.argv[3] ?? "json";
if (!frontierPath || !new Set(["json", "context", "block"]).has(format)) {
  process.stderr.write("ic-frontier: FRONTIER_PATH [json|context|block]\n");
  process.exit(2);
}

const beginMarker = "<!-- LIVE_FRONTIER_BEGIN -->";
const endMarker = "<!-- LIVE_FRONTIER_END -->";
const keys = [
  "id",
  "plan_phase",
  "goal",
  "protected_difference",
  "discriminator",
  "horizon",
  "relevant_decisions",
  "relevant_failures",
  "if_pass",
  "if_fail",
];
const allowed = new Set(keys);

function fail(message) {
  process.stderr.write(`ic-frontier: ${message}\n`);
  process.exit(1);
}

let text;
try {
  text = fs.readFileSync(frontierPath, "utf8").replaceAll("\r\n", "\n");
} catch (error) {
  fail(`cannot read frontier: ${error.message}`);
}

const lines = text.split("\n");
const begins = [];
const ends = [];
for (let index = 0; index < lines.length; index += 1) {
  if (lines[index] === beginMarker) begins.push(index);
  if (lines[index] === endMarker) ends.push(index);
}
if (begins.length !== 1 || ends.length !== 1 || begins[0] >= ends[0]) {
  fail("frontier must contain exactly one ordered pair of exact live-block markers");
}

const blockLines = lines.slice(begins[0] + 1, ends[0]);
const fields = Object.create(null);
for (let offset = 0; offset < blockLines.length; offset += 1) {
  const raw = blockLines[offset];
  if (raw.trim() === "") continue;
  const colon = raw.indexOf(":");
  if (colon < 0) {
    fail(`live block line ${offset + 1} is not key: value`);
  }
  const key = raw.slice(0, colon).trim();
  const value = raw.slice(colon + 1).trim();
  if (!allowed.has(key)) fail(`unknown live frontier key ${JSON.stringify(key)}`);
  if (Object.hasOwn(fields, key)) fail(`duplicate live frontier key ${key}`);
  if (value.length === 0) fail(`live frontier key ${key} is empty`);
  fields[key] = value;
}

const missing = keys.filter((key) => !Object.hasOwn(fields, key));
if (missing.length > 0) fail(`missing live frontier keys: ${missing.join(", ")}`);
if (!/^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)+$/u.test(fields.id)) {
  fail(`live frontier id has an invalid stable form: ${JSON.stringify(fields.id)}`);
}

const canonical = keys.map((key) => `${key}: ${fields[key]}`).join("\n");
const digest = crypto.createHash("sha256").update(canonical).digest("hex").slice(0, 16);
const result = { id: fields.id, digest, fields, block: canonical };

if (format === "context") {
  process.stdout.write(`${result.id}\t${result.digest}\n`);
} else if (format === "block") {
  process.stdout.write(`${result.block}\n`);
} else {
  process.stdout.write(`${JSON.stringify(result)}\n`);
}
