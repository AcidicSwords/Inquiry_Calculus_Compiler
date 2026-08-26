# Inquiry Calculus

This repository contains the Rust reference implementation of Inquiry Calculus v1.1,
including the adopted **Successor Reciprocal-Boundary / Positive-Negation Edition**.
The executable implementation now includes verified semantic increments across Phases 1--11,
the finite positive-negation reciprocal slice, crash-safe external-effect actuality, a complete
finite question cycle with cold replay, paired question/return traces, and an exact finite
sufficient-present/reopening witness. Phase 10 now also contains a narrow OpenAI Responses
transport adapter whose offline contract preserves both success and non-success HTTP returns
before interpretation. Its post-actuality decoder scans heterogeneous output and preserves every
JSON-array completion, and observation support can now be formed after an unpredictable return
without rewriting the source question. Each decoded string also has a minimal replayable value
identity over raw return, decoder version, ordinal, and exact text. A successful live provider return is still pending because the currently
available credential returned HTTP 401 (`F-0001`). This is not yet the complete reference
runtime. The exact demonstrated boundary and next executable obligation are recorded in
`CONFORMANCE_STATUS.md` and `IMPLEMENTATION_FRONTIER.md`.

## Authority by question

Authority is scoped to the question it answers rather than treated as one global
priority list.

| Question | Governing source |
|---|---|
| Task scope and delivery | Explicit user request |
| Standing semantics | `Inquiry_Calculus_Unified_Canonical_Specification_v1_1.tex` and the accepted paired addition |
| Architecture and phase order | `Inquiry_Calculus_v1_1_Comprehensive_Implementation_Plan.md` |
| Standing autonomous implementation objective | `PERSISTENT_CODEX_GOAL.md`, constrained by standing semantics and architecture |
| Strongest live implementation question | `IMPLEMENTATION_FRONTIER.md` |
| Final research-derived breakers and phase constraints | `PROJECT_RESEARCH_IMPLEMENTATION_HANDOFF.md` and `research/final-2026-08-25/`, as derived ancestry rather than semantic authority |
| Accepted local choices | `DECISIONS.jsonl` |
| Repository actuality | Code, Git state, builds, tests, and tool returns |
| Demonstrated conformance | Tests and `CONFORMANCE_STATUS.md` |
| Observed constraints | `FAILURES.jsonl` |

The repository uses three separate control documents. `AGENTS.md` compiles consequential
work into an inspectable, answer-dependent inquiry program; `PERSISTENT_CODEX_GOAL.md`
supplies the long-horizon autonomous objective; and
`PROJECT_RESEARCH_IMPLEMENTATION_HANDOFF.md` compacts the closed final research corpus into
phase-local constraints and breaker candidates. It is not a competing plan. A still-supported
instruction is a cached answer, not a reason to reopen a settled question. The paired-actuality
addition remains an explanatory, derived addition and does not introduce a second history ontology.

## One forward implementation path

The project deliberately separates one objective, one build map, and one moving cursor:

```text
PERSISTENT_CODEX_GOAL.md
    -> why and how far autonomous implementation continues

Inquiry_Calculus_v1_1_Comprehensive_Implementation_Plan.md
    -> the single ordered implementation plan

IMPLEMENTATION_FRONTIER.md
    -> the strongest executable obligation at the current repository head
```

`CONFORMANCE_STATUS.md` records only demonstrated checks, and `DECISIONS.jsonl` records accepted
local choices. The final research bundle supplies breakers when a live residual reaches the phase
it constrains; its chronology is not replayed and its names are not an alternative architecture.

The predecessor implementation plan and canonical specification remain retrievable at
Git commit `49dc381ac230326aa28be6c157ece0d21a31eaa2` as ancestry and regression
evidence; they are not coequal forward authority.

## Workspace

- `ic-core`: canonical artifact envelopes, binding-scoped `TyIR`, typed-form
  declarations, canonical formula artifacts, capture-safe typed terms, formula-defined or
binding-native relation schemas, canonical identity, and structural checking.
- `ic-store`: SQLite migrations, verified immutable artifact storage, transactional insertion of
  explicitly declared artifact dependencies, and an append-only ordinary event ledger.
