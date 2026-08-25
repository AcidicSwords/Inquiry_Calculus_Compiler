# Implementation Frontier

**Accepted semantic authority:** Inquiry Calculus v1.1, Successor
Reciprocal-Boundary / Positive-Negation Edition.

**Accepted implementation state:** Phase 2 relation-schema local gates complete on 2026-08-25.

Adopting the successor changes the standing reciprocal-boundary contract, not the
implemented phase. The repository has a pinned Rust workspace, exact versioned
canonical artifact envelopes, stable SHA-256 content references, one artifact-only
SQLite migration, verified immutable insert/fetch behavior, binding-scoped canonical
type artifacts, typed-form declarations, canonical formula artifacts with capture-safe typed
terms, formula-defined or binding-native relation schemas with checked named atom
signatures, and immutable scoped relation uses. `ic-runtime` and `ic-cli` contain no
semantic machinery.
Referencing artifacts now declare their dependencies explicitly;
the one-writer store checks those dependencies in the insertion transaction rather than
inferring them from opaque payload bytes.

## Strongest live obligation

Before completing Phase 2, resolve:

> What is the smallest canonical data-only representation of plugging, normalization,
> composition, dependent binding, and completion-fiber views now that `Bind` and `Expose`
> preserve the checked direct partial-binding form?

The protected difference is visible in the accepted sources:

- The canonical schema defines a data-only relational query IR around binding, exposure,
  hiding, renaming, and guarded restriction; the implementation plan names the more
  operational transform list.
- Direct `OpenQuery` now enforces the canonical partial-binding partition and nonempty open
  section; checked `Bind` and `Expose` preserve that form without running a relation.
- The next representation must distinguish syntactic data transforms from semantic relation
  evaluation and defer any actual completion fiber to a later executable route.

Different answers change whether query composition is inspectable and canonical or hides an
evaluation policy in a helper. The next discriminator must include a fixed transform encoding,
signature preservation for binding/exposure, composition with named-port provenance, and a
breaker proving that a completion-fiber view is not an actual answer.

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

## Phase 2 relation-schema evidence

- Relation schemas canonically preserve binding, ordered named typed ports, body route,
  law references, and provenance references.
- A formula-defined body must have the schema's exact port-type context; a binding-native
  body has an immutable contract artifact reference and no executable host callback.
- Formula atoms validate arity and argument types against the resolved named signature.
- Fixed relation bytes/SHA-256, decoding failure cases, duplicate-port rejection, and
  formula-context checks pass.
- A relation use is a separately content-addressed occurrence and rejects unknown bound ports.
- Direct open queries retain only a typed complete port partition with a nonempty open section;
  they do not execute relations or claim completion fibers. Checked `Bind` and `Expose` move
  one port only while preserving that invariant.
- A fixed independently calculated direct OpenQuery payload/envelope/SHA-256 fixture passes.
