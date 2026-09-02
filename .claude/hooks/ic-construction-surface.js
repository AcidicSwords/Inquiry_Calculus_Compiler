#!/usr/bin/env node
"use strict";

// Rebuildable projection of the protected construction surface (ProtSurface_chi).
//
// Every item is derived from repository-resident authority or evidence and carries
// stable identity plus provenance. This is construction metainfrastructure: it is
// not an Inquiry Calculus semantic primitive, not semantic authority, and not
// history. It must remain deletable and reconstructible from the repository alone.

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const cp = require("node:child_process");

const sha256 = (bytes) => crypto.createHash("sha256").update(bytes).digest("hex");
const slug = (value) => String(value).toUpperCase()
  .replace(/[^A-Z0-9]+/gu, "-").replace(/^-+|-+$/gu, "").slice(0, 72);

function fail(message) {
  throw new Error(message);
}

function readBytes(root, relative) {
  try {
    return fs.readFileSync(path.join(root, ...relative.split("/")));
  } catch (error) {
    fail(`cannot read ${relative}: ${error.message}`);
  }
}

function readJson(root, relative) {
  const bytes = readBytes(root, relative);
  try {
    return { value: JSON.parse(bytes.toString("utf8")), source: { path: relative, sha256: sha256(bytes) } };
  } catch (error) {
    fail(`${relative} is not JSON: ${error.message}`);
  }
}

