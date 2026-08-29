#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const root = path.resolve(__dirname, "..");
const grammarPath = path.join(root, "formal-successor", "PREDECESSOR_INVENTORY_GRAMMAR.json");
const inventoryPath = path.join(root, "formal-successor", "PREDECESSOR_INVENTORY.json");

function digest(value) {
  return crypto.createHash("sha256").update(value).digest("hex");
}

function normalizeNewlines(value) {
  return value.replace(/\r\n?/gu, "\n");
}

function normalizeUnit(value) {
  return normalizeNewlines(value)
    .split("\n")
    .map((line) => line.replace(/\s+$/u, ""))
    .join("\n")
    .trim();
}

function git(args, encoding = "utf8") {
  const result = spawnSync("git", args, {
    cwd: root,
    encoding,
    maxBuffer: 128 * 1024 * 1024,
    windowsHide: true,
  });
  if (result.status !== 0) throw new Error(`git ${args.join(" ")} failed: ${(result.stderr || "").trim()}`);
  return result.stdout;
}

function blob(revision, relativePath) {
  return git(["show", `${revision}:${relativePath}`], "buffer");
}

function exactLines(bytes, startLine, endLine) {
  const lines = normalizeNewlines(bytes.toString("utf8")).split("\n");
  return lines.slice(startLine - 1, endLine).join("\n");
}

function expectedCoverage(grammar, texBytes) {
  const lines = normalizeNewlines(texBytes.toString("utf8")).split("\n");
  const begin = lines.findIndex((line) => line.trim() === grammar.canonical_tex.document_begin);
  const end = lines.findIndex((line, index) => index > begin && line.trim() === grammar.canonical_tex.document_end);
  const included = [];
  for (let index = begin + 1; index < end; index += 1) {
    const trimmed = lines[index].trim();
    if (trimmed === "" || trimmed.startsWith("%")) continue;
    included.push(`${index + 1}\0${lines[index].replace(/\s+$/u, "")}`);
  }
  return {
    begin: begin + 1,
    end: end + 1,
    count: included.length,
    digest: digest(included.join("\n")),
    lines,
  };
}

function environmentRanges(lines, environments, firstIndex, lastIndex) {
  const admitted = new Set(environments);
  const stack = [];
  const ranges = [];
  for (let index = firstIndex; index <= lastIndex; index += 1) {
    const begin = lines[index].match(/\\begin\{([^}]+)\}/u);
    if (begin && admitted.has(begin[1])) stack.push({ environment: begin[1], start: index });
    const end = lines[index].match(/\\end\{([^}]+)\}/u);
    if (!end || !admitted.has(end[1])) continue;
    const position = stack.map((item) => item.environment).lastIndexOf(end[1]);
    if (position < 0) continue;
    const opened = stack.splice(position, 1)[0];
    ranges.push({ environment: opened.environment, start: opened.start, end: index });
  }
  return ranges;
}

function lineCovered(index, ranges) {
  return ranges.some((range) => index >= range.start && index <= range.end);
}

