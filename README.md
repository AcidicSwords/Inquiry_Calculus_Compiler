# Inquiry Calculus

This repository contains the Rust reference implementation of Inquiry Calculus v1.1,
including the adopted **Successor Reciprocal-Boundary / Positive-Negation Edition**.
The semantic authority has advanced, but the executable implementation remains at
**Phase 0**: repository authority, canonical artifact identity, and the initial
immutable-artifact persistence boundary. No Phase 1 semantic type system is
implemented yet.

## Authority by question

Authority is scoped to the question it answers rather than treated as one global
priority list.

| Question | Governing source |
|---|---|
| Task scope and delivery | Explicit user request |
| Standing semantics | `Inquiry_Calculus_Unified_Canonical_Specification_v1_1.tex` and the accepted paired addition |
| Architecture and phase order | `Inquiry_Calculus_v1_1_Comprehensive_Implementation_Plan.md` |
| Strongest live implementation question | `IMPLEMENTATION_FRONTIER.md` |
| Accepted local choices | `DECISIONS.jsonl` |
| Repository actuality | Code, Git state, builds, tests, and tool returns |
| Demonstrated conformance | Tests and `CONFORMANCE_STATUS.md` |
| Observed constraints | `FAILURES.jsonl` |

`AGENTS.md` compiles repository work into an inspectable, answer-dependent inquiry
program. A still-supported instruction is a cached answer, not a reason to reopen a
settled question. The paired-actuality addition remains an explanatory, derived
addition and does not introduce a second history ontology.

The predecessor implementation plan and canonical specification remain retrievable at
Git commit `49dc381ac230326aa28be6c157ece0d21a31eaa2` as ancestry and regression
evidence; they are not coequal forward authority.

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
tectonic -X compile --keep-logs --outdir target/tex Inquiry_Calculus_Unified_Canonical_Specification_v1_1.tex
```

The system `sqlite3` executable is not required; migrations are embedded and exercised
through SQLx. CI obtains the official Tectonic 0.17.0 binary by a pinned URL and
SHA-256 digest, compiles the canonical specification into ignored `target/tex`, and
does not commit or upload the generated PDF.