// Declaration keywords that introduce a consequential formal object. `example`
// and `variable` are deliberately excluded: they name nothing downstream.
const DECLARATION =
  /^\s*(?:@\[[^\]]*\]\s*)?(?:private\s+|protected\s+|noncomputable\s+|partial\s+)*(theorem|lemma|def|abbrev|structure|inductive|class|instance)\s+([A-Za-z_][A-Za-z0-9_'!?.]*)/u;
const NAMESPACE = /^\s*namespace\s+([A-Za-z_][A-Za-z0-9_'.]*)/u;
const END_NAMESPACE = /^\s*end\s+([A-Za-z_][A-Za-z0-9_'.]*)/u;

function leanFiles(root, relative) {
  const absolute = path.join(root, ...relative.split("/"));
  if (!fs.existsSync(absolute)) return [];
  const output = [];
  const walk = (directory, prefix) => {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      const child = path.join(directory, entry.name);
      const childRelative = `${prefix}/${entry.name}`;
      if (entry.isDirectory()) walk(child, childRelative);
      else if (entry.name.endsWith(".lean")) output.push(childRelative);
    }
  };
  walk(absolute, relative);
  return output;
}

// The checked formal construction surface. Declarations are read from source and
// are candidate declarations at their file's coverage; reading a name here is not
// a kernel return and grants no theorem standing.
function leanDeclarations(root, relative) {
  const declarations = [];
  for (const file of leanFiles(root, relative)) {
    const bytes = readBytes(root, file);
    const lines = stripComments(bytes.toString("utf8")).split(/\r?\n/u);
    const imports = lines.flatMap((line) => /^import\s+(.+)$/u.exec(line)?.[1].trim().split(/\s+/u) ?? []);
    const namespaces = [];
    for (let index = 0; index < lines.length; index += 1) {
      const line = lines[index];
      const opened = NAMESPACE.exec(line);
      if (opened) {
        namespaces.push(opened[1]);
        continue;
      }
      const closed = END_NAMESPACE.exec(line);
      if (closed && namespaces.at(-1) === closed[1]) {
        namespaces.pop();
        continue;
      }
      const declared = DECLARATION.exec(line);
      if (!declared) continue;
      const [, keyword, name] = declared;
      declarations.push({
        keyword,
        name,
        qualified: [...namespaces, name].join("."),
        file,
        line: index + 1,
        sha256: sha256(bytes),
        imports,
        // A proposition-valued declaration carries a claim; a data declaration
        // introduces a carrier. The two generate different obligations.
        claim: keyword === "theorem" || keyword === "lemma",
      });
    }
  }
  return declarations;
}

// Preserve newlines and strings, including nested Lean block comments. This is a
// conservative source inventory, not an elaborator or proof dependency oracle.
function stripComments(text) {
  let depth = 0, quoted = false, output = "";
  for (let i = 0; i < text.length; i += 1) {
    const pair = text.slice(i, i + 2), ch = text[i];
    if (!quoted && pair === "/-") { depth += 1; output += "  "; i += 1; }
    else if (depth && pair === "-/") { depth -= 1; output += "  "; i += 1; }
    else if (depth) output += ch === "\n" ? "\n" : " ";
    else if (!quoted && pair === "--") {
      while (i < text.length && text[i] !== "\n") { output += " "; i += 1; }
      if (i < text.length) output += "\n";
    } else {
      output += ch;
      if (ch === '"' && text[i - 1] !== "\\") quoted = !quoted;
    }
  }
  return output;
}

function item(id, kind, fields) {
  for (const key of ["title", "provenance"]) {
    if (typeof fields[key] !== "string" || !fields[key].trim()) {
      fail(`construction surface item ${id} requires nonempty ${key}`);
    }
  }
  return {
    id,
    kind,
    dependencies: [],
    status: "represented",
    horizon: "declared by source ancestry",
    coverage: "declared by source ancestry",
    ...fields,
  };
}

function build(root) {
  const spine = readJson(root, "formal-successor/REGENERATIVE_SPINE.json");
  const theorems = readJson(root, "formal-successor/INTEGRATED_THEOREM_OBLIGATIONS.json");
  const residuals = readJson(root, "formal-successor/RESIDUAL_OBLIGATIONS.json");
  const contract = readJson(root, "formal-successor/INQUIRY_SPINE_CONTRACT.json");
  const corpusBytes = readBytes(root, "formal-successor/Questions.txt");
  const continuity = readJson(root, "formal-successor/NORMALIZATION_CONTINUITY.json");
  const specBytes = readBytes(root, "formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md");
  const sources = [spine.source, theorems.source, residuals.source, contract.source,
    continuity.source,
    { path: "formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md", sha256: sha256(specBytes) },
    { path: "formal-successor/Questions.txt", sha256: sha256(corpusBytes) }];

  const items = [];

  // Git is the existing ancestry store, not a second protected-target registry.
  // Removed accepted targets survive as explicit repair obligations.
  let priorCapabilities = [];
  try {
    priorCapabilities = JSON.parse(cp.execFileSync("git", ["-C", root, "show", "HEAD:formal-successor/REGENERATIVE_SPINE.json"],
      { encoding: "utf8", windowsHide: true, stdio: ["ignore", "pipe", "pipe"] })).protected_predecessor_capabilities ?? [];
  } catch { /* A source-only fixture has no accepted Git epoch. */ }
  const capabilities = [...(spine.value.protected_predecessor_capabilities ?? [])];
  for (const prior of priorCapabilities) {
    if (!capabilities.some((candidate) => candidate.id === prior.id)) capabilities.push({ ...prior, protection_loss: true });
  }

  // Protected predecessor capabilities: the regeneration obligations the successor
  // must discharge. Representation here is not evidence of construction.
  for (const capability of capabilities) {
    if (!capability.successor_target || !capability.artifacts?.length) fail(`protected target ${capability.id} lost its target or source artifacts`);
    const id = `PCAP-${slug(capability.id)}`;
    items.push(item(id, "protected_capability", {
      title: capability.id,
      provenance: "formal-successor/REGENERATIVE_SPINE.json#protected_predecessor_capabilities",
      status: capability.regeneration_status ?? "OPEN_NO_SUCCESSOR_CONSTRUCTION_OR_CORRESPONDENCE",
      successor_target: capability.successor_target,
      dependency_order: capability.dependency_order ?? 0,
      artifacts: capability.artifacts ?? [],
      ambient_requirements: capability.candidate_ambient_requirements ?? [],
      checked_partial_artifact: capability.checked_partial_artifact ?? null,
      checked_partial_result: capability.checked_partial_result ?? null,
      remaining_correspondence: capability.remaining_correspondence ?? [],
      protection_loss: capability.protection_loss ?? false,
    }));
  }

  // Candidate capability basis. These are candidates, never a minimality claim.
  const kernel = spine.value.current_semantic_kernel ?? {};
  for (const candidate of kernel.primitive_candidates ?? []) {
    const label = candidate.split(":")[0].trim();
    items.push(item(`CAND-${slug(label)}`, "candidate_capability", {
      title: candidate,
      provenance: "formal-successor/REGENERATIVE_SPINE.json#current_semantic_kernel.primitive_candidates",
      status: kernel.status ?? "candidate",
      basis_obligation: kernel.candidate_basis_obligation ?? null,
    }));
  }
  for (const demoted of kernel.not_primitive ?? []) {
    items.push(item(`NOTPRIM-${slug(demoted)}`, "declared_nonprimitive", {
      title: demoted,
      provenance: "formal-successor/REGENERATIVE_SPINE.json#current_semantic_kernel.not_primitive",
    }));
  }

  // Explicit theorem seeds. A seed is an audit surface, not the theorem universe.
  for (const obligation of theorems.value.obligations ?? []) {
    items.push(item(`THM-${slug(obligation.id)}`, "theorem_seed", {
      title: obligation.title ?? obligation.id,
      provenance: "formal-successor/INTEGRATED_THEOREM_OBLIGATIONS.json#obligations",
      status: obligation.status ?? "PLANNED",
      registry_id: obligation.id,
      gate: obligation.gate ?? "unknown",
      statement: obligation.statement ?? "",
      decisive_check: obligation.decisive_check ?? "",
      dependencies: (obligation.depends_on ?? []).map((id) => `THM-${slug(id)}`),
    }));
  }

  // Checked theorem evidence at its exact declared scope. One module can carry several
  // distinct scoped results, so identity pins the scope as well as the artifact.
  for (const proved of spine.value.proved_theorem_spine ?? []) {
    const artifactSlug = slug(proved.artifact.replace(/^.*\//u, "").replace(/\.lean$/u, ""));
    const scopeDigest = sha256(`${proved.artifact}\0${proved.scope ?? ""}\0${proved.theorem}`)
      .slice(0, 8).toUpperCase();
    items.push(item(`PROVED-${artifactSlug}-${scopeDigest}`, "proved_theorem", {
      title: proved.theorem,
      provenance: proved.artifact,
      status: proved.status ?? "proved",
      scope: proved.scope ?? "unstated",
      artifact: proved.artifact,
    }));
  }

  // Derived definition candidates and their governing obligations.
  for (const derived of spine.value.derived_definition_candidates ?? []) {
    items.push(item(`DERIV-${slug(derived.name)}`, "derived_definition_candidate", {
      title: derived.name,
      provenance: "formal-successor/REGENERATIVE_SPINE.json#derived_definition_candidates",
      dependencies: derived.obligation ? [`THM-${slug(derived.obligation)}`] : [],
      registry_id: derived.obligation ?? null,
    }));
  }

  // Candidate inquiries carry missing structure and breakers that are live pressure.
  for (const inquiry of spine.value.candidate_inquiries ?? []) {
    items.push(item(`INQ-${slug(inquiry.id)}`, "candidate_inquiry", {
      title: inquiry.making_question ?? inquiry.id,
      provenance: "formal-successor/REGENERATIVE_SPINE.json#candidate_inquiries",
      status: inquiry.disposition ?? "unresolved",
      missing_structure: inquiry.missing_structure ?? [],
      breaker: inquiry.breaker ?? "",
      dependencies: (inquiry.theorem_obligations ?? []).map((id) => `THM-${slug(id)}`),
      propagate_to: inquiry.propagate_to ?? [],
    }));
  }

  // Explicit countermodels and non-collapse requirements.
  for (const breaker of spine.value.known_breakers ?? []) {
    items.push(item(`BRK-${sha256(breaker).slice(0, 12).toUpperCase()}`, "known_breaker", {
      title: breaker,
      provenance: "formal-successor/REGENERATIVE_SPINE.json#known_breakers",
    }));
  }
  for (const law of spine.value.noncollapse_laws ?? []) {
    items.push(item(`NCL-${sha256(law).slice(0, 12).toUpperCase()}`, "non_collapse_requirement", {
      title: law,
      provenance: "formal-successor/REGENERATIVE_SPINE.json#noncollapse_laws",
    }));
  }
  for (const unresolved of spine.value.unresolved_typed_obligations ?? []) {
    items.push(item(`UNRES-${slug(unresolved)}`, "unresolved_typed_obligation", {
      title: unresolved,
      provenance: "formal-successor/REGENERATIVE_SPINE.json#unresolved_typed_obligations",
      dependencies: [`THM-${slug(unresolved)}`],
      registry_id: unresolved,
    }));
  }

  // Stable project residual seeds and their dependency edges.
  for (const node of residuals.value.nodes ?? []) {
    items.push(item(`RES-${slug(node.id)}`, "residual_seed", {
      title: node.obligation,
      provenance: "formal-successor/RESIDUAL_OBLIGATIONS.json#nodes",
      status: node.state ?? "latent",
      phase: node.phase ?? "unknown",
      coverage: node.coverage ?? "declared by source ancestry",
      dependencies: (node.depends_on ?? []).map((id) => `RES-${slug(id)}`),
      registry_id: node.id,
      reopen_when: node.reopen_when ?? [],
    }));
  }

  // The checked formal construction surface itself. Lean names are case sensitive
  // and a case-insensitive slug alone collides, so identity pins the exact
  // module-qualified name rather than its display form.
  const declarationId = (prefix, declaration) =>
    `${prefix}-${slug(declaration.qualified)}-${sha256(`${declaration.file}\0${declaration.qualified}`).slice(0, 8).toUpperCase()}`;
  const successorDeclarations = leanDeclarations(root, "formal/InquiryCalculus/Successor");
  for (const declaration of successorDeclarations) {
    items.push(item(declarationId("DECL", declaration), "successor_declaration", {
      title: declaration.qualified,
      provenance: `${declaration.file}:${declaration.line}`,
      status: "candidate_declaration_at_file_coverage",
      keyword: declaration.keyword,
      claim: declaration.claim,
      module: declaration.file,
      source_sha256: declaration.sha256,
      imports: declaration.imports,
    }));
  }
  for (const declaration of leanDeclarations(root, "formal/InquiryCalculus/Legacy/V20")) {
    items.push(item(declarationId("LEGACY", declaration), "predecessor_declaration", {
      title: declaration.qualified,
      provenance: `${declaration.file}:${declaration.line}`,
      status: "checked_predecessor_surface",
      keyword: declaration.keyword,
      claim: declaration.claim,
      module: declaration.file,
      source_sha256: declaration.sha256,
      imports: declaration.imports,
    }));
  }

  for (const entry of continuity.value.entries) {
    items.push(item(`CONTINUITY-${entry.pass}`, "normalization_continuity", {
      title: entry.relation, provenance: `${continuity.source.path}#${entry.pass}`,
      status: entry.disposition, dependencies: entry.obligations.map((id) => `THM-${slug(id)}`),
    }));
  }
  // Every source question survives, not only the 26 selected renderings. Exact
  // classification/coverage is an obligation until independently checked.
  const repeatedQuestions = new Map();
  for (const [index, line] of corpusBytes.toString("utf8").split(/\r?\n/u).entries()) {
    if (!line.trim().endsWith("?")) continue;
    const hash = sha256(line.trim()).slice(0, 16), repeat = (repeatedQuestions.get(hash) ?? 0) + 1;
    repeatedQuestions.set(hash, repeat);
    items.push(item(`SOURCEQUESTION-${hash}-${repeat}`, "source_question", {
      title: line.trim(), provenance: `formal-successor/Questions.txt:${index + 1}`,
      source_sha256: sha256(corpusBytes),
    }));
  }
  // Construction obligations arise from governing requirements as well as code.
  // Their exact applicability is open; headings do not certify mathematical truth.
  const sections = specBytes.toString("utf8").split(/(?=^#{1,3} )/mu);
  for (const section of sections) {
    const heading = section.split(/\r?\n/u)[0];
    if (!/^#{1,3} /u.test(heading)) continue;
    const id = `SPEC-${slug(heading)}-${sha256(heading).slice(0, 8)}`;
    items.push(item(id, "construction_requirement", {
      title: heading.replace(/^#+\s*/u, ""), provenance: `formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md#${heading}`,
      source_sha256: sha256(section),
    }));
  }
  for (const file of [...new Set(items.filter((entry) => entry.module).map((entry) => entry.module))]) {
    sources.push({ path: file, sha256: sha256(readBytes(root, file)) });
  }

  // Corpus/language obligations at the represented question-form granularity.
  for (const form of contract.value.question_forms ?? []) {
    items.push(item(`CORPUS-${slug(form.id)}`, "corpus_obligation", {
      title: form.prompt,
      provenance: `formal-successor/INQUIRY_SPINE_CONTRACT.json#question_forms/${form.id}`,
      form_id: form.id,
      source_lines: form.source_lines ?? [],
    }));
  }

  items.sort((left, right) => left.id.localeCompare(right.id));
  const identities = new Set();
  for (const entry of items) {
    if (identities.has(entry.id)) fail(`duplicate construction surface identity ${entry.id}`);
    identities.add(entry.id);
  }

  const surface = {
    schema: 1,
    status: "derived_rebuildable_protected_construction_surface_not_semantics_authority_or_history",
    generated_from: sources,
    counts: Object.fromEntries([...new Set(items.map((entry) => entry.kind))].sort()
      .map((kind) => [kind, items.filter((entry) => entry.kind === kind).length])),
    items,
  };
  return { surface, digest: sha256(JSON.stringify(surface)) };
}

function render(root) {
  const { surface, digest } = build(root);
  return [
    `PROTECTED CONSTRUCTION SURFACE ${digest.slice(0, 16)}`,
    `items: ${surface.items.length}`,
    ...Object.entries(surface.counts).map(([kind, count]) => `  ${kind}: ${count}`),
    "derived projection only: representation is not construction, evidence, or warrant",
    "",
  ].join("\n");
}

module.exports = { build, leanDeclarations, render, slug };

if (require.main === module) {
  try {
    const [command, suppliedRoot, output] = process.argv.slice(2);
    const root = path.resolve(suppliedRoot ?? path.resolve(__dirname, "../.."));
    if (command === "json") process.stdout.write(`${JSON.stringify(build(root).surface, null, 2)}\n`);
    else if (command === "projection") process.stdout.write(render(root));
    else if (command === "digest") process.stdout.write(`${build(root).digest}\n`);
    else if (command === "build") {
      if (!output) fail("build requires an output path");
      fs.writeFileSync(path.resolve(output), `${JSON.stringify(build(root).surface, null, 2)}\n`);
    } else fail("usage: ic-construction-surface.js json|projection|digest|build ROOT [OUTPUT]");
  } catch (error) {
    process.stderr.write(`ic-construction-surface: ${error.message}\n`);
    process.exitCode = 1;
  }
}
