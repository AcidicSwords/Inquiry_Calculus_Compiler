# Inquiry Calculus

Rust reference implementation of Inquiry Calculus v2.0: a typed relational language for answer-dependent inquiry, preserved actuality, regenerative compression, and cold replay.

Version 2.0 consolidates the accepted v1.1 substrate, positive-negation successor, paired actuality, and corrected interrogative succession into one forward authority. The version change does not restart the implementation or create executable conformance by itself.

## Authority

| Question | Active source |
|---|---|
| What the calculus means | `Inquiry_Calculus_v2_0.tex` |
| Final architecture, phase dependencies, and completion contract | `Inquiry_Calculus_v2_0_Comprehensive_Implementation_Plan.md` |
| How consequential engineering work proceeds | `AGENTS.md` and the `.claude` inquiry harness |
| The single strongest live executable residual | `IMPLEMENTATION_FRONTIER.md` |
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

## Current implementation state

Moving status is intentionally not duplicated here:

- `IMPLEMENTATION_FRONTIER.md` names what is live now.
- `CONFORMANCE_STATUS.md` records exactly what executable checks have demonstrated.
- `DECISIONS.jsonl`, `FAILURES.jsonl`, and Git explain why the repository has its present shape.

Consequential changes follow `AGENTS.md`. Generated proposals, actual returns, decoded results, checks, warrant, and standing remain distinct throughout the implementation.