function independentTexCandidateStarts(grammar, coverage) {
  const lines = coverage.lines;
  const first = coverage.begin;
  const last = coverage.end - 2;
  const declarations = environmentRanges(lines, grammar.canonical_tex.declaration_environments, first, last);
  const proofs = environmentRanges(lines, ["proof"], first, last);
  const listings = environmentRanges(lines, ["lstlisting"], first, last);
  const displays = environmentRanges(lines, grammar.canonical_tex.display_environments, first, last);
  for (let index = first; index <= last; index += 1) {
    if (lines[index].trim() !== "\\[") continue;
    const close = lines.findIndex((line, candidate) => candidate > index && candidate <= last && line.trim() === "\\]");
    if (close >= 0) {
      displays.push({ environment: "bracket-display", start: index, end: close });
      index = close;
    }
  }
  const listItems = [];
  for (let index = first; index <= last; index += 1) {
    if (/^\s*\\item(?:\[[^\]]*\])?/u.test(lines[index])) listItems.push(index + 1);
  }
  const excluded = [...declarations, ...proofs, ...listings, ...displays];
  const structural = /^\s*(?:\\(?:part|chapter|section|subsection|subsubsection)\*?\{|\\begin\{|\\end\{|\\item\b|\\label\{|\\\[|\\\]|\\(?:toprule|midrule|bottomrule|endhead|caption|centering)\b)/u;
  const narratives = [];
  let paragraphStart = null;
  for (let index = first; index <= last; index += 1) {
    const trimmed = lines[index].trim();
    const unavailable = lineCovered(index, excluded) || trimmed === "" || trimmed.startsWith("%") || structural.test(lines[index]);
    if (unavailable) {
      if (paragraphStart !== null) narratives.push(paragraphStart + 1);
      paragraphStart = null;
    } else if (paragraphStart === null) {
      paragraphStart = index;
    }
  }
  if (paragraphStart !== null) narratives.push(paragraphStart + 1);
  return {
    declarations: declarations.map((range) => range.start + 1),
    displays: displays.filter((range) => !lineCovered(range.start, declarations)).map((range) => range.start + 1),
    listItems,
    narratives,
  };
}

function findPublicDeclarations(relativePath, bytes, grammar) {
  if (!/^crates\/[^/]+\/src\/.*\.rs$/u.test(relativePath)) return [];
  const publicPrefix = /^\s*pub(?:\s*\([^)]*\))?\s+/u;
  const pattern = /^\s*pub(?:\s*\([^)]*\))?\s+(?:(?:async|unsafe|const)\s+)*(struct|enum|trait|type|fn|const|static|mod)\s+(\$?[A-Za-z_][A-Za-z0-9_]*)/u;
  const usePattern = /^\s*pub(?:\s*\([^)]*\))?\s+use\s+([A-Za-z_][A-Za-z0-9_:]*)/u;
  return normalizeNewlines(bytes.toString("utf8"))
    .split("\n")
    .map((line, index) => ({ line, lineNumber: index + 1, match: line.match(pattern), use: line.match(usePattern) }))
    .filter((entry) => publicPrefix.test(entry.line))
    .map((entry) => ({
      path: relativePath,
      line: entry.lineNumber,
      kind: entry.match?.[1] || (entry.use ? "use" : "surface_line"),
      name: entry.match?.[2] || entry.use?.[1] || null,
    }));
}

function sourceKey(item) {
  return `${item.source.path}\0${item.source.start_line}\0${item.source.end_line}\0${item.source_class}`;
}

