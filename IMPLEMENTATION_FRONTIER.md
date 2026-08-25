# Implementation Frontier

**Accepted semantic authority:** Inquiry Calculus v1.1, Successor
Reciprocal-Boundary / Positive-Negation Edition.

**Accepted implementation state:** Phase 3 first-order source-program syntax/checking, Phase 4
determination-presentation/departure/negation-declaration identity checking, and Phase 5
structural runtime control-flow checking pass on 2026-08-25. Phase 7 typed resolution-path
identity and interface composition checking also pass. Phase 6 now has canonical ordinary-event
identity and append-only journal linkage; supported answer materialization remains reserved for
the Phase 6/7 actuality-and-resolution boundary.

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
  different scope/applicability/grain/horizon. Each source/candidate observation use must also
  bind its declared form and answer, and the incompatibility use must bind the two declared
  answers. Each of the three evidence uses must additionally declare an evidence route other than
  `Generate`: a generator proposes a provisional filling and can never supply positive departure
  evidence. The other four modes are retained, so a `Pure` derivation from already-standing data
  remains lawful and the rule rejects generation rather than everything that is not a probe.
  This is a check on the *declared* route only. A use that declares `Probe` while nothing was
  probed still passes, so the discharge mode is not yet evidence that the route was taken.
  Constructing or checking it evaluates no relation and proves neither incompatibility
  nor exteriority.
- The derived `compare_finite_observation_cells` checker now supplies the exact finite
  cell-exclusion discriminator: an observed unequal coordinate gives a separator; no separator
  with any unknown coordinate remains `Unknown`; a completely observed equal table is only a
  same-table result. Its exhaustive binary fixture covers 65,536 pairs. It neither gives those
  coordinate values typed observation provenance nor establishes relevance, standing
  incompatibility, support, a `DepartureWitness`, or exteriority.
- The derived `check_finite_incompatibility` checker adds the smallest positive pair condition:
  an observed ordered pair listed in a caller-declared finite table yields a witness; an unlisted
  observed pair is `NoWitness`; either unknown input is `Unknown`. It does not type-check values,
  admit the table as standing, prove relevance/coverage/support, connect to a relation use, or
  establish departure/exteriority.
- The derived `check_typed_finite_incompatibility` checker carries `TypedFormRef` values and
  resolves, rehashes, and type-checks each declared table member plus both observed forms. It
  permits distinct source/candidate types, returns `NoWitness` for an unlisted well-typed pair,
  retains `Unknown` for a missing value, and rejects forged or invalid declarations explicitly.
  It does not actualize observations, make the table standing, prove incompatibility relevance,
  coverage, support, or non-circularity, link to a relation use, or establish departure/exteriority.
- `TaggedExteriorClaim` now preserves the next dependent role as a derived view: immutable
  negation-use tag, source, candidate, departure-witness, and separately typed execution-coverage
  identity. Its checker rehashes and structurally checks the named use and witness, then requires
  their source/candidate and presentation/distinction/scope/applicability/grain context to agree.
  The same candidate through different tags remains distinct. This does not evaluate an incidence,
  make a use admitted, establish an exterior, actualize a result, or confer support/warrant.
- `NegationUse` now records one distinct oriented relation-use declaration, its source
  determination, candidate field, structural soundness-program reference, semantic-coverage
  declaration, indexed context, and provenance. Its checker rejects a different presentation
  orientation/context, forged or incompatible schemas/use, and forged program identity. It does
  not execute the program, establish soundness/coverage, or admit a negation incidence.
- `NegationFrontierView` now represents the plan's derived tagged use family: each
  `ActiveNegationUse` retains the `NegationUseRef`, source, candidate-field relation, declared
  semantic coverage, and a separately typed opaque generator/execution-coverage reference.
  The view preserves member order/tags, rejects duplicate uses and a source mismatch, and lets an
  empty frontier remain empty. It has no combined-negation relation, candidate incidence,
  collective-coverage inference, rehashing/admission of its supplied declarations, or closure
  authority.
- `RawReturn` domain-separates and content-addresses exact opaque return bytes. It is admissible
  to the existing artifact store as ordinary immutable content but carries no ledger or
  actualization assertion by itself.
