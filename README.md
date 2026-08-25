# Inquiry Calculus

This repository contains the Rust reference implementation of Inquiry Calculus v1.1,
including the adopted **Successor Reciprocal-Boundary / Positive-Negation Edition**.
The semantic authority has advanced, but the executable implementation remains at
**Phase 2**: repository authority, canonical artifact identity, immutable-artifact
persistence, binding-scoped type artifacts, typed-form declarations, canonical formula
artifacts, checked relation schemas, immutable relation uses, and direct data-only
`OpenQuery` partial bindings. Query composition, reification, program, event, and standing
semantics are not implemented yet.

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

- `ic-core`: canonical artifact envelopes, binding-scoped `TyIR`, typed-form
  declarations, canonical formula artifacts, capture-safe typed terms, formula-defined or
binding-native relation schemas, canonical identity, and structural checking.
- `ic-store`: SQLite migrations, verified immutable artifact storage, and transactional
  insertion of explicitly declared artifact dependencies.
- `ic-runtime`: reserved package boundary; no runtime semantics yet.
- `ic-cli`: reserved binary boundary; no command surface yet.

The initial implementation is single-process and uses one authoritative SQLite writer.
Semantic identities never depend on SQLite row IDs or filesystem locations.

The Phase 1 grammar follows the canonical v1.1 semantics: binary product/sum,
`Prog(A)`, and unary `Code(A)`. A binding-local named type carries its immutable binding
and version identity. `Int`, `Text`, `Bytes`, n-ary product/sum, and input/output
`Code` remain unadopted plan candidates rather than silently assigned semantic identity.
Dependent `Sigma`/`Pi` types preserve an explicit checked family reference; the
binding-native family language and form reification remain later work.

Formula artifacts preserve the entire canonical surface grammar: `top`, `bottom`,
relation atoms, equality, conjunction, disjunction, implication, classical logical
negation, and existential/universal quantification. The classical minimal basis is an
explicit future derivation route, never a normalization that erases source structure.
Logical `Not` is not contextual typed negation and cannot create a `NegationUse`.
Relation schemas keep their ordered named port signature distinct from their semantic route:
their body is either a canonical formula with that exact typed context or a separately
identified binding-native contract artifact. Formula atom checking resolves that signature;
no host callback is admitted as unrecorded relation meaning.
An immutable `RelationUse` records one scoped occurrence, its typed bindings, required
evidence mode, support, and optional warrant separately from the reusable schema.
An `OpenQuery` partitions every relation port into an explicitly typed binding or a nonempty
open section. Checked `Bind` and `Expose` move exactly one port across that boundary while
leaving a nonempty question. `Plug` can construct a complete typed candidate assignment, but it
does not evaluate the relation, establish fiber membership, or manufacture actuality.
A completion fiber is represented only as a source-query-derived view that revalidates the
query; it is never collapsed into a selected completion.
Direct query normalization canonically orders ports by the relation schema and is idempotent;
it does not normalize through relation evaluation.

The Phase 3 source-program kernel has begun: `IProg` is canonical inspectable data with
`Return` and `Ask`; an ask carries a `QueryRef`, a unique ordered explicit environment of
named typed-form references, a named answer slot, and a continuation reference. It is not a
Rust closure or executable runtime program. The environment is part of the program identity and
declared dependency list, so a continuation has no hidden host-state capture.
Structural checking revalidates every named type, typed form, open query, environment value, and
continuation identity; `Return` values and continuations must share the enclosing result type.
The answer slot remains syntax until Phase 6/7 actuality and resolution provide an explicit
supported-answer representation. A `CompletionCandidate` is not such an answer. Substitution,
normalization, registered pure operations, and execution remain deferred rather than being
simulated with an unsupported candidate.

Phase 4 has begun with canonical `DeterminationPresentation` artifacts. A presentation records
one distinction orientation, typed source, claim-local relational-web reference, binding, scope,
applicability, grain, horizon, support, and optional predecessor presentation. It is neither a
complete fact store nor a departure, negation use, exterior, return, or standing revision.

When an artifact declares references, the caller supplies those references explicitly
to the store. The store checks their presence in the same transaction as the insert;
it never discovers references by parsing an opaque payload.

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
