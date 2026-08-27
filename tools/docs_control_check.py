#!/usr/bin/env python3
"""Check the repository's documentation/control topology.

This is deliberately a topology and provenance check, not a semantic proof.  It
keeps moving state in the frontier/evidence ledgers, checks that the closed
research payload remains available, and makes destructive ledger rewrites
visible both before a local commit and across a CI delivery range.
"""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from typing import Iterable


ROOT = Path(__file__).resolve().parents[1]

CANONICAL = "Inquiry_Calculus_v2_0.tex"
PLAN = "Inquiry_Calculus_v2_0_Comprehensive_Implementation_Plan.md"
FRONTIER = "IMPLEMENTATION_FRONTIER.md"

LIVE_BEGIN = "<!-- LIVE_FRONTIER_BEGIN -->"
LIVE_END = "<!-- LIVE_FRONTIER_END -->"
LIVE_KEYS = {
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
}

RETIRED_ROOT_FILES = {
    "PERSISTENT_CODEX_GOAL.md",
    "POST_RESEARCH_INTEGRATION_DIRECTIVE.md",
    "PROJECT_RESEARCH_IMPLEMENTATION_HANDOFF.md",
    "RESEARCH_BUNDLE_MANIFEST.md",
    "Inquiry_Calculus_Unified_Canonical_Specification_v1_1.tex",
    "Inquiry_Calculus_v1_1_Comprehensive_Implementation_Plan.md",
    "Inquiry_Calculus_v1_1_Paired_Actuality_Spine_Addition.tex",
    "Inquiry_Calculus_v1_1_Interrogative_Succession_Extension.tex",
}

# The closed research corpus is derived breaker/regression ancestry.  Its
# README is intentionally excluded from byte pinning because the control
# migration must replace its link to the retired root handoff.
RESEARCH_PAYLOADS = {
    "research/final-2026-08-25/COHESIVE_MACHINE_IMPLEMENTATION_MAP.md":
        "65a6cac9b42601ae42b6a4f2680a71c99e24cf57e83ad7a5fd544ab6813ed68f",
    "research/final-2026-08-25/M001_M066_REGENERATION_INDEX.md":
        "37fe9993354d82dccf2d37daad641b6c6810e3ed4a078da619ce8e0acc24f9e5",
    "research/final-2026-08-25/REGENERATIVE_CONTEXT_LEDGER.md":
        "04852459ebe13498b40e0123e5b472a10db9e74cd8d15bfb28cc8ace01d39b43",
    "research/final-2026-08-25/REGENERATIVE_QUESTION_PROGRAM_NETWORK.md":
        "d2cbaac95fddf52d37b1a345b60173d8a684fe42f5cdec5e7f80f33a3b887e7a",
    "research/final-2026-08-25/SELF_MODIFICATION_DOMAIN_MAP.md":
        "919d94c026053a813bc25631d8896aed7694588c802feb0a75d7e7968c8ae995",
}

V2_FIXTURE_IDS = {
    "QSUCC-OCC-001",
    "QSUCC-PARTIAL-001",
    "QREADY-UNLOCK-001",
    "QREADY-NONUNLOCK-001",
    "QSTATIC-DYNAMIC-001",
    "QCONVERSE-NOT-INVERSE-001",
    "QADJOINT-001",
    "QRECIP-PROV-001",
    "QFRONTIER-REQDISCHARGE-001",
    "QIFP-LOCAL-001",
    "QIFP-REOPEN-001",
    "QROUTE-REGEN-001",
    "QROUTE-ABLATE-001",
    "QRENDER-001",
    "QLOWER-001",
    "QACTUAL-SEPARATION-001",
    "QLIFT-ALLPATHS-001",
    "QRESOLUTION-GATE-001",
    "QCODE-TYPING-001",
}

