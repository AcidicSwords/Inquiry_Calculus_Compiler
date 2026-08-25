# Inquiry Calculus v1.1 — Comprehensive Implementation Plan

**Status:** Implementation-facing successor specification  
**Scope:** Reference implementation architecture and build plan  
**Source of truth:** The Inquiry Calculus v1.1 material, canonical additions, implementation successors, executable probes, and research returns established in this conversation  
**Purpose:** Give an implementation agent enough exact structure to build the system without inventing semantic machinery

---

# 0. Executive statement

The project is ready to move from recursive architecture discovery into implementation.

The semantic architecture is sufficiently settled. The remaining uncertainty is localized to explicit research gates and optimization choices rather than the fundamental shape of the system.

The implementation target is a persistent typed relational inquiry runtime in which:

- questions are partial bindings of represented relations;
- answer-dependent inquiry programs are first-order, inspectable, persistable syntax;
- reciprocal inquiry preserves answer paths and provenance;
- the sixfold reciprocal profile is a derived view rather than a dedicated runtime ontology;
- runtime execution reduces to `Return`, `Branch`, and `Probe`;
- raw actuality is immutable and precedes interpretation;
- question and return traces are derived from one authoritative event spine;
- holes, residual fibers, and protected completion classes drive continued inquiry;
- cue-guided memory traversal is recurrent and question-conditioned;
- residual inquiry, cue extension, reciprocal repair, fold reopening, and fresh discrimination share one separator problem;
- generative question capability is distinct from currently materialized questions and from question-selection policy;
- question generation within a binding can only exploit distinctions expressible by the admitted question language;
- materialization gaps are handled by generation, while expressibility gaps open governed binding-extension inquiry;
- binding growth is classified as definitional extension, conservative observational extension, or rebinding;
- historical actuality is never rewritten under later bindings;
- standing is positive, least-fixed-point, provenance-carrying, and non-self-licensing;
- folds compress traversal or representation only under explicit regenerative and reopening contracts;
- self-revision is predecessor-judged and cannot warrant itself.

The implementation should remain deliberately small until a protected fixture demonstrates the need for stronger infrastructure.

---

# 1. Governing implementation invariant

The implementation must preserve consequential distinctions across every transformation.

For every implementation transformation

\[
T:A\to B,
\]

the required question is:

\[
\boxed{
\text{Can any protected continuation distinguish execution before and after }T?
}
\]

If yes, the distinction must remain represented, versioned, or recoverable.

If no protected continuation can distinguish the difference under the declared horizon and regime, the implementation may fold or quotient it, but only with a recovery and reopening contract where later protected use may inspect it.

The implementation therefore follows:

\[
\boxed{
\text{refine where protected consequences split; fold where they do not.}
}
\]

This law applies to:

- semantic representation;
- compilation;
- prompt rendering;
- provider adaptation;
- memory retrieval;
- cue planning;
- question generation;
- method learning;
- binding extension;
- standing;
- persistence;
- replay;
- self-revision.

---

# 2. Non-negotiable separations

The implementation must never silently identify the following:

\[
\begin{aligned}
\mathsf{Question}
&\neq
\mathsf{Probe},\\
\mathsf{Probe}
&\neq
\mathsf{SurfacePlan},\\
\mathsf{SurfacePlan}
&\neq
\mathsf{BackendRequest},\\
\mathsf{Generated}
&\neq
\mathsf{Actual},\\
\mathsf{RawReturn}
&\neq
\mathsf{Resolution},\\
\mathsf{Resolution}
&\neq
\mathsf{Check},\\
\mathsf{Check}
&\neq
\mathsf{Warrant},\\
\mathsf{Warrant}
&\neq
\mathsf{Standing},\\
\mathsf{Retained}
&\neq
\mathsf{Accessible},\\
\mathsf{Accessible}
&\neq
\mathsf{Active},\\
\mathsf{Active}
&\neq
\mathsf{Standing},\\
\mathsf{MaterializedQuestion}
&\neq
\mathsf{GenerableQuestion},\\
\mathsf{GenerableQuestion}
&\neq
\mathsf{SelectedQuestion},\\
\mathsf{MaterializationGap}
&\neq
\mathsf{ExpressibilityGap},\\
\mathsf{DefinitionalExtension}
&\neq
\mathsf{ConservativeObservationalExtension},\\
\mathsf{ConservativeObservationalExtension}
&\neq
\mathsf{Rebinding},\\
\mathsf{TraversalLearning}
&\neq
\mathsf{SemanticWarrant},\\
\approx_{\mathcal H,\mathcal D}
&\neq
\equiv_{\mathcal H},\\
\mathsf{NotFound}
&\neq
\mathsf{Impossible},\\
\mathsf{SelfApplication}
&\neq
\mathsf{SelfWarrant}.
\end{aligned}
\]

A type, record, cache, database schema, or convenience API that collapses any of these distinctions is a defect unless a protected-equivalence licence explicitly justifies the collapse.

---

# 3. Architectural dependency direction

All implementation decisions must respect:

\[
\boxed{
\text{semantic relations}
\to
\text{typed IR}
\to
\text{compiler/runtime}
\to
\text{persistence/indexes/backends}
}
\]

Never allow:

- provider SDK structure;
- storage schema;
- framework abstractions;
- host-language callbacks;
- cache layout;
- prompt format;
- embedding space;

to determine semantics merely because they are convenient.

No provider, model, database, scheduler, vector index, or framework is a semantic oracle.

---

# 4. Three implementation classes

Every planned component belongs to exactly one of these classes.

## 4.1 Constitutional machinery

Required to preserve the semantics already established.

Examples:

- `TyIR`;
- typed forms;
- relation schemas;
- partial binding;
- `OpenQuery`;
- first-order `IProg`;
- protected path provenance;
- `Return | Branch | Probe`;
- immutable raw return;
- actual event spine;
- hole/fiber representation;
- protected completion fields;
- standing;
- binding versions and bridges;
- fold recovery and reopening.

These must be implemented correctly before optimization.

## 4.2 Derived implementation structures

Useful but not semantic primitives.

Examples:

- sixfold view;
- question trace;
- return trace;
- active view;
- cue plan;
- materialized discriminator basis;
- operator occurrence graph;
- method lookup;
- standing cache;
- access indexes;
- affected-fold index;
- cross-binding transported views.

These should be rebuildable where feasible.

## 4.3 Research/optimization gates

Must remain explicit rather than guessed.

Current examples:

- the smallest useful open-ended separator generator regime;
- question-selection optimization;
- exact-to-approximate cue-planning threshold;
- whether vector retrieval provides protected strict gain;
- when resumable/fair generator machinery becomes necessary;
- cross-binding standing lift into an enlarged horizon;
- when SQLite/serial semantic execution reaches a measured boundary.

---

# 5. Reference implementation stack

Use Rust for the reference implementation.

Recommended baseline:

```text
Rust
Cargo workspace
serde / serde_json
sha2
thiserror
tracing
SQLx
SQLite
Tokio only at external-effect boundaries
proptest
```

Do not require initially:

```text
PostgreSQL
Kafka
Neo4j
vector database
distributed workflow system
generic multi-agent framework
actor framework
Kubernetes
```

The first architecture is a single-process reference runtime with one authoritative writer.

All semantic identities must be independent of:

- SQLite row IDs;
- process memory addresses;
- provider request IDs;
- filesystem locations;
- scheduler task IDs.

---

# 6. Repository shape

Start smaller than the final logical decomposition.

Recommended workspace:

```text
inquiry-calculus/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── README.md
├── AGENTS.md
├── IMPLEMENTATION_FRONTIER.md
├── CONFORMANCE_STATUS.md
├── DECISIONS.jsonl
├── FAILURES.jsonl
│
├── crates/
│   ├── ic-core/
│   ├── ic-runtime/
│   ├── ic-store/
│   └── ic-cli/
│
├── migrations/
├── fixtures/
└── tests/
```

Do not pre-create a large collection of empty crates.

Split `ic-core` only when there is actual independent variation or dependency pressure. Likely later splits are:

```text
ic-types
ic-rel
ic-program
ic-standing
ic-retrieval
ic-learning
ic-provider
```

A module boundary must be earned by a protected implementation distinction, not aesthetic layering.

---

# 7. Immutable artifact identity

All immutable semantic/program objects are content-addressed.

Core references:

```rust
pub struct ArtifactRef([u8; 32]);

pub struct TypeRef(ArtifactRef);
pub struct FormRef(ArtifactRef);
pub struct RelationRef(ArtifactRef);
pub struct QueryRef(ArtifactRef);
pub struct ProgramRef(ArtifactRef);
pub struct OperatorRef(ArtifactRef);
pub struct ReturnRef(ArtifactRef);
pub struct EventRef(ArtifactRef);
pub struct ClaimRef(ArtifactRef);
pub struct MethodRef(ArtifactRef);
pub struct PatchRef(ArtifactRef);
pub struct BindingVersionRef(ArtifactRef);
```

Canonical serialization must be explicitly defined.

Do not assume arbitrary JSON serialization is canonical.

Every hash input includes:

```text
object kind
schema version
canonical payload
```

Required properties:

\[
Decode(Canonical(x))=x
\]

and:

\[
Canonical(x)=Canonical(y)\Rightarrow Ref(x)=Ref(y).
\]

Committed records may never contain dangling artifact references.

Orphan immutable artifacts are acceptable.

---

# 8. Typed form universe

Represent:

\[
\mathsf{Form}_{\mathbb B}
=
\sum_{A:\mathsf{Ty}_{\mathbb B}}
\llbracket A\rrbracket_{\mathbb B}
\]