function inspect(inventory, grammar, grammarBytes) {
  const errors = [];
  const items = Array.isArray(inventory.items) ? inventory.items : [];
  const ids = new Set();
  const sourceKeys = new Map();
  let priorId = "";
  for (const [index, item] of items.entries()) {
    for (const field of grammar.required_item_fields) {
      if (!(field in item)) errors.push(`item ${index} is missing required field ${field}`);
    }
    if (ids.has(item.id)) errors.push(`duplicate id ${item.id}`);
    ids.add(item.id);
    if (priorId && priorId.localeCompare(item.id) > 0) errors.push(`items are not sorted at ${item.id}`);
    priorId = item.id;
    if (!grammar.review_statuses.includes(item.review_status)) errors.push(`${item.id}: invalid review status`);
    if (!grammar.evidence_statuses.includes(item.evidence_status)) errors.push(`${item.id}: invalid evidence status`);
    if (typeof item.destination !== "string" || item.destination === "") errors.push(`${item.id}: empty destination`);
    if (typeof item.disposition !== "string" || item.disposition === "") errors.push(`${item.id}: empty disposition`);
    const key = sourceKey(item);
    if (!sourceKeys.has(key)) sourceKeys.set(key, []);
    sourceKeys.get(key).push(item);
  }

  if (inventory.generated_from?.grammar_sha256 !== digest(grammarBytes)) errors.push("grammar digest is detached from the committed grammar bytes");
  if (inventory.generated_from?.predecessor_commit !== grammar.predecessor_commit) errors.push("predecessor commit differs from the extraction grammar");
  if (inventory.gate_a?.status !== "PENDING") errors.push("inventory attempts to self-promote Gate A");

  const texBytes = fs.readFileSync(path.join(root, grammar.canonical_tex.path));
  if (digest(texBytes) !== grammar.canonical_tex.sha256) errors.push("canonical TeX bytes differ from the extraction grammar");
  const coverage = expectedCoverage(grammar, texBytes);
  if (inventory.coverage?.tex?.document_start_line !== coverage.begin) errors.push("TeX document start coverage differs");
  if (inventory.coverage?.tex?.document_end_line !== coverage.end) errors.push("TeX document end coverage differs");
  if (inventory.coverage?.tex?.nonblank_noncomment_lines !== coverage.count) errors.push("TeX nonblank source coverage count differs");
  if (inventory.coverage?.tex?.ordered_line_sha256 !== coverage.digest) errors.push("TeX ordered source coverage digest differs");

  const candidateStarts = independentTexCandidateStarts(grammar, coverage);
  const declarationItems = items.filter((item) => item.source_class === "tex.declaration");
  const listItems = items.filter((item) => item.source_class === "tex.list_item");
  const displayItems = items.filter((item) => item.source_class === "tex.display");
  const narrativeItems = items.filter((item) => item.source_class === "tex.narrative");
  for (const line of candidateStarts.declarations) {
    const matches = declarationItems.filter((item) => item.source.start_line === line);
    if (matches.length !== 1) errors.push(`TeX declaration at line ${line} has ${matches.length} inventory identities`);
  }
  for (const line of candidateStarts.listItems) {
    const matches = listItems.filter((item) => item.source.start_line === line);
    if (matches.length !== 1) errors.push(`TeX list item at line ${line} has ${matches.length} inventory identities`);
  }
  for (const line of candidateStarts.displays) {
    const matches = displayItems.filter((item) => item.source.start_line === line);
    if (matches.length !== 1) errors.push(`TeX display at line ${line} has ${matches.length} inventory identities`);
  }
  for (const line of candidateStarts.narratives) {
    const matches = narrativeItems.filter((item) => item.source.start_line === line);
    if (matches.length !== 1) errors.push(`TeX narrative at line ${line} has ${matches.length} inventory identities`);
  }

  const repositoryPaths = git(["ls-tree", "-r", "--name-only", grammar.predecessor_commit, "--", ...grammar.repository_surface.roots])
    .split(/\r?\n/u)
    .filter(Boolean)
    .sort();
  const treeEntries = [];
  const baselineBytes = new Map();
  for (const relativePath of repositoryPaths) {
    const bytes = blob(grammar.predecessor_commit, relativePath);
    baselineBytes.set(relativePath, bytes);
    treeEntries.push(`${relativePath}\0${digest(bytes)}`);
    const wholeFile = items.filter((item) => item.category === "repository_file" && item.source.path === relativePath);
    if (wholeFile.length !== 1) errors.push(`${relativePath} has ${wholeFile.length} whole-file inventory identities`);
    else if (wholeFile[0].source.sha256 !== digest(bytes)) errors.push(`${wholeFile[0].id}: whole-file digest differs from the pinned blob`);
  }
  if (inventory.coverage?.repository?.repository_files !== repositoryPaths.length) errors.push("repository file coverage count differs");
  if (inventory.coverage?.repository?.ordered_tree_sha256 !== digest(treeEntries.join("\n"))) errors.push("repository tree coverage digest differs");

  const publicDeclarations = repositoryPaths.flatMap((relativePath) => findPublicDeclarations(relativePath, baselineBytes.get(relativePath), grammar));
  const publicItems = items.filter((item) => item.source_class === "rust.public_item");
  for (const declaration of publicDeclarations) {
    const matches = publicItems.filter(
      (item) => item.source.path === declaration.path && item.source.start_line === declaration.line && item.category === declaration.kind && (declaration.name === null || item.context?.name === declaration.name),
    );
    if (matches.length !== 1) errors.push(`${declaration.path}:${declaration.line} public ${declaration.kind} ${declaration.name} has ${matches.length} identities`);
  }
  if (inventory.coverage?.repository?.public_rust_items !== publicDeclarations.length) errors.push("public Rust surface count differs");

  const conformanceBytes = blob(grammar.predecessor_commit, grammar.conformance_surface.status_path);
  baselineBytes.set(grammar.conformance_surface.status_path, conformanceBytes);
  const registryBytes = blob(grammar.predecessor_commit, grammar.conformance_surface.registry_path);
  baselineBytes.set(grammar.conformance_surface.registry_path, registryBytes);
  const conformanceLines = normalizeNewlines(conformanceBytes.toString("utf8")).split("\n");
  const fixturePattern = new RegExp(`^\\|\\s*\u0060?(${grammar.conformance_surface.fixture_id_pattern})\u0060?(?:\\s+[^|]*)?\\|`, "u");
  const fixtureRows = conformanceLines.flatMap((line, index) => {
    const match = line.match(fixturePattern);
    if (!match) return [];
    const cells = line.split("|").slice(1, -1).map((cell) => cell.trim());
    const statusIndex = cells.findIndex((cell, cellIndex) => cellIndex > 0 && /^(?:PASS|PENDING|Unknown|Blocked|ResourceBounded)$/u.test(cell));
    return [{ fixtureId: match[1], line: index + 1, statusColumn: statusIndex >= 0 ? statusIndex + 1 : null, statusLabel: statusIndex >= 0 ? cells[statusIndex] : null }];
  });
  const registryPattern = /"(Q[A-Z0-9-]+-[0-9]{3})"\s*:\s*\(\s*"([^"]+)"\s*,\s*"([^"]+)"/gu;
  const registryEntries = [...registryBytes.toString("utf8").matchAll(registryPattern)].map((match) => ({ fixtureId: match[1], testPath: match[2], testFunction: match[3] }));
  const fixtureIds = [...new Set([...fixtureRows.map((row) => row.fixtureId), ...registryEntries.map((entry) => entry.fixtureId)])].sort();
  const fixtureItems = items.filter((item) => item.source_class === "conformance.fixture");
  for (const row of fixtureRows) {
    const matches = fixtureItems.filter((item) => item.source.path === grammar.conformance_surface.status_path && item.source.start_line === row.line);
    if (matches.length !== 1) {
      errors.push(`fixture row ${row.fixtureId} at line ${row.line} has ${matches.length} inventory identities`);
      continue;
    }
    const item = matches[0];
    if (item.context?.fixture_id !== row.fixtureId) errors.push(`${item.id}: fixture label differs from its exact status row`);
    if (item.context?.status_row?.line !== row.line) errors.push(`${item.id}: status-row line differs from source incidence`);
    if (item.context?.status_row?.status_column !== row.statusColumn || item.context?.status_row?.status_label !== row.statusLabel) {
      errors.push(`${item.id}: status field differs from the exact status row`);
    }
  }
  for (const entry of registryEntries) {
    const rows = fixtureRows.filter((row) => row.fixtureId === entry.fixtureId);
    const matches = fixtureItems.filter((item) => item.context?.fixture_id === entry.fixtureId && item.context?.registry_entry !== null);
    const expected = rows.length <= 1 ? 1 : 0;
    if (matches.length !== expected) errors.push(`registry fixture ${entry.fixtureId} has ${matches.length} joined identities; expected ${expected}`);
    if (matches.length === 1) {
      const relation = matches[0].context.registry_entry;
      if (relation.test_path !== entry.testPath || relation.test_function !== entry.testFunction) errors.push(`${matches[0].id}: registry route differs from pinned registry bytes`);
      if (rows.length === 0 && matches[0].source.path !== grammar.conformance_surface.registry_path) errors.push(`${matches[0].id}: registry-only source is not the registry entry`);
    }
  }
  const registryWithoutRows = registryEntries.filter((entry) => !fixtureRows.some((row) => row.fixtureId === entry.fixtureId)).length;
  const expectedFixtureRecords = fixtureRows.length + registryWithoutRows;
  if (fixtureItems.length !== expectedFixtureRecords) errors.push("fixture identity count differs from status-row occurrences plus registry-only entries");
  if (inventory.coverage?.repository?.conformance_fixtures !== expectedFixtureRecords) errors.push("conformance fixture count differs");
  if (inventory.coverage?.repository?.conformance_status_row_occurrences !== fixtureRows.length) errors.push("conformance status-row occurrence count differs");
  if (inventory.coverage?.repository?.conformance_fixture_labels !== fixtureIds.length) errors.push("conformance fixture-label count differs");
  if (inventory.coverage?.repository?.registered_executable_fixtures !== registryEntries.length) errors.push("registered executable fixture count differs");
  if (inventory.coverage?.repository?.registry_without_status_row !== registryWithoutRows) errors.push("registry-without-status-row count differs");
  if (JSON.stringify(inventory.coverage?.repository?.fixture_ids) !== JSON.stringify(fixtureIds)) errors.push("fixture label coverage differs");

  for (const item of items) {
    let bytes;
    if (item.source.path === grammar.canonical_tex.path) bytes = texBytes;
    else if (item.source.revision === grammar.predecessor_commit) {
      if (!baselineBytes.has(item.source.path)) baselineBytes.set(item.source.path, blob(grammar.predecessor_commit, item.source.path));
      bytes = baselineBytes.get(item.source.path);
    } else {
      errors.push(`${item.id}: source revision is outside the admitted source universe`);
      continue;
    }
    if (item.category === "repository_file") continue;
    const selected = exactLines(bytes, item.source.start_line, item.source.end_line);
    if (digest(Buffer.from(normalizeUnit(selected), "utf8")) !== item.source.sha256) errors.push(`${item.id}: source range digest differs`);
  }

  const pending = items.filter((item) => item.review_status === "pending").length;
  if (inventory.coverage?.total_items !== items.length) errors.push("total item count differs");
  if (inventory.coverage?.pending_review_items !== pending) errors.push("pending item count differs");
  if (inventory.gate_a?.pending_review_items !== pending) errors.push("Gate A pending count differs");
  if (pending === 0) errors.push("this Phase A extraction artifact unexpectedly has no review residual");
  return errors;
}

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function requireRejected(name, candidate, grammar, grammarBytes) {
  const errors = inspect(candidate, grammar, grammarBytes);
  if (errors.length === 0) throw new Error(`mutation breaker escaped: ${name}`);
}

function main() {
  const grammarBytes = fs.readFileSync(grammarPath);
  const grammar = JSON.parse(grammarBytes.toString("utf8"));
  const inventory = JSON.parse(fs.readFileSync(inventoryPath, "utf8"));
  const errors = inspect(inventory, grammar, grammarBytes);
  if (errors.length > 0) throw new Error(errors.join("\n"));

  const deleted = clone(inventory);
  deleted.items.splice(deleted.items.findIndex((item) => item.source_class === "tex.declaration"), 1);
  requireRejected("deleted declaration", deleted, grammar, grammarBytes);

  const deletedNarrative = clone(inventory);
  deletedNarrative.items.splice(deletedNarrative.items.findIndex((item) => item.source_class === "tex.narrative"), 1);
  requireRejected("deleted narrative", deletedNarrative, grammar, grammarBytes);

  const deletedRepositoryFile = clone(inventory);
  deletedRepositoryFile.items.splice(deletedRepositoryFile.items.findIndex((item) => item.category === "repository_file"), 1);
  requireRejected("deleted repository file", deletedRepositoryFile, grammar, grammarBytes);

  const altered = clone(inventory);
  altered.items.find((item) => item.source_class === "rust.public_item").source.sha256 = "0".repeat(64);
  requireRejected("altered public surface", altered, grammar, grammarBytes);

  const deletedFixtureRow = clone(inventory);
  deletedFixtureRow.items.splice(deletedFixtureRow.items.findIndex((item) => item.source_class === "conformance.fixture" && item.source.path === grammar.conformance_surface.status_path), 1);
  requireRejected("deleted fixture row occurrence", deletedFixtureRow, grammar, grammarBytes);

  const relabeledFixtureRow = clone(inventory);
  relabeledFixtureRow.items.find((item) => item.source_class === "conformance.fixture" && item.source.path === grammar.conformance_surface.status_path).context.fixture_id = "QFABRICATED-999";
  requireRejected("relabeled fixture row occurrence", relabeledFixtureRow, grammar, grammarBytes);

  const duplicated = clone(inventory);
  duplicated.items.push(clone(duplicated.items[0]));
  requireRejected("duplicated identity", duplicated, grammar, grammarBytes);

  const uncovered = clone(inventory);
  uncovered.coverage.tex.ordered_line_sha256 = "f".repeat(64);
  requireRejected("detached TeX coverage", uncovered, grammar, grammarBytes);

  const promoted = clone(inventory);
  promoted.gate_a.status = "PASS";
  requireRejected("self-promoted Gate A", promoted, grammar, grammarBytes);

  const foreignRevision = clone(inventory);
  foreignRevision.items.find((item) => item.source_class === "rust.public_item").source.revision = "working-tree";
  requireRejected("foreign source revision", foreignRevision, grammar, grammarBytes);

  process.stdout.write(
    `independent predecessor inventory checks passed (${inventory.items.length} items; ${inventory.coverage.pending_review_items} pending; 10/10 mutation breakers)\n`,
  );
}

try {
  main();
} catch (error) {
  process.stderr.write(`predecessor inventory check: ${error.message}\n`);
  process.exit(1);
}