# A v2 fixture may become PASS only through a deliberately registered executable
# breaker. The status ledger names the protected boundary and coverage; this
# registry ties that claim to concrete test source. The Rust gate executes the
# breaker independently, so this topology check never treats a prose row as its
# own warrant. Adding a new PASS route is an acceptance-checker change.
V2_PASS_EVIDENCE = {
    "QSUCC-OCC-001": (
        "crates/ic-core/tests/decoder_identity.rs",
        "occurrence_indexed_successors_keep_equal_questions_and_answers_distinct",
    ),
    "QSUCC-PARTIAL-001": (
        "crates/ic-core/tests/decoder_identity.rs",
        "occurrence_indexed_successor_retains_every_member_of_a_supported_answer",
    ),
    "QSTATIC-DYNAMIC-001": (
        "crates/ic-core/tests/decoder_identity.rs",
        "static_question_relation_does_not_manufacture_an_occurrence_successor",
    ),
    "QREADY-UNLOCK-001": (
        "crates/ic-core/tests/decoder_identity.rs",
        "exact_supported_dependency_is_required_for_local_question_readiness",
    ),
    "QREADY-NONUNLOCK-001": (
        "crates/ic-core/tests/decoder_identity.rs",
        "exact_supported_dependency_is_required_for_local_question_readiness",
    ),
    "QCONVERSE-NOT-INVERSE-001": (
        "crates/ic-core/tests/fiber.rs",
        "many_to_one_converse_preserves_the_whole_reverse_fiber_without_an_inverse",
    ),
    "QADJOINT-001": (
        "crates/ic-core/tests/adjunction.rs",
        "finite_adjoint_requires_its_binding_supplied_law_at_every_pair",
    ),
    "QRECIP-PROV-001": (
        "crates/ic-core/tests/decoder_identity.rs",
        "independently_admitted_sides_form_one_reciprocal_occurrence_vertical_slice",
    ),
    "QFRONTIER-REQDISCHARGE-001": (
        "crates/ic-core/tests/decoder_identity.rs",
        "required_discharge_survives_nondominance_without_promoting_generation",
    ),
    "QIFP-LOCAL-001": (
        "crates/ic-core/tests/decoder_identity.rs",
        "finite_local_fixed_point_requires_exact_coverage_and_no_open_obligation",
    ),
    "QIFP-REOPEN-001": (
        "crates/ic-core/tests/decoder_identity.rs",
        "positive_new_live_occurrence_reopens_only_its_exact_predecessor_field",
    ),
    "QROUTE-REGEN-001": (
        "crates/ic-core/tests/decoder_identity.rs",
        "route_regeneration_requires_the_whole_protected_residual_fiber",
    ),
    "QROUTE-ABLATE-001": (
        "crates/ic-core/tests/decoder_identity.rs",
        "route_node_ablation_requires_independent_regeneration_and_protected_equality",
    ),
    "QRENDER-001": (
        "crates/ic-core/tests/decoder_identity.rs",
        "controlled_rendering_never_promotes_shared_words_into_semantic_authority",
    ),
    "QLOWER-001": (
        "crates/ic-core/tests/decoder_identity.rs",
        "interrogative_annotations_erase_only_to_the_checked_ordinary_program",
    ),
    "QACTUAL-SEPARATION-001": (
        "crates/ic-runtime/tests/admitted_resume.rs",
        "source_linked_events_preserve_equal_projection_occurrences_after_restart",
    ),
    "QLIFT-ALLPATHS-001": (
        "crates/ic-core/tests/decoder_identity.rs",
        "finite_supported_family_lift_preserves_every_tagged_child_position_and_unknowns",
    ),
    "QRESOLUTION-GATE-001": (
        "crates/ic-runtime/tests/admitted_resume.rs",
        "source_linked_events_preserve_equal_projection_occurrences_after_restart",
    ),
    "QCODE-TYPING-001": (
        "crates/ic-runtime/tests/code_quote.rs",
        "typed_quotations_cold_decode_and_interpret_only_the_exact_admitted_coordinate",
    ),
}

ERRORS: list[str] = []


def error(message: str) -> None:
    ERRORS.append(message)