without a universal dynamic escape hatch.

```rust
pub struct TypedFormRef {
    pub ty: TypeRef,
    pub value: ArtifactRef,
}
```

Reference type IR:

```rust
enum TyIR {
    Unit,
    Bool,
    Nat,
    Int,
    Text,
    Bytes,

    Named {
        binding: BindingVersionRef,
        name: Symbol,
        version: ArtifactRef,
    },

    Product(Vec<TypeRef>),
    Sum(Vec<TypeRef>),
    List(TypeRef),
    FiniteSet(TypeRef),

    Sigma {
        domain: TypeRef,
        family: TypeExprRef,
    },

    Pi {
        domain: TypeRef,
        family: TypeExprRef,
    },

    Code {
        input: TypeRef,
        output: TypeRef,
    },

    Raw(TypeRef),
    Result(TypeRef),
}
```

The type language is representational syntax.

A binding supplies semantic interpretation where applicable.

Implement:

```rust
fn check_value(
    binding: BindingVersionRef,
    ty: TypeRef,
    value: ArtifactRef,
) -> Result<TypeCertificate>;

fn type_of(form: FormRef) -> Result<TypeRef>;
```

No semantic boundary may accept an untyped generic payload.

---

# 9. Relation schemas

Core relation representation:

```rust
struct RelSchemaIR {
    id: RelationRef,
    binding: BindingVersionRef,

    ports: Vec<NamedPort>,

    body: FormulaRef,

    laws: Vec<LawRef>,
    provenance: Vec<ArtifactRef>,
}
```

Ports:

```rust
struct NamedPort {
    name: Symbol,
    ty: TypeRef,
}
```

Reference formula language:

```rust
enum FormulaIR {
    True,
    False,

    Eq(TermExprRef, TermExprRef),

    Rel {
        relation: RelationRef,
        args: Vec<TermExprRef>,
    },

    And(Vec<FormulaRef>),
    Or(Vec<FormulaRef>),

    Exists {
        domain: TypeRef,
        body: FormulaRef,
    },

    ForAll {
        domain: TypeRef,
        body: FormulaRef,
    },

    BindingNative {
        binding: BindingVersionRef,
        operator: NativeFormulaRef,
        args: Vec<TermExprRef>,
    },
}
```

No host callback can decide semantic truth inside a relation schema.

---

# 10. Questions are partially bound relations

A question is not a separate primitive linguistic species.

```rust
struct OpenQueryIR {
    relation: RelationRef,

    bound_ports: Vec<BoundPort>,
    open_ports: Vec<OpenPort>,

    scope: ScopeRef,
    applicability: ApplicabilityRef,

    grain: GrainRef,
    horizon: HorizonRef,
}
```

Each open port carries discharge authority:

```rust
enum DischargeMode {
    Pure,
    Generate,
    Probe,
    Check,
    Warrant,
}
```

A `Generate` return cannot discharge `Probe`, `Check`, or `Warrant`.

Implement:

```rust
bind_port
expose_port
plug_answer
normalize_query
compose_relations
dependent_bind
completion_fiber
```

Binding one port must preserve the represented relation on every remaining open port.

---

# 11. Source inquiry programs

Canonical source language:

\[
K
::=
\mathsf{Return}_I(a)
\mid
\mathsf{Ask}(q,\kappa).
\]

Do not serialize Rust closures.

Use first-order syntax:

```rust
enum IProgIR {
    Return {
        value: TermExprRef,
    },

    Ask {
        question: QuestionExprRef,
        continuation: IProgRef,
    },
}
```

The continuation is checked under:

```text
Γ, answer : SuppAns(question)
```

and instantiated by capture-safe substitution:

```rust
fn apply_continuation(
    continuation: IProgRef,
    supported_answer: FormRef,
) -> Result<IProgRef>;
```

Use de Bruijn indices or another explicit hygienic binder scheme.

A later question may depend on the actual returned answer.

This must be possible without host controller logic.

---

# 12. Pure registered operators

Deterministic source operations may be registered as typed pure operators:

```rust
struct PureOpContract {
    id: PureOpRef,

    input: Vec<TypeRef>,
    output: TypeRef,

    semantic_relation: RelationRef,

    implementation: ProgramRef,
    implementation_version: ArtifactRef,
}
```

Examples:

```text
projection
finite-set intersection
comparison
arithmetic
tuple construction
normalization
```

The implementation is versioned and tied to represented semantics.

No hidden native callback gains semantic authority merely by being deterministic.

---

# 13. Reciprocal distinction representation

A reciprocal distinction is represented by its actual relation/boundary structure, not by six independent fields.

```rust
struct DistinctionIR {
    x_ty: TypeRef,
    y_ty: TypeRef,

    boundary_ty: TypeRef,

    pi_x: RelationRef,
    pi_y: RelationRef,

    compatibility: FormulaRef,

    traversal: Option<RelationRef>,

    grain: GrainRef,
    horizon: HorizonRef,

    provenance: SupportRef,
}
```

The oriented double cover must reuse the same underlying boundary point rather than duplicate it.

The sixfold:

\[
\Xi_D(z)
=
(S_X,O_X,R_X;S_Y,O_Y,R_Y)
\]

is a derived protected view.

Do not make `SixfoldIR` authoritative.

---

# 14. Reciprocal inquiry compiles to two answer-dependent round trips

Let:

\[
F_z:X\rightsquigarrow Y,
\qquad
G_z:Y\rightsquigarrow X.
\]

The protected first-return path from \(x_0\) is:

\[
x_0\to y_1\to x_1.
\]

Compile it as:

\[
\mathsf{Ask}
\left(
q_F(x_0),
\lambda y_1.
\mathsf{Ask}
\left(
q_G(y_1),
\lambda x_1.
\mathsf{Return}(x_0,y_1,x_1)
\right)
\right).
\]

The opposite orientation is compiled independently from \(y_0\).

Implement:

```rust
fn compile_roundtrip(
    relation: RelationRef,
    orientation: Orientation,
    seed: FormRef,
    context: ContextRef,
) -> Result<IProgRef>;
```

Then:

```rust
fn compile_reciprocal_first_return(
    distinction: DistinctionRef,
    x_seed: FormRef,
    y_seed: FormRef,
    context: ContextRef,
) -> Result<(IProgRef, IProgRef)>;
```

`IT`/"in terms of" is a compiler schema for this construction.

It is not a semantic primitive.

---

# 15. Path preservation is mandatory

Ordinary relational composition may existentially remove intermediate answers:

\[
x\to y\to x'
\quad\mapsto\quad
x\to x'.
\]

That is too coarse whenever a protected continuation can inspect \(y\).

The source/runtime history must preserve:

```text
question
actual answer
answer-dependent successor question
successor answer
ordering
orientation
relevant provenance
```

For nondeterministic relations, preserve branch correlation:

\[
\{(y_i,x_j):F(x,y_i)\land G(y_i,x_j)\},
\]

not merely:

\[
\{y_i\}
\quad\text{and}\quad
\{x_j\}.
\]

This is a compiler conformance obligation.

---

# 16. Sixfold is a derived view

Given two realized reciprocal round-trip paths:

```rust
struct SixfoldView {
    s_x: FormRef,
    o_x: FormRef,
    r_x: FormRef,

    s_y: FormRef,
    o_y: FormRef,
    r_y: FormRef,
}
```

Construct it from authoritative event/path history.

Do not persist it as a second truth unless profiling later justifies a cache.

The semantic roles remain protected.

The runtime representation is compressed.

---

# 17. Runtime semantic core

Lower source inquiry programs to:

\[
P
::=
\mathsf{Return}
\mid
\mathsf{Branch}
\mid
\mathsf{Probe}.
\]

Reference block representation:

```rust
struct ProgramIR {
    result_ty: TypeRef,
    entry: BlockId,
    blocks: Vec<BlockIR>,
}

struct BlockIR {
    id: BlockId,
    params: Vec<BlockParam>,
    instructions: Vec<PureInstr>,
    terminator: Terminator,
}

enum Terminator {
    Return(ValueRef),

    Branch {
        targets: Vec<BlockTarget>,
    },

    Probe {
        operator: ProbeOperatorRef,
        resume: BlockTarget,
    },
}
```

The verifier checks:

```text
block existence
unique definitions
use-before-definition
instruction typing
branch target typing
probe answer/resume typing
result typing
finite local branching
```

Unbounded inquiry is handled by the outer recurrence, not hidden loops inside a single lowered program.

---

# 18. Probe operators and authority

```rust
struct ProbeOperatorIR {
    question: QueryRef,

    authority: DischargeMode,

    distinction_context: DistinctionContextRef,

    view: ActiveViewRef,

    answer_contract: AnswerContractRef,

    resolution: ResolutionPathRef,

    route: RouteRef,
}
```

Routes:

```rust
enum RouteIR {
    Model(ModelRouteIR),
    Environment(EnvironmentRouteIR),
    Checker(CheckerRouteIR),
    Warrant(WarrantRouteIR),
}
```

A structured provider output does not upgrade `Generate` to `Check`.

A checker route does not become `Warrant` merely because it is deterministic.

Authority follows the typed contract.

---

# 19. Answer contracts

```rust
struct AnswerContractIR {
    answer_ty: TypeRef,
    multiplicity: Multiplicity,
    completeness: Completeness,
    authority: DischargeMode,
}
```

```rust
enum Multiplicity {
    One,
    Many,
}

enum Completeness {
    CompleteRequired,
    PartialAllowed,
}
```

