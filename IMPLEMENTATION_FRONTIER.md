# Implementation Frontier

**Accepted semantic authority:** Inquiry Calculus v1.1, Successor
Reciprocal-Boundary / Positive-Negation Edition.

**Accepted implementation state:** Phase 2 formula-kernel local gates complete on 2026-08-25.

Adopting the successor changes the standing reciprocal-boundary contract, not the
implemented phase. The repository has a pinned Rust workspace, exact versioned
canonical artifact envelopes, stable SHA-256 content references, one artifact-only
SQLite migration, verified immutable insert/fetch behavior, binding-scoped canonical
type artifacts, typed-form declarations, and canonical formula artifacts with
capture-safe typed terms. `ic-runtime` and `ic-cli` contain no semantic machinery.
Referencing artifacts now declare their dependencies explicitly;
the one-writer store checks those dependencies in the insertion transaction rather than
inferring them from opaque payload bytes.

## Strongest live obligation

Before completing Phase 2, resolve:

> What is the smallest relation-schema definition boundary that admits both formula
> defined and binding-native relations, while letting formula atoms check their named
> port arity and types without introducing a hidden host callback?

The protected difference is visible in the accepted sources:

- The canonical schema gives a named typed signature and permits binding-native
  relations; it does not say that every relation must be reducible to a formula.
- The implementation plan records `body: FormulaRef` while also requiring relations to
  remain binding-native and forbidding an unrecorded host callback.
- Formula atoms now retain relation references and typed terms, but their signature
  checks cannot be completed until a resolved relation schema supplies port types.

Different answers change whether a primitive binding relation is representable and
whether an atom can be shown well typed. The next discriminator must include a
formula-defined relation, an explicit binding-native relation contract, wrong arity,
wrong port type, duplicate port name, and nonempty open-port fixtures.

## Known later questions

These are recorded now but do not outrank the remaining Phase 2 obligation:

- **Phase 4 determination admission:** test whether the claim-local support/dependency
  web of the standing source determination, with exact scope, applicability, grain,
  horizon, version, and provenance, is the smallest lawful
  `DeterminationPresentation`; keep later minimization reversible.
- **Phase 4 reciprocal core:** implement positive departure, oriented and tagged
  `NegationUse`, distinct semantic/execution coverage, same-use return fibers,
  protected recovery, exact `DetermineThrough` factorization, seed/reorientation,
  dependent sixfold occurrence views, residuals, and downstream `Gamma` checking.
  A family signature is accumulated information, not one jointly actualizable return;
  composite actualization requires supported jointness. Boundary projection, failed
  search, unknown results, and protected non-equivalence alone must not manufacture
  exteriority. Recovery checks remain three-valued; a coverage-indexed constitutive
  characterization is a derived view, not an authoritative object or self-warranting
  horizon.
- **Phase 6:** reconcile the canonical required boundary reference with the planned
  optional distinction reference, and represent request, attempt, raw return, and
  interpretation without collapsing them.
- **Phase 10/12 method boundary:** register typed, law-carrying method contracts;
  preserve raw actual returns; separate certified semantic non-discharge from backend
  failure; and route typed residual handlers/reentry through first-order `IProg` without
  a new runtime opcode.
- **Phases 15-16 extension and approximation:** claim question-language monotonicity
  only for conservative extensions with an explicit old-question embedding, and retain
  directional approximation soundness plus extension-sensitive reopening.
- **Phases 13 and 16 regenerative economy:** compute exact finite sufficient cue bases
  and retain incomparable minima under the declared preorder; minimize only among
  licensed inquiry-regenerative representations. A linear dot-product binding may use
  the exact second-moment consequence-subspace certificate, but sampled/floating
  estimates remain approximate and query-distribution change reopens the fold.

The global cross-binding standing-lift research question remains deferred to Phase 17
and does not block the next implementation phase. None of these queued questions
authorizes Phase 2 completion before its relation-schema discriminator has been established.

## Phase 0 evidence

- Fixed envelope-v1 byte and SHA-256 vector passes.
- Canonical encode/decode, malformed input, domain separation, and property tests pass.
- SQLite migration application/reapplication and verified store behavior pass.
- Formatting, workspace check, warning-denied clippy, and full workspace tests pass.

## Phase 1 evidence

- The canonical `TyIR` grammar encodes/decodes every accepted constructor, including
  binary product/sum, `Prog(A)`, and unary `Code(A)`.
- A fixed independently calculated named-type payload/envelope/SHA-256 fixture passes.
- Canonical type and typed-form envelope decoding reject malformed, truncated, trailing,
  wrong-kind, and wrong-schema encodings.
- A binding-aware type catalog rejects missing children, forged identities, binding scope
  mismatches, and dependent-family domain mismatches.
- Explicit dependency insertion rejects an absent prerequisite without committing the
  dependent artifact; dependency-complete repeat insertion is idempotent.

## Phase 2 formula evidence

- Every canonical surface formula constructor has a distinct canonical encode/decode
  route; no basis normalization is applied during identity construction.
- A fixed independently calculated top-formula payload/envelope/SHA-256 fixture passes.
- Formula decoding rejects malformed, truncated, trailing, wrong-kind, and wrong-schema
  inputs.
- Formula contexts use typed de Bruijn variables; quantifier bodies, typed-form terms,
  equality, and nested formula contexts are structurally checked.
- FormulaIR contains only classical logical `Not`; contextual `NegationUse` remains a
  later relation-use contract.
