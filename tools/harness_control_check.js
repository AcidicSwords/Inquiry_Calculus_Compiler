#!/usr/bin/env node
"use strict";

// Isolated adversarial checks for the repository inquiry harness. These tests
// copy the complete control surface into a temporary project, so neither the
// live engineering trace nor repository state is changed.

const assert = require("node:assert/strict");
const { execFileSync, spawn, spawnSync } = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const repository = path.resolve(__dirname, "..");
const frontierKeys = [
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

function findBash() {
  if (process.platform !== "win32") return "bash";
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
    // Explicit installation paths remain available.
  }
  const found = candidates.find((candidate) => fs.existsSync(candidate));
  if (!found) throw new Error("Git Bash is required for harness checks on Windows");
  return found;
}

const bash = findBash();
const sandbox = fs.mkdtempSync(path.join(os.tmpdir(), "ic-harness-check-"));
const claudeDirectory = path.join(sandbox, ".claude");
const hooks = path.join(claudeDirectory, "hooks");
const traceDirectory = path.join(claudeDirectory, "trace");
const settingsPath = path.join(claudeDirectory, "settings.json");
const frontierPath = path.join(sandbox, "IMPLEMENTATION_FRONTIER.md");
const questionDirectory = path.join(sandbox, "formal-successor");
const questionSourcePath = path.join(questionDirectory, "Questions.txt");
const questionProgramsPath = path.join(questionDirectory, "ENGINEERING_QUESTION_PROGRAMS.json");
const preformalHarnessPath = path.join(
  questionDirectory,
  "PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS.md",
);
const searchAsymmetryPath = path.join(questionDirectory, "PREFORMAL_SEARCH_ASYMMETRY.md");
const consolidatedHarnessPath = path.join(
  questionDirectory,
  "SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md",
);
const residualObligationsPath = path.join(questionDirectory, "RESIDUAL_OBLIGATIONS.json");
const explorationAlgorithmPath = path.join(
  questionDirectory,
  "QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md",
);
const environment = { ...process.env, CLAUDE_PROJECT_DIR: sandbox };

function fileDigest(filePath) {
  return crypto.createHash("sha256").update(fs.readFileSync(filePath)).digest("hex");
}

function unique(values) {
  return [...new Set(values)];
}

function derivedProgramFields(residualClass = "default") {
  const manifest = JSON.parse(fs.readFileSync(questionProgramsPath, "utf8"));
  const harness = manifest.preformal_harness;
  const rhythmId = harness.residual_schedule[residualClass];
  const rhythm = harness.principal_rhythms.find((candidate) => candidate.id === rhythmId);
  const questionMap = new Map(harness.compiled_questions.map((question) => [question.id, question]));
  const challengeMap = new Map(
    harness.reciprocal_challenges.map((challenge) => [challenge.id, challenge]),
  );
  const questions = rhythm.required_questions.map((id) => questionMap.get(id));
  const challenges = rhythm.required_reciprocals.map((id) => challengeMap.get(id));
  const rootSet = new Set([
    ...questions.flatMap((question) => question.roots),
    ...challenges.flatMap((challenge) => challenge.roots),
  ]);
  const dimensionSet = new Set(questions.flatMap((question) => question.dimensions));
  const axisSet = new Set(challenges.flatMap((challenge) => challenge.axes));
  return {
    program: manifest.composition.id,
    rhythm: rhythm.id,
    residual_class: residualClass,
    compiled_questions: rhythm.required_questions.join(","),
    question_families: unique(questions.map((question) => question.family)).join(","),
    coding_questions: unique(questions.flatMap((question) => question.source_lines)).join(","),
    coverage_dimensions: harness.coverage_dimensions
      .filter((dimension) => dimensionSet.has(dimension))
      .join(","),
    root_spans: harness.root_hypothesis.filter((root) => rootSet.has(root)).join(","),
    rhythm_positions: unique(questions.map((question) => question.position)).join(","),
    reciprocal_status: "represented",
    reciprocal_challenges: rhythm.required_reciprocals.join(","),
    blocked_reciprocals: "none",
    reciprocal_pairs: challenges
      .map((challenge) => `${challenge.pair[0]}:${challenge.pair[1]}`)
      .join(";"),
    reciprocal_axes: harness.central_reciprocal_axes
      .map((axis) => axis.id)
      .filter((axis) => axisSet.has(axis))
      .join(","),
  };
}

function questionArgs(overrides = {}) {
  const derived = derivedProgramFields(overrides.residual_class ?? "default");
  const fields = {
    q: "which residual-selected relational moves and reciprocal challenges select the continuation?",
    mode: "Check",
    answer: "supported paired inquiry",
    branch: "continue",
    occurrence: "ask-program-1",
    continuation: "k-program-1",
    bindings: "finite-harness",
    horizon: "one isolated question program",
    coverage: "residual-selected preformal rhythm and declared relational spans",
    authority: "AGENTS question-program contract",
    evidence: "isolated harness return",
    ...derived,
    reciprocal_reason:
      "each residual-required reciprocal motion is represented by an opposed corpus challenge",
    parent_residual: "FORMAL-A-INVENTORY-001",
    condition_ids: "C-PINNED-INPUTS,C-PREDECESSOR-FREEZE",
    breaker_ids: "BRK-HARNESS-STRUCTURAL",
    reciprocal_obligation: "represented",
    question_disposition: "Productive",
    residual_shape: "Generic",
    method_frontier: "NondominatedApplicableMethods",
    condition_keys: "harness.relation@fixture@isolated@applicable@record@forward",
    source_digest: fileDigest(questionSourcePath),
    program_manifest_digest: fileDigest(questionProgramsPath),
    ...overrides,
  };
  return Object.entries(fields).map(([key, value]) => `${key}=${value}`);
}

