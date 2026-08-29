# Inquiry Calculus

> **Branch mode — formal successor:** `codex/formal-successor` is an isolated construction branch.
> Its active objective is the Lean-checked successor under
> `formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md`, not continued semantic expansion of the
> v2.0 Rust implementation. The Rust workspace remains the predecessor reference and regression
> boundary until Formal Gate F.

Rust reference implementation of Inquiry Calculus v2.0: a typed relational language for answer-dependent inquiry, preserved actuality, regenerative compression, and cold replay.

Version 2.0 consolidates the accepted v1.1 substrate, positive-negation successor, paired actuality, and corrected interrogative succession into one forward authority. The version change does not restart the implementation or create executable conformance by itself.

## Authority

| Question | Active source |
|---|---|
| What the calculus means | `Inquiry_Calculus_v2_0.tex` |
| How the successor is constructed and accepted on this branch | `formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md` |
| Machine-checked candidate successor meaning | `formal/` (only to the coverage actually proved) |
| Final architecture, phase dependencies, and completion contract | `Inquiry_Calculus_v2_0_Comprehensive_Implementation_Plan.md` |
| How consequential engineering work proceeds | `AGENTS.md` and the `.claude` inquiry harness |
| The single strongest live executable residual | `IMPLEMENTATION_FRONTIER.md` |
| Successor evidence, decisions, and failures | `formal-successor/CONFORMANCE_STATUS.md`, `formal-successor/DECISIONS.jsonl`, `formal-successor/FAILURES.jsonl` |
| Demonstrated executable evidence | `CONFORMANCE_STATUS.md` |
| Accepted implementation choices and reopen conditions | `DECISIONS.jsonl` |
| Observed failures and environmental constraints | `FAILURES.jsonl` |
| Historical ancestry | Git |
| Derived breaker and regression material | `research/` |

Research is consulted only when a live residual needs one of its distinctions. It is not semantic authority or a second implementation plan.

## Workspace

- `ic-core` defines canonical identities, typed semantic forms, first-order inquiry programs, and derived finite checkers.
- `ic-runtime` verifies and coordinates runtime control, providers, dispatch, resolution, and cold replay without becoming semantic authority.
- `ic-store` provides immutable content-addressed artifacts, ordinary event history, and crash-recovery persistence through SQLite.
- `ic-cli` remains the narrow command-line boundary and contains no independent semantic machinery.
- `formal` is the pinned Lean/Lake proof project for the successor.
- `formal-successor` owns successor inputs, inventories, evidence, decisions, failures, and
  propagation reports. These do not overwrite predecessor ledgers. Its
  `AUTONOMOUS_ITERATION.md` runbook defines how a fresh run resumes and completes one finite ratchet.

## Build and verification

```bash
cargo fmt --all --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
```

The canonical specification is compiled with the pinned Tectonic workflow:

```bash
tectonic -X compile --keep-logs --outdir target/tex Inquiry_Calculus_v2_0.tex
```

See `.github/workflows/ci.yml` for the complete gate set.

The formal branch also runs:

```bash
node tools/successor_control_check.js
node tools/harness_control_check.js
node tools/predecessor_inventory.js check
node tools/predecessor_inventory_check.js
node tools/predecessor_tex_classification.js check
node tools/predecessor_tex_classification_check.js
node tools/predecessor_implementation_classification.js check
node tools/predecessor_implementation_classification_check.js
node tools/predecessor_fixture_classification.js check
node tools/predecessor_fixture_classification_check.js
node tools/phase_a_coverage.js check
node tools/phase_a_coverage_check.js
node tools/phase_b_predecessor_spine.js check
node tools/phase_b_predecessor_spine_check.js
cd formal
lake build --wfail
```

New inquiry traces pin the supplied question corpus and its declared engineering programs. A sealed
cycle cannot close until it records the residual-selected compiled Coding sequence with derived
relational coverage and accounts for every required Reciprocal Why challenge as represented or
individually blocked. The exact default sequence and residual schedule are projected in
`formal-successor/QUESTION_RHYTHM.md`.
The consolidated provisional method is pinned in
`formal-successor/SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md`. Its broad residual index is a deletable
projection rebuilt from append-only trace evidence and
`formal-successor/RESIDUAL_OBLIGATIONS.json`.
The pinned question-bank-derived computational extension is
`formal-successor/QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md`; it remains preformal and does
not authorize pre-Gate-F Rust semantic work.
The Phase A source universe and its non-promotion boundary are documented in
`formal-successor/PHASE_A_INVENTORY.md`; its generated inventory keeps unresolved classification
visible rather than converting source coverage into Gate A.
The source-bound six-way TeX review overlay and its non-promotion rules are documented in
`formal-successor/PHASE_A_TEX_CLASSIFICATION.md`.
The authority-separated Rust/schema overlay and exact-symbol candidate-edge rules are documented
in `formal-successor/PHASE_A_IMPLEMENTATION_CLASSIFICATION.md`.
The first Phase B ambient-metalanguage boundary and source-regenerative predecessor dependency
spine are documented in `formal-successor/PHASE_B_AMBIENT_BOUNDARY.md`; Gate B remains pending.

## Current implementation state

Moving status is intentionally not duplicated here:

- `IMPLEMENTATION_FRONTIER.md` names what is live now.
- `CONFORMANCE_STATUS.md` records exactly what executable checks have demonstrated.
- `DECISIONS.jsonl`, `FAILURES.jsonl`, and Git explain why the repository has its present shape.

Consequential changes follow `AGENTS.md`. Generated proposals, actual returns, decoded results, checks, warrant, and standing remain distinct throughout the implementation.

Test results are recorded as scoped consequence boundaries, not generic proofs of correctness:
each claim states its protected consequence, proposed condition, scope/semantics, probe, checker,
coverage, and reopening condition. A missing counterexample remains `Unknown` unless the declared
breaker field is independently established empty.
