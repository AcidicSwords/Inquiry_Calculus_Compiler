#!/usr/bin/env node
"use strict";

// Normalize recurrence coordinates so formatting and map-key order cannot
// create a false new state fingerprint.

const fs = require("node:fs");

function ordered(value) {
  if (Array.isArray(value)) return value.map(ordered);
  if (value !== null && typeof value === "object") {
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, ordered(value[key])]),
    );
  }
  return value;
}

function compact(value) {
  return value.trim().replace(/\s+/gu, " ");
}

const input = fs.readFileSync(0, "utf8").trim();
if (input.length === 0) {
  process.stderr.write("ic-coordinate: coordinate must be nonempty\n");
  process.exit(1);
}

let normalized;
try {
  normalized = JSON.stringify(ordered(JSON.parse(input)));
} catch {
  const pieces = input.split(/[;,]/u).map(compact).filter(Boolean);
  if (pieces.length > 0 && pieces.every((piece) => piece.includes("="))) {
    normalized = pieces
      .map((piece) => {
        const equals = piece.indexOf("=");
        return [compact(piece.slice(0, equals)), compact(piece.slice(equals + 1))];
      })
      .sort(([leftKey, leftValue], [rightKey, rightValue]) =>
        leftKey.localeCompare(rightKey) || leftValue.localeCompare(rightValue),
      )
      .map(([key, value]) => `${key}=${value}`)
      .join(";");
  } else {
    normalized = compact(input);
  }
}

process.stdout.write(`${normalized}\n`);
