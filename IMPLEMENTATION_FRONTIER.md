# Implementation Frontier

**Accepted semantic authority:** Inquiry Calculus v1.1, Successor
Reciprocal-Boundary / Positive-Negation Edition.

**Accepted implementation state:** Phase 0 local gates complete on 2026-08-24.

Adopting the successor changes the standing reciprocal-boundary contract, not the
implemented phase. The repository still has a pinned Rust workspace, an exact
versioned canonical artifact envelope, stable SHA-256 content references, one
artifact-only SQLite migration, and verified immutable insert/fetch behavior.
`ic-runtime` and `ic-cli` contain no semantic machinery. No current artifact payload
type contains references to another artifact, so the prohibition on dangling committed
references is not yet exercised.

## Strongest live obligation

Before implementing Phase 1, resolve:

> What is the smallest `TyIR` that faithfully realizes the current accepted type
> contract without silently choosing between the canonical specification and the
> implementation plan where their represented constructors differ?

The protected difference is currently visible in these candidates:

- The canonical type grammar includes `Prog(A)` and unary `Code(A)`.
- The implementation plan sketch omits `Prog`, uses input/output `Code`, and adds
  `Int`, `Text`, and `Bytes` constructors.

Different answers change which values are well typed and which serialized type
artifacts receive stable identities. Therefore Phase 0 does not define any of these
constructors. The next discriminator must be an accepted Phase 1 fixture for the exact
type grammar, followed by well-typed, ill-typed, and stable typed-identity tests.

## Known later questions

These are recorded now but do not outrank the Phase 1 obligation:

- **Phase 2:** reconcile the canonical negation/implication basis with the planned
  `FormulaIR` sketch before freezing formula serialization.
- **Phase 4 determination admission:** test whether the claim-local support/dependency
  web of the standing source determination, with exact scope, applicability, grain,
  horizon, version, and provenance, is the smallest lawful
  `DeterminationPresentation`; keep later minimization reversible.
- **Phase 4 reciprocal core:** implement positive departure, oriented and tagged
  `NegationUse`, distinct semantic/execution coverage, same-use return fibers,
  protected recovery, seed/reorientation, dependent sixfold occurrence views,
  residuals, and downstream `Gamma` checking. Boundary projection, failed search,
  unknown results, and protected non-equivalence alone must not manufacture exteriority.
- **Phase 6:** reconcile the canonical required boundary reference with the planned
  optional distinction reference, and represent request, attempt, raw return, and
  interpretation without collapsing them.
- **First referencing artifact:** extend the store transaction so every referenced
  artifact is present before committing a record; do not infer references from opaque
  payload bytes.

The global cross-binding standing-lift research question remains deferred to Phase 17
and does not block the next implementation phase. None of these queued questions
authorizes Phase 1 work during the authority migration.

## Phase 0 evidence

- Fixed envelope-v1 byte and SHA-256 vector passes.
- Canonical encode/decode, malformed input, domain separation, and property tests pass.
- SQLite migration application/reapplication and verified store behavior pass.
- Formatting, workspace check, warning-denied clippy, and full workspace tests pass.
