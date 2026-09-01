#!/usr/bin/env python3
"""Validate the consolidated formal-successor authority and control topology."""

from __future__ import annotations

import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ERRORS: list[str] = []


def fail(message: str) -> None:
    ERRORS.append(message)


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(relative: str) -> str:
    path = ROOT / relative
    if not path.is_file():
        fail(f"missing required file: {relative}")
        return ""
    return path.read_text(encoding="utf-8")


required = [
    "AGENTS.md",
    "Inquiry_Calculus_v2_0.tex",
    "IMPLEMENTATION_FRONTIER.md",
    "formal-successor/ACTIVE_INPUTS.json",
    "formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md",
    "formal-successor/INTEGRATED_THEOREM_OBLIGATIONS.json",
    "formal-successor/INQUIRY_SPINE_CONTRACT.json",
    "formal-successor/REGENERATIVE_SPINE.json",
    "formal-successor/Questions.txt",
    ".claude/hooks/ic-spine.js",
    ".claude/hooks/ic-inject",
]
for name in required:
    read(name)

retired = [
    "formal-successor/AUTONOMOUS_ITERATION.md",
    "formal-successor/QUESTION_RHYTHM.md",
    "formal-successor/ENGINEERING_QUESTION_PROGRAMS.json",
    "formal-successor/PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS.md",
    "formal-successor/PREFORMAL_SEARCH_ASYMMETRY.md",
    "formal-successor/SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md",
    "formal-successor/QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md",
]
for name in retired:
    if (ROOT / name).exists():
        fail(f"retired procedural controller still exists: {name}")

try:
    active = json.loads(read("formal-successor/ACTIVE_INPUTS.json"))
    if active.get("schema") != 2:
        fail("ACTIVE_INPUTS.json must use consolidated schema 2")
    expected_roles = [
        "predecessor_semantic_authority",
        "single_successor_construction_and_acceptance_authority",
        "external_inquiry_corpus_not_semantic_authority",
    ]
    authority = active.get("authority_path", [])
    if [item.get("role") for item in authority] != expected_roles:
        fail("ACTIVE_INPUTS.json does not expose the single forward authority path")
    for item in authority + [active.get("derived_machine_contract", {}), active.get("derived_construction_memory", {})]:
        path = ROOT / str(item.get("path", ""))
        if not path.is_file() or item.get("sha256") != digest(path):
            fail(f"ACTIVE_INPUTS digest mismatch: {item.get('path')}")
    planning_registries = active.get("derived_planning_registries", [])
    if [item.get("role") for item in planning_registries] != [
        "gate_indexed_candidate_theorem_obligations_not_semantic_authority"
    ]:
        fail("ACTIVE_INPUTS.json does not expose the derived theorem-obligation registry")
    for item in planning_registries:
        path = ROOT / str(item.get("path", ""))
        if not path.is_file() or item.get("sha256") != digest(path):
            fail(f"ACTIVE_INPUTS digest mismatch: {item.get('path')}")
    if active.get("derived_machine_contract", {}).get("role") != "rebuildable_implementation_contract_not_independent_authority":
        fail("machine contract is not explicitly derived/non-authoritative")
    if active.get("derived_construction_memory", {}).get("role") != "rebuildable_regenerative_dependency_projection_not_semantic_authority":
        fail("regenerative construction memory is not explicitly derived/non-authoritative")
except (json.JSONDecodeError, OSError, TypeError) as error:
    fail(f"ACTIVE_INPUTS.json: {error}")

try:
    contract = json.loads(read("formal-successor/INQUIRY_SPINE_CONTRACT.json"))
    recurrence = ["RELATE", "OPEN", "TURN", "RETURN", "DISTINGUISH", "FOLD", "CARRY"]
    if contract.get("schema") != 5 or contract.get("model_recurrence") != recurrence:
        fail("machine contract does not expose exactly the schema-5 inquiry recurrence")
    corpus = ROOT / contract.get("corpus", {}).get("path", "")
    if not corpus.is_file() or contract.get("corpus", {}).get("sha256") != digest(corpus):
        fail("machine contract is detached from Questions.txt")
    forbidden_keys = {"rhythm", "inner_phases", "residual_schedule", "method_dispatch", "next_question"}
    serialized = json.dumps(contract, sort_keys=True).lower()
    for key in forbidden_keys:
        if f'"{key}"' in serialized:
            fail(f"machine contract contains retired controller key {key}")
    forms = contract.get("question_forms", [])
    generators = contract.get("generator_registry", [])
    form_ids = {item.get("id") for item in forms}
    covered = {form for generator in generators for form in generator.get("question_forms", [])}
    if not form_ids or form_ids != covered:
        fail("question forms are not exactly covered by relation generators")