- `ic-runtime`: verified first-order `Return | Branch | Probe` execution, crash-safe provider
  dispatch, finite semantic admission/resumption, cold replay, paired traces, and provider
  adapters. Transport actuality remains separate from decoding, support, standing, and warrant.
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
leaving a nonempty question. `Plug` can construct a canonical, content-addressed complete typed
candidate assignment for that question; its checker revalidates the source query, every port,
type, binding, and already-bound value. It does not evaluate the relation, establish fiber
membership, manufacture actuality, or make the candidate a supported answer.
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
The answer slot remains syntax, but the Phase 6/7 boundary now has a derived finite
supported-answer-set representation. `AdmittedFiniteAnswerSet` replays one event-linked finite
decode and retains the entire nonempty decoded set, one exact `Probe` observation use and one
closing relation-support route per completion, plus the preserved event and raw-return identities.
Missing, duplicate, foreign, non-probe, unsupported, or raw-return-disconnected observations
reject. A multi-completion decoded result remains a partial supported answer and cannot be
silently reduced to a convenient singleton. The result is finite-route-relative derived evidence,
not a canonical artifact, warrant, standing mutation, or continuation choice. Capture-safe answer
binding is now available as a derived `BoundFiniteAskContinuation`: it rechecks the source program,
rejects an answer slot that shadows the explicit environment, requires the answer's question to
match, and retains the source/continuation identities, environment, slot, and whole admitted set.
It does not substitute or evaluate the continuation. Normalization, registered pure operations,
general lowering, and external execution remain deferred.

Phase 5 has a first-order structural `ProgramIR` in `ic-runtime`: typed `Return`, nonempty
internal `Branch`, and `Probe` suspension/resume blocks. Verification rechecks typed returns,
closed targets, and the representable guarded-recurrence boundary; a branch-only cycle is
rejected. Stepping a probe only creates a suspension. Raw-only resumption remains explicitly
non-actual and non-admitted. A separate derived finite bridge accepts only a
`BoundFiniteAskContinuation` plus an explicit `ContinuationLowering`: the admitted event's
operator must equal the suspended operator, the lowering source must equal the checked source
continuation, and its target must equal the probe's fixed resume target. The resumed state retains
the whole bound answer, event, and raw-return provenance. It does not dispatch, append history,
evaluate the continuation, establish warrant, or make the generated lowering authoritative.

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

The compiler/backend boundary now also has distinct canonical `SurfacePlan` and
`BackendRequest` identities. A surface plan rechecks its exact operator and repeats the query,
boundary, active view, executable code, and probe contract before adding renderer-version and
rendered-body references. A backend request rechecks that plan/operator pair and repeats query,
boundary, backend, executable code, and compiler version before adding backend-version and exact
request-body references. Different bodies or versions therefore remain different requests, while
borrowed operator fields reject. These artifacts are rendered/request data only: they do not
dispatch, establish actuality, interpret a return, or warrant semantics.

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
grain, and horizon; and rejects an evidence use outside that context. A separate derived check can
also require the presentation's exact claim-targeted support environment to target a claim in a
declared least-fixed-point `Standing` result. Evidence-use support routes remain separate because
they are relation-targeted. A companion resolver verifies that all three name exact
relation-targeted support environments with matching context. The shared least-fixed-point engine
can now carry claim and relation subjects without collapsing their kinds, and a derived check can
require each evidence use's exact environment to be one of the routes that closes for its standing
relation. This remains relative to declared closure inputs and still does not evaluate an
observation by itself. For the exact finite route, `admit_probed_finite_departure` now composes
two event-linked decoded observations, exact standing closure of all three evidence uses, an
explicit source-presentation dependency on the observation relation, and a positively listed
oriented incompatibility pair. Both observation routes must be `Probe`, their support records
must name the preserved raw returns, and the incompatibility support must cover both returns.
The result is a private-field derived admission, not a canonical artifact, standing mutation,
warrant, negation incidence, exterior, or reconciliation. Unknown, unlisted, generated,
unsupported, reversed, projection-derived, and failed-search inputs cannot construct its required
positive evidence.