def read_text(path: str) -> str:
    candidate = ROOT / path
    try:
        return candidate.read_text(encoding="utf-8")
    except FileNotFoundError:
        error(f"required file is missing: {path}")
    except UnicodeDecodeError as exc:
        error(f"{path} is not UTF-8: {exc}")
    return ""


def check_root_topology() -> None:
    required = {
        "README.md",
        "AGENTS.md",
        CANONICAL,
        PLAN,
        FRONTIER,
        "CONFORMANCE_STATUS.md",
        "DECISIONS.jsonl",
        "FAILURES.jsonl",
    }
    for path in sorted(required):
        if not (ROOT / path).is_file():
            error(f"required root control file is missing: {path}")

    for path in sorted(RETIRED_ROOT_FILES):
        if (ROOT / path).exists():
            error(f"retired root control/companion file is still present: {path}")

    root_tex = {path.name for path in ROOT.glob("*.tex") if path.is_file()}
    if root_tex != {CANONICAL}:
        error(
            "root semantic authority must be exactly "
            f"{CANONICAL}; found {sorted(root_tex)}"
        )

    root_plans = {
        path.name for path in ROOT.glob("*Implementation_Plan*.md") if path.is_file()
    }
    if root_plans != {PLAN}:
        error(
            "root implementation-plan authority must be exactly "
            f"{PLAN}; found {sorted(root_plans)}"
        )


def check_research_payload() -> None:
    readme = ROOT / "research/final-2026-08-25/README.md"
    if not readme.is_file():
        error("closed research corpus README is missing")

    for path, expected in RESEARCH_PAYLOADS.items():
        candidate = ROOT / path
        if not candidate.is_file():
            error(f"closed research payload is missing: {path}")
            continue
        actual = hashlib.sha256(candidate.read_bytes()).hexdigest()
        if actual != expected:
            error(
                f"closed research payload changed: {path} "
                f"(expected {expected}, got {actual})"
            )


