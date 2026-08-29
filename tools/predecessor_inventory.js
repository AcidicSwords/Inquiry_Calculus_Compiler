#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const root = path.resolve(__dirname, "..");
const grammarPath = path.join(root, "formal-successor", "PREDECESSOR_INVENTORY_GRAMMAR.json");
const inventoryPath = path.join(root, "formal-successor", "PREDECESSOR_INVENTORY.json");

function sha256(value) {
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

function git(args, options = {}) {
  const result = spawnSync("git", args, {
    cwd: root,
    encoding: options.encoding ?? "utf8",
    maxBuffer: 128 * 1024 * 1024,
    windowsHide: true,
  });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(" ")} failed: ${(result.stderr || "").trim()}`);
  }
  return result.stdout;
}

function gitBlob(revision, relativePath) {
  return git(["show", `${revision}:${relativePath}`], { encoding: "buffer" });
}

function slug(value) {
  const result = value
    .replace(/\\[A-Za-z@]+\*?(?:\[[^\]]*\])?/gu, " ")
    .replace(/[{}$\\]/gu, " ")
    .replace(/[^A-Za-z0-9]+/gu, "-")
    .replace(/^-+|-+$/gu, "")
    .toLowerCase();
  return result || "unscoped";
}

function source(pathName, revision, startLine, endLine, content) {
  return {
    path: pathName,
    revision,
    start_line: startLine,
    end_line: endLine,
    sha256: sha256(Buffer.from(normalizeUnit(content), "utf8")),
  };
}

function contentId(prefix, sourceClass, content, occurrences) {
  const base = `${prefix}-${sha256(`${sourceClass}\0${normalizeUnit(content)}`).slice(0, 16).toUpperCase()}`;
  const occurrence = (occurrences.get(base) || 0) + 1;
  occurrences.set(base, occurrence);
  return occurrence === 1 ? base : `${base}-${String(occurrence).padStart(2, "0")}`;
}

function lineRange(lines, startIndex, endIndex) {
  return lines.slice(startIndex, endIndex + 1).join("\n");
}

function findEnvironmentRanges(lines, environments) {
  const wanted = new Set(environments);
  const ranges = [];
  const stack = [];
  for (let index = 0; index < lines.length; index += 1) {
    const begin = lines[index].match(/\\begin\{([^}]+)\}/u);
    if (begin && wanted.has(begin[1])) stack.push({ environment: begin[1], start: index });
    const end = lines[index].match(/\\end\{([^}]+)\}/u);
    if (!end || !wanted.has(end[1])) continue;
    let position = stack.length - 1;
    while (position >= 0 && stack[position].environment !== end[1]) position -= 1;
    if (position < 0) continue;
    const opened = stack.splice(position, 1)[0];
    ranges.push({ environment: opened.environment, start: opened.start, end: index });
  }
  if (stack.length > 0) {
    throw new Error(`unclosed admitted TeX environment(s): ${stack.map((item) => item.environment).join(", ")}`);
  }
  return ranges.sort((left, right) => left.start - right.start || left.end - right.end);
}

function covered(index, ranges) {
  return ranges.some((range) => index >= range.start && index <= range.end);
}

function headingContexts(lines, headingCommands) {
  const pattern = new RegExp(`^\\s*\\\\(${headingCommands.join("|")})\\*?\\{(.+)\\}\\s*(?:%.*)?$`, "u");
  const contexts = [];
  let current = { command: "document", title: "Document", slug: "document", line: 1 };
  for (let index = 0; index < lines.length; index += 1) {
    const match = lines[index].match(pattern);
    if (match) {
      current = { command: match[1], title: match[2], slug: slug(match[2]), line: index + 1 };
    }
    contexts[index] = current;
  }
  return contexts;
}

function normativeSignal(content, grammar) {
  const lower = normalizeUnit(content).toLowerCase();
  return grammar.canonical_tex.normative_signals.filter((signal) => lower.includes(signal));
}

function extractTex(grammar, bytes) {
  if (sha256(bytes) !== grammar.canonical_tex.sha256) {
    throw new Error(`canonical TeX digest differs from the extraction grammar: ${sha256(bytes)}`);
  }
  const text = normalizeNewlines(bytes.toString("utf8"));
  const allLines = text.split("\n");
  const begin = allLines.findIndex((line) => line.trim() === grammar.canonical_tex.document_begin);
  const end = allLines.findIndex((line, index) => index > begin && line.trim() === grammar.canonical_tex.document_end);
  if (begin < 0 || end < 0 || end <= begin) throw new Error("canonical TeX document boundaries are not uniquely formable");

  const lines = allLines.slice(begin + 1, end);
  const lineOffset = begin + 1;
  const contexts = headingContexts(lines, grammar.canonical_tex.heading_commands);
  const occurrences = new Map();
  const items = [];
  const declarationRanges = findEnvironmentRanges(lines, grammar.canonical_tex.declaration_environments);
  const proofRanges = findEnvironmentRanges(lines, ["proof"]);
  const listingRanges = findEnvironmentRanges(lines, ["lstlisting"]);
  const displayRanges = findEnvironmentRanges(lines, grammar.canonical_tex.display_environments);

  for (let index = 0; index < lines.length; index += 1) {
    if (lines[index].trim() !== "\\[") continue;
    const close = lines.findIndex((line, candidate) => candidate > index && line.trim() === "\\]");
    if (close < 0) throw new Error(`unclosed display math beginning at TeX line ${index + lineOffset + 1}`);
    displayRanges.push({ environment: "bracket-display", start: index, end: close });
    index = close;
  }
  displayRanges.sort((left, right) => left.start - right.start || left.end - right.end);

  for (const range of declarationRanges) {
    const content = lineRange(lines, range.start, range.end);
    const label = content.match(/\\label\{([^}]+)\}/u)?.[1];
    const title = lines[range.start].match(/\\begin\{[^}]+\}(?:\[([^\]]+)\])?/u)?.[1] || range.environment;
    const context = contexts[range.start];
    const id = label
      ? `PRED-TEX-DECL-${slug(label).toUpperCase()}`
      : contentId("PRED-TEX-DECL", `tex.declaration.${range.environment}`, content, occurrences);
    items.push({
      id,
      source_class: "tex.declaration",
      authority: "predecessor_semantics",
      category: range.environment,
      disposition: "formalize_predecessor",
      destination: `phase:B/formal-predecessor/${context.slug}`,
      review_status: "pending",
      evidence_status: "source_actuality",
      source: source(
        grammar.canonical_tex.path,
        `sha256:${grammar.canonical_tex.sha256}`,
        range.start + lineOffset + 1,
        range.end + lineOffset + 1,
        content,
      ),
      context: { heading: context.title, heading_command: context.command, declaration_title: title, label: label || null },
    });
  }

  for (const range of displayRanges) {
    if (covered(range.start, declarationRanges)) continue;
    const content = lineRange(lines, range.start, range.end);
    const context = contexts[range.start];
    items.push({
      id: contentId("PRED-TEX-DISPLAY", "tex.display", content, occurrences),
      source_class: "tex.display",
      authority: "predecessor_semantics",
      category: "mathematical_display_candidate",
      disposition: "requires_claim_boundary_review",
      destination: `phase:A/tex-review/${context.slug}`,
      review_status: "pending",
      evidence_status: "source_actuality",
      source: source(
        grammar.canonical_tex.path,
        `sha256:${grammar.canonical_tex.sha256}`,
        range.start + lineOffset + 1,
        range.end + lineOffset + 1,
        content,
      ),
      context: { heading: context.title, heading_command: context.command, display_environment: range.environment },
    });
  }

  const itemStarts = [];
  for (let index = 0; index < lines.length; index += 1) {
    if (/^\s*\\item(?:\[[^\]]*\])?/u.test(lines[index])) itemStarts.push(index);
  }
  for (let position = 0; position < itemStarts.length; position += 1) {
    const start = itemStarts[position];
    let itemEnd = position + 1 < itemStarts.length ? itemStarts[position + 1] - 1 : start;
    for (let index = start + 1; index <= itemEnd; index += 1) {
      if (/^\s*\\end\{(?:itemize|enumerate|description)\}/u.test(lines[index])) {
        itemEnd = index - 1;
        break;
      }
      if (lines[index].trim() === "" && index > start + 1) {
        itemEnd = index - 1;
        break;
      }
    }
    if (itemEnd < start) itemEnd = start;
    const content = lineRange(lines, start, itemEnd);
    const context = contexts[start];
    const signals = normativeSignal(content, grammar);
    items.push({
      id: contentId("PRED-TEX-ITEM", "tex.list_item", content, occurrences),
      source_class: "tex.list_item",
      authority: "predecessor_semantics",
      category: signals.length > 0 ? "normative_prose_candidate" : "prose_candidate",
      disposition: signals.length > 0 ? "requires_normative_review" : "requires_prose_review",
      destination: `phase:A/tex-review/${context.slug}`,
      review_status: "pending",
      evidence_status: "source_actuality",
      source: source(
        grammar.canonical_tex.path,
        `sha256:${grammar.canonical_tex.sha256}`,
        start + lineOffset + 1,
        itemEnd + lineOffset + 1,
        content,
      ),
      context: { heading: context.title, heading_command: context.command, normative_signals: signals },
    });
  }

  const excluded = [...declarationRanges, ...proofRanges, ...listingRanges, ...displayRanges];
  const structural = /^\s*(?:\\(?:part|chapter|section|subsection|subsubsection)\*?\{|\\begin\{|\\end\{|\\item\b|\\label\{|\\\[|\\\]|\\(?:toprule|midrule|bottomrule|endhead|caption|centering)\b)/u;
  let paragraphStart = null;
  function flushParagraph(lastIndex) {
    if (paragraphStart === null || lastIndex < paragraphStart) {
      paragraphStart = null;
      return;
    }
    const content = lineRange(lines, paragraphStart, lastIndex);
    if (normalizeUnit(content) === "") {
      paragraphStart = null;
      return;
    }
    const context = contexts[paragraphStart];
    const signals = normativeSignal(content, grammar);
    items.push({
      id: contentId("PRED-TEX-PROSE", "tex.narrative", content, occurrences),
      source_class: "tex.narrative",
      authority: "predecessor_semantics",
      category: signals.length > 0 ? "normative_prose_candidate" : "prose_candidate",
      disposition: signals.length > 0 ? "requires_normative_review" : "requires_prose_review",
      destination: `phase:A/tex-review/${context.slug}`,
      review_status: "pending",
      evidence_status: "source_actuality",
      source: source(
        grammar.canonical_tex.path,
        `sha256:${grammar.canonical_tex.sha256}`,
        paragraphStart + lineOffset + 1,
        lastIndex + lineOffset + 1,
        content,
      ),
      context: { heading: context.title, heading_command: context.command, normative_signals: signals },
    });
    paragraphStart = null;
  }
  for (let index = 0; index < lines.length; index += 1) {
    const trimmed = lines[index].trim();
    const unavailable = covered(index, excluded) || trimmed === "" || trimmed.startsWith("%") || structural.test(lines[index]);
    if (unavailable) {
      flushParagraph(index - 1);
    } else if (paragraphStart === null) {
      paragraphStart = index;
    }
  }
  flushParagraph(lines.length - 1);

  const coverageLines = [];
  const lexicalCounts = {};
  for (let index = 0; index < lines.length; index += 1) {
    const trimmed = lines[index].trim();
    if (trimmed === "" || trimmed.startsWith("%")) continue;
    let lexicalClass = "plain_or_inline_math";
    if (/^\\(?:part|chapter|section|subsection|subsubsection)\*?\{/u.test(trimmed)) lexicalClass = "heading";
    else if (/^\\(?:begin|end)\{/u.test(trimmed)) lexicalClass = "environment_boundary";
    else if (/^\\item\b/u.test(trimmed)) lexicalClass = "list_item_boundary";
    else if (/^\\label\{/u.test(trimmed)) lexicalClass = "label";
    else if (/^\\(?:\[|\])/u.test(trimmed)) lexicalClass = "display_boundary";
    else if (/^\\[A-Za-z@]+/u.test(trimmed)) lexicalClass = "control_or_structured_line";
    lexicalCounts[lexicalClass] = (lexicalCounts[lexicalClass] || 0) + 1;
    coverageLines.push(`${index + lineOffset + 1}\0${lines[index].replace(/\s+$/u, "")}`);
  }

  return {
    items,
    coverage: {
      document_start_line: begin + 1,
      document_end_line: end + 1,
      nonblank_noncomment_lines: coverageLines.length,
      ordered_line_sha256: sha256(coverageLines.join("\n")),
      lexical_class_counts: Object.fromEntries(Object.entries(lexicalCounts).sort(([left], [right]) => left.localeCompare(right))),
      extracted_declarations: declarationRanges.length,
      extracted_list_items: itemStarts.length,
      extracted_displays: items.filter((item) => item.source_class === "tex.display").length,
      extracted_narratives: items.filter((item) => item.source_class === "tex.narrative").length,
    },
  };
}

function repositoryClass(relativePath) {
  if (/^crates\/[^/]+\/src\/.*\.rs$/u.test(relativePath)) return "rust.semantic_module_candidate";
  if (/^crates\/[^/]+\/tests\/.*\.rs$/u.test(relativePath)) return "rust.conformance_module";
  if (/^crates\/[^/]+\/Cargo\.toml$/u.test(relativePath)) return "rust.crate_manifest";
  if (relativePath.startsWith("fixtures/")) return "schema.fixture";
  if (relativePath.startsWith("migrations/")) return "schema.migration";
  return "repository.support_file";
}

function repositoryDisposition(sourceClass) {
  if (sourceClass === "rust.semantic_module_candidate") return ["predecessor_implementation_evidence", "phase:F/predecessor-correspondence", "pending"];
  if (sourceClass === "rust.conformance_module") return ["predecessor_conformance_evidence", "phase:A/fixture-review", "pending"];
  if (sourceClass.startsWith("schema.")) return ["predecessor_schema_evidence", "phase:F/schema-correspondence", "pending"];
  if (sourceClass === "rust.crate_manifest") return ["predecessor_build_evidence", "phase:F/build-correspondence", "classified"];
  return ["predecessor_support_evidence", "phase:A/source-review", "pending"];
}

function parsePublicItems(relativePath, text, revision, allowedKinds) {
  const items = [];
  const occurrences = new Map();
  const lines = normalizeNewlines(text).split("\n");
  const publicPrefix = /^\s*pub(?:\s*\([^)]*\))?\s+/u;
  const pattern = /^\s*pub(?:\s*\([^)]*\))?\s+(?:(?:async|unsafe|const)\s+)*(struct|enum|trait|type|fn|const|static|mod)\s+(\$?[A-Za-z_][A-Za-z0-9_]*)/u;
  const usePattern = /^\s*pub(?:\s*\([^)]*\))?\s+use\s+([A-Za-z_][A-Za-z0-9_:]*)/u;
  for (let index = 0; index < lines.length; index += 1) {
    if (!publicPrefix.test(lines[index])) continue;
    const match = lines[index].match(pattern);
    const use = lines[index].match(usePattern);
    const kind = match?.[1] || (use ? "use" : "surface_line");
    const name = match?.[2] || use?.[1] || `line-${sha256(lines[index]).slice(0, 10)}`;
    if (!allowedKinds.includes(kind)) throw new Error(`public surface kind ${kind} is outside the extraction grammar`);
    const content = lines[index];
    const pathDigest = sha256(relativePath).slice(0, 10).toUpperCase();
    const base = `PRED-RUST-PUB-${pathDigest}-${kind.toUpperCase()}-${slug(name).toUpperCase()}`;
    const occurrence = (occurrences.get(base) || 0) + 1;
    occurrences.set(base, occurrence);
    items.push({
      id: occurrence === 1 ? base : `${base}-${String(occurrence).padStart(2, "0")}`,
      source_class: "rust.public_item",
      authority: "predecessor_implementation_evidence",
      category: kind,
      disposition: kind === "surface_line" ? "requires_public_surface_parse" : "requires_correspondence_review",
      destination: `phase:F/predecessor-correspondence/${relativePath}`,
      review_status: "pending",
      evidence_status: "source_actuality",
      source: source(relativePath, revision, index + 1, index + 1, content),
      context: { name, module_class: repositoryClass(relativePath) },
    });
  }
  return items;
}

function parseFixtureRegistry(text) {
  const evidence = new Map();
  const pattern = /"(Q[A-Z0-9-]+-[0-9]{3})"\s*:\s*\(\s*"([^"]+)"\s*,\s*"([^"]+)"/gu;
  const lines = normalizeNewlines(text).split("\n");
  for (const match of text.matchAll(pattern)) {
    const sourceLine = normalizeNewlines(text.slice(0, match.index)).split("\n").length;
    evidence.set(match[1], {
      test_path: match[2],
      test_function: match[3],
      source_line: sourceLine,
      source_text: lines[sourceLine - 1],
    });
  }
  return evidence;
}

function extractRepository(grammar) {
  const revision = grammar.predecessor_commit;
  const listed = git(["ls-tree", "-r", "--name-only", revision, "--", ...grammar.repository_surface.roots])
    .split(/\r?\n/u)
    .filter(Boolean)
    .sort();
  const items = [];
  const treeEntries = [];
  for (const relativePath of listed) {
    const bytes = gitBlob(revision, relativePath);
    const sourceClass = repositoryClass(relativePath);
    const [disposition, destinationRoot, reviewStatus] = repositoryDisposition(sourceClass);
    const blobDigest = sha256(bytes);
    treeEntries.push(`${relativePath}\0${blobDigest}`);
    items.push({
      id: `PRED-REPO-${sha256(relativePath).slice(0, 20).toUpperCase()}`,
      source_class: sourceClass,
      authority: sourceClass.startsWith("rust.") ? "predecessor_implementation_evidence" : "predecessor_schema_evidence",
      category: "repository_file",
      disposition,
      destination: `${destinationRoot}/${relativePath}`,
      review_status: reviewStatus,
      evidence_status: "source_actuality",
      source: {
        path: relativePath,
        revision,
        start_line: 1,
        end_line: normalizeNewlines(bytes.toString("utf8")).split("\n").length,
        sha256: blobDigest,
      },
    });
    if (sourceClass === "rust.semantic_module_candidate") {
      items.push(...parsePublicItems(relativePath, bytes.toString("utf8"), revision, grammar.repository_surface.public_item_kinds));
    }
  }

  const conformanceBytes = gitBlob(revision, grammar.conformance_surface.status_path);
  const registryBytes = gitBlob(revision, grammar.conformance_surface.registry_path);
  const registry = parseFixtureRegistry(registryBytes.toString("utf8"));
  const conformanceLines = normalizeNewlines(conformanceBytes.toString("utf8")).split("\n");
  const fixturePattern = new RegExp(`^\\|\\s*\u0060?(${grammar.conformance_surface.fixture_id_pattern})\u0060?(?:\\s+[^|]*)?\\|\\s*([^|]+)`, "u");
  const fixtureRows = new Map();
  for (let index = 0; index < conformanceLines.length; index += 1) {
    const match = conformanceLines[index].match(fixturePattern);
    if (!match) continue;
    fixtureRows.set(match[1], { line: index + 1, text: conformanceLines[index], adjacent_field: match[2].trim().toLowerCase() });
  }
  const fixtureIds = [...new Set([...fixtureRows.keys(), ...registry.keys()])].sort();
  for (const fixtureId of fixtureIds) {
    const row = fixtureRows.get(fixtureId);
    const registered = registry.get(fixtureId);
    const itemSource = row
      ? source(grammar.conformance_surface.status_path, revision, row.line, row.line, row.text)
      : source(grammar.conformance_surface.registry_path, revision, registered.source_line, registered.source_line, registered.source_text);
    items.push({
      id: `PRED-CONFORMANCE-${fixtureId}`,
      source_class: "conformance.fixture",
      authority: "predecessor_conformance_evidence",
      category: row ? row.adjacent_field : "registry_without_status_row",
      disposition: registered && row ? "registered_fixture_evidence" : registered ? "requires_status_row_review" : "requires_fixture_evidence_review",
      destination: `phase:A/fixture-review/${fixtureId}`,
      review_status: registered && row ? "classified" : "pending",
      evidence_status: registered ? "registered_executable_evidence" : "source_actuality",
      source: itemSource,
      context: {
        status_row: row ? { path: grammar.conformance_surface.status_path, line: row.line } : null,
        registry_entry: registered
          ? { path: grammar.conformance_surface.registry_path, line: registered.source_line, test_path: registered.test_path, test_function: registered.test_function }
          : null,
      },
    });
  }

  return {
    items,
    coverage: {
      repository_files: listed.length,
      ordered_tree_sha256: sha256(treeEntries.join("\n")),
      semantic_module_candidates: items.filter((item) => item.source_class === "rust.semantic_module_candidate").length,
      public_rust_items: items.filter((item) => item.source_class === "rust.public_item").length,
      schema_files: items.filter((item) => item.source_class.startsWith("schema.")).length,
      conformance_fixtures: fixtureIds.length,
      registered_executable_fixtures: fixtureIds.filter((id) => registry.has(id)).length,
      registry_without_status_row: fixtureIds.filter((id) => registry.has(id) && !fixtureRows.has(id)).length,
      fixture_ids: fixtureIds,
    },
  };
}

function grammarAndBytes() {
  const bytes = fs.readFileSync(grammarPath);
  return { grammar: JSON.parse(bytes.toString("utf8")), bytes };
}

function generateInventory() {
  const { grammar, bytes: grammarBytes } = grammarAndBytes();
  const texBytes = fs.readFileSync(path.join(root, grammar.canonical_tex.path));
  const tex = extractTex(grammar, texBytes);
  const repository = extractRepository(grammar);
  const items = [...tex.items, ...repository.items].sort((left, right) => left.id.localeCompare(right.id));
  const classCounts = {};
  for (const item of items) classCounts[item.source_class] = (classCounts[item.source_class] || 0) + 1;
  return {
    schema: 1,
    status: "generated_phase_a_predecessor_inventory_not_gate_a",
    generated_from: {
      grammar_path: path.relative(root, grammarPath).split(path.sep).join("/"),
      grammar_sha256: sha256(grammarBytes),
      predecessor_commit: grammar.predecessor_commit,
      canonical_tex_path: grammar.canonical_tex.path,
      canonical_tex_sha256: grammar.canonical_tex.sha256,
    },
    coverage: {
      tex: tex.coverage,
      repository: repository.coverage,
      total_items: items.length,
      pending_review_items: items.filter((item) => item.review_status === "pending").length,
      classified_items: items.filter((item) => item.review_status === "classified").length,
      source_class_counts: Object.fromEntries(Object.entries(classCounts).sort(([left], [right]) => left.localeCompare(right))),
    },
    gate_a: {
      status: "PENDING",
      reason: "Source coverage is generated, but consequential candidates still require explicit destination/disposition review and fixture-to-claim edges.",
      pending_review_items: items.filter((item) => item.review_status === "pending").length,
    },
    items,
  };
}

function validateInventoryShape(inventory, grammar) {
  const errors = [];
  if (inventory.schema !== 1) errors.push("inventory schema must be 1");
  if (inventory.gate_a?.status !== "PENDING") errors.push("generated inventory must not self-promote Gate A");
  if (!Array.isArray(inventory.items)) errors.push("inventory items must be an array");
  const ids = new Set();
  let prior = "";
  for (const [index, item] of (inventory.items || []).entries()) {
    for (const field of grammar.required_item_fields) {
      if (!(field in item)) errors.push(`item ${index} is missing ${field}`);
    }
    if (ids.has(item.id)) errors.push(`duplicate inventory id ${item.id}`);
    ids.add(item.id);
    if (prior && prior.localeCompare(item.id) > 0) errors.push(`inventory items are not sorted at ${item.id}`);
    prior = item.id;
    if (!grammar.review_statuses.includes(item.review_status)) errors.push(`${item.id}: invalid review_status`);
    if (!grammar.evidence_statuses.includes(item.evidence_status)) errors.push(`${item.id}: invalid evidence_status`);
    if (!item.source || !item.source.path || !item.source.revision || !item.source.sha256) errors.push(`${item.id}: incomplete source identity`);
  }
  const pending = (inventory.items || []).filter((item) => item.review_status === "pending").length;
  if (inventory.coverage?.pending_review_items !== pending) errors.push("pending review count does not match items");
  if (inventory.gate_a?.pending_review_items !== pending) errors.push("Gate A pending count does not match items");
  return errors;
}

function compareInventories(actual, expected) {
  const errors = [];
  const actualText = `${JSON.stringify(actual, null, 2)}\n`;
  const expectedText = `${JSON.stringify(expected, null, 2)}\n`;
  if (actualText !== expectedText) errors.push("committed inventory does not exactly regenerate from the pinned sources and grammar");
  return errors;
}

function main() {
  const command = process.argv[2] || "check";
  const { grammar } = grammarAndBytes();
  if (command === "generate") {
    const inventory = generateInventory();
    const errors = validateInventoryShape(inventory, grammar);
    if (errors.length > 0) throw new Error(errors.join("\n"));
    fs.writeFileSync(inventoryPath, `${JSON.stringify(inventory, null, 2)}\n`);
    process.stdout.write(`generated ${inventory.items.length} predecessor inventory items; Gate A remains PENDING (${inventory.coverage.pending_review_items} pending review)\n`);
    return;
  }
  if (command === "check") {
    const actual = JSON.parse(fs.readFileSync(inventoryPath, "utf8"));
    const expected = generateInventory();
    const errors = [...validateInventoryShape(actual, grammar), ...compareInventories(actual, expected)];
    if (errors.length > 0) throw new Error(errors.join("\n"));
    process.stdout.write(`predecessor inventory regenerates exactly (${actual.items.length} items; Gate A ${actual.gate_a.status})\n`);
    return;
  }
  throw new Error(`unknown command ${command}; expected generate or check`);
}

module.exports = {
  compareInventories,
  extractRepository,
  extractTex,
  generateInventory,
  normalizeNewlines,
  normalizeUnit,
  repositoryClass,
  sha256,
  validateInventoryShape,
};

if (require.main === module) {
  try {
    main();
  } catch (error) {
    process.stderr.write(`predecessor inventory: ${error.message}\n`);
    process.exit(1);
  }
}