- `ActualEvent` resolves the previously live canonical/plan record mismatch conservatively: it
  has the canonical required `BoundaryRef` and the plan's separate optional `DistinctionRef`.
  It also preserves parent, state, question, operator, raw return, grain, route, binding,
  backend-version, and provenance identities. `ArtifactStore` appends that event envelope and its
  ledger edge transactionally, requires the supplied parent to equal the current head, rechecks
  the raw-return artifact kind and hash, and detects event/ledger corruption on reads. It does
  not dispatch an operator, independently prove that a tool call occurred, validate opaque state
  or boundary semantics, decode a raw return, resolve an answer, or establish a claim.
- Event and ledger checks now require the named `BoundaryRef` to resolve and rehash as a genuine
  `BoundaryChart`, in addition to requiring the raw-return artifact. They do not validate the
  chart's opaque projections, determinations, use frontiers, seed, compatibility, or open roles.
- `ActualEvent` now uses the shared `ProbeOperatorRef` type and checks the named artifact as a
  genuine `ProbeOperator` alongside boundary and raw return. This verifies only operator identity,
  not backend, executable code, contracts, rendering, dispatch, or effect actuality.
- Actual-event checking and ledger append/read now also rehash a genuine `OpenQuery` and require
  one coherent occurrence context: event question equals both the chart and operator questions,
  operator boundary equals event boundary, query/chart/event grains agree, and query/chart
  horizons agree. These equalities do not validate query semantics, chart fields, operator
  contracts, route, dispatch, decoding, raw-return causality, or interpretation.
- The file-backed SQLite journal has a restart witness: after the authoritative connection closes,
  reopening and reapplying embedded migrations preserves the immutable event identity and ordered
  ledger chain. This is persistence/revalidation only, not replay of state transition semantics,
  dispatch, resolution, or accepted interpretation.
- `ProgramIR` has only the canonical runtime terminators: typed `Return`, nonempty `Branch`, and
  `Probe` with an explicit resume target. Its verifier rechecks returns and target closure and
  rejects presently unguarded branch-only recurrence. A probe step suspends and a resumption
  carries a `RawReturnRef`; neither step calls an operator, records an event, decodes a result, or
  chooses a raw-return-dependent continuation.
- `ProbeOperator` is a canonical compiled-operator artifact shared by the runtime's `Probe`
  terminator. It records query, boundary chart, active view, backend, executable-code, return
  type, decoder contract, probe contract, and compiler-version identities. It is deliberately
  distinct from a backend request and raw return, and does not validate, render, dispatch, decode,
  or actualize any field.
- `ProbeContract` now canonically identifies the recurrent probe fields: relational role, binding
  version, grain, applicability, comparator, protected horizon, decoder version, and bridge
  policy. `ProbeOperator` carries its typed reference. The artifact has no `SameContract` or
  `Bridge` evaluator, makes no occurrences comparable, and does not turn a bridge policy into a
  standing bridge relation.
- `ResolutionPath` now records typed identity, decoder, relation, composition, and program route
  syntax with exact source/target type composition. Its checker validates only types and composed
  interfaces; it does not run or admit the referenced route, derive a decoder result, or create a
  supported answer.
- The reusable finite exact `DetermineThrough` checker validates shared indexed context and
  coverage before applying kernel inclusion. It returns an explicit factor map or a concrete
  separator pair; it does not accept incomplete, working, or nondeterministic data as exact and
  does not itself create recovery, standing, or actuality.
- The exact finite family factorization checker preserves an ordered tagged product of separately
  exact signatures. It rejects empty families and mismatched indexed domains, reports a product
  factor map only under kernel inclusion, and otherwise gives a pair with identical family values
  and different targets. The product is accumulated information, not co-applicability,
  joint-realizability, a composite raw return, or a composite actual event.
- `positive_negation_query` now builds the plan's step-10 question `?y[N_u(x, y)]` as an ordinary
  `OpenQuery`: the presented source stays bound, the remaining schema ports are exposed under a
  declared discharge route, and the context is inherited from the negation use. It refuses a
  relation whose ports are all bound, because that is a proposition rather than a question, and
  refuses a use that does not bind the presented source, because that asks which pairs the
  relation relates rather than what is exterior to this source. The result retains the negation-use
  tag and the declared semantic coverage so neither is dropped on the way to an answer. Building
  the question evaluates no relation, produces no candidate, and creates no exterior; an answer
  reached generatively would still be a generated `O_X`.