def check_canonical_source() -> None:
    """Check structural properties of the single semantic source.

    This does not claim a semantic proof.  It protects the consolidation
    boundaries that can be decided directly from the TeX source; Tectonic and
    executable fixtures remain independent checks.
    """
    text = read_text(CANONICAL)
    if not text:
        return

    required_fragments = {
        r"\operatorname{Embed}_{1.1\to2.0}": "v1.1 conservative embedding",
        r"\mathsf{BIND}": "master recurrence BIND position",
        r"\mathsf{OPEN}": "master recurrence OPEN position",
        r"\mathsf{VARY}": "master recurrence VARY position",
        r"\mathsf{RETURN}": "master recurrence RETURN position",
        r"\mathsf{DETERMINE}": "master recurrence DETERMINE position",
        r"\mathsf{REFACTOR}": "master recurrence REFACTOR position",
        r"\QSucc(\AskRef,\widehat S,q')": "occurrence-indexed question succession",
        r"\RequiredDischarge(\Sigma,\AskRef,d)": "required discharge",
        r"\AskRef\to\mathcal D\to\zeta=\mathsf{Supported}(\widehat S)":
            "actuality/resolution/answer non-collapse chain",
        r"\section{One master regenerative recurrence}": "single master recurrence",
        r"\begin{definition}[Authoritative ancestry]": "single history authority",
    }
    for fragment, meaning in required_fragments.items():
        if fragment not in text:
            error(f"{CANONICAL}: missing {meaning}")

    exact_once = {
        r"\section{One master regenerative recurrence}": "master recurrence section",
        r"\section{Derived interrogative algebra}": "interrogative algebra section",
        r"\begin{definition}[Authoritative ancestry]":
            "authoritative ancestry definition",
    }
    for fragment, meaning in exact_once.items():
        count = text.count(fragment)
        if count != 1:
            error(f"{CANONICAL}: {meaning} must occur exactly once (found {count})")

    forbidden_patterns = {
        r"\\QSucc\s*\(\s*q\s*,\s*S\s*,":
            "question succession keyed only by semantic q and S",
        r"(?i)projection\s+(?:alone\s+)?creates?\s+exteriority":
            "projection-created exteriority",
        r"(?i)\\mathsf\{Recip\}[^\n]{0,100}six\s+independent":
            "six independent Recip slots",
        r"(?i)generic\s+hole\s+solving[^\n]{0,80}(?:performs|causes)\s+redetermination":
            "generic-hole redetermination",
        r"(?i)\\Gamma_D[^\n]{0,80}(?:supplies|generates|manufactures)\s+(?:missing\s+)?roles":
            "Gamma role generation",
        r"(?i)generated\s+exterior(?:ity)?[^\n]{0,80}changes?\s+standing":
            "generated exterior changing standing",
    }
    for pattern, meaning in forbidden_patterns.items():
        if re.search(pattern, text):
            error(f"{CANONICAL}: contains stale claim: {meaning}")

    labels = re.findall(r"\\label\{([^{}]+)\}", text)
    duplicates = sorted({label for label in labels if labels.count(label) > 1})
    for label in duplicates:
        error(f"{CANONICAL}: duplicate label {label}")
    label_set = set(labels)
    for targets in re.findall(r"\\(?:c|C)?ref\{([^{}]+)\}", text):
        for target in (item.strip() for item in targets.split(",")):
            if target and target not in label_set:
                error(f"{CANONICAL}: unresolved source reference {target}")

    commands = re.findall(r"\\newcommand\{\\([A-Za-z@]+)\}", text)
    for command in sorted({name for name in commands if commands.count(name) > 1}):
        error(f"{CANONICAL}: duplicate newcommand \\{command}")

    plan = read_text(PLAN)
    conformance = read_text("CONFORMANCE_STATUS.md")
    for fixture_id in sorted(V2_FIXTURE_IDS):
        for path, source in (
            (CANONICAL, text),
            (PLAN, plan),
            ("CONFORMANCE_STATUS.md", conformance),
        ):
            count = source.count(fixture_id)
            if count != 1:
                error(
                    f"{path}: fixture {fixture_id} must occur exactly once "
                    f"(found {count})"
                )
        status_lines = [
            line for line in conformance.splitlines() if fixture_id in line
        ]
        if len(status_lines) == 1:
            cells = [cell.strip() for cell in status_lines[0].split("|")]
            status = cells[3] if len(cells) >= 4 else ""
            if status not in {"PENDING", "PASS"}:
                error(
                    "CONFORMANCE_STATUS.md: adopted fixture "
                    f"{fixture_id} must be PENDING or PASS, got {status!r}"
                )
                continue
            if status != "PASS":
                continue

            evidence = V2_PASS_EVIDENCE.get(fixture_id)
            if evidence is None:
                error(
                    "CONFORMANCE_STATUS.md: adopted fixture "
                    f"{fixture_id} cannot be PASS without registered executable evidence"
                )
                continue
            evidence_path, test_name = evidence
            evidence_source = read_text(evidence_path)
            if f"fn {test_name}" not in evidence_source:
                error(
                    f"{evidence_path}: registered v2 evidence test {test_name} is missing"
                )
            if test_name not in status_lines[0]:
                error(
                    "CONFORMANCE_STATUS.md: PASS fixture "
                    f"{fixture_id} must name its registered test {test_name}"
                )