function questionRecord(overrides = {}) {
  return Object.fromEntries(
    questionArgs(overrides).map((entry) => {
      const split = entry.indexOf("=");
      return [entry.slice(0, split), entry.slice(split + 1)];
    }),
  );
}

function execute(file, args = [], input = "") {
  return spawnSync(file, args, {
    cwd: sandbox,
    env: environment,
    input,
    encoding: "utf8",
    windowsHide: true,
  });
}

function hook(name, args = [], input = "") {
  return execute(bash, [path.join(hooks, name), ...args], input);
}

function runner(mode, input = "") {
  return execute(process.execPath, [path.join(hooks, "ic-run.js"), mode], input);
}

function requireSuccess(result, label) {
  assert.equal(
    result.status,
    0,
    `${label} failed:\n${result.stdout ?? ""}${result.stderr ?? ""}`,
  );
}

function requireFailure(result, label, pattern) {
  assert.notEqual(result.status, 0, `${label} unexpectedly succeeded`);
  if (pattern) {
    assert.match(`${result.stdout ?? ""}${result.stderr ?? ""}`, pattern, label);
  }
}

function guard(payload) {
  return runner("guard", `${JSON.stringify(payload)}\n`);
}

function assertGuardAllowed(payload, label) {
  const result = guard(payload);
  requireSuccess(result, label);
  assert.equal(result.stdout, "", `${label} was unexpectedly denied: ${result.stdout}`);
}

function assertGuardDenied(payload, label, pattern) {
  const result = guard(payload);
  requireSuccess(result, label);
  assert.notEqual(result.stdout, "", `${label} unexpectedly passed the guard`);
  const response = JSON.parse(result.stdout);
  assert.equal(
    response?.hookSpecificOutput?.permissionDecision,
    "deny",
    `${label} did not fail closed`,
  );
  if (pattern) {
    assert.match(
      response.hookSpecificOutput.permissionDecisionReason ?? "",
      pattern,
      label,
    );
  }
}

function assertMalformedGuardDenied(input, label) {
  const result = runner("guard", input);
  requireSuccess(result, label);
  const response = JSON.parse(result.stdout);
  assert.equal(response?.hookSpecificOutput?.permissionDecision, "deny", label);
  assert.match(
    response.hookSpecificOutput.permissionDecisionReason ?? "",
    /invalid|malformed|missing|unsupported|hook payload/i,
    label,
  );
}

function frontierFields(overrides = {}) {
  return {
    id: "HARNESS-001",
    plan_phase: "test",
    goal: "exercise the isolated harness",
    protected_difference: "authorized mutation versus an unsealed or unscoped mutation",
    discriminator: "adversarial hook return",
    horizon: "one isolated finite harness run",
    relevant_decisions: "none",
    relevant_failures: "none",
    if_pass: "HARNESS-002",
    if_fail: "repair the earliest failed control relation",
    ...overrides,
  };
}

function writeFrontier(fields = frontierFields(), extraLines = []) {
  const lines = ["# Frontier", "", "<!-- LIVE_FRONTIER_BEGIN -->"];
  for (const key of frontierKeys) {
    if (Object.hasOwn(fields, key)) lines.push(`${key}: ${fields[key]}`);
  }
  lines.push(...extraLines, "<!-- LIVE_FRONTIER_END -->", "");
  fs.writeFileSync(frontierPath, lines.join("\n"));
}

function trace(kind, fields = []) {
  const keys = new Set(fields.map((entry) => entry.slice(0, entry.indexOf("="))));
  const completed = [...fields];
  if (kind === "seal" && !keys.has("coverage")) {
    completed.push("coverage=isolated finite harness fixture");
  }
  if (kind === "residual") {
    const defaults = {
      next: "continue isolated harness fixture",
      parent_residual: "FORMAL-A-INVENTORY-001",
      open_relation: "isolated harness control relation",
      condition_ids: "C-PINNED-INPUTS,C-PREDECESSOR-FREEZE",
      condition_keys: "harness.relation@fixture@isolated@applicable@record@forward",
      blocker_ids: "none",
      breaker_ids: "BRK-HARNESS-STRUCTURAL",
      separator_ids: "SEP-HARNESS-CONTROL",
      survived_contrast_ids: "CTR-HARNESS-AUTHORIZED",
      conflict_ids: "none",
      gap_ids: "none",
      failed_fold_ids: "none",
      reopen_condition_ids: "REOPEN-HARNESS-POLICY-CHANGE",
      overlap_ids: "none",
      coverage: "isolated finite harness fixture",
      resolution_class: "Supported",
      residual_shape: "Generic",
      method_frontier: "NondominatedApplicableMethods",
      next_question_family: "Q14_QUESTION_PRUNE_INVENT",
    };
    for (const [key, value] of Object.entries(defaults)) {
      if (!keys.has(key)) completed.push(`${key}=${value}`);
    }
  }
  return hook("ic-trace", [kind, ...completed]);
}