A renderer/compiler may never silently strengthen:

```text
Many -> One
PartialAllowed -> CompleteRequired
Generate -> Check
Check -> Warrant
```

---

# 20. LLM compiler factorization

Preserve four distinct stages:

\[
\boxed{
\mathsf{ProbeOperatorIR}
\neq
\mathsf{SurfacePlanIR}
\neq
\mathsf{BackendRequestIR}
\neq
\mathsf{RawReturnEnvelopeIR}.
}
\]

## 20.1 Surface plan

```rust
struct SurfacePlanIR {
    semantic_probe: ProbeRef,

    renderer: RendererRef,
    renderer_version: ArtifactRef,

    segments: Vec<SurfaceSegmentIR>,

    context_selection: ContextSelectionRef,

    surface_answer_contract: SurfaceAnswerContract,
}
```

## 20.2 Backend request

```rust
struct BackendRequestIR {
    surface_plan: SurfacePlanRef,

    binding: ModelBindingRef,
    endpoint: EndpointRef,

    model_requested: ModelRef,

    request_body: ArtifactRef,

    adapter_version: ArtifactRef,

    local_template: Option<ArtifactRef>,
    tokenizer_version: Option<ArtifactRef>,
}
```

The exact submitted request body is immutable.

Provider/session/cache state must not become authoritative semantic state.

## 20.3 Raw provider return

```rust
enum RawReturnEnvelopeIR {
    ResponseObject {
        raw: ArtifactRef,
    },

    Stream {
        frames: Vec<ArtifactRef>,
    },
}
```

If stream order is protected, preserve ordered frames.

Do not store only extracted text.

---

# 21. Actuality and event spine

One authoritative event record:

```rust
struct ActualEvent {
    id: EventRef,

    ledger_parent: Option<EventRef>,

    state_before: StateRef,

    question: QueryRef,
    distinction: Option<DistinctionRef>,

    operator: OperatorRef,

    raw_return: ReturnRef,

    state_after: StateRef,

    grain: GrainRef,

    route: RouteRef,

    binding_version: BindingVersionRef,

    backend_version: ArtifactRef,

    provenance: ProvenanceRef,
}
```

Append actuality before interpretation.

Never mutate an old event to reflect a later interpretation, decoder, model, or binding.

---

# 22. Paired actuality

From the authoritative event spine derive:

\[
Q\xrightarrow{\alpha}R\xrightarrow{\kappa}Q.
\]

Question trace:

\[
T_Q=\kappa\circ\alpha.
\]

Return trace:

\[
T_R=\alpha\circ\kappa.
\]

These are views over one history.

Do not store competing authoritative question and return histories.

Missing return:

\[
q_t\to\square\to q_{t+1}
\]

becomes residual fiber:

\[
\{r:\alpha(q_t,r)\land\kappa(r,q_{t+1})\}.
\]

Missing question is reconstructed reciprocally.

No separate memory ontology is required.

---

# 23. Resolution

Raw returns become supported semantic completions only through explicit resolution.

```rust
enum ResolutionPathIR {
    Identity,

    Decode {
        decoder: DecoderRef,
    },

    Relation {
        relation: RelationRef,
    },

    Compose {
        first: ResolutionPathRef,
        second: ResolutionPathRef,
    },

    Program {
        program: ProgramRef,
    },
}
```

Decoder result:

```rust
enum DecodeResult {
    Complete(Vec<TypedSubstitution>),

    Partial(PartialSubstitution),

    Ambiguous(Vec<TypedSubstitution>),

    Invalid(DecodeFailure),
}
```

Partial answers remain partial.

Ambiguity remains ambiguity.

No finite search failure may be promoted into semantic impossibility.

---

# 24. Holes and residual fibers

```rust
struct HoleIR {
    open_ty: TypeRef,

    constraints: Vec<ResidualConstraintRef>,

    horizon: HorizonRef,

    provenance: Vec<ArtifactRef>,
}
```

```rust
struct FiberIR {
    hole: HoleRef,

    solution_query: RelationRef,

    coverage: FiberCoverage,
}
```

```rust
enum FiberCoverage {
    ExactFinite,
    Symbolic,
    ResourceBounded,
}
```

A form is regeneratively determined at protected grain only when the residual field is certified to contain one protected equivalence class.

Otherwise retain the residual.

---

# 25. Protected completion field

The separator architecture operates on:

```rust
struct ProtectedCompletionFieldIR {
    source_fiber: FiberRef,

    classes: ProtectedClassRepresentation,

    horizon: HorizonRef,
    grain: GrainRef,

    coverage: CompletionCoverage,

    provenance: Vec<ArtifactRef>,
}
```

For finite exact cases:

```rust
ProtectedClassRepresentation::Finite(
    Vec<ProtectedClassRef>
)
```

For open cases, retain symbolic/residual representation.

This is the common substrate for:

- next-question generation;
- cue refinement;
- reciprocal repair;
- fold reopening;
- separator testing;
- question selection.

---

# 26. Memory state distinctions

Implement the explicit separation:

\[
\boxed{
Retained(m)
\neq
Accessible(m\mid c)
\neq
Active(m,t)
\neq
Standing(m).
}
\]

## 26.1 Retained

A form remains reachable through:

- authoritative event history;
- accepted semantic ancestry;
- a fold recovery contract;
- lawful reconstruction/reacquisition route.

## 26.2 Accessible

There is a current route from cue/question \(c\) to the retained form.

## 26.3 Active

The form currently changes the live inquiry field.

## 26.4 Standing

The form/claim has current warranted support.

No implication among these is assumed without an explicit relation.

---

# 27. Active view

Use:

```rust
struct ActiveViewIR {
    question: QueryRef,

    active: Vec<FormRef>,

    reserve: Vec<FormRef>,

    occluded: Vec<OccludedRef>,
}
```

```rust
struct OccludedRef {
    artifact: ArtifactRef,
    licence: CompressionLicenceRef,
}
```

The three-way interpretation is:

```text
active/relevant
relevance-undetermined reserve
certified irrelevant under licence
```

Only licensed irrelevant structure may be aggressively occluded.

Reserve remains reachable.

---

# 28. Access routes

Question-conditioned memory retrieval uses route witnesses.

```rust
struct AccessWitnessIR {
    cue: FormRef,
    target: FormRef,

    route: AccessRouteRef,

    route_contract: AccessContract,

    evidence: Vec<ArtifactRef>,

    route_version: ArtifactRef,
}
```

Initial exact routes:

```text
relation adjacency
event ancestry
ledger neighborhood
question/return adjacency
provenance backlink
operator occurrence
method occurrence
support linkage
fold expansion
unlock linkage
```

Working routes may later include:

```text
lexical search
vector similarity
generated locators
```

A retrieval adjacency does not establish a semantic relation.

A retrieval miss does not establish absence.

---

# 29. Activation

Accessible structure becomes active only through an ordinary consequential witness.

Examples:

\[
StrictFiberRefinement,
\]

\[
DischargesOpenDependency,
\]

\[
ApplicableMethod,
\]

\[
UnlocksOcclusion,
\]

\[
ChangesProtectedContinuation.
\]

```rust
struct ActivationWitnessIR {
    cue: FormRef,
    target: FormRef,

    consequence_relation: RelationRef,

    before: ArtifactRef,
    after: ArtifactRef,

    evidence: Vec<ArtifactRef>,
}
```

Accessible forms without activation witnesses remain in reserve.

No generic learned "relevance score" has semantic authority.

---

# 30. Cue-guided recurrent memory crawl

Retrieval is recurrent:

\[
c_n
\to
Retrieve(c_n)
\to
Activate
\to
V_{n+1}
\to
ResidualQuestion
\to
c_{n+1}.
\]

Represent operational crawl state:

```rust
struct CrawlStateIR {
    root_question: QueryRef,

    current_cue: FormRef,

    active_view: ActiveViewRef,

    residual: ProtectedCompletionFieldRef,

    budget: Budget,

    visited_routes: Vec<AccessRouteRef>,
}
```

Stop on:

```text
Resolved
NoProductiveResidual
ResourceBounded
Blocked
Unknown
```

None means "nothing else exists in memory."

---

# 31. One generic separator problem

This is the main compression produced by the recent exploration.

Use:

```rust
struct SeparatorProblemIR {
    residual: ProtectedCompletionFieldRef,

    target: Option<ProtectedClassRef>,

    grain: GrainRef,
    horizon: HorizonRef,

    binding: BindingVersionRef,

    available_structure: StructureViewRef,

    generator_regime: GeneratorRegimeRef,

    effectivity: EffectivityRef,
}
```

Interpretation:

\[
\boxed{
\text{Which currently surviving protected classes still need to be kept apart?}
}
\]

`target=None`:

ordinary unresolved inquiry.

`target=Some(t)`:

regenerate or refine one target class, as in cue extension.

The same object is used for:

- residual next-question generation;
- cue extension;
- reciprocal mismatch repair;
- reopening old folds;
- fresh discrimination.

Do not implement these as separate separator engines.

---

# 32. Lawful internal question construction

The current question language is generated from represented structure using only existing operations.

The core constructor family is:

\[
\boxed{
Bind,\ Expose,\ Continue,\ Residualize,\ Instantiate.
}
\]

These derive:

- reverse orientation;
- reciprocal return;
- alternative open ports;
- answer-dependent next questions;
- cue extension;
- parameter variation;
- registered probe invocation;
- residual separator questions.

Record derivation provenance:

