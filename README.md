# Inquiry Calculus

This repository contains the Rust reference implementation of Inquiry Calculus v1.1,
including the adopted **Successor Reciprocal-Boundary / Positive-Negation Edition**.
The semantic authority has advanced. The executable implementation has verified structural
increments through Phases 3--7 and a limited Phase 10 compiled-operator boundary; it is not a
completed semantic runtime. The accepted and demonstrated boundary is recorded precisely in
`CONFORMANCE_STATUS.md` and `IMPLEMENTATION_FRONTIER.md`.

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
- `ic-store`: SQLite migrations, verified immutable artifact storage, transactional insertion of
  explicitly declared artifact dependencies, and an append-only ordinary event ledger.
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

Phase 5 has a first-order structural `ProgramIR` in `ic-runtime`: typed `Return`, nonempty
internal `Branch`, and `Probe` suspension/resume blocks. Verification rechecks typed returns,
closed targets, and the representable guarded-recurrence boundary; a branch-only cycle is
rejected. Stepping a probe only creates a suspension, and resumption carries a `RawReturnRef`
without dispatching, decoding, or treating it as an actual event. Probe operator schemas and
raw-return-dependent continuation selection remain later contracts.

`ProbeOperator` now has a canonical compiled-operator identity shared with the runtime probe
terminator. Its explicit fields are query, boundary chart, active view, backend, executable-code,
return type, decoder contract, probe contract, and compiler version. The artifact remains distinct
from surface plans, backend requests, and raw returns; creating or naming it does not render a
request, dispatch a backend, decode a result, or create an actual event.

`ProbeContract` now gives the recurrent probe-contract fields their own canonical identity:
relational role, binding version, grain, applicability, comparator, protected horizon, decoder
version, and bridge policy. `ProbeOperator` references that typed identity. This is structural
identity only: neither `SameContract`/`Bridge` comparability nor bridge-policy evaluation,
request construction, rendering, dispatch, decoding, or actuality is implemented.

Phase 4 has begun with canonical `DeterminationPresentation` artifacts. A presentation records
one distinction orientation, typed source, claim-local relational-web reference, binding, scope,
applicability, grain, horizon, support, and optional predecessor presentation. It is neither a
complete fact store nor a departure, negation use, exterior, return, or standing revision.

Phase 4 also has canonical `DepartureWitness` identity. It binds a source determination
presentation to typed source/candidate observations, their typed fillings, an incompatibility-use
identity, support, and context. The artifact itself is not an exterior predicate and cannot turn
failed equality, search, retrieval, projection, unknown status, or protected non-equivalence into
departure. Its current structural check rehashes and checks the named presentation, forms, and
relation uses; requires the presentation's distinction, source, binding, scope, applicability,
grain, and horizon; and rejects a use outside that context. It still does not evaluate an
observation, establish incompatibility, or certify non-circular positive evidence. Each
observation use must bind its claimed form/answer pair, and the incompatibility use must bind
the two claimed answers; those structural links are not relation evaluation.

For the exact finite special case, a derived cell comparator can expose one observed unequal
coordinate, retain `Unknown` when the table lacks a positive separator, and distinguish fully
observed equality only within that table. Its 65,536-pair binary fixture is a mathematical
separator breaker, not a typed observation result, incompatibility certificate, departure witness,
or exteriority claim.

The companion finite incompatibility checker accepts only a caller-declared ordered pair table:
one listed observed pair yields a positive finite witness, an unlisted observed pair yields
`NoWitness`, and an unknown observation remains `Unknown`. It neither type-checks values nor
establishes that the table is standing, supported, relevant, covered, or connected to a
`RelationUse` or `DepartureWitness`.

The typed finite companion retains `TypedFormRef` values instead. Before it returns a listed-pair
witness, it resolves, rehashes, and type-checks every table entry and both observed declarations;
the two sides may have distinct declared types. An invalid declaration is an explicit validation
error, an unlisted valid pair remains `NoWitness`, and an unavailable value remains `Unknown`.
This is still derived finite evidence only: it does not actualize an observation, admit a standing
incompatibility relation, establish relevance, coverage, support, or non-circularity, connect a
`RelationUse`, or establish departure/exteriority.