function appendConcurrently(program, tracePath, records) {
  return Promise.all(
    records.map(
      (record) =>
        new Promise((resolve) => {
          const child = spawn(process.execPath, [program, "append", tracePath], {
            cwd: sandbox,
            env: environment,
            stdio: ["pipe", "pipe", "pipe"],
            windowsHide: true,
          });
          let stdout = "";
          let stderr = "";
          child.stdout.setEncoding("utf8");
          child.stderr.setEncoding("utf8");
          child.stdout.on("data", (chunk) => (stdout += chunk));
          child.stderr.on("data", (chunk) => (stderr += chunk));
          child.on("close", (status) => resolve({ status, stdout, stderr }));
          child.stdin.end(`${JSON.stringify(record)}\n`);
        }),
    ),
  );
}

function validateMustReject(program, name, bytes, pattern) {
  const tracePath = path.join(sandbox, name);
  fs.writeFileSync(tracePath, bytes);
  const result = execute(process.execPath, [program, "validate", tracePath]);
  requireFailure(result, `malformed trace ${name}`, pattern);
}

async function main() {
  fs.mkdirSync(hooks, { recursive: true });
  fs.mkdirSync(questionDirectory, { recursive: true });
  fs.cpSync(path.join(repository, ".claude", "hooks"), hooks, { recursive: true });
  fs.copyFileSync(path.join(repository, ".claude", "settings.json"), settingsPath);
  fs.copyFileSync(
    path.join(repository, "formal-successor", "Questions.txt"),
    questionSourcePath,
  );
  fs.copyFileSync(
    path.join(repository, "formal-successor", "ENGINEERING_QUESTION_PROGRAMS.json"),
    questionProgramsPath,
  );
  fs.copyFileSync(
    path.join(repository, "formal-successor", "PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS.md"),
    preformalHarnessPath,
  );
  fs.copyFileSync(
    path.join(repository, "formal-successor", "PREFORMAL_SEARCH_ASYMMETRY.md"),
    searchAsymmetryPath,
  );
  fs.copyFileSync(
    path.join(repository, "formal-successor", "SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md"),
    consolidatedHarnessPath,
  );
  fs.copyFileSync(
    path.join(repository, "formal-successor", "RESIDUAL_OBLIGATIONS.json"),
    residualObligationsPath,
  );
  fs.copyFileSync(
    path.join(repository, "formal-successor", "QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md"),
    explorationAlgorithmPath,
  );
  writeFrontier();

  // The committed settings must route every configured hook through the tested
  // cross-platform launcher, not around it to an untested shell path.
  const settings = JSON.parse(fs.readFileSync(settingsPath, "utf8"));
  const configured = [
    ["PreToolUse", "guard"],
    ["UserPromptSubmit", "inject"],
    ["Stop", "stop"],
  ];
  for (const [event, mode] of configured) {
    const command = settings.hooks[event][0].hooks[0].command;
    assert.match(command, /ic-run\.js/iu, `${event} must use ic-run.js`);
    assert.match(command, new RegExp(`${mode}$`, "u"), `${event} must select ${mode}`);
  }

  requireSuccess(trace("init", ["test"]), "trace init through copied hook");
  const initializedTrace = path.join(
    traceDirectory,
    fs.readFileSync(path.join(traceDirectory, ".state"), "utf8"),
  );
  const initializedPolicy = JSON.parse(
    fs.readFileSync(initializedTrace, "utf8").trimEnd().split("\n")[0],
  );
  assert.deepEqual(
    {
      kind: initializedPolicy.kind,
      schema: initializedPolicy.question_program_schema,
      source: initializedPolicy.source_digest,
      programs: initializedPolicy.program_manifest_digest,
    },
    {
      kind: "policy",
      schema: "3",
      source: fileDigest(questionSourcePath),
      programs: fileDigest(questionProgramsPath),
    },
    "trace init must pin the active question corpus and program manifest",
  );
  requireSuccess(
    trace("ensure", ["task=test", "authority=user", "invariants=none"]),
    "ensure append",
  );
  const questionProgramValidator = path.join(hooks, "ic-question-program.js");
  const scheduledResiduals = Object.keys(
    JSON.parse(fs.readFileSync(questionProgramsPath, "utf8")).preformal_harness.residual_schedule,
  );
  for (const residualClass of scheduledResiduals) {
    const result = execute(process.execPath, [
      questionProgramValidator,
      "validate",
      sandbox,
      JSON.stringify(questionRecord({ residual_class: residualClass })),
    ]);
    requireSuccess(result, `residual-selected rhythm ${residualClass}`);
  }

  // A small exact safe set remains probeable before a seal. Commands with
  // mutating options must not inherit safety merely from their executable name.
  for (const command of ["pwd", "rg --files", "git status --short", "git diff --no-ext-diff -- ."]) {
    assertGuardAllowed(
      { tool_name: "Bash", tool_input: { command } },
      `safe command ${command}`,
    );
  }
  for (const command of [
    "find . -delete",
    "git branch harness-bypass",
    "git remote add harness-bypass https://example.invalid/repo",
    "git show HEAD:AGENTS.md --output=AGENTS.md",
    "cargo test --workspace",
    "rg --pre touch --files",
  ]) {
    assertGuardDenied(
      { tool_name: "Bash", tool_input: { command } },
      `mutating command ${command}`,
      /No open inquiry cycle/i,
    );
  }

  assertGuardDenied(
    { tool_name: "Edit", tool_input: { file_path: "src/lib.rs" } },
    "ordinary edit without a seal",
    /No open inquiry cycle/i,
  );
  assertGuardDenied(
    { tool_name: "Bash", tool_input: { command: "bash /tmp/foreign/ic-trace status" } },
    "foreign same-named ic-trace",
    /trace|repository|direct/i,
  );
  assertGuardAllowed(
    { tool_name: "Bash", tool_input: { command: ".claude/hooks/ic-trace status" } },
    "exact repository trace command",
  );
  assertGuardAllowed(
    {
      tool_name: "Bash",
      tool_input: {
        command:
          ".claude/hooks/ic-trace policy-transition authority=user reason=controlled",
      },
    },
    "exact repository policy-transition trace command",
  );
  assertGuardDenied(
    {
      tool_name: "Bash",
      tool_input: { command: ".claude/hooks/ic-trace note text=x & echo bypass" },
    },
    "compound trace command",
    /append-only|trace/i,
  );
  assertGuardDenied(
    {
      tool_name: "Bash",
      tool_input: { command: "node .claude/hooks/ic-append.js validate forged" },
    },
    "direct append-helper invocation",
    /internal trace primitive|append/i,
  );

  // Hook input is an authority boundary. Missing, malformed, or unsupported
  // structure must produce an explicit denial, never the default allow branch.
  assertMalformedGuardDenied("not-json\n", "non-JSON guard payload");
  assertMalformedGuardDenied("{}\n", "empty guard payload");
  assertMalformedGuardDenied(
    `${JSON.stringify({ tool_name: "Bash", tool_input: {} })}\n`,
    "missing Bash command",
  );
  assertMalformedGuardDenied(
    `${JSON.stringify({ tool_name: "FutureWrite", tool_input: {} })}\n`,
    "unknown mutation tool",
  );

  // The trace is an ordered protocol, not merely a collection of individually
  // well-shaped records.
  requireFailure(
    trace("raw", ["cmd=too-early", `digest=${"0".repeat(64)}`, "sensitive=true"]),
    "raw before seal",
  );
  requireFailure(trace("check", ["verdict=too-early", "coverage=none"]), "check before seal");
  requireFailure(trace("residual", ["class=none"]), "residual before seal");
  requireFailure(trace("stop", ["state=Unknown", "warrant=none"]), "stop before seal");

  requireSuccess(
    trace("seal", [
      "should_change=test",
      "invariants=trace",
      "discriminator=guard",
      "wrong_impl=bypass",
    ]),
    "seal append",
  );
  requireFailure(trace("init", ["must-not-replace-open"]), "init while cycle open", /open/i);
  requireFailure(trace("check", ["verdict=too-early", "coverage=none"]), "check before raw");
  requireFailure(trace("residual", ["class=none"]), "residual before raw/check");
  requireFailure(
    trace("stop", ["state=Unknown", "warrant=check:test"]),
    "stop before raw/check/residual",
  );
  requireSuccess(
    trace("question", questionArgs({ occurrence: "ask-program-before-return" })),
    "valid question before the actual return",
  );

  let result = runner("stop", `${JSON.stringify({ stop_hook_active: false })}\n`);
  requireSuccess(result, "stop launcher while cycle open");
  assert.match(result.stdout, /cycle is still open/i);

  // Routine project-state ratchets need an open prediction but not a fabricated
  // control-authority grant. Constitutional/control surfaces do require exact scope.
  for (const filePath of [
    "README.md",
    "IMPLEMENTATION_FRONTIER.md",
    "CONFORMANCE_STATUS.md",
    "DECISIONS.jsonl",
    "FAILURES.jsonl",
  ]) {
    assertGuardAllowed(
      { tool_name: "Edit", tool_input: { file_path: filePath } },
      `routine state document ${filePath}`,
    );
  }

  const protectedFiles = [
    ["Inquiry_Calculus_v2_0.tex", "canonical"],
    ["Inquiry_Calculus_v2_0_Comprehensive_Implementation_Plan.md", "plan"],
    ["AGENTS.md", "agents"],
    [".claude/hooks/ic-guard", "harness"],
    [".claude/hooks/ic-question-program.js", "harness"],
    [".claude/settings.json", "harness"],
    [".claude/skills/inquire/SKILL.md", "harness"],
    [".github/workflows/ci.yml", "ci"],
    ["tools/docs_control_check.py", "ci"],
    ["tools/harness_control_check.js", "ci"],
  ];
  for (const [filePath] of protectedFiles) {
    assertGuardDenied(
      { tool_name: "Edit", tool_input: { file_path: filePath } },
      `unscoped protected file ${filePath}`,
      /protected authority file|control/i,
    );
  }
  assertGuardDenied(
    {
      tool_name: "Bash",
      tool_input: { command: "git show HEAD:AGENTS.md --output=AGENTS.md" },
    },
    "protected output hidden behind git show",
    /protected authority file|control/i,
  );

  requireSuccess(
    trace("control", [
      "authority=user-explicit-test",
      "residual=harness-test",
      "predecessor=current",
      "scope=not-canonical",
    ]),
    "near-match control append",
  );
  assertGuardDenied(
    { tool_name: "Edit", tool_input: { file_path: "Inquiry_Calculus_v2_0.tex" } },
    "near-match control scope",
    /protected authority file|control/i,
  );
  requireSuccess(
    trace("control", [
      "authority=user-explicit-test",
      "residual=harness-test",
      "predecessor=current",
      "scope=canonical,plan,agents,harness,ci",
    ]),
    "exact multi-scope control append",
  );
  for (const [filePath] of protectedFiles) {
    assertGuardAllowed(
      { tool_name: "Edit", tool_input: { file_path: filePath } },
      `authorized protected file ${filePath}`,
    );
  }
  assertGuardDenied(
    { tool_name: "Edit", tool_input: { file_path: ".claude/trace/forged.jsonl" } },
    "direct trace rewrite",
    /append-only|trace/i,
  );

  fs.writeFileSync(path.join(sandbox, "raw-return.txt"), "actual return\n");
  requireSuccess(
    trace("raw", ["cmd=fixture", "file=raw-return.txt", "sensitive=false"]),
    "raw append",
  );
  requireFailure(trace("residual", ["class=none"]), "residual before check");
  requireSuccess(
    trace("check", ["verdict=matches-seal", "coverage=isolated-harness"]),
    "check append",
  );
  requireFailure(
    trace("residual", ["class=none"]),
    "residual without a post-return engineering question program",
    /question program/i,
  );
  requireFailure(
    trace(
      "question",
      questionArgs().filter((entry) => !entry.startsWith("program=")),
    ),
    "question missing composed program identity",
    /program=/i,
  );
  requireFailure(
    trace("question", questionArgs({ coding_questions: "5554" })),
    "reciprocal question substituted for coding question",
    /coding_questions/i,
  );
  requireFailure(
    trace(
      "question",
      questionArgs({
        coding_questions: derivedProgramFields().coding_questions.replace("1251", "1035"),
      }),
    ),
    "surface rewording substituted for the compiled role/port relation",
    /coding_questions/i,
  );
  requireFailure(
    trace("question", questionArgs({ reciprocal_pairs: "5942" })),
    "one-sided reciprocal question",
    /two-orientation pairs/i,
  );
  requireFailure(
    trace(
      "question",
      questionArgs({
        reciprocal_challenges: "RCP-ORIENT-REVERSE",
        blocked_reciprocals: "none",
        reciprocal_pairs: "5942:5944",
        reciprocal_axes: "none",
      }),
    ),
    "one reciprocal movement substituted for the residual-required closure",
    /reciprocal closure/i,
  );
  requireFailure(
    trace("question", questionArgs({ reciprocal_pairs: "5942:5946" })),
    "undeclared reciprocal composition",
    /reciprocal_pairs/i,
  );
  requireFailure(
    trace(
      "question",
      questionArgs({
        residual_class: "relational_roles_unclear",
        rhythm: "RHYTHM-DEFAULT-SUCCESSOR-CONSTRUCTION",
      }),
    ),
    "predecessor-independent residual schedule bypass",
    /requires rhythm RHYTHM-FRAME/i,
  );
  requireFailure(
    trace(
      "question",
      questionArgs({
        rhythm: "RHYTHM-BIND-OPEN-VARY-RETURN-DETERMINE-REFACTOR",
      }),
    ),
    "predecessor recurrence substituted for the preformal successor rhythm",
    /requires rhythm RHYTHM-DEFAULT-SUCCESSOR-CONSTRUCTION/i,
  );
  requireFailure(
    trace(
      "question",
      questionArgs({
        coverage_dimensions: "ADMISSIBILITY,DISCRIMINATION",
      }),
    ),
    "prompt count substituted for relational-span coverage",
    /coverage_dimensions/i,
  );
  const defaultProgram = derivedProgramFields();
  const representedChallenges = defaultProgram.reciprocal_challenges.split(",");
  const representedPairs = defaultProgram.reciprocal_pairs.split(";");
  const blockedIndex = representedChallenges.indexOf("RCP-ORIENT-REVERSE");
  requireSuccess(
    trace(
      "question",
      questionArgs({
        q: "is one named reciprocal challenge executable under this binding?",
        answer: "one typed challenge is blocked while the rest are represented",
        occurrence: "ask-program-partially-blocked-reciprocal",
        reciprocal_status: "partially_blocked",
        reciprocal_challenges: representedChallenges
          .filter((_, index) => index !== blockedIndex)
          .join(","),
        blocked_reciprocals: "RCP-ORIENT-REVERSE",
        reciprocal_pairs: representedPairs.filter((_, index) => index !== blockedIndex).join(";"),
        reciprocal_reason: "the binding admits no reverse-direction probe but all other challenges execute",
      }),
    ),
    "validated individually blocked reciprocal challenge",
  );
  requireFailure(
    trace(
      "question",
      questionArgs({
        reciprocal_status: "blocked",
        reciprocal_challenges: "none",
        blocked_reciprocals: defaultProgram.reciprocal_challenges,
        reciprocal_pairs: "5942:5944",
        reciprocal_axes: "none",
        reciprocal_reason: "claimed inapplicable",
      }),
    ),
    "blocked reciprocal set with a fabricated pair",
    /reciprocal_pairs/i,
  );
  requireSuccess(trace("question", questionArgs()), "validated coding/reciprocal program");
  requireSuccess(
    trace(
      "question",
      questionArgs({
        q: "are the required reciprocal challenge relations executable under this binding?",
        answer: "typed blocked challenge set",
        occurrence: "ask-program-blocked-reciprocals",
        reciprocal_status: "blocked",
        reciprocal_challenges: "none",
        blocked_reciprocals: defaultProgram.reciprocal_challenges,
        reciprocal_pairs: "none",
        reciprocal_axes: "none",
        root_spans: unique(
          derivedProgramFields().root_spans
            .split(",")
            .filter((root) =>
              new Set(
                JSON.parse(fs.readFileSync(questionProgramsPath, "utf8"))
                  .preformal_harness.compiled_questions
                  .filter((question) => defaultProgram.compiled_questions.split(",").includes(question.id))
                  .flatMap((question) => question.roots),
              ).has(root),
            ),
        ).join(","),
        reciprocal_reason: "every named challenge is typed but unavailable to this isolated binding",
      }),
    ),
    "validated explicit per-challenge reciprocal blocking",
  );
  requireFailure(
    trace("stop", ["state=Satisfied", "warrant=independent-check"]),
    "stop before residual",
  );
  requireSuccess(trace("residual", ["class=none"]), "residual append");
  requireFailure(trace("init", ["must-not-skip-stop"]), "init before stop");
  requireFailure(
    trace("stop", ["state=Satisfied", "warrant=none"]),
    "Satisfied with a self/empty warrant",
    /warrant/i,
  );
  requireSuccess(
    trace("stop", ["state=Satisfied", "warrant=independent-check:isolated-harness"]),
    "warranted stop append",
  );
  const detachedRecords = fs
    .readFileSync(initializedTrace, "utf8")
    .trimEnd()
    .split("\n")
    .map(JSON.parse);
  detachedRecords[0].source_digest = "f".repeat(64);
  validateMustReject(
    path.join(hooks, "ic-append.js"),
    "detached-question-policy.jsonl",
    `${detachedRecords.map(JSON.stringify).join("\n")}\n`,
    /detached from the active trace policy/i,
  );
  result = runner("stop", `${JSON.stringify({ stop_hook_active: false })}\n`);
  requireSuccess(result, "stop launcher after closure");
  assert.equal(result.stdout, "", "closed cycle must not block Stop");
  result = runner("stop", `${JSON.stringify({ stop_hook_active: true })}\n`);
  requireSuccess(result, "recursive stop launcher");
  assert.equal(result.stdout, "", "active Stop hook must yield without recursion");

  // A user-authorized, pre-return policy transition permits the question
  // program to evolve without detaching earlier question occurrences from the
  // policy that checked them.
  requireSuccess(trace("init", ["policy-transition"]), "policy-transition trace init");
  requireSuccess(
    trace("ensure", ["task=policy-transition", "authority=user", "invariants=ancestry"]),
    "policy-transition ensure",
  );
  requireSuccess(
    trace("control", [
      "authority=user-explicit-test",
      "residual=question-program-evolution",
      "predecessor=current-policy",
      "scope=harness",
    ]),
    "policy-transition control",
  );
  requireSuccess(
    trace("seal", [
      "should_change=program-digest",
      "invariants=prior-question-policy",
      "discriminator=transition-ancestry",
      "wrong_impl=silent-policy-replacement",
    ]),
    "policy-transition seal",
  );
  const transitionedManifest = JSON.parse(fs.readFileSync(questionProgramsPath, "utf8"));
  transitionedManifest.harness_transition_probe = "retained";
  fs.writeFileSync(questionProgramsPath, `${JSON.stringify(transitionedManifest, null, 2)}\n`);
  requireSuccess(
    trace("policy-transition", [
      "authority=user-explicit-test",
      "reason=exercise exact question-program policy ancestry",
    ]),
    "authorized policy transition",
  );
  requireSuccess(
    trace("raw", ["cmd=transition-fixture", "file=raw-return.txt", "sensitive=false"]),
    "policy-transition raw",
  );
  requireSuccess(
    trace("question", questionArgs({ occurrence: "ask-after-policy-transition" })),
    "question under transitioned policy",
  );
  requireSuccess(
    trace("check", ["verdict=transition-preserved-ancestry", "coverage=one-policy-change"]),
    "policy-transition check",
  );
  requireSuccess(trace("residual", ["class=none"]), "policy-transition residual");
  requireSuccess(
    trace("stop", ["state=Unknown", "warrant=check:policy-transition-harness"]),
    "policy-transition stop",
  );

  // Composition is per actual return, not merely one question somewhere in a
  // cycle containing arbitrarily many returns.
  requireSuccess(trace("init", ["multi-return"]), "multi-return trace init");
  requireSuccess(
    trace("ensure", ["task=multi-return", "authority=user", "invariants=none"]),
    "multi-return ensure",
  );
  requireSuccess(
    trace("seal", [
      "should_change=composition-count",
      "invariants=trace",
      "discriminator=per-return-composition",
      "wrong_impl=one-question-for-many-returns",
    ]),
    "multi-return seal",
  );
  requireSuccess(
    trace("raw", ["cmd=fixture-one", "file=raw-return.txt", "sensitive=false"]),
    "first multi-return raw append",
  );
  requireSuccess(
    trace("raw", ["cmd=fixture-two", "file=raw-return.txt", "sensitive=false"]),
    "second multi-return raw append",
  );
  requireSuccess(
    trace("question", questionArgs({ occurrence: "ask-multi-return-1" })),
    "first multi-return question",
  );
  requireSuccess(
    trace("check", ["verdict=one-return-composed", "coverage=two-returns"]),
    "multi-return check",
  );
  requireFailure(
    trace("residual", ["class=none"]),
    "one question cannot compose two actual returns",
    /question program/i,
  );
  requireSuccess(
    trace("question", questionArgs({ occurrence: "ask-multi-return-2" })),
    "second multi-return question",
  );
  requireSuccess(trace("residual", ["class=none"]), "multi-return residual");
  requireSuccess(
    trace("stop", ["state=Unknown", "warrant=check:multi-return-harness"]),
    "multi-return stop",
  );

  // Normalization prevents equivalent map/order spellings from simulating a
  // new recurrence state, while a new occurrence remains distinct.
  requireSuccess(trace("init", ["normalized-coordinate"]), "normalized trace init");
  requireSuccess(
    trace("ensure", ["task=normalization", "authority=user", "invariants=none"]),
    "normalization ensure",
  );
  const firstQuestion = questionArgs({
    q: "q-test",
    mode: "Pure",
    answer: "same-answer",
    branch: "continue",
    occurrence: "ask-1",
    continuation: "k-1",
    bindings: '{"b":2,"a":1}',
    horizon: "finite",
    coverage: "exact",
    authority: '{"version":2,"source":"AGENTS"}',
    evidence: '{"b":2,"a":1}',
  });
  requireSuccess(trace("question", firstQuestion), "first normalized question");
  const reorderedQuestion = firstQuestion.map((entry) => {
    if (entry.startsWith("bindings=")) return 'bindings={ "a": 1, "b": 2 }';
    if (entry.startsWith("authority=")) return 'authority={"source":"AGENTS","version":2}';
    if (entry.startsWith("evidence=")) return 'evidence={"a":1,"b":2}';
    return entry;
  });
  requireFailure(
    trace("question", reorderedQuestion),
    "equivalent normalized recurrence",
    /repeated state/i,
  );
  assert.equal(fs.readFileSync(path.join(traceDirectory, ".fuel"), "utf8"), "23");
  const distinctOccurrence = reorderedQuestion.map((entry) =>
    entry === "occurrence=ask-1" ? "occurrence=ask-2" : entry,
  );
  requireSuccess(trace("question", distinctOccurrence), "distinct occurrence question");
  assert.equal(fs.readFileSync(path.join(traceDirectory, ".fuel"), "utf8"), "22");

  // The launcher-visible projection must contain the one canonical ten-field
  // block, and must expose every malformed variant rather than guessing.
  result = runner("inject", "{}\n");
  requireSuccess(result, "valid frontier injection through launcher");
  const injected = JSON.parse(result.stdout).hookSpecificOutput.additionalContext;
  for (const [key, value] of Object.entries(frontierFields())) {
    assert.ok(injected.includes(`${key}: ${value}`), `injection omitted ${key}`);
  }
  for (const fragment of [
    "QUESTION PROGRAM RHYTHM-DEFAULT-SUCCESSOR-CONSTRUCTION",
    "families: Q1,Q2,Q3,Q4,Q5,Q6,Q7,Q8,Q9,Q10,Q11,Q12,Q13,Q14",
    "CONSTRAIN<->RELEASE",
    "DISTINGUISH<->COARSEN",
    "selection: residual-selected",
    "RESIDUAL INDEX",
    "active: HARNESS-001",
    "closure: local obligation/binding/horizon/coverage only",
  ]) {
    assert.ok(injected.includes(fragment), `injection omitted question rhythm ${fragment}`);
  }

  const missing = frontierFields();
  delete missing.if_fail;
  writeFrontier(missing);
  result = runner("inject", "{}\n");
  requireSuccess(result, "missing-field frontier injection");
  assert.match(result.stdout, /INVALID: .*live frontier/i);

  writeFrontier(frontierFields(), ["unexpected: value"]);
  result = runner("inject", "{}\n");
  requireSuccess(result, "unknown-field frontier injection");
  assert.match(result.stdout, /INVALID: .*live frontier/i);

  writeFrontier(frontierFields(), ["id: DUPLICATE"]);
  result = runner("inject", "{}\n");
  requireSuccess(result, "duplicate-field frontier injection");
  assert.match(result.stdout, /INVALID: .*live frontier/i);
  writeFrontier();

  const appendProgram = path.join(hooks, "ic-append.js");
  const concurrentTrace = path.join(sandbox, "concurrent.jsonl");
  fs.writeFileSync(concurrentTrace, "");
  const concurrent = await appendConcurrently(
    appendProgram,
    concurrentTrace,
    Array.from({ length: 12 }, (_, index) => ({
      kind: "note",
      ts: "2026-08-26T00:00:00Z",
      text: `note-${index}`,
    })),
  );
  assert.ok(concurrent.every((entry) => entry.status === 0), JSON.stringify(concurrent));
  requireSuccess(
    execute(process.execPath, [appendProgram, "validate", concurrentTrace]),
    "concurrent validation",
  );
  const concurrentRecords = fs
    .readFileSync(concurrentTrace, "utf8")
    .trimEnd()
    .split("\n")
    .map(JSON.parse);
  assert.deepEqual(
    concurrentRecords.map((record) => record.seq),
    Array.from({ length: 12 }, (_, index) => index + 1),
  );
  assert.deepEqual(
    concurrentRecords.map((record) => record.parent),
    Array.from({ length: 12 }, (_, index) => index),
  );

  const fuelTrace = path.join(sandbox, "fuel.jsonl");
  const fuel = path.join(sandbox, "fuel");
  fs.writeFileSync(fuelTrace, "");
  fs.writeFileSync(fuel, "2");
  const question = {
    kind: "question",
    ts: "2026-08-26T00:00:00Z",
    fp: "same-state",
    ...questionRecord({ answer: "same-answer" }),
  };
  result = execute(
    process.execPath,
    [appendProgram, "append", fuelTrace, fuel],
    `${JSON.stringify(question)}\n`,
  );
  requireSuccess(result, "first direct question append");
  result = execute(
    process.execPath,
    [appendProgram, "append", fuelTrace, fuel],
    `${JSON.stringify(question)}\n`,
  );
  requireFailure(result, "duplicate direct state", /repeated state/i);
  assert.equal(fs.readFileSync(fuel, "utf8"), "1", "duplicate must not consume fuel");
  const secondQuestion = { ...question, fp: "next-state" };
  result = execute(
    process.execPath,
    [appendProgram, "append", fuelTrace, fuel],
    `${JSON.stringify(secondQuestion)}\n`,
  );
  requireSuccess(result, "second direct question append");
  assert.equal(fs.readFileSync(fuel, "utf8"), "0");
  const thirdQuestion = { ...question, fp: "third-state" };
  result = execute(
    process.execPath,
    [appendProgram, "append", fuelTrace, fuel],
    `${JSON.stringify(thirdQuestion)}\n`,
  );
  requireFailure(result, "exhausted direct fuel", /fuel exhausted/i);

  validateMustReject(appendProgram, "invalid-json.jsonl", "not-json\n", /valid JSON/i);
  validateMustReject(
    appendProgram,
    "fused-records.jsonl",
    '{"seq":1,"parent":0,"kind":"note"}{"seq":2,"parent":1,"kind":"note"}\n',
    /valid JSON/i,
  );
  validateMustReject(
    appendProgram,
    "duplicate-sequence.jsonl",
    '{"seq":1,"parent":0,"kind":"note"}\n{"seq":1,"parent":1,"kind":"note"}\n',
    /seq=2/i,
  );
  validateMustReject(
    appendProgram,
    "wrong-parent.jsonl",
    '{"seq":1,"parent":0,"kind":"note"}\n{"seq":2,"parent":0,"kind":"note"}\n',
    /parent=1/i,
  );
  validateMustReject(
    appendProgram,
    "truncated-record.jsonl",
    '{"seq":1,"parent":0,"kind":"note"}',
    /record boundary/i,
  );

  const staleTrace = path.join(sandbox, "stale.jsonl");
  fs.writeFileSync(staleTrace, "");
  const staleLock = `${staleTrace}.lock`;
  fs.writeFileSync(staleLock, `${JSON.stringify({ pid: 2147483647 })}\n`);
  const old = new Date(Date.now() - 60_000);
  fs.utimesSync(staleLock, old, old);
  result = execute(
    process.execPath,
    [appendProgram, "append", staleTrace],
    `${JSON.stringify({ kind: "note", ts: "2026-08-26T00:00:00Z" })}\n`,
  );
  requireSuccess(result, "stale lock recovery");
  assert.equal(fs.existsSync(staleLock), false);

  const corruptLockTrace = path.join(sandbox, "corrupt-lock.jsonl");
  fs.writeFileSync(corruptLockTrace, "");
  const corruptLock = `${corruptLockTrace}.lock`;
  fs.writeFileSync(corruptLock, "truncated-owner");
  fs.utimesSync(corruptLock, old, old);
  result = execute(
    process.execPath,
    [appendProgram, "append", corruptLockTrace],
    `${JSON.stringify({ kind: "note", ts: "2026-08-26T00:00:00Z" })}\n`,
  );
  requireSuccess(result, "corrupt stale lock recovery");
  assert.equal(fs.existsSync(corruptLock), false);

  // End-to-end Stop invocation must fail closed on an unreadable active trace,
  // while the host-controlled recursion flag still prevents a Stop-hook loop.
  fs.writeFileSync(path.join(traceDirectory, ".state"), "malformed-active.jsonl");
  fs.writeFileSync(path.join(traceDirectory, "malformed-active.jsonl"), "not-json\n");
  result = runner(
    "stop",
    `${JSON.stringify({ text: '\"stop_hook_active\":true', stop_hook_active: false })}\n`,
  );
  requireSuccess(result, "malformed active trace stop");
  assert.match(result.stdout, /malformed or unreadable/i);
  result = runner("stop", `${JSON.stringify({ stop_hook_active: true })}\n`);
  requireSuccess(result, "active malformed stop recursion");
  assert.equal(result.stdout, "", "active Stop hook must yield even on malformed ancestry");

  process.stdout.write("harness control checks passed\n");
}

main()
  .catch((error) => {
    process.stderr.write(`${error.stack ?? error.message}\n`);
    process.exitCode = 1;
  })
  .finally(() => {
    const tempRoot = path.resolve(os.tmpdir());
    const resolvedSandbox = path.resolve(sandbox);
    if (
      path.dirname(resolvedSandbox) === tempRoot &&
      path.basename(resolvedSandbox).startsWith("ic-harness-check-")
    ) {
      fs.rmSync(resolvedSandbox, { recursive: true, force: true });
    }
  });