```rust
enum GeneratorDerivationIR {
    BindPort {
        relation: RelationRef,
        port: PortRef,
    },

    ExposePort {
        relation: RelationRef,
        port: PortRef,
    },

    ContinueFromAnswer {
        predecessor: QueryRef,
        continuation: IProgRef,
    },

    ResidualSeparator {
        residual: ProtectedCompletionFieldRef,
    },

    InstantiateFamily {
        family: QueryFamilyRef,
        parameter: FormRef,
    },

    InvokeRegisteredProbe {
        family: ProbeFamilyRef,
        parameters: Vec<FormRef>,
    },
}
```

These are not new semantic primitives.

They are provenance labels describing ordinary query/program construction.

---

# 33. Generator regime

The authoritative open-ended object is not a static discriminator list.

```rust
struct GeneratorRegimeIR {
    routes: Vec<GeneratorRouteRef>,

    coverage: GeneratorCoverage,

    binding: BindingVersionRef,

    effectivity: EffectivityRef,
}
```

Coverage:

```rust
enum GeneratorCoverage {
    ExactFinite {
        certificate: ArtifactRef,
    },

    ConstructivelyComplete {
        regime: BindingRegimeRef,
        certificate: ArtifactRef,
    },

    FairOpenEnded {
        effectivity: EffectivityRef,
    },

    Working {
        effectivity: EffectivityRef,
    },
}
```

The currently materialized discriminator set is only:

\[
D_t
=
Materialize(\mathcal G_t,\epsilon).
\]

Do not confuse finite materialization with generative capability.

---

# 34. Separator generator

```rust
trait SeparatorGenerator {
    fn generate(
        &self,
        problem: &SeparatorProblemIR,
        budget: GeneratorBudget,
    ) -> SeparatorSearchResult;
}
```

Result:

```rust
enum SeparatorSearchResult {
    Generated {
        candidates: Vec<GeneratedInquiry>,
        coverage: SearchCoverage,
    },

    ExactNoSeparatorWithinRegime {
        certificate: NoSeparatorCertificateRef,
    },

    ResourceBounded {
        frontier: SearchFrontierRef,
    },

    Unknown {
        residual: ProtectedCompletionFieldRef,
    },
}
```

Candidate:

```rust
struct GeneratedInquiry {
    program: IProgRef,

    origin: GeneratorDerivationRef,

    predicted_partition:
        Option<ProtectedPartitionRef>,

    applicability: ApplicabilityRef,

    cost_estimate: Option<CostVectorRef>,

    provenance: Vec<ArtifactRef>,
}
```

The output is `IProgRef`, not merely `QueryRef`, because later questions may depend on earlier answers.

---

# 35. Generation and selection are separate

Never identify:

\[
\text{can generate }q
\]

with:

\[
\text{will ask }q.
\]

Selection:

```rust
trait QuestionPolicy {
    fn select(
        &self,
        problem: &SeparatorProblemIR,
        candidates: &[GeneratedInquiry],
    ) -> SelectionResult;
}
```

Initial deterministic policy:

1. require type-valid candidate;
2. require applicable route;
3. require sufficient discharge authority;
4. prefer exact known separator over speculative candidate;
5. prefer already executable route;
6. deterministic stable tie-break.

Do not implement learned question ranking in the first reference runtime.

---

# 36. Adaptive question omission

A candidate reciprocal/question role \(r\) may be omitted only when its answer cannot distinguish the current protected completion field.

Let:

\[
\pi_r(\mathcal Z_t)
\]

be the answers \(r\) would induce over surviving protected completions.

If:

\[
\left|
\pi_r(\mathcal Z_t)/\equiv_{\mathcal H}
\right|
=1,
\]

then \(r\) is currently non-separating.

It may be skipped under the current horizon.

This is the lawful source of adaptive reciprocal compression.

If the residual later splits, \(r\) may become live again.

---

# 37. Cue sufficiency

Given target class:

\[
t=[z]_{\mathcal H,g},
\]

and candidate cue atoms \(d\), define:

\[
Sep_t(d)
=
\{
r\neq t:
d\text{ separates }t\text{ from }r
\}.
\]

Cue \(C\) is sufficient iff:

\[
\bigcup_{d\in C}Sep_t(d)
=
\mathcal Z_t\setminus\{t\}.
\]

Equivalently:

\[
Sol_{W+C}/\equiv_{\mathcal H,g}
=
\{t\}.
\]

Cue sufficiency is derived from the same separator relation.

No `MinimalCue` semantic primitive exists.

---

# 38. Cue minimization

For small finite exact fields, use exact search.

Possible claims:

```rust
enum CueOptimality {
    ExactInclusionMinimal {
        certificate: ArtifactRef,
    },

    ExactMinimumCardinality {
        certificate: ArtifactRef,
    },

    ExactMinimumCost {
        order: CostOrderRef,
        certificate: ArtifactRef,
    },

    ParetoFrontier {
        order: PartialOrderRef,
    },

    WorkingApproximation {
        method: MethodRef,
        regime: EffectivityRef,
    },
}
```

Exact and approximate planning must never share the same warrant tag.

Minimum cues need not be unique.

Do not impose a universal scalar cost.

---

# 39. Grain-sensitive cue reopening

Suppose coarse cue \(C_g\) gives:

\[
Sol_{W+C_g}/\equiv_{\mathcal H,g}
=
\{[z]_g\}.
\]