Phase 4 also represents an oriented `NegationUse` declaration as a distinct immutable relation
use tied to one determination presentation. Its semantic coverage (`ExactExhaustive`,
`ExactOnField`, `CertifiedPartial`, or `WorkingOpen`) is identity data and deliberately differs
from later execution/materialization coverage. The checker validates the presentation context,
relation-use and candidate-field schemas, and first-order soundness-program identity. It neither
executes that program nor admits soundness, coverage, a frontier member, or a positive-negation
incidence.

A derived `NegationFrontierView` retains the tagged family of active use views without merging
them: each member keeps its use identity, source, candidate-field relation, semantic coverage,
and separately typed execution-coverage reference. It rejects duplicate use tags and mixed
sources, but does not rehash or admit supplied declarations, infer collective coverage, generate
candidates, or establish a negation incidence.

Phase 6 preserves exact opaque probe-return bytes as immutable, domain-separated `RawReturn`
artifacts before decoding. A raw return alone is not an actuality assertion, attempt, completion,
interpretation, check, or warrant. The ordinary `ActualEvent` record resolves the former source
shape discrepancy conservatively: it requires the canonical `BoundaryRef` and separately retains
the plan's optional `DistinctionRef`, alongside parent, state, question, operator, raw return,
grain, route, binding, backend-version, and provenance references. The SQLite event ledger writes
the event artifact and head-linked ledger row atomically; it rejects forks/stale parents and
non-raw return, non-query, non-chart boundary, or non-operator artifacts. It also requires the
event's rehashed query to equal both the chart and compiled operator query, the operator boundary
to equal the event boundary, the query/chart/event grains to agree, and the query/chart horizons
to agree; these are occurrence identity links, not semantic evaluation. It does not dispatch a
probe, validate opaque state/boundary/operator contracts, decode the result, resolve an answer,
or prove a semantic interpretation.

The event ledger has a file-backed restart witness: closing its single connection and reopening the
database preserves canonical event identity and parent-linked order after embedded migrations are
reapplied. This is a persistence and integrity check only; state-transition replay, dispatch, raw
return resolution, and accepted-state reconstruction remain later contracts.

Phase 7 has a typed, first-order `ResolutionPath`: identity, decoder, relation, composition, and
program routes each preserve their input/output types and referenced route identity. Checking
revalidates types and exact composition interfaces, including cycle rejection. It does not run a
decoder, relation, or program; name a supported answer set; resolve a raw return; or turn a
partial result into an exact answer.

The reusable exact finite `DetermineThrough` facility implements the canonical kernel-inclusion
test for deterministic, fully covered signatures sharing binding, scope, applicability, grain,
horizon, and domain. It returns either the explicit factor map or a pair witnessing equal
available output with protectedly different target output. Working, partial, nondeterministic, or
incompletely covered inputs are not accepted as exact signatures.

Phase 4 also has derived three-valued recovery result data: `Recovered` requires an explicit
certificate reference, `NotRecovered` requires an explicit separator reference, and `Unknown`
retains an open-query residual. The exact finite recovery checker can only inspect a separately
certified complete same-use return-fiber signature table: it reports constant protected
signatures or supplies two differing candidates. It rejects an empty table and does not create a
return fiber, validate its certificate, turn incomplete coverage into loss, or establish an
actual occurrence.

For exact finite signatures, a tagged `ExactFamilySignature` can preserve the product of several
separately covered component observations. Its factorization checker demonstrates joint
information gain when the product distinguishes a protected target that no member does alone, or
emits a concrete joint kernel separator. This is a derived information calculation only: it does
not claim that component observations were co-applicable, jointly realized, or one composite
actual event.

`BoundaryChart` is now a derived, content-addressed local chart record. It retains the query,
`X`/`Y`/boundary type references, projections, determination references, oriented use frontiers,
seed, compatibility formula, optional traversal, grain, and horizon. It preserves missing fields
as missing and does not infer a partition, exterior, return, coverage, reciprocal role, or sixfold
view from the record. Field-level semantic validation awaits the typed boundary/operator/state
contracts that those fields reference.

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