def parse_jsonl(path: str, prefix: str) -> set[str]:
    candidate = ROOT / path
    if not candidate.is_file():
        return set()

    raw = candidate.read_bytes()
    if raw and not raw.endswith(b"\n"):
        error(f"{path} must end with a newline so future appends remain JSONL")
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError as exc:
        error(f"{path} is not UTF-8: {exc}")
        return set()

    ids: set[str] = set()
    id_pattern = re.compile(rf"{re.escape(prefix)}-\d{{4}}")

    def reject_constant(value: str) -> None:
        raise ValueError(f"non-finite JSON number {value}")

    def reject_duplicate_keys(pairs: list[tuple[str, object]]) -> dict[str, object]:
        result: dict[str, object] = {}
        for key, value in pairs:
            if key in result:
                raise ValueError(f"duplicate JSON key {key!r}")
            result[key] = value
        return result

    for line_number, line in enumerate(text.splitlines(), start=1):
        if not line.strip():
            error(f"{path}:{line_number}: blank lines are not valid JSONL records")
            continue
        try:
            record = json.loads(
                line,
                object_pairs_hook=reject_duplicate_keys,
                parse_constant=reject_constant,
            )
        except (json.JSONDecodeError, ValueError) as exc:
            error(f"{path}:{line_number}: invalid JSON: {exc}")
            continue
        if not isinstance(record, dict):
            error(f"{path}:{line_number}: each JSONL record must be an object")
            continue
        record_id = record.get("id")
        if not isinstance(record_id, str) or id_pattern.fullmatch(record_id) is None:
            error(
                f"{path}:{line_number}: id must match {prefix}-NNNN; "
                f"got {record_id!r}"
            )
            continue
        if record_id in ids:
            error(f"{path}:{line_number}: duplicate id {record_id}")
        ids.add(record_id)

        if prefix == "D":
            required_strings = {
                "question",
                "protected_difference",
                "chosen",
                "status",
            }
            required_lists = {"alternatives", "evidence", "reopen_if"}
            allowed_status = {"WORKING", "EXACT", "SUPERSEDED"}
        else:
            required_strings = {
                "observed_at",
                "operation",
                "actual_return",
                "constraint",
                "status",
            }
            required_lists = {"evidence", "reopen_when"}
            allowed_status = {"OPEN", "RESOLVED"}

        for key in sorted(required_strings):
            value = record.get(key)
            if not isinstance(value, str) or not value.strip():
                error(f"{path}:{line_number}: {key} must be a nonempty string")
        for key in sorted(required_lists):
            value = record.get(key)
            if (
                not isinstance(value, list)
                or not value
                or any(not isinstance(item, str) or not item.strip() for item in value)
            ):
                error(
                    f"{path}:{line_number}: {key} must be a nonempty list "
                    "of nonempty strings"
                )
        if record.get("status") not in allowed_status:
            error(
                f"{path}:{line_number}: status {record.get('status')!r} is not "
                f"one of {sorted(allowed_status)}"
            )
    return ids


def parse_frontier(decision_ids: set[str], failure_ids: set[str]) -> dict[str, str]:
    parser = ROOT / ".claude/hooks/ic-frontier.js"
    try:
        completed = subprocess.run(
            ["node", str(parser), str(ROOT / FRONTIER), "json"],
            cwd=ROOT,
            text=True,
            encoding="utf-8",
            capture_output=True,
            check=False,
        )
    except FileNotFoundError:
        error(f"{FRONTIER}: Node.js is unavailable for the normative parser")
        return {}
    if completed.returncode != 0:
        detail = completed.stderr.strip() or completed.stdout.strip()
        error(f"{FRONTIER}: normative parser rejected the live block: {detail}")
        return {}
    try:
        parsed = json.loads(completed.stdout)
        fields = parsed["fields"]
    except (json.JSONDecodeError, KeyError, TypeError) as exc:
        error(f"{FRONTIER}: normative parser returned invalid JSON: {exc}")
        return {}
    if not isinstance(fields, dict) or set(fields) != LIVE_KEYS:
        error(f"{FRONTIER}: normative parser returned the wrong field schema")
        return {}

    frontier_id = fields.get("id", "")
    if frontier_id and re.fullmatch(r"[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)+", frontier_id) is None:
        error(f"{FRONTIER}: live id has an invalid stable form: {frontier_id!r}")

    check_frontier_refs(
        fields.get("relevant_decisions", ""), "D", decision_ids
    )
    check_frontier_refs(
        fields.get("relevant_failures", ""), "F", failure_ids
    )
    return fields


def check_frontier_refs(value: str, prefix: str, known: set[str]) -> None:
    field = "relevant_decisions" if prefix == "D" else "relevant_failures"
    if not value:
        return
    if value.casefold() == "none":
        return

    tokens = [token for token in re.split(r"[\s,]+", value) if token]
    expected = re.compile(rf"{prefix}-\d{{4}}")
    for token in tokens:
        if expected.fullmatch(token) is None:
            error(f"{FRONTIER}: {field} contains invalid token {token!r}")
        elif token not in known:
            error(f"{FRONTIER}: {field} references missing {token}")