`AdmittedFiniteNegationExtension` now applies that result pointwise to one typed finite negation
extension. Every declared `(source, candidate)` row must have its own admitted departure; extra,
missing, or duplicate evidence rejects. The result keeps the use's semantic coverage separate
from each tagged exterior's execution-coverage identity and derives return fibers only from the
fully matched finite set. A vertical fixture composes two independently admitted opposite
orientations through a checked seed, entire-fiber recovery, selected returns, residuals, and
downstream-only `Gamma`; a same-orientation second side rejects. This is finite-route-relative
derived evidence, not global coverage, standing revision, reconciliation, or authoritative
history.

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
Its derived use-bound checker additionally requires a named, non-generated `RelationUse` to
rehash, check, and bind a positive listed pair. Generic relation uses do not yet name typed
left/right incompatibility roles, so a second derived checker receives explicit distinct named
source/candidate ports and verifies the positive pair at those exact ports; it never infers roles
from a naming convention. These forms remain derived finite evidence only: they do not actualize
an observation, admit a standing incompatibility relation, establish relevance, coverage, support,
or non-circularity, or establish departure/exteriority.

`TaggedExteriorClaim` is a derived role view, not a new `PosNeg` carrier or stored history
species. It retains one immutable negation-use tag, source, candidate, named departure-witness,
and separately supplied execution-coverage identity; validation rehashes both declarations and
requires their source/candidate and determination context to agree. Thus the same candidate can
remain distinct under different uses. The view does not evaluate `N_u(source, candidate)`, admit
the use, establish a positive exterior, actualize a result, or confer support or warrant.

Phase 13 has an exact finite discriminator-basis checker. Given caller-certified total,
deterministic, exactly covered cue and protected signatures over one indexed domain, it verifies
that every protectedly distinct pair has a differing cue answer or returns a concrete unseparated
pair. It permits an empty basis only when the protected signature is constant. It does not certify
the input tables' coverage/support/applicability, claim global minimality or impossibility, or
define a query-selection policy. Given a separate finite declared resource preorder and a
caller-supplied candidate set, it can additionally retain the nondominated sufficient candidates
and preserve concrete separators for insufficient ones. That frontier is relative to the supplied
candidates; it neither generates candidates nor proves the candidate set exhaustive.

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

The Phase 6 crash breaker now has a separate operational `external_effect_journal`, introduced by
`0003_create_external_effect_journal.sql`. The typed `prepare_backend_request` path rechecks a
canonical surface-plan/backend-request/operator chain before durably recording an opaque
idempotency token, already-stored request reference, verified compiled operator, and expected
ledger parent before a caller may dispatch. A restart with an unresolved row reports `Pending`—an
unknown outcome that is never safe to retry automatically. `complete_external_effect` accepts the
exact operator/parent and raw return, then atomically inserts the immutable raw artifact, checked
ordinary event, ledger edge, and completion link. The operational row is not a semantic attempt,
actuality claim, second event history, decoded answer, or warrant; this deliberately leaves the
canonical/plan attempt-record shape open.

Completed rows can now be reconstructed after a file-backed restart as a checked
`ReplayedExternalEffect`. The store revalidates the exact backend request, ledger event,
raw-return identity and bytes, operator/parent linkage, and backend version without invoking a
provider. Completion rejects backend-version drift before history is committed.

Preparation also distinguishes a newly committed intent from an exact pre-existing row. Only the
new `DispatchAuthorized` result permits the current caller to invoke a provider. An exact repeat,
including one recovered after restart, returns `Existing`; a pending existing row therefore remains
unknown and cannot accidentally become permission to dispatch again.

`ic-runtime::dispatch_probe` now supplies the first injected provider coordinator. It accepts only
a verified suspension's exact checked backend request, dispatches only after a fresh durable
authorization, converts the provider's opaque bytes into `RawReturn`, and atomically completes the
ordinary event before returning anything to a decoder. The fixture proves one call on success,
zero redispatches for completed or recovered-pending rows, and a separate operational provider
failure path. This is a mock-provider boundary, not method admission, semantic resolution, a
standing decision, or real-provider integration.