- `SeedReorientation` now represents the plan's step-14 seed `Seed_Y(O_X, S_Y)`. It takes a
  `TaggedExteriorClaim` rather than a bare form, so `O_X` arrives carrying the `X`-side use tag
  that section 38 requires to remain occurrence provenance. `exterior_form` and
  `reoriented_source` are separate fields and stay separate when they hold the same form: an
  identity seed still names a relation use, and that use must bind the form twice, once for each
  role. Checking revalidates the exterior claim, rehashes the seed use, refuses a merely
  generative route, and requires the seed to share the negation use's indexed context. It does not
  evaluate the seed relation, select `S_Y`, admit the reorientation, or make the reciprocal side
  actual; `is_identity_seed` reports a coincidence of fillings, never a collapse of roles.
- Steps 10, 11, and 14 are joined by a declared finite extension rather than by evaluation. Plan
  section 77 makes a deterministic finite list the first lawful realisation of a negation
  frontier, so this is the planned route and not a stopgap; what remains out of phase is the
  general case, where a formula-bodied relation would need binding-supplied denotation to be
  decided. `TypedFiniteNegationExtension` resolves and rehashes its negation use, reads that
  use's relation, requires one bound source port and one open candidate port, and type-checks
  every declared incidence against the port it fills. It supplies the forward section
  `NegField_u(s)` that section 26 makes the candidate field of a positive-negation question, and
  erases to the untyped extension so the existing reverse-section, selection, and closure entry
  points apply unchanged. Membership in that field is candidacy under a declared list: a candidate
  there is not an exterior and becomes one only through its own departure witness, and an empty
  field reports an empty declaration rather than the absence of an exterior. The extension is
  still declared, not derived from a relation body.
- `check_declared_incidence` closes the last unchecked joint in the `X` chain. The specification
  defines the role as recording `e` in `NegField_u(s)`, and `TaggedExteriorClaim` had to omit that
  membership for want of a field, saying so in its own documentation. A claim checked against a
  declared extension must now name an incidence that extension relates to its own source under its
  own use; a candidate related to some *other* source in the same extension is refused. This
  establishes the declared incidence only, and does not relieve the departure witness of
  establishing exteriority. The claim artifact deliberately holds no extension reference, so
  `TaggedExteriorClaim::check` alone still does not require an incidence: the requirement applies
  when a caller supplies an extension, and making it unconditional would mean admitting an
  extension into the negation use itself.
  This does **not** put the sixfold view out of reach. Plan section 43 marks `ox_occurrence`,
  `rx`, `ry`, and `compatibility` optional and calls the view non-authoritative, so a partial
  occurrence carrying `Unknown` where actuality is absent is its designed shape, not a
  degenerate one. What the view still lacks is a checked notion behind its `rx` field: section 28
  makes `R_X` a supported selection *from* the return fiber and distinct from the fiber.
- `SelectedReturn` now supplies that role. Selection refuses any source the return does not admit,
  so `R_X` is drawn from the fiber by construction, and `check_return_closure` takes the selection
  but derives its verdict from every source the fiber admits: a selected return equal to the
  source, entirely stable when read alone, still reports `Open` with the concrete surviving pair
  while a second protected class remains. Membership is enforced; the selection's *support* is not
  represented, because supported answers await the Phase 6/7 boundary.
- `ReciprocalOccurrence` now assembles the derived sixfold view. Its fields are the dependency
  chain rather than six slots: the seed carries `u_X`, `S_X`, `O_X`, and `S_Y` as one checked unit,
  and the `Y`-side claim must continue from that `S_Y`, so a flat record of six roles cannot
  reinstate the independent-openings conception the successor edition replaced. A one-sided
  inquiry is not a value of the type at all. Checking additionally requires each return fiber to
  be the reverse section of its own use taken at its own exterior, and both orientations to belong
  to one distinction. `gamma_reachable` refuses while a selected return is missing and supplies
  none, keeping `Gamma_D` downstream. The four section-40 comparisons report only `Coincident` or
  `Undecided`: identical fillings are protected-equivalent under every horizon, while differing
  fillings are undecided here, because deciding `equiv_H` needs a protected-horizon evaluator that
  no phase supplies. Section 43's `ox_occurrence`/`oy_occurrence` event links and stored recovery
  references are deliberately absent, having no producer; the view carries no canonical identity,
  since section 43 calls it derived and not authoritative history.
