# Implementation Frontier

**Accepted semantic authority:** Inquiry Calculus v1.1, Successor
Reciprocal-Boundary / Positive-Negation Edition.

**Accepted implementation state:** Phase 3 first-order source-program syntax/checking, Phase 4
determination-presentation/departure/negation-declaration identity checking, and Phase 5
structural runtime control-flow checking pass on 2026-08-25. Phase 7 typed resolution-path
identity and interface composition checking also pass. Supported answer materialization remains
reserved for the Phase 6/7 actuality-and-resolution boundary.

Adopting the successor changes the standing reciprocal-boundary contract, not the
implemented phase. The repository has a pinned Rust workspace, exact versioned
canonical artifact envelopes, stable SHA-256 content references, one artifact-only
SQLite migration, verified immutable insert/fetch behavior, binding-scoped canonical
type artifacts, typed-form declarations, canonical formula artifacts with capture-safe typed
terms, formula-defined or binding-native relation schemas with checked named atom
signatures, immutable scoped relation uses, opaque raw-return identity, and a first-order
runtime control-flow verifier. `ic-cli` contains no semantic machinery. A `RawReturn` preserves
exact external bytes before decode but is not an actual event, attempt, or semantic completion.
Referencing artifacts now declare their dependencies explicitly;
the one-writer store checks those dependencies in the insertion transaction rather than
inferring them from opaque payload bytes.

## Strongest live obligation

Before admitting a departure witness as evidence for a later negation use, resolve:

> What is the smallest typed observation-result and incompatibility-certificate boundary that
> can positively establish departure without turning failed search, projection, or incomplete
> coverage into exteriority or silently collapsing semantic and execution coverage?

The protected difference is visible in the accepted sources:

- `DeterminationPresentation` now has a canonical artifact identity for distinction, `X`/`Y`
  orientation, typed source, relational-web reference, binding, scope, applicability, grain,
  horizon, support, and optional predecessor. These fields all domain-separate the presentation.
- The relational web is deliberately not inferred from every retained fact or every relation
  mentioning the source. The canonical reversible default remains the support/dependency web of
  the specific standing claim; its exact admission/minimization is a research gate.
- A `CompletionCandidate` remains only a full typed query filling. It does not establish relation
  membership, support, actuality, a resolution path, or a selected answer.
- Determination checking rehashes and checks the typed source, requires its binding to match the
  presentation, and recursively checks an optional predecessor. An ancestry edge may change the
  web or support, but not distinction, orientation, source, binding, scope, applicability, grain,
  or horizon. It does not admit the web or generate departure.
- `DepartureWitness` canonically retains the source/candidate forms, source presentation,
  source and candidate observation uses, their typed fillings, incompatibility use, support,
  scope, applicability, and grain. Its checker rehashes every resolved identity, type-checks its
  forms, requires the presentation's source and indexed context, and rejects relation uses with
  different scope/applicability/grain/horizon. Constructing or checking it evaluates no relation
  and proves neither incompatibility nor exteriority.
- `NegationUse` now records one distinct oriented relation-use declaration, its source
  determination, candidate field, structural soundness-program reference, semantic-coverage
  declaration, indexed context, and provenance. Its checker rejects a different presentation
  orientation/context, forged or incompatible schemas/use, and forged program identity. It does
  not execute the program, establish soundness/coverage, or admit a negation incidence.
- `RawReturn` domain-separates and content-addresses exact opaque return bytes. It is admissible
  to the existing artifact store as ordinary immutable content but carries no ledger or
  actualization assertion by itself; the unresolved event-record boundary/distinction shape
  remains deferred.
- `ProgramIR` has only the canonical runtime terminators: typed `Return`, nonempty `Branch`, and
  `Probe` with an explicit resume target. Its verifier rechecks returns and target closure and
  rejects presently unguarded branch-only recurrence. A probe step suspends and a resumption
  carries a `RawReturnRef`; neither step calls an operator, records an event, decodes a result, or
  chooses a raw-return-dependent continuation.
- `ResolutionPath` now records typed identity, decoder, relation, composition, and program route
  syntax with exact source/target type composition. Its checker validates only types and composed
  interfaces; it does not run or admit the referenced route, derive a decoder result, or create a
  supported answer.

Different answers determine whether a positive certificate can distinguish an actual supported
departure from merely coexisting identifiers, failed work, and incomplete evidence. The next
discriminator must reject a purported departure whose observation result, incompatible answers,
or derivation route is not independently represented and checked.

## Known later questions

These are recorded now but do not outrank the Phase 4 determination boundary:

- **Phase 4 determination admission:** test whether the claim-local support/dependency
  web of the standing source determination, with exact scope, applicability, grain,
  horizon, version, and provenance, is the smallest lawful
  `DeterminationPresentation`; keep later minimization reversible.
- **Phase 3/7 answer materialization:** retain the `Ask` answer binder as first-order syntax
  until the Phase 6 actuality and Phase 7 resolution/fiber contracts supply a supported answer
  representation. Do not coerce a `CompletionCandidate` into an answer, perform substitution,
  select a branch, or claim normalization based on it.
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
- Plugged candidate assignments preserve query provenance and complete typed bindings, but have
  no semantic membership, actuality, support, or warrant status.
- CompletionFiberView is a source-query-derived, revalidated view only; it neither enumerates
  the fiber nor identifies any candidate as a member.
- Direct normalization reorders bound and open ports by the checked schema signature and is
  idempotent; it changes no bindings, modes, relation body, or completion claim.
- RelationExprArtifact canonically stores `Relation`, `Bind`, `Join`, `Expose`, `Hide`, `Rename`,
  and `Guard` as data-only syntax with explicit dependencies; it does not evaluate them.
- First-order IProg artifacts canonically encode `Return` and `Ask` with explicit QueryRef,
  unique named typed-form environment, answer-slot name, and continuation IProgRef; no host
  closure or hidden captured state exists.
- A fixed independently calculated direct OpenQuery payload/envelope/SHA-256 fixture passes.