`ic-runtime::replay_completed_finite_probe` now composes the durable actuality spine with the
existing finite decoder, exact relation-support standing, complete answer-set admission,
capture-safe source binding, and verified runtime resumption. A file-backed fixture drops all
pre-restart objects, reloads canonical artifacts, regenerates the runtime mapping, retains two
supported completions, and distinguishes every RPL-005 failure family. The fixture now also runs
the live side through `dispatch_probe`: exactly one provider call commits the raw return and event,
the answer is admitted and resumed to `Return`, and the same result is regenerated after restart
with the provider-call counter unchanged. The first finite slice needs no persisted lowering recipe
or new opcode; the next frontier is a derived paired question/return trace and the first exact
question-conditioned sufficient-present/fold/reopen witness over the same event spine.

`ic-runtime::PairedActualityTrace` now supplies that first derived trace boundary. It pairs only an
exact admitted event and resumption, keeps question/source provenance separate from
return/decoder/path/continuation provenance, and regenerates identically from cold replay. Two
admitted resolution paths with the same event, completion set, and runtime endpoint remain distinct
return traces. The next frontier is the sufficient-present fold/reopen witness; no trace is stored
as a second history.

`ic-core::ExactFiniteSufficientPresent` now supplies the first protected-continuation-indexed fold
check. Every protected observation must factor through the proposed present. The first fixture
folds two path-distinct histories into the single coarsest class under a continuation that cannot
distinguish them, regenerates the same factorization after restart, and returns those exact
histories as a reopen witness when a path-sensitive continuation is added. This derived check is
not active mutable memory, standing, a compression licence, or a general bounded-memory claim.
The next frontier is one real LLM provider behind the already stable dispatch/replay boundary.

The event ledger and external-effect recovery journal have file-backed restart witnesses. Closing
the single connection and reopening the database preserves pending/complete operational state,
canonical event identity, immutable raw bytes, and parent-linked order after embedded migrations
are reapplied. This proves persistence, integrity, and the first finite execute/replay path only;
general state-transition semantics, multi-event paired actuality, sufficient-present derivation,
and accepted-state reconstruction remain later contracts.

Phase 7 has a typed, first-order `ResolutionPath`: identity, decoder, relation, composition, and
program routes each preserve their input/output types and referenced route identity. A complete
query filling now has independent canonical identity before it is admitted into any result.
Checking revalidates types and exact composition interfaces, including cycle rejection. It does
not run a decoder, relation, or program; resolve a raw return by itself; or turn a partial result
into an exact answer. The derived finite admission described above is the first supported-answer
bridge over the narrower event-linked decoder and standing-route boundary; general route execution
and first-order answer-dependent continuation binding remain open.

Phase 11 now has a canonical `ClaimArtifact`: it retains an opaque proposition/payload identity,
source question, preserved raw-return references, resolution paths, scope, applicability, and a
declared lifecycle status. Its checker rehashes the typed/provenance references, rejects duplicate
references and malformed status tags, and is intentionally not an admission rule: `Standing` in a
claim artifact is only a stated status, not proof that any support route closed. The derived
least-fixed-point standing engine remains separate.

`SupportEnvironmentArtifact` now gives a candidate support route canonical identity: tagged
claim-or-relation target, generic premise references, preserved raw returns, checker and assumption references, open
dependencies, applicability, and scope. Its checker rehashes a claim target and actual
returns; it rehashes a relation target as a relation schema, and requires a claim target's declared
context to agree. It deliberately does not run
opaque checkers, evaluate assumptions or open dependencies, assert closure, or admit the target
claim as standing. A derived relation-use link can resolve only a matching relation-targeted
environment through its exact `SupportRef`, scope, and applicability; that link is not admission.

`standing_from_declared_support` preserves its claim-only interface, while the subject-aware
variant checks claim and relation targets and typed premises before applying one shared
least-fixed-point operator. Claim and relation subjects remain distinct even when their underlying
digests coincide. The result retains the exact canonical environments that close for each subject,
so one route cannot borrow another route's standing. Applicability, checker success, and
invalidation remain explicit assessments; this is not proof, warrant, relation evaluation, or an
independent-ingress validator.