def check_control_migration_closure() -> None:
    try:
        decisions = {
            record["id"]: record
            for record in (
                json.loads(line)
                for line in read_text("DECISIONS.jsonl").splitlines()
                if line.strip()
            )
        }
        failures = {
            record["id"]: record
            for record in (
                json.loads(line)
                for line in read_text("FAILURES.jsonl").splitlines()
                if line.strip()
            )
        }
    except (json.JSONDecodeError, KeyError, TypeError):
        # parse_jsonl already emits the precise structural error.
        return

    supersession = decisions.get("D-0130")
    if supersession is None or supersession.get("status") != "EXACT":
        error("D-0130 must retain the exact v2.0 predecessor supersession map")
    else:
        chosen = supersession.get("chosen", "")
        for predecessor, successor in (
            ("D-0005", "D-0129"),
            ("D-0093", "D-0125"),
            ("D-0105", "D-0125"),
            ("D-0104", "D-0126"),
        ):
            if predecessor not in chosen or successor not in chosen:
                error(
                    f"D-0130 must map predecessor {predecessor} to {successor}"
                )

    harness = decisions.get("D-0131")
    if harness is None or harness.get("status") != "EXACT":
        error("D-0131 must retain the accepted exact harness contract")
    resolution = failures.get("F-0004")
    if resolution is None or resolution.get("status") != "RESOLVED":
        error("F-0004 must retain the checked resolution record for F-0003")
    elif "D-0131" not in resolution.get("evidence", []):
        error("F-0004 must retain D-0131 as its replacement-path evidence")


def active_text_paths() -> Iterable[Path]:
    explicit = [
        ROOT / "README.md",
        ROOT / "AGENTS.md",
        ROOT / CANONICAL,
        ROOT / PLAN,
        ROOT / FRONTIER,
        ROOT / "CONFORMANCE_STATUS.md",
    ]
    yielded: set[Path] = set()
    for path in explicit:
        if path.is_file() and path not in yielded:
            yielded.add(path)
            yield path

    patterns = [
        ".github/workflows/*.yml",
        ".github/workflows/*.yaml",
        ".claude/settings*.json",
        ".claude/hooks/*",
        ".claude/skills/inquire/**/*.md",
        "research/**/README*.md",
    ]
    for pattern in patterns:
        for path in ROOT.glob(pattern):
            if path.is_file() and path not in yielded:
                yielded.add(path)
                yield path


def check_active_references() -> None:
    retired_names = sorted(RETIRED_ROOT_FILES)
    stale_revised_filename = re.compile(
        r"Inquiry_Calculus_Unified_Canonical_Specification_v1_1_"
        r"REVISED_AGAIN[^\s`]*"
    )
    for path in active_text_paths():
        relative = path.relative_to(ROOT).as_posix()
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            error(f"active control surface is not UTF-8: {relative}")
            continue
        if stale_revised_filename.search(text):
            error(f"{relative}: stale REVISED_AGAIN authority reference")
        for retired in retired_names:
            if retired in text:
                error(f"{relative}: references retired active-root path {retired}")


