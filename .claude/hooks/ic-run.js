#!/usr/bin/env node
"use strict";

// Cross-platform launcher for the POSIX inquiry hooks. On Windows, avoid the
// Microsoft Store/WSL bash alias and locate Git Bash from the installed Git.

const { spawnSync, execFileSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

const allowed = new Set(["guard", "inject", "stop"]);
const mode = process.argv[2];

if (!allowed.has(mode)) {
  process.stderr.write("ic-run: expected guard, inject, or stop\n");
  process.exit(2);
}

function findBash() {
  if (process.platform !== "win32") {
    return "bash";
  }

  const candidates = [];
  if (process.env.ProgramFiles) {
    candidates.push(path.join(process.env.ProgramFiles, "Git", "bin", "bash.exe"));
  }
  if (process.env["ProgramFiles(x86)"]) {
    candidates.push(
      path.join(process.env["ProgramFiles(x86)"], "Git", "bin", "bash.exe"),
    );
  }

  try {
    const execPath = execFileSync("git", ["--exec-path"], {
      encoding: "utf8",
      windowsHide: true,
    }).trim();
    candidates.push(path.resolve(execPath, "..", "..", "..", "bin", "bash.exe"));
  } catch {
    // The explicit candidates below still provide a useful diagnostic.
  }

  for (const candidate of candidates) {
    if (fs.existsSync(candidate)) {
      return candidate;
    }
  }

  throw new Error(
    "Git Bash was not found; install Git for Windows or update ic-run.js with its exact path",
  );
}

let bash;
try {
  bash = findBash();
} catch (error) {
  process.stderr.write(`ic-run: ${error.message}\n`);
  process.exit(1);
}

const script = path.join(__dirname, `ic-${mode}`);
const result = spawnSync(bash, [script], {
  cwd: path.resolve(__dirname, "..", ".."),
  env: process.env,
  input: fs.readFileSync(0),
  encoding: "utf8",
  windowsHide: true,
});

if (result.stdout) {
  process.stdout.write(result.stdout);
}
if (result.stderr) {
  process.stderr.write(result.stderr);
}
if (result.error) {
  process.stderr.write(`ic-run: ${result.error.message}\n`);
}
process.exit(result.status ?? 1);