A determination presentation can now resolve the exact claim-targeted environment named by its
`SupportRef`, require matching scope/applicability, and require that target claim to occur in the
declared least-fixed-point standing result through that exact environment. This does not assert that the opaque claim payload
denotes the presentation source form, that the web is admitted or relevant, or that caller-declared
closure conditions are independently grounded.

Phase 14 now has `DeclaredFiniteGeneratorRegime`, a derived finite view that keeps declared route
availability separate from current materialization and from selection. A fresh declared route is
not evidence of a representation gap, and absence from this finite view is not evidence that no
lawful route exists.

Its exact finite companion requires one caller-supplied exact signature for every route in that
declared regime. When every route fails to separate a protected pair, it retains only an
`ExactNoSeparatorWithinRegime` witness; it does not claim global inexpressibility or propose a
representation extension.

`MaterializationGap` is now constructible only for a fresh route already present in the declared
regime. `ProposedRegimeExtension` instead retains a route outside that regime against a separator
problem. Both are candidates: neither selects, runs, admits, or changes a binding.

Phase 15 now has a finite `BindingBridgeIR`: explicit old-to-new `QueryRef` transport is injective
and rechecked against the source/target relation-schema bindings and declared scope/horizon. Only
a declared conservative observational extension may carry a target question outside the finite
transport image as a strict-growth witness. This is finite demonstrated transport, not an
unbounded inclusion theorem; rebinding carries no automatic question-language inclusion.

The first concrete resolution boundary is a canonical finite decoder table for one query and raw
input type. Each listed raw return either decodes to a nonempty set of checked complete candidates
or is explicitly outside the decoder; an unlisted raw return remains `Unknown`. A direct
single-open-port decoder route can be checked against an ordinary `ActualEvent` record, its
operator's return type, the decoder's identity, and the query answer-port type. This preserves
route identity without making the result a relation fact, standing claim, check, warrant, actual
dispatch, or departure witness. A derived `DecodedObservationUse` can then require one preserved
candidate and one declared `RelationUse` to rehash and spell exactly the same relation, complete
named bindings, and scope/applicability/grain/horizon/mode/support/warrant context as the source
query. It is a structural association only: it neither evaluates the relation nor proves dispatch,
admission, standing, incompatibility, non-circular evidence, or departure. Multi-port
answer-carrier construction, general decoder execution, and an admitted incompatibility
certificate remain pending.

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

`SeparatorProblem` now canonically preserves the generic residual boundary shared by reciprocal
ambiguity, recovery loss, cue extension, fold reopening, and later representation-gap inquiry:
the protected completion field, optional target class, binding/grain/horizon, available
structure, generator regime, and effectivity. It is neither a generator nor a policy; it cannot
produce a question, prove a residual, or turn an ungenerated separator into absence or
impossibility.

`GeneratedInquiry` now canonically records one generated candidate question for one separator
problem and one declared generation-route identity. Its checker rehashes both inputs and requires
their binding, grain, and protected horizon to agree. It does not establish route lawfulness or
materialization, select a question, run a probe, or confer actuality, support, or warrant.

Phase 16 now has a canonical `CompressionLicense` boundary. It preserves the folded/quotiented
artifact, protected horizon and continuations, scope, evidence, residual, recovery contract, and
unlock conditions. Exact and approximate licences have different identities; the latter must name
an explicit distortion contract. This records a claimed licence, not a proof of regeneration,
recovery, approximation soundness, or an authorized active fold.

Phase 10 now also has a canonical `MethodContract` registry record for native and learned
traversal methods. It names a checked implemented relation, applicability, law, coverage,
authority, extension domain, backend, optional checker/cost, failure schemas, and provenance.
This is neither standing acceptance nor a statement that the backend is available, applicable,
executed, or semantically warranted.

Phase 16 now also has a derived `OperatorOccurrence` identity. It is constructed from, and
rechecked against, one ordinary `ActualEvent`; its event, operator, states, raw return, and boundary
must all match exactly. It adds no second history, dispatch assertion, or return interpretation.

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