def check_state_ownership(frontier: dict[str, str]) -> None:
    readme = read_text("README.md")
    frontier_id = frontier.get("id")
    if frontier_id:
        token = re.compile(
            rf"(?<![A-Za-z0-9_-]){re.escape(frontier_id)}(?![A-Za-z0-9_-])"
        )
        if token.search(readme):
            error(f"README.md contains the moving live-frontier id {frontier_id}")

    plan = read_text(PLAN)
    moving_plan_patterns = {
        r"(?im)^#{1,6}\s+\d+(?:\.\d+)?\.\s+Current relative fixed point\s*$":
            "current-relative-fixed-point section",
        r"(?im)^#{1,6}\s+\d+(?:\.\d+)?\.\s+Remaining implementation order from the current state\s*$":
            "current-state implementation-order section",
        r"(?i)\bnext executable (?:step|obligation)\b":
            "next-executable narration",
        r"(?i)\bcurrent observed repository coordinate\b":
            "current repository-coordinate narration",
        r"(?i)\bcurrent provider sequence\b":
            "current-provider narration",
    }
    for pattern, description in moving_plan_patterns.items():
        if re.search(pattern, plan):
            error(f"{PLAN} contains moving state: {description}")

    conformance = read_text("CONFORMANCE_STATUS.md")
    if re.search(r"(?im)^\s*Implemented boundary\s*:", conformance):
        error("CONFORMANCE_STATUS.md contains a scalar 'Implemented boundary:' claim")


def git_bytes(*args: str) -> bytes | None:
    try:
        return subprocess.check_output(
            ["git", "-C", str(ROOT), *args], stderr=subprocess.DEVNULL
        )
    except (FileNotFoundError, subprocess.CalledProcessError):
        return None


def git_success(*args: str) -> bool:
    try:
        completed = subprocess.run(
            ["git", "-C", str(ROOT), *args],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )
    except FileNotFoundError:
        return False
    return completed.returncode == 0


def explicit_ci_base() -> str | None:
    value = os.environ.get("DOCS_CONTROL_BASE", "").strip()
    if not value or set(value) == {"0"}:
        return None
    return value


def committed_ledger_baseline() -> str | None:
    # On a clean CI checkout, compare the committed result to the event's base
    # revision (which can span a multi-commit push), then fall back to HEAD^1.
    ci_base = explicit_ci_base()
    if ci_base is not None:
        if git_success("rev-parse", "--verify", f"{ci_base}^{{commit}}"):
            return ci_base
        error(
            f"DOCS_CONTROL_BASE {ci_base!r} is unavailable; "
            "CI must check out sufficient history"
        )
        return None

    if git_success("rev-parse", "--verify", "HEAD^1^{commit}"):
        return "HEAD^1"
    return None


def check_append_only_ledger(path: str) -> None:
    candidate = ROOT / path
    if not candidate.is_file():
        return
    # Check each local layer independently.  This prevents a staged rewrite
    # from hiding behind different working-tree bytes (or vice versa).
    staged_changed = not git_success(
        "diff", "--cached", "--quiet", "HEAD", "--", path
    )
    unstaged_changed = not git_success("diff", "--quiet", "--", path)

    head_bytes = git_bytes("show", f"HEAD:{path}") or b""
    index_bytes = git_bytes("show", f":{path}")
    if index_bytes is None:
        index_bytes = b""
    worktree_bytes = candidate.read_bytes()

    if staged_changed and not index_bytes.startswith(head_bytes):
        error(f"{path} staged content is not append-only relative to HEAD")
    if unstaged_changed and not worktree_bytes.startswith(index_bytes):
        error(f"{path} working content is not append-only relative to the index")
    if staged_changed or unstaged_changed:
        return

    baseline = committed_ledger_baseline()
    if baseline is None:
        return
    old = git_bytes("show", f"{baseline}:{path}")
    if old is None:
        # The ledger did not exist at the baseline; creation is an append from
        # the empty byte string.
        old = b""
    if not worktree_bytes.startswith(old):
        error(f"{path} is not append-only relative to {baseline}")


def main() -> int:
    check_root_topology()
    check_research_payload()
    check_canonical_source()
    decision_ids = parse_jsonl("DECISIONS.jsonl", "D")
    failure_ids = parse_jsonl("FAILURES.jsonl", "F")
    frontier = parse_frontier(decision_ids, failure_ids)
    check_control_migration_closure()
    check_active_references()
    check_state_ownership(frontier)
    check_append_only_ledger("DECISIONS.jsonl")
    check_append_only_ledger("FAILURES.jsonl")

    if ERRORS:
        print("documentation/control check FAILED:")
        for message in ERRORS:
            print(f"  - {message}")
        return 1
    print("documentation/control check PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