At finer grain \(g'\):

\[
\left|
Sol_{W+C_g}/\equiv_{\mathcal H,g'}
\right|
>1.
\]

Then:

```text
old cue
-> residual protected rivals
-> separator problem
-> additional question/probe
-> extended cue
-> finer regeneration
```

Do not store one horizon-independent `HigherResolutionCue`.

The correct cue is regenerated relative to:

```text
grain
horizon
binding
available generator regime
resource regime
```

---

# 40. Materialization gap versus expressibility gap

This is now constitutional implementation behavior.

Let:

\[
D_t
\]

be currently materialized discriminators.

Let:

\[
\mathcal Q_t
\]

be the full currently admitted generable question language.

## 40.1 Materialization gap

\[
\forall d\in D_t,\ d(x)=d(y)
\]

but:

\[
\exists q\in\mathcal Q_t,\ q(x)\neq q(y).
\]

Action:

```text
continue lawful generation
instantiate parameter family
expose another port
follow answer-dependent continuation
invoke registered probe
```

No representation change is required.

## 40.2 Expressibility gap

\[
\forall q\in\mathcal Q_t,\ q(x)=q(y)
\]

while independent protected evidence requires the classes to remain distinct.

Action:

```text
RepresentationGapIR
-> binding/representation extension inquiry
```

Do not search forever inside the same observational language.

Do not claim equivalence merely because current questions fail.

---

# 41. Representation gap

```rust
struct RepresentationGapIR {
    protected_difference:
        ProtectedDifferenceWitnessRef,

    admitted_regime:
        GeneratorRegimeRef,

    no_separator_certificate:
        NoSeparatorCertificateRef,

    residual:
        ProtectedCompletionFieldRef,

    grain: GrainRef,
    horizon: HorizonRef,
}
```

This object is only valid when:

1. a protected difference is independently witnessed; and
2. there is exact evidence of no separator in the declared admitted regime.

Resource-bounded search failure is not sufficient.

---

# 42. Fresh probe split

`FreshProbe` must distinguish:

\[
\boxed{
FreshWithinRegime
\neq
ProposedRegimeExtension.
}
\]

A registered but unused sensor/query/operator family belongs to `FreshWithinRegime`.

An LLM proposing an unregistered measurement belongs to `ProposedRegimeExtension`.

The latter is not executable until admitted through the binding-extension path.

---

# 43. Binding versions

Every semantic/event artifact carries a binding version.

```rust
struct BindingVersionIR {
    id: BindingVersionRef,

    parent: Option<BindingVersionRef>,

    type_environment: ArtifactRef,

    relation_environment: ArtifactRef,

    probe_environment: ArtifactRef,

    decoder_environment: ArtifactRef,

    provenance: Vec<ArtifactRef>,
}
```

Never mutate an old binding in place.

---

# 44. Three kinds of binding growth

## 44.1 Definitional extension

Every new observable factors through the old observational signature.

Then the old observational quotient is unchanged.

Examples:

```text
new name for old composition
derived predicate
compiled convenience method
new index
new exact macro/operator
```

No consequence-fold reopening is required merely because the language has more names.

## 44.2 Conservative observational extension

All old observations retain their meanings, while at least one genuinely new observation adds discriminatory power.

Then:

\[
\equiv_{\mathbb B'}
\subseteq
\equiv_{\mathbb B}.
\]

Old classes may split.

They cannot merge if all old observations are faithfully preserved.

This triggers targeted reopening.

## 44.3 Rebinding

Some old typed meaning changes, or no faithful bridge exists.

There is no quotient-monotonicity guarantee.

Old and new meanings remain separately versioned.

Cross-version use requires an explicit bridge.

---

# 45. Binding bridge

```rust
struct BindingBridgeIR {
    from: BindingVersionRef,
    to: BindingVersionRef,

    scope: ScopeRef,
    horizon: HorizonRef,

    type_transport:
        Vec<TypeTransportRef>,

    form_transport:
        Vec<FormTransportRef>,

    relation_transport:
        Vec<RelationTransportRef>,

    answer_transport:
        Vec<AnswerTransportRef>,

    preservation:
        PreservationStatus,

    provenance:
        Vec<ArtifactRef>,
}
```

The transports are represented typed programs/relations, not host callbacks.

Preservation condition for old query \(q\):

\[
J_R(
Ans_{\mathbb B}(q,x)
)
=
Ans_{\mathbb B'}(
J_Qq,
J_Ax
).
\]

For nondeterministic/path-sensitive questions, compare the transported answer/path relation.

---

# 46. Bridge preservation warrant

```rust
enum PreservationStatus {
    Exact {
        certificate: ArtifactRef,
    },

    ExactOnScope {
        scope: ScopeRef,
        certificate: ArtifactRef,
    },

    Working {
        fixtures: Vec<FixtureRef>,
        liabilities: Vec<OpenRef>,
    },

    Refuted {
        breaker: ArtifactRef,
    },
}
```

A finite test suite cannot prove global conservativity unless the checker contract itself establishes exhaustive coverage.

A bridge proposal does not classify itself.

---

# 47. Binding extension admission

Binding growth uses the existing patch/revision machinery.

```rust
struct BindingDeltaIR {
    predecessor: BindingVersionRef,

    proposed_types: Vec<TypeChangeRef>,
    proposed_relations: Vec<RelationChangeRef>,
    proposed_probes: Vec<ProbeChangeRef>,
    proposed_decoders: Vec<DecoderChangeRef>,

    proposed_bridge:
        Option<BindingBridgeRef>,

    target_gap:
        Option<RepresentationGapRef>,
}
```

Derived classification:

```rust
enum BindingChangeClass {
    DefinitionalExtension {
        elimination: EliminationCertificateRef,
    },

    ConservativeExtension {
        bridge: BindingBridgeRef,
        new_discrimination:
            Vec<ProtectedSplitRef>,
    },

    Rebinding {
        failed_preservations:
            Vec<BreakerRef>,

        cross_version_bridge:
            Option<BindingBridgeRef>,
    },
}
```

Admission pipeline:

```text
RepresentationGap
-> CandidateBindingDelta
-> TypeCheck
-> PreservationCheck
-> ExecutabilityCheck
-> DiscriminationCheck
-> IndependentWarrant
-> Accept / Reject
```

---

# 48. Historical actuality is version-local

An old event remains:

```text
Event @ BindingVersion_t
```

forever.

Do not rewrite:

```rust
event.binding_version = latest_binding;
```

Later use under another binding is a derived bridge operation:

\[
Bridge_{t\to t'}(Event).
\]

This is essential for replay and provenance.

---

# 49. Targeted reopening after conservative extension

A new discriminator \(d\) affects fold \(F\) only if:

\[
\exists x,y\in F:
Obs_d(x)\neq Obs_d(y).
\]

Define:

```rust
fn fold_affected_by_extension(
    fold: FoldRef,
    delta: BindingDeltaRef,
) -> Result<bool>;
```

Reopen only implicated folds.

Purely definitional extension should not reopen consequence-equivalence folds.

A rebinding requires broader bridge-specific reevaluation because quotient refinement is not monotone.

---

# 50. Claims and standing

A claim exists before it stands.

```rust
struct ClaimIR {
    proposition: FormRef,

    support_envs:
        Vec<SupportEnvironmentRef>,

    applicability: FormulaRef,

    scope: FormulaRef,
}
```

Support environment:

```rust
struct SupportEnvironmentIR {
    standing_dependencies:
        Vec<FormRef>,

    ingress_refs:
        Vec<ArtifactRef>,

    checker_refs:
        Vec<ArtifactRef>,

    assumptions:
        Vec<FormRef>,

    open_dependencies:
        Vec<OpenRef>,
}
```

Compute:

\[
Stand=\mu T.
\]

Use a simple monotone worklist first.

Seed only from independent ingress.

Rootless support cycles remain unstanding automatically.

---

# 51. Cross-binding standing lift

This is the principal remaining semantic research gate.

Safe initial implementation behavior:

1. transport the old claim and evidence through an exact conservative bridge;
2. preserve the original:
   - scope;
   - applicability;
   - grain;
   - protected horizon;
   - support provenance;
3. retain its old standing as standing **relative to the transported old regime**;
4. do not automatically promote it to an enlarged horizon;
5. if the new observation refines applicability, scope, or protected classes, create explicit open dependencies/liabilities;
6. require ordinary standing recomputation before granting standing over the enlarged regime.

Represent the derived result as:

```rust
struct LiftedClaimViewIR {
    source_claim: ClaimRef,

    bridge: BindingBridgeRef,

    transported_claim: FormRef,

    preserved_scope: ScopeRef,
    preserved_applicability: FormulaRef,
    preserved_grain: GrainRef,
    preserved_horizon: HorizonRef,

    inherited_support:
        Vec<SupportRef>,

    new_open_dependencies:
        Vec<OpenRef>,
}
```

This is a derived cross-binding view, not a new claim ontology.

Research gate:

> determine the least rule that permits automatic reuse beyond the old horizon without silently broadening warrant.

Until that is settled, use the conservative rule above.

---

# 52. Operator occurrences

```rust
struct OperatorOccurrenceIR {
    event: EventRef,

    operator: OperatorRef,

    state_before: StateRef,

    raw_return: ReturnRef,

    state_after: StateRef,

    distinction:
        Option<DistinctionRef>,
}
```

The occurrence graph is a derived index into authoritative event history.

It is not memory itself.

---

# 53. Method folding

```rust
struct MethodFoldIR {
    method: OperatorRef,

    expansion:
        Vec<OperatorRef>,

    applicability:
        FormulaRef,

    horizon:
        HorizonRef,

    evidence_events:
        Vec<EventRef>,

    gain:
        GainRef,

    recovery:
        RecoveryRef,

    unlock:
        Vec<UnlockConditionRef>,
}
```

A method changes traversal/execution preference.

It does not grant standing to its outputs.

Folding never deletes actual history.

---

# 54. Recovery contract

Every consequential fold carries:

```rust
struct RecoveryContractIR {
    folded: FormRef,

    protected_target:
        RelationalWebRef,

    base_grain:
        GrainRef,

    base_horizon:
        HorizonRef,

    retained_constraints:
        Vec<RelationRef>,

    generator_regime:
        GeneratorRegimeRef,

    ancestry:
        Vec<ArtifactRef>,

    unlocks:
        Vec<UnlockRef>,

    recovery_kind:
        RecoveryKind,
}
```

```rust
enum RecoveryKind {
    Direct,
    Reconstruct,
    Reacquire,
}
```

Do not store every possible future higher-resolution cue.

Regenerate it from the reopened residual.

---

# 55. Compression licence

```rust
struct CompressionLicenceIR {
    fold: FormRef,

    kind: CompressionKind,

    protected_horizon:
        HorizonRef,

    protected_continuations:
        Vec<FormRef>,

    scope:
        ScopeRef,

    evidence:
        Vec<ArtifactRef>,

    residual:
        ArtifactRef,

    recovery:
        RecoveryContractRef,

    unlock:
        Vec<UnlockRef>,

    distortion:
        Option<ArtifactRef>,
}
```

Exact and approximate compression remain distinct.

No approximation is allowed to masquerade as exact quotienting.

---

# 56. Unlock and reopen

Possible unlock triggers:

```rust
enum UnlockReasonIR {
    NewSeparator,

    NewProtectedContinuation,

    NewObservation,

    BindingExtended,

    BindingRebound,

    GrainChanged,

    ScopeChanged,

    ApplicabilityChanged,

    AssumptionsChanged,

    RecoveryContractInvalidated,

    CompilerChanged,
}
```

Reopening must:

```text
retain old fold as ancestry
locate smallest implicated identification
restore retained residual/provenance
regenerate current separator problem
refine active representation
record why the old licence no longer applies
```

---

# 57. Surprise

Mismatch requires a positive protected discriminator.

Do not implement:

```text
if !prove_equivalent(predicted, actual):
    mismatch
```

Instead:

\[
Mismatch_{\mathcal H}(\hat r,r)
\iff
\exists K\in\mathcal H:
Obs(K,\hat r)\neq Obs(K,r).
\]

Then:

```text
MismatchWitness
-> Unlock
-> Reopen
-> SeparatorProblem
-> inquiry
```

Failure to find a separator is not proof of equivalence.

---

# 58. Persistence

Use:

\[
\boxed{
SQLite
+
immutable content-addressed artifacts.
}
\]

Authoritative:

```text
immutable artifacts
actual events
accepted patches
binding versions
binding bridge certificates
```

Derived/rebuildable:

```text
question trace
return trace
operator occurrence index
active views
access indexes
standing cache
support reverse index
method lookup
unlock index
cue plans
separator candidates
affected-fold index
cross-binding transported views
```

No authoritative `memory_state` table.

No authoritative vector index.

No authoritative current summary.

---

# 59. Journal ordering

Preserve distinctions among:

```text
ledger append order
domain succession
boundary traversal
program control flow
```

One does not imply another.

If a protected relation depends on one of these, store/derive it explicitly.

---

# 60. Replay

Replay must reconstruct the protected accepted presentation from:

```text
authoritative event prefix
accepted patch prefix
historical binding versions
compiler versions
decoder versions
renderer versions
nondeterministic external returns
```

Pure deterministic computation may be rerun.

External returns are immutable inputs to replay.

Never silently rerun an irreversible probe when replaying.

---

# 61. Failure taxonomy

At minimum:

```rust
enum FailureClass {
    TypeError,

    InvalidRelation,

    CompilerDefect,

    PathProjectionDefect,

    RendererDefect,

    ProviderFailure,

    UnknownActuality,

    DecoderFailure,

    ResolutionFailure,

    RetrievalMiss,

    MaterializationGap,

    ExpressibilityGap,

    RepresentationGap,

    BindingBridgeFailure,

    WarrantGap,

    ReplayDefect,

    FoldFailure,

    RecoveryFailure,

    PerformanceBoundary,

    SpecificationGap,

    FixtureDefect,
}
```

Classify failure before modifying architecture.

---

# 62. Concurrency

Initial execution model:

```text
semantic controller: serial
authoritative database commits: one writer
pure computation: parallel if desired
independent external probes: optionally concurrent only with represented independence
```

Do not let scheduler order become semantic order accidentally.

Add stronger concurrency only after an actual protected throughput or independence fixture requires it.

---

# 63. Recurrent inquiry controller

Use a bounded recurrence, not a generic autonomous-agent framework.

```rust
fn inquire(
    state: InquiryStateRef,
    fuel: u64,
) -> Result<InquiryCheckpoint>;
```

One iteration:

```text
1. materialize current accepted presentation
2. recompute/refresh standing
3. recover current open question/residual
4. construct active view
5. run cue-guided access crawl as needed
6. construct ProtectedCompletionField
7. construct SeparatorProblem
8. generate lawful candidate IProgs
9. select one using deterministic QuestionPolicy
10. lower to Return/Branch/Probe
11. persist request/suspension if external
12. actualize Probe
13. preserve raw return
14. append ActualEvent
15. resolve/decode
16. refine completion field
17. update claims/support
18. recompute standing
19. fire unlocks/reopenings
20. produce next residual question
21. decrement fuel
```

Stop statuses:

```rust
enum InquiryStop {
    Satisfied,
    Impossible,
    Equivalent,
    Blocked,
    Unknown,
    ResourceBounded,
}
```

---

# 64. Self-revision

All semantic/compiler/binding/protection changes are candidate patches.

```rust
enum PatchRole {
    Semantic,
    Traversal,
    Compiler,
    Binding,
    Protection,
    Implementation,
}
```

```rust
struct PatchIR {
    role: PatchRole,

    predecessor:
        VersionRef,

    changes:
        Vec<ChangeRef>,

    preserved_obligations:
        Vec<ObligationRef>,

    dispositions:
        Vec<DispositionRef>,

    strict_gain_or_defect:
        ArtifactRef,

    evidence:
        Vec<ArtifactRef>,

    regression_fixtures:
        Vec<FixtureRef>,

    reopening_effects:
        Vec<UnlockRef>,
}
```

Acceptance is predecessor-judged.

A candidate patch may not modify its own acceptance criteria and then use the modified criteria to pass.

---

# 65. LLM role

An LLM may:

```text
generate candidate distinctions
generate candidate relations
propose questions
propose separator programs
propose interpretations
propose implementation patches
render natural language
generate candidate binding extensions
```

An LLM may not, merely by generation:

```text
establish external actuality
certify semantic equivalence
prove no separator exists
grant standing
admit a binding extension
warrant its own patch
declare a retrieval miss as absence
```

Every LLM output crosses an explicit typed authority boundary.

---

# 66. Conformance suite

The repository must eventually include executable fixtures for at least the following.

## Typing and relation structure

1. Well-typed relation composition succeeds.
2. Ill-typed relation composition fails.
3. Partial binding preserves every remaining open port.
4. One relation schema can produce multiple differently oriented questions.
5. No dynamic untyped payload crosses a semantic boundary.

## Question and program structure

6. `Ask` continuation can construct a question not determined before the answer.
7. No host callback is required for answer-dependent continuation.
8. `Generate` cannot discharge `Probe`.
9. `Generate` cannot discharge `Check`.
10. `Check` cannot automatically discharge `Warrant`.

## Reciprocal distinction

11. Boundary state is not duplicated by orientation.
12. Boundary traversal is not forced to cross sides.
13. Sixfold can be regenerated from two reciprocal path occurrences.
14. One-way crossing can fail to expose return feedback.
15. Two one-way crossings can fail to expose a distinction exposed by return questions.
16. Explicit sixfold first-return and two-seed compiled round trips produce the same protected traces over finite exact fixtures.
17. Nondeterministic branch correlation is preserved.
18. Aggregate crossing and aggregate return sets fail the path-correlation breaker.
19. Partial answer objects remain actual continuation binders.
20. A reciprocal role is skipped only when every surviving protected completion agrees on its answer.

## Actuality and resolution

21. Raw external return is persisted before decoding.
22. Raw return remains immutable after interpretation.
23. Same operator can occur multiple times with different event IDs.
24. Resolution provenance is explicit.
25. Partial answer narrowing leaves a residual fiber.
26. Ambiguity does not become a guessed singleton.

## Paired actuality

27. Question trace and return trace derive from one event spine.
28. Missing return can be reconstructed from its residual cue when uniquely determined.
29. Missing question can be reconstructed reciprocally.
30. Reconstruction does not create a second memory ontology.

## Memory/cue traversal

31. A retained form may be inaccessible under the initial cue.
32. Retrieval failure does not imply absence.
33. Recurrent cue traversal can reach material inaccessible to one-shot retrieval.
34. Accessibility does not imply activation.
35. Activation does not imply standing.
36. Reserve structure remains reachable.
37. Licensed occluded structure requires reopening before re-entry.

## Separator engine

38. Ordinary residual inquiry and cue extension produce equivalent `SeparatorProblemIR` for extensionally identical residual fields.
39. Separator candidates are `IProgRef`, not just flat `QueryRef`.
40. Generation capability is independent from selection policy.
41. A materialization gap can be solved without binding extension.
42. Resource-bounded generation failure remains `Unknown`/`ResourceBounded`.
43. Exact no-separator certification is distinguished from failure to find a separator.

## Cue planning

44. Finite sufficient cue hits every protected rival class.
45. Inclusion-minimal cue need not be unique.
46. Minimum-cardinality and minimum-cost cues can differ.
47. Approximate cue planner cannot claim exact minimality.
48. Finer grain can invalidate coarse cue sufficiency.
49. Residual finer-grain classes generate separator obligations.
50. Different protected horizons can require different cue extensions.

## Expressibility and fresh discrimination

51. In a linear fixture, queries inside the current represented span cannot distinguish a hidden orthogonal direction.
52. A represented but unmaterialized independent direction repairs the materialization gap.
53. A protected split outside the admitted observational language yields `RepresentationGapIR`.
54. Boolean recombination of observationally identical finite signatures cannot create a separator.
55. Registered-but-unused probe and unregistered proposed probe remain distinct.

## Binding evolution

56. Definitional extension leaves the old observational quotient unchanged.
57. Conservative observational extension refines but never coarsens the old quotient.
58. Reinterpretation of an old observation fails conservative-extension certification.
59. Typed representation renaming with a faithful bridge may preserve semantics.
60. Old `ActualEvent` retains its original binding version after extension.
61. Cross-binding interpretation uses a bridge.
62. Conservative extension preserves old `IProg` traces on its certified scope.
63. Rebinding may change an early answer and therefore the future inquiry path.
64. New conservative discriminators reopen only affected folds.
65. Definitional extension does not reopen consequence folds merely because new names exist.

## Standing and warrant

66. Candidate claim does not automatically stand.
67. Multiple support environments can support one claim.
68. Rootless support cycle remains unstanding.
69. Applicability and support remain separate.
70. LLM-generated argument cannot self-promote.
71. Cross-binding lifted evidence retains original horizon/scope.
72. Enlarged horizon creates liabilities rather than automatic promotion.

## Folding and reopening

73. Fold retains evidence-event references.
74. Fold never deletes authoritative events.
75. Reopening restores the smallest implicated hidden structure.
76. New separator can reopen old fold.
77. New binding discriminator can reopen old quotient.
78. Approximate fold retains distortion contract.

## Self-revision

79. Candidate patch cannot alter its own predecessor acceptance contract.
80. Rejected patch leaves accepted state unchanged.
81. Accepted patch retains predecessor ancestry.
82. Historical replay uses historical compiler/binding versions.
83. Self-revision cannot self-warrant.

---

# 67. Build phases

The agent should build in this order.

## Phase 0 — repository authority and scaffolding

Implement:

```text
workspace
pinned Rust toolchain
Cargo.lock
schema migrations
artifact canonicalization
decision/failure logs
CI
```

Gate:

```text
cargo check
cargo test
canonical encode/decode test
stable content identity
```

---

## Phase 1 — typed form kernel

Implement:

```text
TyIR
TypedFormRef
type checking
canonical artifacts
BindingVersionRef
```

Gate:

```text
well-typed values accepted
ill-typed values rejected
stable typed form identity
```

---

## Phase 2 — relation and open-query kernel

Implement:

```text
RelSchemaIR
FormulaIR
TermExprIR
Bind
Expose
OpenQueryIR
completion fibers
```

Gate:

```text
partial-binding fixtures
relation composition fixtures
multiple question orientations
```

---

## Phase 3 — first-order inquiry programs

Implement:

```text
IProgIR
answer binder
capture-safe substitution
normalization
PureOpContract
```

Gate:

```text
serialized program round trip
no host callbacks
answer-dependent next question
```

---

## Phase 4 — reciprocal compilation

Implement:

```text
DistinctionIR
orientation
compile_roundtrip
compile_reciprocal_first_return
derived sixfold view
path-preserving trace normalization
```

Gate:

```text
return-feedback breaker
orientation-asymmetry breaker
nondeterministic path-correlation breaker
finite compiler-equivalence fixtures
```

---

## Phase 5 — runtime machine

Implement:

```text
ProgramIR
Return
Branch
Probe
verifier
pure stepping
suspension/resume
```

Gate:

```text
well-typed lowering
invalid program rejection
live/replay resume consistency
```

---

## Phase 6 — persistence and actuality

Implement:

```text
SQLite journal
content-addressed artifacts
ActualEvent
request-before-dispatch
raw-return persistence
crash/restart fixture
```

Gate:

```text
no committed dangling references
raw return preserved before interpretation
event binding version immutable
```

---

## Phase 7 — resolution and fibers

Implement:

```text
ResolutionPathIR
decoder contracts
partial answers
HoleIR
FiberIR
ProtectedCompletionFieldIR
```

Gate:

```text
complete/partial/ambiguous results
strict fiber refinement
no finite-search false uniqueness
```

---

## Phase 8 — paired actuality

Implement:

```text
question trace view
return trace view
missing return residual
missing question residual
event-path provenance
```

Gate:

```text
one authoritative spine
paired reconstruction fixtures
```

---

## Phase 9 — active views and memory crawl

Implement:

```text
ActiveViewIR
reserve
occlusion licence
access routes
AccessWitnessIR
ActivationWitnessIR
CrawlStateIR
```

Gate:

```text
retained != accessible
accessible != active
one-shot miss vs recurrent recovery
occlusion unlock
```

---

## Phase 10 — LLM/backend compiler

Implement:

```text
ProbeOperatorIR
AnswerContractIR
SurfacePlanIR
ContextSelectionIR
BackendRequestIR
RawReturnEnvelopeIR
MockProvider
one real provider adapter
```

Gate:

```text
semantic probe != surface plan
surface plan != backend request
raw response != extracted text
renderer cannot strengthen answer contract
```

---

## Phase 11 — standing

Implement:

```text
ClaimIR
SupportEnvironmentIR
independent ingress
least-fixed-point standing
simple monotone worklist
```

Gate:

```text
rootless cycle
alternative support environments
open dependencies
applicability != support
```

---

## Phase 12 — generic separator engine

Implement:

```text
SeparatorProblemIR
GeneratorRegimeIR
GeneratorDerivationIR
SeparatorGenerator
GeneratedInquiry
deterministic QuestionPolicy
```

Use only current admitted constructors:

```text
Bind
Expose
Continue
Residualize
Instantiate
```

Gate:

```text
residual inquiry == cue separator form
answer-dependent candidate program
materialized basis != generator regime
```

---

## Phase 13 — cue planning

Implement:

```text
CuePlanIR
finite separator matrix
exact small-instance planner
working approximate planner
grain/horizon cue refinement
```

Gate:

```text
minimality nonuniqueness
exact vs approximate warrant
finer-grain reopening
```

---

## Phase 14 — representation-gap detection

Implement:

```text
ExactNoSeparatorWithinRegime
RepresentationGapIR
FreshWithinRegime
ProposedRegimeExtension
```

Start with exact finite/analytic fixture bindings.

Gate:

```text
materialization gap solved by generation
expressibility gap cannot be solved internally
no-separator failure != no-separator certificate
```

---

## Phase 15 — binding extension and bridges

Implement:

```text
BindingBridgeIR
BindingDeltaIR
definitional extension checker
conservative-extension checker
rebinding classification
targeted fold affectedness
```

Gate:

```text
definitional quotient unchanged
conservative quotient refinement
rebind breaker
historical event version locality
old IProg trace preservation under bridge
```

---

## Phase 16 — method learning and folds

Implement:

```text
OperatorOccurrenceIR
MethodFoldIR
RecoveryContractIR
CompressionLicenceIR
Unlock
Reopen
```

Gate:

```text
fold preserves protected behavior
history retained
method applicability explicit
reopening works
binding extension reopens affected folds
```

---

## Phase 17 — cross-binding standing lift

Implement the conservative initial rule:

```text
transport old claim/evidence
retain original scope
retain original applicability
retain original grain
retain original horizon
reuse unaffected support
create open liabilities for newly inspectable distinctions
recompute standing before enlarged-horizon promotion
```

Gate:

```text
insensitive claim reuses support
split equivalence claim reopens
refined applicability creates dependency
no silent horizon enlargement
```

Keep stronger automatic lift rules as a research gate.

---

## Phase 18 — predecessor-judged self-revision

Implement:

```text
PatchIR
PatchRole
predecessor tests
regression locking
versioned replay
binding/compiler/protection patch admission
```

Gate:

```text
candidate cannot self-promote
candidate cannot rewrite its own judge
old presentation reconstructible
```

---

## Phase 19 — autonomous breadth only after evidence

Only now consider:

```text
learned question-selection policy
open-ended fair generator execution
resumable generator cursors
vector retrieval
parallel semantic scheduling
PostgreSQL
distributed effects
more provider adapters
more binding-native methods
```

Each requires a protected breaker or measured performance boundary.

---

# 68. Vertical slices

Do not wait for all phases before exercising end-to-end semantics.

## Vertical slice A — one real recurrence

Demonstrate:

```text
typed relation
-> OpenQuery
-> reciprocal distinction
-> IProg
-> Probe
-> BackendRequest
-> RawReturn
-> ActualEvent
-> Resolution
-> residual next question
```

No fake semantic step.

---

## Vertical slice B — standing

Extend A:

```text
resolved completion
-> claim
-> independent support
-> standing
```

Then demonstrate a rootless support cycle that does not stand.

---

## Vertical slice C — relational memory

Demonstrate:

```text
old distinction leaves active context
many unrelated events occur
later residual cue makes old relation accessible
recurrent crawl retrieves it
later inquiry changes because of it
```

Removing the retained relation must break the fixture.

---

## Vertical slice D — separator/cue unification

Construct one residual field and feed it to:

```text
ordinary residual inquiry
cue refinement
```

Both must compile to the same underlying separator obligation where extensionally identical.

---

## Vertical slice E — representation gap

Demonstrate:

```text
protected split
+ exact no separator in current language
-> RepresentationGapIR
-> CandidateBindingDelta
-> conservative extension
-> reopened fold
-> newly executable separator
```

Old actuality remains unchanged.

---

## Vertical slice F — method folding

Demonstrate:

```text
repeated inquiry path
-> method fold
-> cheaper traversal
-> new protected separator
-> reopen
-> recover original constituent path
```

---

## Vertical slice G — self-revision

Reify a small renderer/compiler policy.

Generate candidate successor.

Judge it using locked predecessor obligations.

Accept only with independent evidence and strict gain/repair.

---

# 69. What should remain algorithmically simple first

Implement semantically correct, deliberately simple versions of:

```text
standing:
    full fixed-point worklist

question selection:
    deterministic stable policy

retrieval:
    exact relation/index traversal

fiber solving:
    finite enumeration where possible,
    otherwise symbolic residual

cue minimization:
    brute force below small threshold

method discovery:
    repeated exact path detection

active view:
    conservative inclusion

checkpointing:
    simple periodic snapshots

provider support:
    mock + one real adapter

binding preservation:
    exact fixture-specific checkers first
```

Optimize only after measurement.

---

# 70. What must not be mocked in a "complete" slice

These must be real:

```text
typed relation identity
partial binding
question identity
answer-dependent continuation
path provenance
raw return preservation
event ancestry
historical binding version
explicit resolution
standing separation
residual question generation
operator occurrence linkage
fold recovery
reopening
binding bridge semantics
```

External environments/providers may be mocked only as explicit bindings.

---

# 71. Deferred architecture

Do not build yet:

```text
general agent framework
global task ontology
generic scheduler
distributed log
graph database
vector database
provider-session semantic state
global learned relevance model
global scalar question utility
universal minimal cue
universal inquiry basis
automatic semantic rebinding
```

Each remains absent until a concrete protected use requires it.

---

# 72. Implementation-agent protocol

Put the following into `AGENTS.md`.

```text
FOR EVERY CONSEQUENTIAL CHANGE:

1. Read IMPLEMENTATION_FRONTIER.md.
2. Reconstruct the current accepted implementation state.
3. Identify the strongest live obligation.
4. State the protected observable difference.
5. State the smallest executable fixture.
6. Check whether existing typed machinery already regenerates it.
7. Prefer reuse over extension.
8. Prefer extension over new abstraction.
9. Implement the smallest responsible change.
10. Run the targeted fixture.
11. Preserve all actual returns and failure evidence.
12. Classify failure before redesigning architecture.
13. Run all completed phase gates.
14. Run cold replay if authoritative or semantic state changed.
15. Remove newly redundant machinery.
16. Update DECISIONS.jsonl and FAILURES.jsonl.
17. Regenerate IMPLEMENTATION_FRONTIER.md.
18. Commit code, tests, and decision evidence together.

NEVER:

- add a host semantic callback because a relation is inconvenient;
- treat model output as standing;
- treat retrieval failure as absence;
- treat finite failure to distinguish as exact equivalence;
- rewrite historical actuality under a newer binding;
- let a candidate patch define its own acceptance rule;
- add architecture solely because an earlier exploratory design contained it.
```

---

# 73. Required project-state files

Maintain:

```text
IMPLEMENTATION_FRONTIER.md
CONFORMANCE_STATUS.md
DECISIONS.jsonl
FAILURES.jsonl
```

Decision record:

```json
{
  "id": "D-0001",
  "question": "...",
  "alternatives": ["...", "..."],
  "protected_difference": "...",
  "evidence": ["..."],
  "chosen": "...",
  "status": "EXACT|WORKING",
  "reopen_if": ["..."]
}
```

Failure record:

```json
{
  "id": "F-0001",
  "fixture": "...",
  "expected": "...",
  "observed": "...",
  "classification": "...",
  "responsible_layer": "...",
  "status": "OPEN|RESOLVED"
}
```

These bootstrap files may later become ordinary retained project artifacts.

---

# 74. CI gates

Every change:

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
migration test
artifact canonicalization test
type/IR verifier tests
```

Semantic/history changes additionally require:

```text
cold replay
incremental vs replay equality
historical version fixture
```

Compiler changes additionally require:

```text
source-program preservation
path-provenance preservation
renderer contract fixtures
```

Standing changes additionally require:

```text
full fixed point vs incremental result
rootless-cycle fixture
support-environment fixtures
```

Binding changes additionally require:

```text
bridge preservation fixtures
old-event immutability fixture
old-IProg trace transport
affected-fold reopening
```

---

# 75. Observability

Log represented transformations, not hidden reasoning.

Every inquiry occurrence should expose references for:

```text
binding_version
question
distinction
active_view
separator_problem
generated_inquiry
selection_policy
probe
surface_plan
backend_request
raw_return
event
resolution
claim
standing_update
method
fold
patch
grain
horizon
```

The runtime should be able to answer:

```text
Why was this question generable?
Why was it selected?
What residual classes did it separate?
What context was active?
What remained reserve?
What was occluded and under which licence?
What actually returned?
How was it resolved?
Why did a claim stand?
Why was a fold reopened?
Which binding interpreted this event?
Why was a binding extension considered conservative or not?
```

---

# 76. Performance policy

Instrument before optimizing:

```text
artifact I/O
journal writes
relation queries
fiber solve time
active-view construction
memory crawl depth
separator generation count
question-selection cost
cue-planner search size
standing iterations
prompt context size
provider latency/cost
fold recovery cost
replay time
bridge-check cost
```

Cross architecture boundaries only when measured.

Examples:

```text
SQLite write bottleneck
    -> evaluate PostgreSQL

exact cue search too expensive
    -> working planner

relational retrieval insufficient
    -> test lexical/vector route

standing recomputation dominates
    -> semi-naive incrementalization

open generator starvation appears
    -> import fair/resumable generation

parallel independent probes useful
    -> introduce represented concurrency
```

---

# 77. Production migration path

Preserve semantic independence from physical layout.

Expected path:

\[
\boxed{
\begin{array}{c}
Rust\ monolith\\
+
SQLite\\
+
local\ content\ store\\
\downarrow\\
better\ reconstructible\ indexes\\
\downarrow\\
PostgreSQL/object\ storage\ if\ measured\\
\downarrow\\
parallel/distributed\ effect\ execution\ if\ measured.
\end{array}
}
\]

Do not migrate semantics with storage.

---

# 78. Current research gates

The architecture no longer depends on resolving these before implementation.

## 78.1 Cross-binding standing lift

Current safe rule:

```text
transport old standing only at its old scope/applicability/grain/horizon
reuse unaffected evidence
create liabilities for newly inspectable distinctions
recompute before broader promotion
```

Research question:

> Under what exact conditions can more of the old standing be promoted automatically through a conservative bridge?

---

## 78.2 Open-ended generator completeness

Finite and binding-native exact regimes can certify separator generation.

The general open-ended/LLM regime remains:

```text
Working
or
FairOpenEnded
```

until stronger coverage is established.

Do not claim a universal finite inquiry basis.

---

## 78.3 Question-policy optimization

Later compare:

```text
worst-case residual contraction
expected protected information gain
cost-aware decision trees
learned program policies
```

The object of eventual optimization is an answer-dependent inquiry policy, not necessarily a flat question ranking.

---

## 78.4 Retrieval basis

Only after recurrent relational retrieval is implemented should the project test whether lexical/vector routes produce protected strict gain.

No vector memory architecture is assumed.

---

## 78.5 Resumable/fair generators

Import stronger scheduler/generator machinery only if an actual fixture shows:

```text
starvation
restart loss
large open-ended generation
generator-specific resumable state
```

---

# 79. Current relative fixed point

The implementation should now treat the following as settled unless a future breaker reopens them:

\[
\boxed{
\text{Question}
=
\text{partial represented relation binding}.
}
\]

\[
\boxed{
IProg
=
Return
\mid
Ask(q,\kappa)
}
\]

with first-order answer-dependent continuation.

\[
\boxed{
\text{runtime}
=
Return
\mid
Branch
\mid
Probe.
}
\]

\[
\boxed{
\text{sixfold first-return behavior}
=
\text{two independently seeded path-preserving reciprocal round trips}.
}
\]

\[
\boxed{
\text{sixfold is a derived view, not a separate authoritative runtime object}.
}
\]

\[
\boxed{
\text{path correlation must survive nondeterminism}.
}
\]

\[
\boxed{
\text{retained}
\neq
\text{accessible}
\neq
\text{active}
\neq
\text{standing}.
}
\]

\[
\boxed{
\text{cue motion is recurrent answer-dependent inquiry over retained relational structure}.
}
\]

\[
\boxed{
\text{residual inquiry}
=
\text{cue extension}
=
\text{reciprocal repair}
=
\text{fold reopening}
}
\]

at the level of the common separator problem, with different boundary conditions.

\[
\boxed{
\text{generator regime}
\neq
\text{materialized discriminator set}
\neq
\text{question-selection policy}.
}
\]

\[
\boxed{
\text{internal question synthesis cannot create an observational dimension absent from the admitted language}.
}
\]

\[
\boxed{
\text{materialization gap}
\neq
\text{expressibility gap}.
}
\]

\[
\boxed{
\text{expressibility gap}
\to
\text{governed binding-extension inquiry}.
}
\]

\[
\boxed{
\text{binding growth}
=
\text{definitional extension}
\mid
\text{conservative observational extension}
\mid
\text{rebinding}.
}
\]

\[
\boxed{
\text{conservative extension preserves old meaning and may only refine old observational equivalence}.
}
\]

\[
\boxed{
\text{historical actuality is never rewritten under later bindings}.
}
\]

\[
\boxed{
\text{standing is positive, provenance-carrying, and non-self-licensing}.
}
\]

\[
\boxed{
\text{folds preserve ancestry and remain reopenable}.
}
\]

---

# 80. Definition of first complete implementation

The first complete reference implementation exists when one repository can execute and replay:

\[
\boxed{
\begin{aligned}
&\text{typed forms and relations}\\
&\to
\text{partial binding / OpenQuery}\\
&\to
\text{answer-dependent IProg}\\
&\to
\text{path-preserving reciprocal inquiry}\\
&\to
\text{question-conditioned active view}\\
&\to
\text{generic separator problem}\\
&\to
\text{lawfully generated inquiry program}\\
&\to
\text{Return/Branch/Probe runtime}\\
&\to
\text{immutable raw actuality}\\
&\to
\text{paired event history}\\
&\to
\text{explicit resolution}\\
&\to
\text{protected completion refinement}\\
&\to
\text{standing}\\
&\to
\text{residual next question}\\
&\to
\text{recurrent cue-guided retrieval}\\
&\to
\text{operator occurrence learning}\\
&\to
\text{regenerative fold}\\
&\to
\text{separator-triggered reopening}\\
&\to
\text{representation-gap detection}\\
&\to
\text{governed binding extension/rebinding}\\
&\to
\text{cross-version replay}\\
&\to
\text{predecessor-judged self-revision}.
\end{aligned}
}
\]

No step may be supplied by an untyped hidden semantic controller.

---

# 81. Immediate coding order

For an implementation agent starting now:

```text
1. Phase 0: repository/scaffolding/canonical artifacts
2. Phase 1: typed forms + binding version identity
3. Phase 2: relation schemas + OpenQuery
4. Phase 3: first-order IProg
5. Phase 4: reciprocal compiler + path-preservation tests
6. Phase 5: Return/Branch/Probe runtime
7. Phase 6: event store + raw actuality
8. Phase 7: resolution + holes/fibers
9. Phase 8: paired actuality views
10. Phase 9: active views + recurrent memory access
11. Phase 10: LLM/backend compiler
12. Phase 11: standing
13. Phase 12: generic separator engine
14. Phase 13: cue planning
15. Phase 14: representation-gap detection
16. Phase 15: binding extension/bridge/rebinding
17. Phase 16: folds/recovery/method learning
18. Phase 17: conservative cross-binding standing lift
19. Phase 18: self-revision
20. Phase 19: only measured breadth/optimization
```

Do not reorder later optimization work ahead of the semantic spine.

---

# 82. Principal live frontier after implementation begins

The architecture is no longer blocked by a broad unknown.

The principal remaining semantic research question is:

\[
\boxed{
\textbf{What is the least lawful cross-binding standing lift that
reuses old support through a conservative observational extension
without silently enlarging scope, applicability, grain, or protected horizon?}
}
\]

Until that is settled, the reference implementation uses the conservative lift rule from Phase 17.

Everything earlier in the build can proceed independently.

---

# 83. Final implementation directive

The implementation agent should not attempt to reproduce the exploratory history of the project.

It should implement the smallest current architecture capable of regenerating the protected behavior established by that history.

The design center is:

\[
\boxed{
\textbf{REPRESENT RELATIONS EXACTLY;
ASK THROUGH OPEN PORTS;
LET ACTUAL RETURNS DETERMINE CONTINUATIONS;
PRESERVE PATHS AND ACTUALITY;
LET RESIDUAL PROTECTED CLASSES GENERATE SEPARATORS;
FOLD ONLY WITH REGENERATION;
AND EXTEND THE LANGUAGE ONLY THROUGH GOVERNED, VERSIONED BRIDGES.}
}
\]

Everything else is either a derived view, an optimization, or a research gate.