- The derived `exact_return_fiber` now supplies the same-use reverse section `N_u^{-1}[e]` over a
  caller-declared finite negation extension, retaining the use tag so two uses reaching one
  exterior keep two distinct returns. It is the whole fiber, never a selected return: the
  successor fixtures that every admitted incidence returns its source, and that source membership
  does not imply unique return determination, are both executable. An exterior with no declared
  incidence is refused rather than reported as an empty fiber, so a missing declaration does not
  become a negative result. `check_fiber_recovery` then requires the protected-signature domain to
  equal the derived fiber, closing the gap in which recovery could be reported against a table
  unconnected to any incidence. The extension remains caller-declared: the retained
  `NegationUseRef` is not resolved, so nothing yet checks that it names an admitted use or that
  the declared pairs match that relation. Admission, evaluation, coverage, and actuality are
  untouched.
- A derived `RecoveryStatusIR` keeps `Recovered`, positively witnessed `NotRecovered`, and
  `Unknown` distinct. Its finite checker accepts only a separately certified complete finite
  same-use fiber table: constant protected signatures produce a finite recovery result, differing
  signatures produce a concrete pair, and an empty table is rejected. It neither constructs or
  certifies a fiber nor turns missing coverage into a loss result.
- `BoundaryChart` now assigns a canonical derived identity to the plan's declared local-chart
  inputs: query, `X`/`Y`/boundary types, projections, determinations, tagged use frontiers, seed,
  compatibility, optional traversal, grain, and horizon. It preserves empty or omitted roles as
  such; it does not validate the opaque field contracts yet or derive an exterior, return fiber,
  coverage, reverse negation, global partition, or sixfold occurrence.
- The exact finite `check_exact_finite_cue_basis` checker now tests the canonical sufficient-basis
  condition over caller-certified total, deterministic, exactly covered signatures. It returns
  `Sufficient` only when every protectedly distinct finite pair differs on some cue, otherwise a
  concrete protected separator with the common cue answers. It permits an empty basis only for a
  constant protected signature and rejects context/domain mismatches. It does not certify input
  coverage/support/applicability, select a resource-minimal basis, claim global minimality or
  impossibility, or schedule a query.
- `select_nondominated_exact_finite_cue_bases` now adds finite declared-preorder selection over a
  caller-supplied candidate set. It validates reflexivity/transitivity over the represented
  resource identities, retains every incomparable sufficient basis, removes only strictly
  dominated sufficient candidates, and preserves a concrete separator for each insufficient
  candidate. It neither supplies resource facts, generates candidates, establishes supplied-set
  exhaustiveness, certifies exact input coverage/support/applicability, nor claims global minima,
  impossibility, or policy selection.

Different answers determine whether a positive certificate can distinguish an actual supported
departure from merely coexisting identifiers, failed work, and incomplete evidence. The next
discriminator must reject a purported departure whose observation result, incompatible answers,
or derivation route is not independently represented and checked.

Phase 11 has a least-fixed-point standing engine. `standing()` iterates
`T_t(X) = Ingress ∪ { λ : some declared environment for λ is closed against X }` from the empty
set until it closes. Growing from nothing rather than shrinking from everything is what the
no-rootless-positive-support-cycle theorem requires: a family of claims supporting only one
another is entirely self-consistent and belongs to the greatest fixed point, while the least fixed
point never reaches it. Grounding one member by ingress admits the whole cycle, so the theorem
forbids rootless cycles rather than cycles. `SupportEnvironment` decides the two closed-support
conditions that depend on the standing set — premises requiring standing, and an empty open
dependency boundary — and carries applicability, independent-check success, and inconsistency
policy as caller declarations, none of which has an evaluator in this phase. Several incomparable
environments may support one claim, and one closing route suffices. The engine reasons from the
declarations it is given: it does not verify that an ingress fact is grounded or that a declared
check ran, and the `SupportRef` fields carried by relation uses and departure witnesses are still
opaque and unconnected to any `ClaimRef`.

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
- **Phase 6:** represent request-before-dispatch and a typed attempt boundary without collapsing
  the completed event record into a proposed request. Then validate boundary, operator, state,
  route, backend, and provenance contracts and add crash/restart replay over the ordinary ledger.
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
