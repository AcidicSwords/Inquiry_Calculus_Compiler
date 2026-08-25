# Inquiry Calculus

This repository contains the Rust reference implementation of Inquiry Calculus v1.1.
It is currently at **Phase 0**: repository authority, canonical artifact identity, and
the initial immutable-artifact persistence boundary. No Phase 1 semantic type system
is implemented yet.

## Authority

Development follows the current sources in this order:

1. `IMPLEMENTATION_FRONTIER.md`
2. `Inquiry_Calculus_v1_1_Comprehensive_Implementation_Plan.md`
3. `Inquiry_Calculus_Unified_Canonical_Specification_v1_1.tex` and accepted additions
4. `DECISIONS.jsonl` and `FAILURES.jsonl`
5. `CONFORMANCE_STATUS.md` and executable tests
6. Current code and observed build/runtime results

`AGENTS.md` governs repository work. The paired-actuality addition is an explanatory,
derived addition and does not introduce a second history ontology.

## Workspace

- `ic-core`: canonical artifact envelopes and content references.
- `ic-store`: SQLite migrations and verified immutable artifact storage.
- `ic-runtime`: reserved package boundary; no runtime semantics yet.
- `ic-cli`: reserved binary boundary; no command surface yet.

The initial implementation is single-process and uses one authoritative SQLite writer.
Semantic identities never depend on SQLite row IDs or filesystem locations.

## Checks

```text
cargo fmt --all --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
```

The system `sqlite3` executable is not required; migrations are embedded and exercised
through SQLx.