except (json.JSONDecodeError, OSError, TypeError) as error:
    fail(f"INQUIRY_SPINE_CONTRACT.json: {error}")

try:
    theorem_registry = json.loads(read("formal-successor/INTEGRATED_THEOREM_OBLIGATIONS.json"))
    if theorem_registry.get("status") != "planned_candidate_theorem_family_not_successor_semantics":
        fail("integrated theorem registry claims semantic authority")
    obligations = theorem_registry.get("obligations", [])
    if len(obligations) != 34 or any(item.get("status") != "PLANNED" for item in obligations):
        fail("integrated theorem registry is incomplete or contains an unsupported promotion")
except (json.JSONDecodeError, OSError, TypeError) as error:
    fail(f"INTEGRATED_THEOREM_OBLIGATIONS.json: {error}")

spec = read("formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md")
agents = read("AGENTS.md")
inject = read(".claude/hooks/ic-inject")
spine = read(".claude/hooks/ic-spine.js")
canonical = "RELATE -> OPEN -> TURN -> RETURN -> DISTINGUISH -> FOLD -> CARRY -> RELATE"
if canonical not in agents:
    fail("AGENTS.md does not point to the one inquiry spine")
if "ic-spine.js" not in inject or "SPECIFY ->" in inject or "FIELD -> ASK" in inject:
    fail("model injection is not exclusively generated through ic-spine.js")
for required_fragment in [
    "D_{i,t}=C_{q_t}\\circ P_{i:t}",
    "There is no active",
    "The accountability protocol underneath `RETURN` is not a second reasoning recurrence",
]:
    if required_fragment not in spec:
        fail(f"construction specification lacks absorbed spine relation: {required_fragment}")
for fragment in ["derivePaths", "transport", "selectExecutable", "evaluateClosure"]:
    if fragment not in spine:
        fail(f"ic-spine.js lacks {fragment}")

active_control_files = [
    "AGENTS.md",
    "formal-successor/ACTIVE_INPUTS.json",
    "formal-successor/README.md",
    ".claude/settings.json",
    ".claude/skills/inquire/SKILL.md",
] + [str(path.relative_to(ROOT)).replace("\\", "/") for path in (ROOT / ".claude/hooks").glob("*") if path.is_file()]
retired_basenames = {Path(name).name for name in retired}
for relative in active_control_files:
    text = read(relative)
    for basename in retired_basenames:
        if basename in text:
            fail(f"{relative} still actively references retired {basename}")
    if "LEGACY_METHOD_DISPATCH" in text or "chooseMethodFrontier" in text:
        fail(f"{relative} retains residual-class method dispatch")

frontier = read("IMPLEMENTATION_FRONTIER.md")
if frontier.count("<!-- LIVE_FRONTIER_BEGIN -->") != 1 or frontier.count("<!-- LIVE_FRONTIER_END -->") != 1:
    fail("IMPLEMENTATION_FRONTIER.md must contain exactly one live cursor")
match = re.search(r"<!-- LIVE_FRONTIER_BEGIN -->\s*(.*?)\s*<!-- LIVE_FRONTIER_END -->", frontier, re.S)
if not match or len(re.findall(r"^id:\s*\S+", match.group(1) if match else "", re.M)) != 1:
    fail("live Frontier block is malformed")

for ledger_name in ["formal-successor/DECISIONS.jsonl", "formal-successor/FAILURES.jsonl"]:
    seen: set[str] = set()
    for number, line in enumerate(read(ledger_name).splitlines(), 1):
        if not line.strip():
            continue
        try:
            record = json.loads(line)
        except json.JSONDecodeError as error:
            fail(f"{ledger_name}:{number}: {error}")
            continue
        identity = record.get("id")
        if not isinstance(identity, str) or not identity or identity in seen:
            fail(f"{ledger_name}:{number}: missing or duplicate id")
        seen.add(identity)
        if ledger_name.endswith("DECISIONS.jsonl") and record.get("status") not in {"EXACT", "WORKING", "CONDITIONAL", "SUPERSEDED"}:
            fail(f"{ledger_name}:{number}: invalid decision status {record.get('status')!r}")

if ERRORS:
    print("documentation/control check FAILED:")
    for error in ERRORS:
        print(f"  - {error}")
    sys.exit(1)

print("documentation/control check passed (one inquiry spine)")
