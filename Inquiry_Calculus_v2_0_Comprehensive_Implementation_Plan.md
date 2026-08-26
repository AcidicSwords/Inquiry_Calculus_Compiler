# Inquiry Calculus v2.0 — Comprehensive Implementation Plan
## Typed Relational Inquiry, Interrogative Succession, and Regenerative Control

**Status:** Stable forward implementation plan

**Standing semantic authority:** `Inquiry_Calculus_v2_0.tex`

**Ancestry:** The v1.1 canonical specification, paired-actuality addition,
interrogative-succession extension, predecessor implementation plan, and their
derivations remain recoverable through Git. They are compatibility and regression
ancestry, not competing active authorities.

**Purpose:** Specify one architecture, phase order, and completion contract for the
smallest complete implementation of Inquiry Calculus v2.0. Moving repository state,
fixture status, provider availability, failures, and the live work cursor
belong only to the frontier and evidence ledgers.

---

# 0. Executive statement

Inquiry Calculus v2.0 is a well-typed relational inquiry programming language. Its
implementation conservatively continues the v1.1 substrate while making
interrogative succession, realized actuality, and regenerative control one coherent
language/compiler stack:

- relations are the semantic substrate;
- forms are typed and recursively reifiable;
- questions are partially bound typed relations;
- inquiry programs are first-order `Return | Ask(q, κ)` structures;
- runtime execution lowers to `Return | Branch | Probe`;
- actual returns remain distinct from generated possibilities and interpretations;
- holes retain surrounding relational constraints;
- protected equivalence remains consequence-relative;
- standing remains independently warranted;
- authoritative history is append-only and actuality-preserving;
- folds retain ancestry, residuals, recovery, and reopening conditions;
- representation and probe bases may grow;
- self-revision remains predecessor-judged and cannot self-warrant.

The reciprocal-boundary subprogram retains the positive-negation correction:

\[
\boxed{
\text{determination}
\to
\text{typed negation}
\to
\text{positive determination of that negation}
\to
\text{return}
}
\]

and then recursively repeats the same act from the positively determined exterior.

The sixfold remains:

\[
\boxed{
\Xi_D
=
(S_X,O_X,R_X;S_Y,O_Y,R_Y)
}
\]

but the six roles are no longer jointly open independent slots.

They arise through a dependent reciprocal program:

\[
\boxed{
\begin{aligned}
S_X
&\to
\operatorname{NegField}_D^X(S_X)
\to
O_X
\to
\operatorname{RetField}_D^X(O_X),\\
O_X
&\rightsquigarrow
S_Y
\to
\operatorname{NegField}_D^Y(S_Y)
\to
O_Y
\to
\operatorname{RetField}_D^Y(O_Y).
\end{aligned}
}
\]

The architecture preserves the following exact constraints:

1. **Typed negation is not Boolean complement.** It is a role played by an ordinary oriented typed relation with positive exteriority evidence and explicit semantic coverage.

2. **Exteriority is not an unexplained predicate.** A negation incidence is sound when it can be transformed into a positive determination-departure witness: source and candidate are placed in incompatible cells by a standing discriminator relevant to the current source determination.

3. **Exteriority is determination-relative, not equivalent to protected non-equivalence.** A candidate may be exterior to the current determination while still protected-equivalent to the source under the current horizon.

4. **A live determination may admit multiple lawful negation relations.** They form a tagged family, not an untagged semantic union. Provenance must survive the return.

5. **Semantic negation coverage is distinct from execution/generator coverage.** A semantically exhaustive relation may be only partly explored; a semantically partial relation may be exhaustively executed.

6. **Return is reverse section, not state mutation.** For \(N\subseteq X\times Y\), after \(N(x,y)\), the pure return field is:
   \[
   N^{-1}[y].
   \]

7. **A source is always a candidate in the reverse section of the same incidence, but need not be uniquely recovered.** Exact return stability is therefore a fiber-level determination claim.

8. **Recovery is not a new semantic primitive.** A return fiber recovers a protected source relation when the protected observation induced by that relation is constant over the return fiber.

9. **Multiple negation relations can jointly carry information that no one relation carries alone.** Their exploratory exterior space is a tagged sum; their accumulated return information is a product signature. That informational product is not one jointly actualizable return without supported co-applicability and joint-realizability evidence.

10. **State-changing redetermination is not pure hole re-solving after adding a compatible exterior constraint.** Monotone compatible constraint addition cannot change an already determined source class. Genuine semantic change requires reconciliation/revision of the standing web, changed applicability, changed grain, changed binding, or prior underdetermination.

11. **\(\Gamma_D\) becomes a downstream compatibility contract.** It does not manufacture six arbitrary role fillings into a sixfold.

12. **The sixfold is still a derived view over ordinary typed questions, fibers, resolutions, and actual events.** No dedicated authoritative sixfold history or runtime species is required.

These constraints do not create a second controller. They specialize the same
question, actuality, separator, representation-growth, folding, binding-extension,
and self-revision machinery used by the rest of the language.

The reciprocal subprogram is therefore:

\[
\boxed{
\begin{array}{c}
\text{standing determination presentation}\\
\downarrow\\
\text{positive determination-departure relation}\\
\downarrow\\
\text{coverage-indexed typed negation use}\\
\downarrow\\
\text{tagged positive exterior occurrence}\\
\downarrow\\
\text{reverse-section return fiber}\\
\downarrow\\
\text{protected recovery}\\
\downarrow\\
\text{seed/reorientation}\\
\downarrow\\
\text{reciprocal repetition}\\
\downarrow\\
\text{residual separator}\\
\downarrow\\
\text{representation / probe / binding growth when required}.
\end{array}
}
\]

No Boolean complement, symmetric "other side", scalar similarity metric, hidden exterior oracle, or state-changing `Redet` primitive belongs in the implementation.

The language/compiler spine is:

```text
surface interrogative / domain-native syntax
-> elaboration
-> derived interrogative operators, roots, and route annotations
-> conservative interrogative lowering
-> typed open relation / OpenQuery
-> source IProg: Return | Ask
-> runtime IR: Return | Branch | Probe
-> RawReturn / ActualEvent
-> resolution / support / check / warrant / standing
-> protected residual / successor live-question frontier
```

The single high-level control recurrence is:

\[
\boxed{
\mathsf{BIND}
\to
\mathsf{OPEN}
\to
\mathsf{VARY}
\to
\mathsf{RETURN}
\to
\mathsf{DETERMINE}
\to
\mathsf{REFACTOR}
\circlearrowleft
}
\]

- **BIND** establishes carriers, relations, arrangement, scope, applicability,
  grain, authority, bindings, and protected horizon.
- **OPEN** exposes the smallest consequential unresolved relational position or an
  explicit discharge obligation.
- **VARY** constructs or admits a lawful contrast, alternative, transformation,
  breaker, or reciprocal exterior candidate.
- **RETURN** obtains the required pure, generated, actual, checked, or warranted
  return without collapsing discharge modes.
- **DETERMINE** establishes exactly what the return supports under declared
  coverage and provenance.
- **REFACTOR** refines, folds, reopens, reconstructs, reorients, or rebinds the
  representation and produces the successor bound state.

These names are a derived control grammar, not runtime opcodes. The interrogative
root family

\[
\Omega_Q=
\{
\mathsf{Expose},
\mathsf{Orient},
\mathsf{Factor},
\mathsf{Polarize},
\mathsf{Vary},
\mathsf{Ground}
\}
\]

is an algebra of available derived operators, not another sequence or wheel.
Positive-negation traversal, separator inquiry, cue reconstruction, method folding,
and the software-engineering clock are specializations, projections, bindings, or
renderings of this recurrence.

## 0.1 Project completion contract

The plan distinguishes four stable completion levels. Repository evidence decides
whether a level has been reached; this plan records only its requirements.

### `REFERENCE_CALCULUS_COMPLETE`

Required:

- constitutional v2.0 semantics and the conservative v1.1 embedding are executable;
- typed source-to-runtime lowering preserves relations, bindings, modes, supported
  answer sets, continuation identity, and actuality obligations;
- determination, positive departure, same-use reciprocal return, standing,
  separators, cues, folds, bridges, method contracts, cross-binding rules, and
  predecessor-judged revision have executable evidence at declared scopes;
- one complete inquiry cycle and every required derived view cold-replay from
  accepted roots without provider redispatch or hidden pre-restart state;
- no unrepresented semantic step is required to execute the complete successor
  chain.

### `SELF_HOSTED_INQUIRY_CONTROLLER_COMPLETE`

Required in addition to reference-calculus completion:

- the implemented calculus, not a prose scheduler, constructs and evaluates the
  live question frontier;
- `Formable`, `Applicable`, `Executable`, `Answerable`, `Productive`, `ResolvedQ`,
  `Ready`, `RequiredDischarge`, coverage, residuals, and declared resource order
  constrain selection;
- whole proof-carrying supported-answer records drive occurrence-indexed `QSucc` and reconstruct the
  next frontier;
- an LLM remains a `Generate`/provider capability inside the controller and cannot
  supply actuality, checking, warrant, or standing by generation alone;
- roots, operational interrogatives, route labels, and learned methods lower
  transparently to ordinary relations, questions, and first-order `IProg`;
- residual-to-question-to-answer-to-successor recurrence is executable and cold
  replayable, and every route/method fold remains regenerable and reopenable.

### `THESIS_EVALUATION_COMPLETE`

Required after the self-hosted controller is available:

- compare calculus-controlled inquiry with a conventional instruction/tool-loop
  agent using the same LLM, tools, task set, and comparable resource budgets;
- record unsupported promotion, `Unknown`-to-negative collapse, premature singleton
  selection, raw-return loss, context/replay drift, separator discovery,
  recurrence/fold reuse, post-compression regression, task quality, and resource
  cost;
- state scope, uncertainty, and negative results. Architecture alone never warrants
  the project's empirical thesis.

### `ADAPTIVE_ROUTING_EXPERIMENTAL`

Only after measured protected benefit from the self-hosted controller may Phase 19
add learned question policy, latent-capability routing, masks, adapters/experts,
vector or approximate retrieval, parallelism, or distribution. These remain
implementation/learning realizations unless an executable breaker and independent
semantic warrant establish otherwise.

---

# 1. Authority, v1.1 embedding, and implementation continuity

This plan implements `Inquiry_Calculus_v2_0.tex`; it does not define semantics.
Inquiry Calculus v1.1 is conservatively embedded in v2.0:

\[
\boxed{
\operatorname{Embed}_{1.1\to2.0}:
\mathcal L_{1.1}\hookrightarrow\mathcal L_{2.0}.
}
\]

The embedding preserves, without authority strengthening:

```text
v1.1 typed relation                  -> same semantic relation
v1.1 question ?_I R[beta]            -> same semantic question
v1.1 supported answer                -> same answer semantics
v1.1 Return | Ask source program     -> same source program
v1.1 Code(A)                         -> typed nonexecuting Code(A), with versioned partial interpretation
v1.1 discharge modes                 -> unchanged authority discipline
v1.1 ActualEvent / RawReturn         -> unchanged actuality authority
v1.1 standing                        -> unchanged least-fixed-point discipline
v1.1 positive-negation recurrence    -> unchanged reciprocal semantics
v1.1 folds / bridges / revision      -> preserved ancestry and judgment
```

The v2.0 interrogative authoring layer must conservatively lower:

\[
\boxed{
\operatorname{Lower}_Q(K_{v2})
\in
\mathsf{IProg}_{v1.1}
}
\]

for every admitted derived root, macro, and route annotation. A major-version
boundary does not authorize semantic strengthening by accident.

Implementation does not restart at the version boundary. Existing executable
evidence for conservatively embedded behavior remains evidence at exactly its
demonstrated scope and coverage. Do not rename Rust types merely to contain `v2`,
and do not refactor working code solely to mirror document headings.

The following remain standing:

\[
\boxed{
\text{RELATION is the semantic primitive.}
}
\]

\[
\boxed{
\mathsf{Form}_{\mathbb B}
=
\sum_{A:\mathsf{Ty}_{\mathbb B}}
\llbracket A\rrbracket_{\mathbb B}.
}
\]

\[
\boxed{
\text{Question}
=
\text{partially bound typed relation}.
}
\]

\[
\boxed{
\text{Hole}
=
\text{removed filling with surrounding relations retained}.
}
\]

\[
\boxed{
\text{actuality}
\neq
\text{generation}
\neq
\text{interpretation}
\neq
\text{warrant}.
}
\]

\[
\boxed{
\mathsf{IProg}
=
\mathsf{Return}_I
\mid
\mathsf{Ask}(q,\kappa).
}
\]

\[
\boxed{
\mathsf{Prog}
=
\mathsf{Return}
\mid
\mathsf{Branch}
\mid
\mathsf{Probe}.
}
\]

\[
\boxed{
\mathsf{Stand}
=
\mu T.
}
\]

\[
\boxed{
\text{folds preserve ancestry and reopening}.
}
\]

\[
\boxed{
\text{self-revision is predecessor-judged}.
}
\]

The regenerative determination criterion remains constitutional:

\[
\boxed{
Determines_{\mathcal H}(W,x)
\iff
\operatorname{Sol}_W/\equiv_{\mathcal H}
=
\{[x]_{\mathcal H}\}.
}
\]

V2.0 retains the reciprocal interpretation: a web determines \(x\) exactly when
its remaining lawful fillings contain only one protected source class.
Positive-negation inquiry explores forms outside the live determination and asks
what source structure survives the reciprocal return.

---

# 2. Governing implementation invariant

For every implementation transformation

\[
T:A\to B,
\]

ask:

\[
\boxed{
\text{Can a protected continuation distinguish execution before and after }T?
}
\]

If yes, preserve the distinction explicitly or retain enough authoritative ancestry/residual structure to regenerate it.

If no, quotienting/folding is permitted only relative to a declared:

- horizon;
- grain;
- scope;
- applicability regime;
- continuation family;
- recovery contract;
- unlock condition.

Among licensed representations that preserve inquiry-regenerative sufficiency, retain
the minimal or nondominated frontier under the declared resource preorder.  Inquiry
expands until protected live classes are separable and then subtracts until another
removal would lose regeneration, continuation behavior, warrant provenance, or
reopening.  This is an optimization discipline, not a universal scalar objective or a
claim that one global minimum exists.

The v2.0 reciprocal semantics adds a second implementation discipline:

\[
\boxed{
\text{a candidate exterior cannot become exterior merely because a generator calls it "other".}
}
\]

Every oriented negative relation must have a non-circular positive departure contract.

---

# 3. Constitutional positive-negation law

The semantic primitive remains relation.

The primitive act of reciprocal inquiry is:

\[
\boxed{
\textbf{POSITIVELY DETERMINE A LAWFUL TYPED NEGATION OF THE LIVE
DETERMINATION, THEN RETURN THROUGH THE RELATION THAT LICENSED IT.}
}
\]

For source determination \(x\):

\[
x
\to
N_D^X[x]
\to
?y[N_D^X(x,y)]
\to
y
\to
(N_D^X)^{-1}[y].
\]

Logical negation, a negative relation, its section, and one positive exterior filling remain distinct:

\[
\boxed{
\neg x
\neq
N_D^X
\neq
N_D^X[x]
\neq
y.
}
\]

No new ontological carrier called `PosNeg` is introduced.

`PosNeg` is a role:

\[
\operatorname{PosNeg}_D^X(x,y)
\iff
N_D^X(x,y)
\]

under an admitted `NegationUse`.

---

# 4. Non-negotiable separations

The implementation must preserve at least:

\[
\begin{aligned}
\mathsf{Question}
&\neq
\mathsf{RenderedPrompt},\\
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
\mathsf{Claim}
&\neq
\mathsf{StandingClaim},\\
\mathsf{Retained}
&\neq
\mathsf{Accessible}
\neq
\mathsf{Active}
\neq
\mathsf{Standing},\\
\mathsf{BoundaryProjection}
&\neq
\mathsf{PositiveNegation},\\
\mathsf{NegationUse}
&\neq
\mathsf{BooleanComplement},\\
\mathsf{ExteriorDeparture}
&\neq
\mathsf{ProtectedNonEquivalence},\\
\mathsf{Unknown}
&\neq
\mathsf{Negative},\\
\mathsf{NegationCoverage}
&\neq
\mathsf{GeneratorCoverage},\\
\mathsf{NegationFrontier}
&\neq
\mathsf{CombinedNegationRelation},\\
\mathsf{ReturnFiber}
&\neq
\mathsf{SelectedReturnFilling},\\
\mathsf{PureReturn}
&\neq
\mathsf{SemanticReconciliation},\\
\mathsf{LocalRecovery}
&\neq
\mathsf{SchemaRecovery},\\
\mathsf{OccurrenceRecovery}
&\neq
\mathsf{FamilyRecovery},\\
\mathsf{GenerationCapability}
&\neq
\mathsf{MaterializedQuestionSet}
\neq
\mathsf{SelectionPolicy},\\
\mathsf{MaterializationGap}
&\neq
\mathsf{ExpressibilityGap},\\
\mathsf{DefinitionalExtension}
&\neq
\mathsf{ConservativeObservationalExtension}
\neq
\mathsf{Rebinding},\\
\mathsf{SelfApplication}
&\neq
\mathsf{SelfWarrant}.
\end{aligned}
\]

Any representation that fuses these without an explicit equivalence licence is defective.

---

# 5. Implementation classes

Every named component carries one of the v2.0 statuses `CONSTITUTIONAL`,
`CANONICAL-RESTATED`, `DERIVED`, `BINDING-SUPPLIED`, or `IMPLEMENTATION-ONLY`.
Moving a construct into one file or one crate never promotes its status.

## 5.1 Constitutional and canonical-restated contracts

Required to realize the standing language:

- type universe;
- typed forms;
- represented relation schemas;
- partial binding;
- first-order `IProg`;
- runtime `Return | Branch | Probe`;
- actual event spine;
- raw-return preservation;
- holes/fibers;
- protected completion fields;
- determination presentations;
- departure witnesses;
- coverage-indexed negation uses;
- return fibers;
- protected recovery;
- standing;
- binding versions;
- folds/recovery/reopening;
- predecessor-judged revision.

## 5.2 Derived structures

Rebuildable where practical:

- Ask/continuation occurrence identity and occurrence-indexed `QSucc`;
- static question-relation and dynamic route views;
- `Formable`, `Applicable`, `Executable`, `Answerable`, `Productive`, `ResolvedQ`,
  `Ready`, and `RequiredDischarge` predicates;
- unlock, live-question frontier, and local interrogative fixed point;
- transparent interrogative roots and operational macros;
- question-route occurrence, regeneration, ablation, and fold views;
- active view;
- boundary chart;
- negation frontier;
- duplicate exterior grouping;
- sixfold occurrence view;
- question/return trace projections;
- return-signature family;
- recovery profile;
- cue plan;
- operator occurrence graph;
- affected-fold index;
- transported cross-binding view.

## 5.3 Binding-supplied structure

Admitted only through an explicit binding contract:

- domain types, native relations, causal/intervention laws, and succession;
- provider and tool capabilities;
- decoders, checkers, warrant routes, and native methods;
- cost, risk, probability, resource, and nondominance orders;
- adjoints, weakest-condition transformers, inverse/uniqueness laws, and concurrency
  semantics.

## 5.4 Implementation-only structure

Allowed only when erasure or regeneration preserves semantics:

- content-addressed encodings and database indexes;
- caches, lookup tables, scheduling metadata, and optimization annotations;
- root/route labels and renderer hints;
- derived-view materializations with accepted-root provenance;
- backend adapters, tracing, metrics, and deployment layout.

## 5.5 Deferred and conditional gates

The following require their stated breaker or stronger theorem before promotion:

- exact admission/minimization rule for the source determination presentation \(W_D(x)\);
- open-ended negation/generator fairness;
- learned question-selection policy;
- cross-binding standing lift beyond the conservative rule;
- vector retrieval strict-gain threshold;
- production scaling thresholds.

These must not block implementation of the settled semantic spine.

---

# 6. Reference implementation stack

Use:

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
generic agent framework
distributed workflow engine
Kubernetes
```

The first implementation is a single-process semantic controller with one authoritative writer.

---

# 7. Repository shape

Recommended starting structure:

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

Do not create a crate merely because a conceptual noun exists.

Split only after real independent variation or dependency pressure appears.

---

# 8. Immutable semantic identity

Use content-addressed immutable artifacts.

```rust
pub struct ArtifactRef([u8; 32]);

pub struct TypeRef(ArtifactRef);
pub struct FormRef(ArtifactRef);
pub struct RelationRef(ArtifactRef);
pub struct RelationUseRef(ArtifactRef);
pub struct QueryRef(ArtifactRef);
pub struct IProgRef(ArtifactRef);
pub struct ProgramRef(ArtifactRef);
pub struct CodeRef(ArtifactRef);
pub struct SourceConfigRef(ArtifactRef);
pub struct AskOccurrenceRef(ArtifactRef);
pub struct SupportedAnswerRef(ArtifactRef);
pub struct EventRef(ArtifactRef);
pub struct ClaimRef(ArtifactRef);
pub struct BindingVersionRef(ArtifactRef);
pub struct NegationUseRef(ArtifactRef);
```

Canonical serialization must explicitly define ordering and encoding.

Semantic identity must never depend on:

- database row IDs;
- process memory addresses;
- provider request IDs;
- filesystem paths;
- scheduler IDs.

---

# 9. Typed form universe

Retain:

\[
\mathsf{Form}_{\mathbb B}
=
\sum_{A:\mathsf{Ty}_{\mathbb B}}
\llbracket A\rrbracket_{\mathbb B}.
\]

Reference IR:

```rust
enum TyIR {
    Unit,
    Bool,
    Nat,

    Named {
        binding: BindingVersionRef,
        name: Symbol,
        version: ArtifactRef,
    },

    Product {
        left: TypeRef,
        right: TypeRef,
    },
    Sum {
        left: TypeRef,
        right: TypeRef,
    },
    List(TypeRef),
    Finite(TypeRef),

    Sigma {
        domain: TypeRef,
        family: TypeExprRef,
    },

    Pi {
        domain: TypeRef,
        family: TypeExprRef,
    },

    Raw(TypeRef),
    Result(TypeRef),
    IProg(TypeRef),
    Prog(TypeRef),
    Code(TypeRef),
}
```

No dynamically typed semantic escape hatch. `IProg(A)` is first-order source syntax,
`Prog(A)` is executable runtime syntax, and unary `Code(A)` is nonexecuting
reified program syntax with versioned partial interpretation. Quotation does not
execute or warrant code.

Represent reified program code and its version boundary explicitly:

```rust
enum CodeIR {
    Source {
        result_type: TypeRef,
        program: IProgRef,
    },
    Runtime {
        result_type: TypeRef,
        program: ProgramRef,
    },
}

fn quote_iprog(program: IProgRef) -> CodeRef;
fn quote_program(program: ProgramRef) -> CodeRef;

fn interpret_code(
    binding: BindingVersionRef,
    compiler_version: ArtifactRef,
    code: CodeRef,
) -> Result<InterpretedProgramRef, CodeInterpretationError>;
```

Interpretation checks the quoted result type and admitted binding/compiler version.
A version mismatch is an undefined interpretation, never execution, generation, or
warrant.

---

# 10. Relation schemas

Relations remain authoritative semantic structure.

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

Relations may be:

- deterministic;
- nondeterministic;
- partial;
- multiport;
- cross-typed;
- binding-native.

No host callback silently decides relation meaning.

---

# 11. Relation uses

A relation schema and a relation's use in a live inquiry are protected-different.

Use:

```rust
struct RelationUseIR {
    relation: RelationRef,

    scope: ScopeRef,
    applicability: ApplicabilityRef,

    grain: GrainRef,
    horizon: HorizonRef,

    authority: DischargeMode,

    support: SupportRef,
}
```

A relation may be standing globally while one specific use is inapplicable.

A relation may be semantically defined while one current use has only working support.

---

# 12. Questions remain partial bindings

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

Per open port:

```rust
enum DischargeMode {
    Pure,
    Generate,
    Probe,
    Check,
    Warrant,
}
```

Required operations:

```text
Bind
Expose
Plug
Normalize
Compose
DependentBind
CompletionFiber
```

---

# 13. Source inquiry programs

Retain:

\[
K
::=
\mathsf{Return}_I(a)
\mid
\mathsf{Ask}(q,\kappa).
\]

Use first-order inspectable syntax.

```rust
enum IProgIR {
    Return {
        value: TypedFormRef,
    },

    Ask {
        question: QueryRef,
        environment: Vec<ProgramBinding>,
        answer_slot: TypeSymbol,
        continuation: IProgRef,
    },
}
```

No arbitrary Rust closures.

```rust
struct SourceConfigIR {
    result_type: TypeRef,
    program: IProgRef,
    environment: Vec<ProgramBinding>,
    binding_version: BindingVersionRef,
    compiler_version: ArtifactRef,
    provenance: ProvenanceRef,
}

struct AskOccurrenceIR {
    source_config: SourceConfigRef,
    structural_position: ProgramPositionRef,
    question: QueryRef,
    environment: Vec<ProgramBinding>,
    answer_slot: TypeSymbol,
    continuation: IProgRef,
    binding_version: BindingVersionRef,
    compiler_version: ArtifactRef,
    provenance: ProvenanceRef,
}
```

The occurrence checker re-walks the exact source program and rejects a question,
structural position, slot, environment, binding/compiler version, provenance, or
continuation copied from another occurrence. `structural_position` is validated by
walking the stored first-order syntax; the environment and structural provenance are
derived at that position, and the answer slot plus continuation are re-typechecked
capture-safely. No field is trusted merely because its reference exists.

Represent a supported answer as proof-carrying first-order data:

```rust
enum AnswerSetPresentationIR {
    FiniteCanonical { members: NonEmptyVec<TypedFormRef> },
    Intensional {
        membership: RelationUseRef,
        extensional_identity: AnswerSetEqualityRef,
    },
}

struct NonEmptyAnswerSetIR {
    question: QueryRef,
    presentation: AnswerSetPresentationIR,
    nonempty_witness: TypedFormRef,
}

struct SupportedAnswerIR {
    question: QueryRef,
    route: RouteRef,
    evidence: SupportEnvironmentRef,
    members: NonEmptyAnswerSetRef,
    coverage: CoverageRef,
    component_provenance: ComponentProvenanceMapRef,
    support_witness: SupportWitnessRef,
}
```

The whole record may construct the later question. Its semantic member projection is
written `|S_hat|`. A partial supported answer must not be reduced to a
caller-selected singleton, and equal member projections do not erase different route,
coverage, evidence, or provenance identities.

The witness checker is indexed by that exact tuple. Finite members are canonically
sorted and duplicate-free; an infinite or binding-native nonempty subset is not
narrowed to a `Vec` for host convenience.

Dynamic succession is indexed by a checked `Ask` occurrence or continuation
identity:

\[
\boxed{
\operatorname{QSucc}(\mathfrak a,\widehat S,q')
\iff
\operatorname{HeadQ}(\kappa_{\mathfrak a}(\widehat S),q').
}
\]

Here \(\mathfrak a\) reifies the source-program identity, structural `Ask` position,
explicit environment/bindings, semantic question, and checked continuation. It is
derived from ordinary program identity and provenance; it is not a semantic
primitive or runtime effect. Two occurrences may expose the same semantic question
and receive the same proof-carrying supported answer while their checked continuations lawfully expose
different successor questions. Therefore no learned route, replay recipe, or
controller fingerprint may identify dynamic succession by `(q, S_hat)` alone.

`LiftQ_F(S_hat, question_family, coverage)` is a finite, coverage-indexed dependent
product. For a nonempty `F` contained in `|S_hat|`, its open positions are the
dependent sum of every child port `(a, i)` for `a` in `F`; its answer carrier is the
dependent product of the child answer carriers; and a completion is admitted exactly
when every tagged child completion is admitted. It retains every parent tag and child
support route and inherits each child port's exact discharge mode. It rejects
arbitrary member selection, completion/port index mismatch, and common-mode collapse.
Whole-parent coverage requires a checked proof that `F == |S_hat|`; otherwise every
unmaterialized parent member remains explicitly `Unknown` and the result cannot be
used as a complete family.

Required operations include:

```text
AskQuestion
AskContinuation
HeadQ
QSucc
LiftQ_F with finite coverage certificate
capture-safe proof-carrying-answer substitution
successor-question reconstruction
```

This remains essential to the reciprocal sixfold.

---

# 14. The source determination presentation

The v2.0 reciprocal semantics requires an explicit source determination presentation:

\[
W_D^X(x).
\]

This is a represented relational web relative to which current reciprocal departure is judged.

It is not automatically:

- every fact known about \(x\);
- every standing relation mentioning \(x\);
- every protected continuation;
- one globally unique essence.

Reference:

```rust
struct DeterminationPresentationIR {
    distinction: DistinctionRef,
    orientation: Orientation,

    source: FormRef,

    web: RelationalWebRef,

    scope: ScopeRef,
    applicability: ApplicabilityRef,

    grain: GrainRef,
    horizon: HorizonRef,

    support: SupportRef,

    status: DeterminationPresentationStatus,
}
```

The exact canonical admission/minimization law for `web` remains a research gate.

The safe initial implementation rule is:

> use the support/dependency web of the specific standing determination occupying the source role; do not automatically add unrelated standing facts merely because they concern the same form.

Regenerative minimization may later compress this web, but compression must retain the predecessor presentation as ancestry.

---

# 15. Departure witness: the least positive exterior certificate

Exteriority itself is not a primitive.

For source \(x:X\), candidate \(y:Y\), use positive represented observations:

\[
d_X:X\rightsquigarrow A,
\]

\[
d_Y:Y\rightsquigarrow B,
\]

with supported answers:

\[
d_X(x,a),
\qquad
d_Y(y,b),
\]

and a standing incompatibility relation:

\[
\perp\hookrightarrow A\times B,
\qquad
a\perp b.
\]

Additionally, the source-side discriminator must be relevant to the current determination presentation.

Then:

\[
\boxed{
Depart_D(x,y)
}
\]

is witnessed.

A derived certificate may be:

```rust
struct DepartureWitnessIR {
    distinction: DistinctionRef,

    source: FormRef,
    candidate: FormRef,

    source_presentation:
        DeterminationPresentationRef,

    source_observation:
        RelationUseRef,

    candidate_observation:
        RelationUseRef,

    source_answer:
        FormRef,

    candidate_answer:
        FormRef,

    incompatibility:
        RelationUseRef,

    support:
        SupportRef,

    scope:
        ScopeRef,

    applicability:
        ApplicabilityRef,

    grain:
        GrainRef,
}
```

No undefined `ExteriorIR` is required.

---

# 16. Departure is not protected non-equivalence

The implementation must permit:

\[
Depart_D(x,y)
\land
x\equiv_{\mathcal H}y.
\]

This is a useful positive near-negation: the current determination presentation places \(y\) outside the source, while the protected horizon cannot yet inspect a consequential difference.

Likewise:

\[
x\not\equiv_{\mathcal H}y
\]

does not automatically prove:

\[
Depart_D(x,y).
\]

The protected distinction may depend on a relation outside the current determination presentation.

Therefore:

\[
\boxed{
\text{determination departure}
\neq
\text{protected consequence separation}.
}
\]

---

# 17. Cell exclusion and boundary crossing are derived

For exact same-carrier conjunctive determination cells:

\[
C_W(x)
=
\bigcap_jC_j(x),
\]

positive constitutive separator existence and cell exclusion coincide.

The implementation recursion exhaustively checked 65,536 finite feature/source/candidate cases with no mismatch.

But in incomplete regimes, raw signature mismatch is not enough. A separate exhaustive ternary fixture showed 12 of 36 source/candidate combinations where raw signatures differed but no positive incompatible observation existed.

Therefore:

\[
\boxed{
\text{unknown}
\neq
\text{departure}.
}
\]

Boundary crossing is stronger:

\[
\boxed{
BoundaryCross
=
DepartureWitness
+
Traversal/SuccessionProvenance.
}
\]

A candidate can be positively exterior without an observed crossing path.

---

# 18. Typed negation is a supported relation role

An oriented typed negation is an ordinary relation:

\[
N_D^X
\hookrightarrow
X\times Y.
\]

A use is lawful when:

1. it is well typed;
2. orientation is explicit;
3. applicability and scope are explicit;
4. every admitted edge has a sound departure derivation at the claimed authority;
5. semantic coverage is explicit;
6. warrant is explicit.

Use:

```rust
struct NegationUseIR {
    relation: RelationRef,

    distinction: DistinctionRef,
    orientation: Orientation,

    source_determination:
        DeterminationPresentationRef,

    candidate_field:
        RelationRef,

    soundness_derivation:
        ProgramRef,

    soundness:
        ExteriorSoundness,

    semantic_coverage:
        NegationCoverage,

    applicability:
        ApplicabilityRef,

    scope:
        ScopeRef,

    grain:
        GrainRef,

    horizon:
        HorizonRef,

    provenance:
        Vec<ArtifactRef>,
}
```

---

# 19. Negation soundness is non-circular

For exact soundness:

\[
\boxed{
N_D^X(x,y)
\Rightarrow
\exists w:
DepartureWitness_D(x,y,w).
}
\]

The negation relation cannot use its own asserted "negativity" as the sole reason the candidate is exterior.

Possible independent formation routes include:

- exact disjoint answer cells;
- standing incompatibility/apartness;
- exact residual determination classes;
- binding-native incompatibility;
- actual checked boundary departure;
- cross-binding comparison with a standing bridge.

These are provenance routes, not semantic negation variants.

---

# 20. Typed negation is not Boolean complement

The implementation must never use:

```text
not source -> negative
not proved source -> negative
not in current results -> negative
```

unless a binding explicitly supplies an exact exhaustive complement law.

The implementation recursion established that soundness and exhaustiveness are different.

In the exact four-element finite comparison there were:

- 284 sound source-relative exterior fields;
- 60 exhaustive complements;
- 168 nonempty sound strict partial fields.

Therefore exhaustive complement cannot be a constitutional requirement.

---

# 21. Negation semantic coverage

Use:

```rust
enum NegationCoverage {
    ExactExhaustive {
        regime: RegimeRef,
        certificate: ArtifactRef,
    },

    ExactOnField {
        field: RelationRef,
        certificate: ArtifactRef,
    },

    CertifiedPartial,

    WorkingOpen,
}
```

Interpretation:

- `ExactExhaustive`: all admissible exteriors in the declared full regime;
- `ExactOnField`: complete only on a declared candidate field;
- `CertifiedPartial`: every represented edge is sound, but no completeness claim;
- `WorkingOpen`: soundness and/or coverage remains provisional.

An empty exact exhaustive field is protected-different from an empty unsearched working field.

---

# 22. Semantic coverage and execution coverage remain separate

A relation may be semantically exhaustive while the runtime has explored only part of its section.

A semantically partial relation may be fully enumerated.

Therefore:

\[
\boxed{
NegationCoverage
\neq
GeneratorCoverage.
}
\]

Do not reuse one enum for both.

---

# 23. Negative frontiers are tagged families

A determination may admit:

\[
\mathcal N_D^X
=
\{\mathfrak N_i\}_{i\in I}.
\]

Targets may differ:

\[
N_i
\hookrightarrow
X\times Y_i.
\]

For source \(x\):

\[
\boxed{
NegFront_D^X(x)
=
\sum_{i\in I_x}
N_i[x].
}
\]

Each occurrence is tagged:

\[
(i,y).
\]

Do not create an authoritative untagged union.

The same exterior form reached by two uses may have different reverse return fibers.

The recursion established a direct breaker:

\[
N_1^{-1}[y]
\neq
N_2^{-1}[y]
\]

even when both produce the same \(y\).

Therefore relation-use provenance is part of the reciprocal occurrence.

---

# 24. Frontier implementation

Derived:

```rust
struct ActiveNegationUseIR {
    use_ref: NegationUseRef,

    source: FormRef,

    candidate_field: FiberRef,

    semantic_coverage:
        NegationCoverage,

    execution_status:
        GeneratorCoverage,
}
```

Optional derived view:

```rust
struct NegationFrontierViewIR {
    source: FormRef,

    distinction: DistinctionRef,
    orientation: Orientation,

    members:
        Vec<ActiveNegationUseIR>,

    collective_coverage:
        Option<CollectiveCoverageRef>,

    regime:
        ArtifactRef,
}
```

No `CombinedNegationIR`.

---

# 25. Collective coverage requires a certificate

From:

\[
CertifiedPartial(N_1),
\quad
CertifiedPartial(N_2)
\]

one may not infer exhaustive collective coverage.

Collective exactness over candidate field \(C\) requires:

\[
\forall y\in C(x),
\quad
Depart_D(x,y)
\Rightarrow
\exists i:N_i(x,y).
\]

Store a derived certificate when established.

---

# 26. Positive negation

Given an admitted `NegationUse`:

\[
\operatorname{NegField}_D^X(x)
=
\operatorname{Sec}^{Y}_{N_D^X}(x).
\]

Then:

\[
?y[N_D^X(x,y)]
\]

is an ordinary `OpenQuery`.

The result occupies the role:

\[
O_X.
\]

Its authority is inherited from the query route.

A generated \(O_X\) is not an actualized \(O_X\).

A working negation relation cannot support an exact closure claim merely because a candidate was generated successfully.

---

# 27. Pure return is reverse section

After:

\[
N_D^X(x,y),
\]

define:

\[
\boxed{
RetField_D^X(y)
=
\operatorname{Sec}^{X}_{N_D^X}(y).
}
\]

Implementation:

```rust
fn return_field(
    negation_use: NegationUseRef,
    exterior: FormRef,
) -> Result<FiberRef>;
```

No generic `Redet` primitive is required.

Because \(N_D^X(x,y)\) holds:

\[
\boxed{
x\in RetField_D^X(y).
}
\]

But source membership does not imply unique source recovery.

The previous recursion exhaustively confirmed many finite incidences with ambiguous reverse sections.

---

# 28. Selected return roles and return fibers are different

The sixfold role \(R_X\) is a supported selected filling/class from:

\[
RetField_D^X(O_X).
\]

It is not the fiber itself.

Exact return stability requires:

\[
\boxed{
RetField_D^X(O_X)
/\equiv_{\mathcal H}
=
\{[S_X]_{\mathcal H}\}.
}
\]

One observed \(R_X\equiv_{\mathcal H}S_X\) is insufficient when another protected class remains in the return fiber.

The same applies to \(R_Y\).

---

# 29. Protected relation recovery

Let \(\rho\) be a source relation use.

Derive the protected observation signature:

\[
\chi_{\rho,\mathcal H}.
\]

Then:

\[
x
\equiv_{\rho,\mathcal H}
x'
\iff
\chi_{\rho,\mathcal H}(x)
=
\chi_{\rho,\mathcal H}(x').
\]

For exterior \(y\), local recovery is:

\[
\boxed{
Recover_{\mathcal H}(x\mid y;\rho)
\iff
RetField_D^X(y)
/\equiv_{\rho,\mathcal H}
=
\{[x]_{\rho,\mathcal H}\}.
}
\]

Equivalently, every source still possible through the return agrees on the protected consequence of \(\rho\).

No source relation is added to the return fiber to make this true.

The relation inspects the fiber; it does not construct it.

---

# 30. Recovery profile of a source web

For source web:

\[
W_x,
\]

define:

\[
\boxed{
Recov_{W_x,\mathcal H}(x,y)
=
\{
\rho\in W_x:
Recover_{\mathcal H}(x\mid y;\rho)
\}.
}
\]

An exterior may recover some source relations while failing to recover the entire source class.

This gives the exact interpretation of:

> positive negation discovers how much of the source may be recovered without identity.

No scalar recovery percentage belongs in the semantic core.

Executable recovery must retain three outcomes rather than defining loss by set
complement:

```rust
enum RecoveryStatusIR {
    Recovered { certificate: ArtifactRef },
    NotRecovered { separator: ArtifactRef },
    Unknown { residual: OpenRef },
}
```

`NotRecovered` requires two source candidates in the same return fiber whose protected
relation signatures differ.  Absence of a recovery certificate is insufficient.  Only
an exact decision/coverage certificate may identify the irrecoverable residue with
`W - Recov(W)`.

A source characterization may be exposed as a derived view over:

```text
supported determination presentation
certified admitted internal variation
tagged negation frontier
three-valued recovery/loss profiles
scope/applicability/grain/horizon/coverage/provenance
open residuals and reopening routes
```

This view may support an external goal horizon or a recursively developed constitutive
inquiry horizon.  Generated constitutive discriminators remain candidates until
independently actualized, checked, warranted, and admitted.  Do not add an authoritative
`CharacterizationIR` table or a new runtime opcode; construct the view from ordinary
relations, events, fibers, and support.

---

# 31. Near-negation order

For two exterior candidates:

\[
y_1,y_2,
\]

define:

\[
y_1
\succeq_{W_x,\mathcal H}
y_2
\]

when:

\[
Recov_{W_x,\mathcal H}(x,y_2)
\subseteq
Recov_{W_x,\mathcal H}(x,y_1).
\]

This is a partial/preorder of protected recovery.

Cost/risk/time may be combined by a product/Pareto order.

Do not create a universal scalar negation distance.

---

# 32. Family return information

For each negation use:

\[
\sigma_i(x)
=
N_i[x].
\]

Include applicability status where protected.

The family signature is:

\[
\boxed{
\sigma_{\mathcal N}(x)
=
\prod_i\bar\sigma_i(x).
}
\]

For deterministic/exact signatures:

\[
\boxed{
\ker\sigma_{\mathcal N}
=
\bigcap_i\ker\bar\sigma_i.
}
\]

Adding another lawful signature can refine but not coarsen the family observational partition.

This product is an informational view, not an actuality constructor.  Component
signatures supported under different occurrences or mutually exclusive applicability
contexts remain individually usable information, but they do not become one realized
composite return without explicit co-applicability and joint-realizability evidence.

Use a derived certificate reference only when a consumer requires simultaneous
realization:

```rust
struct JointnessCertificateIR {
    components: Vec<SignatureRef>,
    applicability: ApplicabilityRef,
    joint_context: ContextRef,
    evidence: Vec<ArtifactRef>,
}
```

Do not require this certificate merely to retain or compare separately actualized
information.  Require it when an operation treats the product as one actual composite
observation.

---

# 33. Schema recovery

A protected source observation \(\chi_{\rho,\mathcal H}\) is recoverable from the entire negation family when:

\[
\boxed{
\ker\sigma_{\mathcal N}
\subseteq
\ker\chi_{\rho,\mathcal H}.
}
\]

Equivalently, when factorization is available:

\[
\boxed{
\chi_{\rho,\mathcal H}
=
h\circ\sigma_{\mathcal N}.
}
\]

The implementation recursion established a minimal three-state witness where neither member signature alone recovered a target observation but the product did.

Therefore joint gain belongs in:

\[
\boxed{
\text{return-signature product}
}
\]

not in a semantic union of negation relations.

The exact deterministic case is one instance of the generic factorization contract:

\[
\boxed{
DetermineThrough(\sigma,\chi)
\iff
\ker\sigma\subseteq\ker\chi
\iff
\exists h.\ \chi=h\circ\sigma.
}
\]

Implement one exact facility:

```rust
enum ExactDeterminationResult {
    Exact { certificate: ArtifactRef },
    NotDetermined { separator_certificate: ArtifactRef },
}

fn determine_through_exact(
    available_signature: SignatureRef,
    target_signature: SignatureRef,
    scope: ScopeRef,
    applicability: ApplicabilityRef,
    horizon: HorizonRef,
) -> ExactDeterminationResult;
```

The implementation must verify common scope, applicability, grain, binding, and
horizon before applying the kernel test.  Working, partial, nondeterministic, or
incompletely covered inputs use a separately typed result that may retain liabilities
or return `Unknown`; they must not reuse `Exact` by convention.

---

# 34. Local recovery and family recovery remain separate

Historical occurrence:

\[
(i,y)
\]

uses:

\[
Ret_i(y).
\]

Later addition of another negation relation must not retroactively narrow that historical return fiber.

Family/schema recovery concerns the currently available representation.

Occurrence recovery concerns the information carried by one realized path.

History remains version-local.

---

# 35. Pure return is not state-changing redetermination

This is a critical correction to the motivating patch.

Suppose source web \(W_X\) already determines \(S_X\):

\[
Sol_{W_X}/\equiv_{\mathcal H}
=
\{[S_X]_{\mathcal H}\}.
\]

Add a compatible exterior constraint \(C_O\) that \(S_X\) itself satisfies.

Then:

\[
Sol_{W_X\cup\{C_O\}}
=
Sol_{W_X}\cap Sol_{C_O}.
\]

Since \(S_X\) remains in the intersection and the intersection is a subset of the already single protected class:

\[
\boxed{
Sol_{W_X\cup\{C_O\}}/\equiv_{\mathcal H}
=
\{[S_X]_{\mathcal H}\}.
}
\]

Therefore compatible monotone constraint addition cannot transform an already determined source into a protectedly different source.

So the statement:

> redetermination is hole re-solve after adding the exterior

does **not** survive as the generic state-changing semantics.

Instead:

\[
\boxed{
\text{pure return}
=
\text{reverse-section reconstruction}.
}
\]

\[
\boxed{
\text{state-changing redetermination}
=
\text{reconciliation/revision of the standing web}.
}
\]

---

# 36. Reconciliation / semantic revision

A sufficiently authoritative positive exterior may invalidate or refine standing relations.

Then:

\[
W_t
\to
W_{t+1}
\]

may:

- narrow applicability;
- retract a relation;
- supersede a relation;
- split a representation;
- revise a binding;
- reopen a fold;
- replace a support environment.

Only afterward is a new determination solved under the successor web.

This path requires actuality/check/warrant authority appropriate to the claim being revised.

A generated exterior alone cannot mutate standing semantics.

---

# 37. Seed/reorientation

After \(O_X\), construct the \(Y\)-oriented seed:

\[
Seed_Y(O_X,S_Y).
\]

If the representation is already appropriate:

\[
S_Y=O_X.
\]

Otherwise use an explicit represented transformation/bridge.

The roles remain distinct even if protected-equivalent:

\[
O_X
\equiv_{\mathcal H}
S_Y
\]

does not collapse the role provenance.

Reorientation is an inquiry transformation, not proof of reciprocal symmetry.

---

# 38. Dependent sixfold semantics

The six roles remain:

\[
\Xi_D
=
(S_X,O_X,R_X;S_Y,O_Y,R_Y).
\]

Their dependency is:

\[
\boxed{
\begin{aligned}
S_X
&\xrightarrow{N_i^X}
O_X\\
&\xrightarrow{(N_i^X)^{-1}}
Ret_X(O_X)
\ni R_X,\\[1mm]
O_X
&\xrightarrow{Seed_Y}
S_Y\\
&\xrightarrow{N_j^Y}
O_Y\\
&\xrightarrow{(N_j^Y)^{-1}}
Ret_Y(O_Y)
\ni R_Y.
\end{aligned}
}
\]

Both negation-use tags \(i,j\) remain part of the occurrence provenance.

The sixfold closure claim is indexed by:

- source determination presentations;
- negation-use regimes;
- semantic negation coverage;
- execution/generator coverage;
- protected horizon;
- grain;
- binding.

No context-free `Close_D(Ξ)` claim is sufficient.

---

# 39. Sixfold source program

The normative source expansion is canonical definition `def:recip-program`; do not
maintain a second scalar/lambda formulation. Compile it as ordinary first-order
`IProgIR::Ask` nodes with explicit answer slots, environments, and named continuation
references. Every dependent edge uses `LiftQ_F` and consumes the whole proof-carrying
parent answer.

Its checked question-family dependency graph is:

```text
admitted source DeterminationPresentation W_X
-> applicable admitted NegationUse_X
-> positive exterior plus DepartureWitness_X
-> exact same-use ReturnFiber_X
-> selected return membership AND whole-fiber RecoveryProfile_X
-> supported Seed_X_to_Y with provenance
-> warranted DeterminationPresentation W_Y
-> independently applicable admitted NegationUse_Y
-> positive exterior plus DepartureWitness_Y
-> exact same-use ReturnFiber_Y
-> selected return membership AND whole-fiber RecoveryProfile_Y
-> complete dependent occurrence and residuals
-> downstream-only GammaOutcome check
```

The fiber question is `Pure`; the recovery and Gamma questions are `Check`; the
presentation-admission question is `Warrant`; every other port preserves its relation
use's declared mode. `RecoveryProfile` is indexed by source web, protected horizon,
use, exterior, entire fiber, semantic coverage, execution coverage, occurrence
evidence, and provenance. `GammaOutcome` is a tagged compatibility certificate,
incompatibility witness, or `Unknown` coverage residual—`Check` is never encoded as a
result value.

No step selects one member of a non-singleton answer. A finite `F` strictly smaller
than the parent member projection leaves every uncovered branch `Unknown` and cannot
construct a complete sixfold. The second orientation is admitted independently from
the supported seed presentation. `Gamma` cannot fill a missing role. The expansion
adds no source constructor, host closure, runtime opcode, or authoritative sixfold
record.

---

# 40. Canonical reciprocal residuals

Retain the four useful role comparisons as witnessed breakers:

\[
S_X
\stackrel?{\equiv}_{\mathcal H}
R_X,
\]

\[
O_X
\stackrel?{\equiv}_{\mathcal H}
S_Y,
\]

\[
O_Y
\stackrel?{\equiv}_{\mathcal H}
S_X,
\]

\[
S_Y
\stackrel?{\equiv}_{\mathcal H}
R_Y.
\]

But exact closure uses fibers and coverage.

For example:

\[
Ret_X(O_X)/\equiv_{\mathcal H}
=
\{[S_X]_{\mathcal H}\}
\]

is stronger than observing one stable \(R_X\).

No breaker found under partial negation coverage remains `Unknown` with respect to unrepresented exteriors.

---

# 41. \(\Gamma_D\) becomes a downstream check

`\Gamma_D` no longer fills six arbitrary slots.

Generation order is:

1. source determination;
2. applicable negation use;
3. positive exterior;
4. return fiber/filling;
5. seed bridge;
6. reciprocal negation use;
7. reciprocal exterior;
8. reciprocal return;
9. residual computation;
10. \(\Gamma_D\) compatibility checking.

Therefore:

\[
\boxed{
\text{generation}
\neq
\text{compatibility}.
}
\]

`\Gamma_D` cannot supply missing role fillings.

---

# 42. Boundary representation

A distinction may still retain:

\[
D
=
(X,Y,B_D,\pi_X,\pi_Y,\Gamma_D).
\]

But boundary projection is only candidate incidence.

Do not infer:

\[
\pi_X(z)=x
\land
\pi_Y(z)=y
\Rightarrow
N_D^X(x,y).
\]

Reference derived boundary chart:

```rust
struct BoundaryChartIR {
    id: BoundaryRef,

    query: QueryRef,

    x_ty: TypeRef,
    y_ty: TypeRef,

    boundary_ty: TypeRef,

    pi_x: RelationRef,
    pi_y: RelationRef,

    x_determination:
        DeterminationPresentationRef,

    y_determination:
        Option<DeterminationPresentationRef>,

    negation_frontier_x:
        Vec<NegationUseRef>,

    negation_frontier_y:
        Vec<NegationUseRef>,

    seed_y:
        RelationUseRef,

    compatibility:
        FormulaRef,

    traversal:
        Option<RelationRef>,

    grain: GrainRef,
    horizon: HorizonRef,
}
```

No stored return relation is needed generically; it is reverse section of the actual negation use.

---

# 43. Sixfold occurrence view

Derived:

```rust
struct SixfoldOccurrenceViewIR {
    distinction: DistinctionRef,

    sx: FormRef,

    neg_x_use: NegationUseRef,
    ox: FormRef,
    ox_occurrence: Option<EventRef>,

    return_x_fiber: FiberRef,
    rx: Option<FormRef>,

    sy: FormRef,
    seed_support: SupportRef,

    neg_y_use: NegationUseRef,
    oy: FormRef,
    oy_occurrence: Option<EventRef>,

    return_y_fiber: FiberRef,
    ry: Option<FormRef>,

    recovery_x:
        Vec<RecoveryCheckRef>,

    recovery_y:
        Vec<RecoveryCheckRef>,

    residuals:
        Vec<SeparatorProblemRef>,

    compatibility:
        Option<ArtifactRef>,
}
```

This is not authoritative history.

---

# 44. Variation and the determination boundary

The successor gives variation and positive negation complementary roles.

Interior search:

\[
?x'[
V(x,x')
\land
\text{no warranted departure under the declared determination regime}
].
\]

Positive exterior search:

\[
?y[
N_D(x,y)
].
\]

Important:

\[
\boxed{
\text{no departure witness}
\neq
\text{interior}
}
\]

when determination/separator coverage is incomplete.

Conceptually:

\[
\boxed{
\begin{aligned}
\text{interior frontier}
&=
\text{maximum variation absorbed by the determination},\\
\text{exterior frontier}
&=
\text{maximum protected relational recovery despite departure}.
\end{aligned}
}
\]

The boundary is established by the interaction of these fronts, not by a hidden metric.

---

# 45. Holes and regenerative understanding

The existing hole machinery remains unchanged.

For a relational web:

\[
W,
\]

remove filling \(x\):

\[
Hole_x(W).
\]

Then solve:

\[
Sol(Hole_x(W)).
\]

Regenerative determination remains:

\[
Sol(Hole_x(W))/\equiv_{\mathcal H}
=
\{[x]_{\mathcal H}\}.
\]

The successor clarifies two different directions:

\[
\boxed{
\begin{aligned}
\text{backward regeneration}:&
\quad
W_x\to Hole_x(W_x)\to[x]_{\mathcal H},\\
\text{forward reciprocal inquiry}:&
\quad
x\to\text{positive exterior}\to\text{return/recovery}\to\Delta.
\end{aligned}
}
\]

They share fiber/determination machinery but are not the same operation.

---

# 46. Generic separator problem remains the common residual engine

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

The same problem represents:

- ordinary residual inquiry;
- cue extension;
- return-fiber ambiguity;
- recovery failure;
- reciprocal residual;
- fold reopening;
- representation-gap witness refinement.

No sixfold-specific separator subsystem.

---

# 47. Recovery failure becomes separator inquiry

If:

\[
x_1,x_2\in Ret_D(y)
\]

and:

\[
x_1
\not\equiv_{\rho,\mathcal H}
x_2,
\]

then the return does not recover \(\rho\).

That pair directly generates a `SeparatorProblemIR`.

Likewise, if a selected sixfold role comparison fails, the mismatching protected classes become the residual field.

---

# 48. Question construction remains small

The admitted internal constructor family remains:

\[
\boxed{
Bind,\ Expose,\ Continue,\ Residualize,\ Instantiate.
}
\]

Derived uses include:

- positive-negation questions;
- reverse return questions;
- seed questions;
- reciprocal orientation;
- cue extension;
- parameter variation;
- registered probe invocation;
- separator follow-up.

No special `NegateQuestion`, `ReturnQuestion`, or `SixfoldQuestion` primitive is required.

The v2.0 interrogative root family is a transparent derived algebra over these
constructors and the ordinary relation language:

```text
Expose     -> expose typed ports of an admitted relation
Orient     -> expose a relation in a forward or converse orientation
Factor     -> join/expose/hide a represented path or composition
Polarize   -> open an admitted breaker, separator, or positive-negation field
Vary       -> open a binding-supplied transformation relation
Ground     -> open typed support, Check, or Warrant relations
```

Classical formula negation used by a logical breaker is not contextual typed
negation and does not create a `NegationUse`. `Why`, `How`, `WhyNot`, `WhatIf`,
`Ablate`, `Construct`, `Backchain`, `Contrast`, and `Localize` are surface
renderings or transparent macros. Root and route annotations must erase under
conservative lowering; none becomes a source constructor, runtime opcode, discharge
authority, or semantic fact.

---

# 49. Generator regimes remain distinct from materialization and policy

Retain:

\[
\boxed{
\text{generable inquiry}
\neq
\text{currently materialized candidates}
\neq
\text{selected inquiry}.
}
\]

`NegationUse` families participate in the existing generator regime.

An open family uses the same fairness machinery as any other open generator; do not create negation-specific scheduling.

---

# 50. Adaptive omission remains consequence-relative

A reciprocal role, root, question, or route may be skipped only when it cannot
change the live protected residual and is not required to discharge another open
obligation.

Keep the derived availability predicates distinct:

\[
\boxed{
\mathsf{Formable}
\neq
\mathsf{Applicable}
\neq
\mathsf{Executable}
\neq
\mathsf{Answerable}
\neq
\mathsf{Productive}
\neq
\mathsf{ResolvedQ}.
}
\]

`Productive` identifies a discretionary question whose lawful supported answers can
lead to protected-different continuations. It is not the only lawful reason to ask.
`RequiredDischarge` identifies an exact open program/standing obligation whose
declared `Probe`, `Check`, `Warrant`, support, reconstruction, or other discharge
must occur even when no two discretionary answer branches are live. The obligation
retains its source and authority provenance and cannot be asserted by scheduling
policy.

For the executable controller context, derive:

\[
\boxed{
\mathsf{LiveQFrontier}
=
ND^{req}_{\preceq,\Sigma}\{\mathfrak a:
WF(\mathfrak a)
\land Applicable(q_{\mathfrak a})
\land Executable(q_{\mathfrak a})
\land(Productive(\Sigma,\mathfrak a)
      \lor\exists d.\,RequiredDischarge(\Sigma,\mathfrak a,d))
\}.
}
\]

For every candidate occurrence set `C`, the required-safe operator is exactly

\[
ND^{req}_{\preceq,\Sigma}(C)
=
Req_\Sigma(C)\cup ND_\preceq(C).
\]

An unexecutable required obligation remains an explicit `Blocked`, `Unknown`,
resource, representation, or authority residual; it is not silently removed. A
question may be answer-resolved while its required actuality/check/warrant route is
still undischarged.

The frontier is keyed by checked Ask occurrence, not normalized question. The
required-safe nondominance operator retains every occurrence with an undischarged
dependency even when a cheaper candidate strictly dominates it; substitution by a
retained occurrence requires a typed same-dependency discharge proof. Ordinary
nondominance is computed over the full candidate set, so a required occurrence still
removes an optional occurrence that it strictly dominates.

The semantic frontier remains present even when traversal omits it.

Question policy may prefer:

- stronger recovery;
- new coverage;
- predicted residual contraction;
- lower execution cost;
- stronger warrant;
- currently executable routes.

Policy is never semantic authority.

---

# 51. Materialization gap versus expressibility gap

Retain:

## 51.1 Materialization gap

A lawful separator/question/negation route exists in the admitted language but is not currently materialized.

Continue generation.

## 51.2 Expressibility gap

Independent protected evidence requires a distinction, but the admitted representation/question/probe language cannot express any lawful separator.

Create:

\[
RepresentationGap.
\]

Do not search forever in the same language.

Positive-negation inquiry is one canonical source of such witnessed gaps.

---

# 52. Representation invention

Suppose a sixfold/recovery residual establishes:

\[
x\not\equiv_{\mathcal H}y
\]

while current representation gives:

\[
\eta(x)=\eta(y).
\]

Then:

\[
\boxed{
RepresentationGap(x,y).
}
\]

Candidate repair may be:

- new relation;
- new context;
- finer grain;
- new representation coordinate;
- new probe;
- new decoder;
- binding extension.

Thus:

\[
\boxed{
\text{positive reciprocal failure creates representation pressure}.
}
\]

---

# 53. Attribute learning

Represent an attribute as an ordinary relation:

\[
A
\hookrightarrow
X\times V_A.
\]

If recurring reciprocal/separator residuals are repeatedly discharged by \(A\), it becomes a candidate reusable coordinate.

Admission may refine:

\[
\eta_{t+1}
=
\langle\eta_t,A\rangle.
\]

An attribute is therefore not a primitive property ontology.

It is a reusable distinction axis with explicit applicability, range, support, and reopening conditions.

Its own meaning remains open to reciprocal inquiry.

---

# 54. Question-space growth

After a conservative representation extension:

\[
\eta_t
\to
\eta_{t+1},
\]

every old type, relation, question constructor, typing rule, and protected
interpretation remains transportable, while new typed relation schemas/questions may
become constructible.

Therefore generative inquiry can change its future question language:

\[
\boxed{
\operatorname{EmbedQ}:
Q^\infty(\Sigma_t)
\hookrightarrow
Q^\infty(\Sigma_{t+1})
}
\]

Strict inclusion requires a well-typed successor question outside the image of
\(\operatorname{EmbedQ}\).  Definitional replacement or rebinding need not preserve old question
constructors, so it carries no unqualified monotonicity claim.

This is the implementation mechanism for open-ended inquiry.

---

# 55. Probe and instrument invention

If a protected distinction is known:

\[
x\not\equiv_{\mathcal H}y
\]

but every current probe agrees:

\[
\forall p\in\mathcal P_t,
\quad
p(x)=p(y),
\]

open:

\[
?p'[p'(x)\neq p'(y)].
\]

A new instrument is an implementation of a previously unavailable separator route.

Distinguish:

\[
\boxed{
\text{registered-but-unused probe}
\neq
\text{new probe capability proposal}.
}
\]

The latter must pass binding/representation-extension admission.

---

# 56. Cross-domain transport

A relation that survives reciprocal attack may be reified and transported.

Do not transport source-domain mechanism automatically.

Instead:

\[
m_{\mathbb B_1}
\to
A
\to
?m_{\mathbb B_2}
\]

where \(A\) is the relation preserved and the target binding supplies an implementation.

Cross-domain transport therefore reuses the existing binding-bridge architecture.

---

# 57. Runtime semantic core

Retain:

\[
P
::=
\mathsf{Return}
\mid
\mathsf{Branch}
\mid
\mathsf{Probe}.
\]

Reference:

```rust
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

For every runtime program lowered from a source `Ask`, retain and verify:

```rust
struct PortLoweringIR {
    port: Symbol,
    mode: DischargeMode,
    route: RouteRef,
    operator: Option<ProbeOperatorRef>,
    resolution_route: ResolutionPathRef,
    provenance: ProvenanceRef,
}

struct SourceAskLoweringIR {
    source_config: SourceConfigRef,
    ask_occurrence: AskOccurrenceRef,
    port_lowerings: NonEmptyVec<PortLoweringIR>,
    program_resolver: ResolutionProcedureRef,
    runtime_program: ProgramRef,
    compiler_version: ArtifactRef,
}
```

The verifier re-walks the source question and requires exactly one lowering for every
open port, with its declared mode. At execution it builds a port-indexed discharge
bundle. A Probe component contains exact `EventFor`, source-port-to-operator,
operator/request, event-to-raw-return, and resolution-route evidence; Pure, Generate,
Check, and Warrant components retain their own typed result, authority, route,
versions, and provenance without manufacturing an event. The bundle may contain
zero, one, or several ordinary events, and shared events require an explicit checked
multi-port lowering.

The program-wide resolver validates relation-wide correlations after every component
is available. Only
`Supported(SupportedAnswerRef)` may compile the checked source continuation; the
other four resolution constructors route to their own residual/stop and may not use
`resume` to bypass that gate.

No new `Negate`, `ReturnFromNegation`, `Redet`, or `Sixfold` runtime opcode.

All reciprocal operations compile from ordinary typed questions and fibers.

---

# 58. LLM compiler factorization

Retain distinct:

\[
\boxed{
ProbeOperatorIR
\neq
SurfacePlanIR
\neq
BackendRequestIR
\neq
RawReturnEnvelopeIR.
}
\]

A prompt renderer for positive-negation inquiry must preserve:

- source determination identity;
- distinction/orientation;
- actual negation-use identity;
- open target port type;
- scope;
- applicability;
- grain;
- horizon;
- discharge authority;
- candidate coverage status;
- whether a proposed answer is generative or must be actualized;
- continuation dependency.

The renderer must not paraphrase a partial certified opposition into "all alternatives to \(x\)."

Controlled interrogative rendering must round-trip to the same normalized open
relation and preserve:

- converse fiber versus strict inverse;
- existential preimage versus universal guarantee/adjoint;
- generic backward relation versus same-use reciprocal return;
- exact versus partial coverage;
- open-port discharge mode and relation identity.

An implementation-only root/route annotation may guide wording or bounded search,
but erasing it must recover the accepted ordinary `OpenQuery`/`IProg`. A surface
interrogative never strengthens authority or supplies a hidden semantic predicate.

---

# 59. Realized inquiry occurrence and one authoritative history

One authoritative occurrence record:

```rust
struct ActualEvent {
    id: EventRef,

    ledger_parent: Option<EventRef>,

    state_before: StateRef,

    source_ask_occurrence: Option<AskOccurrenceRef>,
    question: QueryRef,
    distinction: Option<DistinctionRef>,

    operator: OperatorRef,

    raw_return: ReturnRef,

    state_after: StateRef,

    grain: GrainRef,

    route: RouteRef,

    binding_version: BindingVersionRef,

    compiler_version: ArtifactRef,

    backend_version: ArtifactRef,

    provenance: ProvenanceRef,
}
```

The general source occurrence is derived from a port-indexed bundle, not one scalar
event:

```rust
struct PortDischargeEvidenceIR {
    port: Symbol,
    mode: DischargeMode,
    route: RouteRef,
    resolution_path: ResolutionPathRef,
    binding_version: BindingVersionRef,
    compiler_version: ArtifactRef,
    provenance: ProvenanceRef,
    payload: PortDischargePayloadIR,
}

enum PortDischargePayloadIR {
    Pure { result: TypedFormRef },
    Generate { proposal: TypedFormRef },
    Probe {
        operator: ProbeOperatorRef,
        event: EventRef,
        raw_return: ReturnRef,
    },
    Check { result: TypedFormRef },
    Warrant { result: TypedFormRef },
}

struct DischargeBundleIR {
    ask_occurrence: AskOccurrenceRef,
    components: NonEmptyVec<PortDischargeEvidenceIR>,
}

enum NextSourceControlIR {
    Ask {
        source_config: SourceConfigRef,
        occurrence: AskOccurrenceRef,
    },
    Return(TypedFormRef),
}

struct ResolvedOccurrenceIR {
    source_config: SourceConfigRef,
    ask_occurrence: AskOccurrenceRef,
    discharge_bundle: DischargeBundleIR,
    program_resolver: ResolutionProcedureRef,
    supported_answer: SupportedAnswerRef,
    continuation: IProgRef,
    next: NextSourceControlIR,
}
```

The bundle checker covers every open port exactly once, enforces agreement between its declared
mode and evidence payload, and checks the exact route, resolution path, binding/compiler versions,
and provenance retained by every component. It preserves each Probe
event/request/raw-return/path chain independently. A
`ResolvedOccurrenceIR` exists only when the program-wide resolver returns
`Supported(supported_answer)`, the continuation is the exact occurrence continuation
after whole-answer capture-safe substitution, and pure normalization checks `next`.
The other four resolution outcomes retain their residual/stop but cannot inhabit this
record.

A generated exterior and an actualized exterior are not interchangeable.

`source_ask_occurrence` is `Some` for every event lowered from a source
`Ask`. Replay must verify exact occurrence, question, request/operator,
binding/compiler versions, ledger membership, and provenance. Direct or legacy
runtime probes may use `None`; equality of question/operator/raw bytes/endpoints
does not substitute for the occurrence link.

`EventFor(event, ask_occurrence)` rechecks the source occurrence, semantic question,
exact request/operator, raw-return reference, binding/compiler versions, ledger
membership, and provenance. The resolver additionally checks that its raw input is
the payload named by that event's `raw_return` reference.

The old compressed alternation `q -> r -> q'` must lower to the non-collapse
occurrence chain:

\[
\boxed{
\mathfrak a_t
\xrightarrow{\mathsf{discharge}}
\mathcal D_t
\xrightarrow{\mathsf{resolve}}
\zeta_t=\mathsf{Supported}(\widehat S_t)
\xrightarrow{\kappa_t}
\mathsf{NextSourceControl}_t.
}
\]

Here:

- \(\mathfrak a_t\) is the checked `Ask`/probe occurrence and its source-program,
  environment, binding, operator, and continuation identity;
- \(\mathcal D_t\) is the complete port-indexed discharge bundle; every Probe
  component contains immutable ordinary `ActualEvent`, `RawReturn`, operator, port,
  and resolution-path provenance, while non-Probe components manufacture no event;
- \(\widehat S_t\) is the whole proof-carrying supported answer obtained through an
  explicit program-wide five-way resolution gate, with \(|\widehat S_t|\) its
  nonempty semantic member projection;
- \(\kappa_t\) is the checked first-order continuation belonging to \(\mathfrak a_t\).

Raw provider bytes never select a semantic continuation directly. Actuality,
interpretation, supported answer, and control succession remain different relations.

Question traces, raw-return traces, supported-answer traces, alternating traces,
paired actuality, sufficient presents, cue views, route/method occurrences, and
reacquisition indexes are derived projections over accepted source-program,
artifact, event, resolution, and continuation ancestry. They are not independent
authoritative stores.

Preserve these distinctions:

```text
ledger order != binding-supplied causal order
resume != replay
same endpoint != same event/path/provenance
active sufficient present != authoritative history
derived route graph != second history
```

Resume continues one checked occurrence from an admitted return. Replay reconstructs
the occurrence and derived continuations from accepted roots after restart without a
new external actualization. Endpoint or answer equality cannot erase event,
resolution-path, or continuation provenance.

No new history species or history database is introduced absent an executable
irreducibility breaker.

---

# 60. Resolution

Retain explicit resolution:

```rust
enum ResolutionPathIR {
    Identity,
    Decode { decoder: DecoderRef },
    Relation { relation: RelationRef },
    Compose { first: ResolutionPathRef, second: ResolutionPathRef },
    Program { program: ProgramRef },
}

enum ResolutionOutcomeIR {
    Supported(SupportedAnswerRef),
    ExactEmpty(EmptyCertificateRef),
    Undefined(ResolutionResidualRef),
    Unsupported(SupportResidualRef),
    Unknown(CoverageResidualRef),
}
```

`ResolutionPathIR` denotes a typed relation, not a scalar host function:

```text
Run(path) : input_type <-> output_type
Run(Identity, a, b) iff a == b
Run(Compose(p1, p2), a, c) iff exists b. Run(p1, a, b) and Run(p2, b, c)
```

`Relation` and `Program` edges may have zero, one, or many related outputs. The
candidate set for a question is every related output that also satisfies its
completion relation. Absence of a related output plus a typed path residual is
`Undefined`; an exhaustive checked empty candidate set is `ExactEmpty`; neither is
inferred from scalar failure or cardinality alone.

Each payload checker is indexed by the exact question, event/raw return, resolution
path, versions, route, and coverage. `ExactEmpty` requires exhaustive admitted
coverage; `Unsupported` retains decoded candidates and failed support obligations;
`Unknown` retains uncovered regions. Partial answers remain partial.

A partial exterior answer becomes a partial section/fiber and cannot silently become an exact sixfold role.

---

# 61. Memory state distinctions

Retain:

\[
Retained
\neq
Accessible
\neq
Active
\neq
Standing.
\]

A negation-use relation may be retained but not currently applicable.

A previously learned exterior route may be accessible but irrelevant to the live determination.

A generated candidate may be active without standing.

---

# 62. Active view and recurrent crawl

Question-conditioned retrieval is a derived `REFACTOR -> BIND -> OPEN`
specialization of the master recurrence:

\[
cue_n
\to
Retrieve
\to
Activate
\to
Residual
\to
cue_{n+1}.
\]

Positive-negation and return-fiber residuals simply provide new cue/question forms to the same process.

It is an access/index view, not another controller or history. No `NegationMemory`
subsystem.

---

# 63. General route reconstruction and actuality specialization

Define question/answer route reconstruction once over checked occurrences and whole
supported sets. For example, the supported-answer cue between an occurrence and a
successor question is:

\[
\operatorname{Cue}_S(\mathfrak a_t,q_{t+1})
=
\{\widehat S:
\operatorname{QSucc}(\mathfrak a_t,\widehat S,q_{t+1})
\}.
\]

The reciprocal construction opens a missing question occurrence from neighboring
supported-answer and checked-continuation constraints. A filler is regenerable only
when its residual fiber collapses to one protected class under declared coverage.
Otherwise its surviving classes become a separator residual.

The paired-actuality specialization additionally requires the event, raw return,
resolution path, supported answer, source continuation, and binding provenance from
Section 59. Regeneration of a semantic supported answer does not imply reconstruction
of the historical raw bytes, event identity, or resolution path unless those
protected signatures also factor through the cue.

Reciprocal inquiry:

\[
x
\to
(i,y)
\to
Ret_i(y)
\]

opens outward and asks what source information returns.

They are complementary uses of the same relation/fiber machinery.

Do not identify them, but implement them through the same generic hole/fiber solvers
and separator engine. Define route learning, ablation, compression, and reopening
once; actuality contributes historical provenance rather than a second learning
theory.

---

# 64. Claims and standing

A generated positive negation does not stand merely because it is structurally useful.

Retain:

```rust
struct ClaimIR {
    proposition: FormRef,

    support_envs:
        Vec<SupportEnvironmentRef>,

    applicability:
        FormulaRef,

    scope:
        FormulaRef,
}
```

Standing remains least fixed point:

\[
Stand=\mu T.
\]

A negation relation's `soundness_derivation` must itself rely on standing support appropriate to its exact/working status.

---

# 65. Binding evolution

Retain the three-way distinction:

\[
\boxed{
\text{definitional extension}
\neq
\text{conservative observational extension}
\neq
\text{rebinding}.
}
\]

Positive-negation inquiry frequently produces candidate conservative observational extensions:

- new attribute;
- new incompatibility relation;
- new probe;
- new observation coordinate.

Old actuality is never rewritten under a new binding.

---

# 66. Binding bridges

Retain:

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

Cross-binding recovery transports only when both return/opposition and protected observation diagrams commute.

---

# 67. Folds and reopening

A fold remains lawful only under its protected horizon/licence.

A new positive departure or new negation-use signature may distinguish forms previously identified.

Then:

\[
\boxed{
\text{new departure/return separator}
\to
Unlock
\to
Reopen.
}
\]

The positive-negation successor therefore gives a canonical source of unlock witnesses but does not alter the compression machinery.

Approximation licences must carry a directional or binding-specific soundness
relation.  Equal scalar error does not make over-approximation, under-approximation,
and other directional contracts interchangeable.  A method or fold whose validity
depends on the admitted language, binding, probe basis, or protected horizon must also
declare that extension domain through its applicability and unlock contract; a
successor outside it forces `Reopen` rather than inheriting admissibility.

Use the regenerative economy frontier for active representations and folds:

\[
\boxed{
Economy_{\mathcal H,\preceq}(z)
=
Min_{\preceq}
\{
m:
Regen^{inq}_{\mathcal H}(m,z)
\land
Licensed_{\mathcal H}(m,z)
\}.
}
\]

Retain every incomparable minimal candidate unless the declared preorder separates it.
Do not assume a unique global `argmin` or that a minimum exists.  A current-consequence
kernel check is not enough when continuation behavior, provenance, discriminators,
residuals, or reopening fail to regenerate.  This optimization applies to licensed
active structure; authoritative event history remains append-only.

For a live linear dot-product binding, the exact consequence-subspace specialization
may provide a compression certificate.  With

\[
M_Q=\mathbb E_{q\sim P_Q}[qq^\top],
\]

the exact quotient is

\[
\mathbb R^d/\ker M_Q\cong\operatorname{im}M_Q
\]

and has vector-space dimension `rank(M_Q)`.  The dimension lower bound applies to
linear representations (or another explicitly admitted representation class), not to
arbitrary encodings.  A certificate must retain the query-distribution version,
second-moment derivation, arithmetic/rank exactness, horizon, scope, and unlock
conditions.  Centered covariance is unlawful unless the mean direction is proven
irrelevant; estimated matrices and floating thresholds remain working/approximate
without certified bounds.  This theorem does not justify a vector database or a
vector-first retrieval architecture.

---

# 68. Method learning

Repeated reciprocal inquiry paths may be folded into methods.

A method may accelerate:

- choosing productive negation routes;
- executing seed bridges;
- solving common return fibers;
- generating recurrent separators;
- proposing recurring representation repairs.

Method utility remains traversal learning.

It does not warrant semantic outputs.

Native and learned methods share one typed registry contract:

```rust
struct MethodContractIR {
    id: MethodRef,
    implemented_relation: RelationRef,
    applicability: ApplicabilityRef,
    law: ArtifactRef,
    coverage: CoverageRef,
    authority: DischargeMode,
    extension_domain: ExtensionDomainRef,
    backend: BackendRef,
    checker: Option<CheckerRef>,
    cost: Option<CostModelRef>,
    failure_schemas: Vec<ResidualSchemaRef>,
    provenance: Vec<ArtifactRef>,
}
```

Derive rather than collapse these states:

```text
admitted = the contract has standing acceptance
runnable = its backend is available under current binding/resources
usable(q) = admitted + runnable + applicable + type/coverage/authority match for q
```

Method execution preserves the existing actuality boundary.  A pure registered method
may return without an event; an actualized backend compiles through `Probe` and stores
its raw return before classification.  After decoding and checking, represent the
semantic boundary as:

```rust
enum MethodResolutionIR {
    Success(SupportedAnswerRef),
    Residual(OpenRef),
    Terminal(StopStatusRef),
}
```

This is not the raw backend return.  In particular:

```text
certified empty solution -> typed semantic residual or exact terminal result
backend unavailable/crash/timeout -> Blocked, ResourceBounded, or Unknown
```

Residual handlers are typed registry entries, not universal hard-coded branches:

```rust
struct MethodBridgeIR {
    from_method: MethodRef,
    residual_schema: ResidualSchemaRef,
    to_method: MethodRef,
    transport: IProgRef,
    reentry_guard: FormulaRef,
    reconstruct_input: IProgRef,
}
```

A successful handler may reconstruct a new input and resume the suspended method
through ordinary first-order `IProg`.  This layer adds no runtime opcode and no
authoritative method-suspension table.  Conflict-core/repair, counterexample
refinement, discriminator design, and sound approximation are candidate registry
instances only when their own applicability, law, coverage, authority, and reentry
guards are admitted.

---

# 69. Self-revision

The calculus presentation itself may occupy the source role.

Generate candidate departure presentations that preserve much of the current protected structure while differing on a live residual.

Return them against the predecessor.

But acceptance remains predecessor-judged.

The positive-negation generator can propose self-revision; it cannot license it.

---

# 70. Persistence model

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
relation/type/program artifacts
actual events
raw returns
accepted patches
binding versions
standing support artifacts
accepted negation-use contracts
accepted determination presentations
```

Derived/rebuildable:

```text
negation frontier
departure witness indexes
return fibers when deterministically reconstructible
sixfold occurrence views
recovery profiles
coverage-indexed characterization views
return-signature families
active views
cue plans
operator occurrence graph
affected-fold index
```

Do not create an authoritative `current_sixfold` or `current_negation_frontier` table.

---

# 71. Replay

Replay uses historical:

- binding version;
- compiler version;
- decoder version;
- renderer version;
- accepted relation-use contracts;
- actual raw returns;
- patch history.

Later learned negation uses do not retroactively become part of earlier historical reciprocal occurrences.

---

# 72. Failure taxonomy

At minimum:

```rust
enum FailureClass {
    TypeError,
    InvalidRelation,

    InvalidDeterminationPresentation,

    DepartureWitnessFailure,
    NegationSoundnessFailure,
    NegationCoverageGap,

    CompilerDefect,
    PathProjectionDefect,

    RendererDefect,
    ProviderFailure,

    UnknownActuality,

    DecoderFailure,
    ResolutionFailure,

    ReturnAmbiguity,
    RecoveryFailure,

    RetrievalMiss,

    MaterializationGap,
    ExpressibilityGap,
    RepresentationGap,

    BindingBridgeFailure,

    WarrantGap,

    ReplayDefect,

    FoldFailure,
    RecoveryContractFailure,

    PerformanceBoundary,
    SpecificationGap,
    FixtureDefect,
}
```

A failure to find a departure witness under partial coverage is not a proof of interiority.

---

# 73. Regenerative inquiry controller

The controller implements the one `BIND -> OPEN -> VARY -> RETURN -> DETERMINE ->
REFACTOR` recurrence. One bounded iteration is:

```text
1. BIND
   reconstruct accepted artifacts, source program, event ancestry, standing,
   explicit environments, bindings, scope, applicability, grain, horizon,
   coverage, resource order, and protected residual

2. OPEN
   derive Formable / Applicable / Executable / Answerable / Productive /
   ResolvedQ / Ready and exact RequiredDischarge provenance
   construct the nondominated LiveQFrontier
   retain blocked, resource, authority, and representation obligations

3. VARY
   use ordinary relations and transparent roots/macros to form the smallest
   continuation-relevant question; when reciprocal inquiry is live, derive the
   exact DeterminationPresentation, tagged NegationUse, and positive departure
   obligations rather than inventing an exterior

4. RETURN
   lower the selected source Ask through Return / Branch / Probe
   preserve any ActualEvent and RawReturn before resolution
   discharge each open port only through its declared mode

5. DETERMINE
   resolve through explicit paths while preserving every supported completion
   return Supported(proof-carrying answer), ExactEmpty, Undefined, Unsupported,
   or Unknown without collapsing those outcomes
   derive exactly supported standing, recovery, coverage, and residual claims

6. REFACTOR
   only for Supported(S_hat), bind the whole answer record capture-safely and
   compute QSucc from the checked Ask/continuation occurrence
   for every other resolution outcome, preserve its typed residual or justified
   stop and do not invoke the source continuation
   reconstruct the successor program and live frontier
   refine, fold, reopen, regenerate, reorient, or rebind as warranted
   send ambiguity to the generic SeparatorProblem
   treat inexpressible witnessed distinctions as representation/probe/binding gaps
   change standing semantics only through separate checked and warranted revision

7. recur only across a new return, occurrence, distinction, representation,
   repository state, authority state, or strictly reduced finite frontier;
   fingerprint the occurrence/context and decrement fuel
```

Positive-negation sixfold traversal is one dependent `VARY -> RETURN -> DETERMINE ->
REFACTOR` specialization. It retains same-use fibers, recovery, seed/reorientation,
independent reciprocal admission, residuals, and downstream-only `Gamma`; it is not a
second controller.

A local interrogative fixed point holds only under declared root, binding,
effectivity, coverage, horizon, and resource bounds when every relevant question is
determined, factorable/redundant, inapplicable, nonproductive with no required
discharge, explicitly blocked/resource-bounded, or dependent on a represented
extension obligation. A new separator, probe, binding, representation, protected
horizon, or required discharge reopens it.

Stop states remain:

```text
Satisfied
Impossible
Equivalent
Blocked
Unknown
ResourceBounded
```

---

# 74. Stable acceptance fixture requirements

V2.0 requires every conservatively embedded predecessor fixture plus the following
reciprocal-boundary and interrogative obligations. Status and evidence belong only
in `CONFORMANCE_STATUS.md`; listing an obligation here makes no conformance claim.

## 74.1 Determination and departure

1. A source determination presentation is explicit and versioned.
2. Unrelated retained facts are not automatically constitutive.
3. Same-carrier exact cell exclusion has a constitutive separator witness.
4. Exact finite cell exclusion and constitutive separator existence coincide.
5. The 65,536-case finite feature fixture must produce zero mismatches.
6. Unknown observation does not establish departure.
7. Raw signature difference caused only by unknown remains unresolved.
8. Protected non-equivalence alone does not establish departure.
9. Departure may hold while source and candidate remain protected-equivalent.
10. Boundary crossing contains departure plus traversal provenance.
11. Departure does not imply observed crossing.
12. Cross-typed departure may use two observations plus an incompatibility relation.
13. Binding-native direct incompatibility may supply the witness without a shared observation codomain.

## 74.2 Typed negation

14. Boundary projection does not imply positive negation.
15. A negation use maps every exact admitted edge to a positive departure witness.
16. A proposed negation relation cannot use itself as its sole departure warrant.
17. Exact exhaustive partition complement forms lawful exact negation.
18. Certified partial opposition forms lawful partial negation.
19. Unknown is not converted into negative incidence.
20. Inapplicability is not converted into negative incidence.
21. One oriented negation use does not synthesize its reverse.
22. Exact semantic coverage and execution coverage remain distinct.
23. Empty exact exhaustive field differs from empty unsearched field.
24. No breaker under partial negation coverage does not establish reciprocal closure.

## 74.3 Multiple negation uses

25. Same exterior form through different negation uses remains distinct when return fibers differ.
26. Duplicate exterior grouping retains every witness/use.
27. Heterogeneous target carriers combine through a tagged dependent family.
28. `CertifiedPartial + CertifiedPartial` does not imply collective exhaustive coverage.
29. Collective exactness requires a cover certificate.
30. Untagged union fails the return-provenance breaker.
31. Intersection and union introduce no new exterior candidate absent from their members.
32. Ordinary relation composition may create a candidate relation but not automatic negation authority.
33. Open negation-family traversal uses generic generator fairness.

## 74.4 Return and recovery

34. For every admitted incidence, the source belongs to the reverse return section.
35. Source membership does not imply unique return determination.
36. Return fiber containing two protected source classes yields a separator obligation.
37. One selected stable `R_X` does not establish exact return closure if another protected class survives.
38. Exact singleton protected return fiber establishes return stability.
39. Raw relation differences ignored by the horizon do not constitute recovery failure.
40. Protected relation difference inside a return fiber does constitute recovery failure.
41. Local recovery equals protected-signature constancy on the return fiber.
42. Raw containment recovery is only a special case.
43. A source web can be partially recovered without unique source regeneration.
44. Family schema recovery may succeed where each member signature fails.
45. The three-state joint-recovery witness satisfies its declared recovery law.
46. Adding a return-signature coordinate refines but does not coarsen exact family observational equivalence.
47. Historical local recovery is not retroactively strengthened by later learned negation uses.

## 74.5 Return versus reconciliation

48. Compatible monotone constraint addition cannot produce a protectedly different filling from an already determining web.
49. The general monotonicity theorem is tested against finite fixtures.
50. Generated positive exterior does not mutate standing state.
51. Actual/warranted exterior may open reconciliation.
52. State-changing redetermination must expose revised/retracted/split predecessor relations, changed applicability, changed grain/binding, or prior underdetermination.

## 74.6 Sixfold

53. Sixfold roles are generated dependently, not independently.
54. `O_X` is generated from a specific negation use.
55. `R_X` arises from that use's reverse section.
56. `O_X` and `S_Y` retain distinct roles even if seed identity is used.
57. `O_Y` is independently generated from the \(Y\)-oriented frontier.
58. `R_X != O_Y` and `R_Y != O_X` remain role distinctions.
59. \(\Gamma_D\) cannot supply missing role fillings.
60. Stable \(X\)-return may coexist with unstable \(Y\)-return.
61. One-way negation does not imply reciprocal negation.
62. Sixfold occurrence view reconstructs from ordinary history/fibers.
63. Exact closure is indexed by negation semantic and execution coverage.
64. No breaker under partial frontier remains `Unknown`, not absolute closure.

## 74.7 Representation and learning

65. Sixfold/recovery residual can generate `RepresentationGap`.
66. New attribute can separate a previously collapsed protected pair.
67. Conservative attribute admission embeds old questions and enlarges the
    constructible question space only with a witnessed new well-typed question;
    rebinding does not imply inclusion.
68. New probe capability remains unadmitted until binding-extension checks pass.
69. New negation-use signature may reopen a previous fold.
70. Recurrent separator may become a candidate attribute without automatic standing.

## 74.8 Cross-cutting derived breakers

These are additional later-phase obligations, not new reciprocal roles or Phase 0
acceptance requirements:

1. Exact deterministic factorization agrees with kernel inclusion, while incomplete
   coverage cannot return `Exact`.
2. Individually supported signatures under mutually exclusive contexts cannot become
   one actual composite return without a jointness certificate.
3. Conservative extension preserves old questions; a rebinding that removes an old
   constructor defeats unqualified question-space inclusion.
4. Certified semantic emptiness and operational backend failure route to distinct
   method outcomes even when their surface error strings agree.
5. Equal scalar error on over- and under-approximations does not license the same
   protected inference.
6. Current-consequence kernel inclusion may hold while a protected reopening
   discriminator fails to factor, defeating inquiry-regenerative sufficiency.
7. Missing recovery evidence remains `Unknown`; it does not enter a complement-defined
   irrecoverable residue without witnessed non-recovery or exact decision coverage.
8. One protected four-class quotient admits several incomparable minimal two-cue
   separator bases.
9. An exact finite rational dot-product binding reconstructs the quotient, consequence
   subspace, and rank certificate.
10. A deterministic nonzero-mean query defeats centered-covariance substitution.
11. Query-distribution change reopens the consequence-subspace certificate; sampled or
    floating-rank estimates remain working/approximate without certified bounds.

## 74.9 Interrogative succession, control, and realized occurrence

These named obligations are owned by the phases identified in Section 75:

1. `QSUCC-OCC-001` — the same semantic `q` and same whole proof-carrying `S_hat`
   under two checked `Ask` continuations yield their distinct lawful successors (Phase 3).
2. `QSUCC-PARTIAL-001` — a proof-carrying answer with a partial member projection constructs a successor without
   implicit singleton selection (Phase 3).
3. `QSTATIC-DYNAMIC-001` — a static refinement, converse, or complementary relation
   does not manufacture a realized route edge (Phases 3 and 9).
4. `QREADY-UNLOCK-001` — readiness is false before an exact supported prerequisite
   and true afterward, with the changed prerequisite identified (Phase 9).
5. `QREADY-NONUNLOCK-001` — adjacency without a readiness change is rejected as
   `Unlock` (Phase 9).
6. `QCONVERSE-NOT-INVERSE-001` — a many-to-one reverse fiber remains
   non-singleton absent a uniqueness contract (Phases 2 and 10).
7. `QADJOINT-001` — a weakest-condition or adjoint pair requires a checked
   binding-supplied law (Phases 10 and 15).
8. `QRECIP-PROV-001` — a generic backward question with the wrong use identity is
   not same-use reciprocal return (Phases 4 and 10).
9. `QFRONTIER-REQDISCHARGE-001` — a required `Probe`, `Check`, `Warrant`, support,
   or reconstruction occurrence remains live without two discretionary answer branches
   and despite strict cost domination by another candidate; an optional occurrence
   strictly dominated by a required occurrence is removed
   (Phase 9).
10. `QIFP-LOCAL-001` — a finite declared root/effectivity frontier closes only under
    its declared coverage and explicit residual exits (Phase 9).
11. `QIFP-REOPEN-001` — a new separator, probe, representation, binding, horizon, or
    discharge obligation reopens local closure (Phases 9 and 14).
12. `QROUTE-REGEN-001` — a missing question or answer position regenerates only when
    its residual fiber collapses protectedly with provenance intact (Phases 8 and 16).
13. `QROUTE-ABLATE-001` — a route node is removable only when typing, authority,
    provenance, protected behavior, and reopening regenerate (Phase 16).
14. `QRENDER-001` — controlled wording preserves existential converse/preimage,
    universal guarantee/adjoint, and same-use reciprocal return distinctions
    (Phase 10).
15. `QLOWER-001` — erasing root/route annotations yields an accepted ordinary typed
    relation/question/`IProg` in the conservatively embedded substrate, with unchanged modes and actuality obligations
    (Phases 3 and 10).
16. `QACTUAL-SEPARATION-001` — two checked `Ask` occurrences with equal semantic
    question, compiled operator, raw bytes, and semantic member projection but
    protected-different continuations produce events linked to distinct occurrence
    references; formation independently rejects a forged structural position, question,
    answer slot, environment, continuation, binding/compiler version, or provenance
    field; cold replay reconstructs the corresponding occurrence-specific successor and
    a mixed-port case retains separate port-indexed route/path/event chains, including
    two ports sharing one explicitly checked event without collapsing their evidence, with zero redispatch,
    while rejecting a missing/swapped occurrence, request/operator, raw-return, version,
    ledger-membership, or provenance link
    (Phases 6--8).
17. `QLIFT-ALLPATHS-001` — non-singleton dependent stages preserve every tagged
    supported path and child discharge mode; a proper finite materialization leaves
    every uncovered parent member `Unknown` and cannot claim whole-family coverage
    (Phases 3--4).
18. `QRESOLUTION-GATE-001` — all five resolution constructors retain distinct
    checked payloads and only `Supported` enters the source continuation or constructs
    `ResolvedOccurrenceIR`; a non-singleton relational `Run`/`Compose` result retains every
    compatible completion and rejects first-output selection (Phases 7--8).
19. `QCODE-TYPING-001` — typed source/runtime quotation and binding/compiler-version
    interpretation reject mismatched versions without execution or warrant (Phases 3 and 5).

All predecessor typing, compiler, actuality, history, standing, fold, binding, and self-revision fixtures remain required.

---

# 75. Build phases

## Phase 0 — repository authority and scaffolding

Implement:

```text
workspace
toolchain pinning
Cargo.lock
canonical artifacts
migration framework
decision/failure logs
CI
```

## Phase 1 — typed forms and binding version identity

Implement:

```text
TyIR
TypedFormRef
BindingVersionRef
type checking
canonical identity
```

## Phase 2 — relation and OpenQuery kernel

Implement:

```text
RelSchemaIR
RelationUseIR
FormulaIR
Bind
Expose
OpenQuery
sections/fibers
```

## Phase 3 — first-order IProg

Implement:

```text
Return | Ask
answer binders
capture-safe substitution
program normalization
pure registered operations
checked Ask/continuation occurrence identity
SourceConfigIR / AskOccurrenceIR validation
AskQuestion / AskContinuation / HeadQ
occurrence-indexed QSucc
partial-supported-set successor reconstruction
proof-carrying support-witness validation
LiftQ all-path and per-child-mode preservation
source Code quotation
conservative derived-annotation erasure
```

`QSucc` is a derived relation over source-program identity and explicit environments,
not a new source constructor. The phase must reject `(q, S_hat)` as sufficient dynamic
route identity when two checked continuations differ.

## Phase 4 — determination, departure, typed negation, and reciprocal return

This phase implements the dependent positive-negation and reciprocal-return contract.

Implement:

```text
DeterminationPresentationIR
DepartureWitnessIR
incompatibility checking
NegationUseIR
NegationCoverage
NegationFrontierView
tagged exterior occurrence
positive-negation OpenQuery
use-specific reverse return fiber
protected relation signature
local RecoveryCheck
three-valued RecoveryStatusIR
derived coverage-indexed characterization view
family return signatures
schema recovery
exact determine-through factorization
jointness certificate when family information is actualized as one composite return
seed/reorientation
SixfoldOccurrenceView
fiber-level reciprocal residuals
Gamma post-check
```

Do **not** implement:

```text
Boolean complement fallback
primitive Exterior predicate
CombinedNegationIR
symmetric-negation assumption
state-changing Redet primitive
scalar near-negation metric
authoritative sixfold store
```

Phase 4 acceptance is governed by the reciprocal fixture obligations above.

## Phase 5 — runtime machine

Implement:

```text
ProgramIR
Return
Branch
Probe
verifier
suspension/resume
runtime Code quotation and binding/compiler-version-indexed partial interpretation
SourceAskLoweringIR verifier
```

## Phase 6 — persistence and actuality

Implement:

```text
SQLite journal
content-addressed store
ActualEvent
checked Ask/probe occurrence linkage
request-before-dispatch
raw-return persistence
crash/restart
```

## Phase 7 — resolution and general fibers

Implement:

```text
ResolutionPathIR
decoder contracts
partial/ambiguous results
Supported | ExactEmpty | Undefined | Unsupported | Unknown separation
question-indexed outcome certificate/residual payload validation
event-linked resolution provenance
HoleIR
FiberIR
ProtectedCompletionFieldIR
```

## Phase 8 — paired actuality

Implement:

```text
one-authority occurrence projection:
  AskOccurrence -> port-indexed DischargeBundle
  -> per-Probe ActualEvent/RawReturn/ResolutionPath
  -> ResolutionOutcome == Supported(whole proof-carrying SuppAns)
  -> checked continuation -> NextSourceControl(Ask | Return)
exact ActualEvent.source_ask_occurrence linkage
mixed-port route/event separation and exact whole-question resolution gate
derived question/raw-return/supported-answer/alternating trace views
equal question/operator/raw/endpoint with distinct Ask occurrence/continuation provenance
resume versus replay
missing supported-answer fiber
missing question-occurrence reconstruction
cold regeneration from accepted roots without redispatch
```

Do not add question, return, route, method, or memory history authorities. Paired
actuality specializes the general route/hole laws with ordinary historical
provenance.

## Phase 9 — active views and recurrent memory access

Implement:

```text
ActiveView
reserve
occlusion licences
access routes
activation witnesses
CrawlState
Formable / Applicable / Executable / Answerable
Productive / ResolvedQ / Ready / RequiredDischarge
LiveQFrontier under declared resource preorder
local interrogative fixed point
question-conditioned reopening
explicit blocked/resource/authority/extension residuals
```

The executable frontier retains `(Productive OR RequiredDischarge)`. A local fixed
point cannot close over an executable undischarged obligation, and incomplete
coverage returns `Unknown` rather than nonproductivity or resolution.

## Phase 10 — LLM/backend compiler

Implement:

```text
ProbeOperator
AnswerContract
SurfacePlan
BackendRequest
RawReturnEnvelope
MethodContractIR
admitted/runnable/usable method classification
typed method resolution after raw-return preservation
MockProvider
one real provider
controlled interrogative rendering and elaboration
implementation-only root/route annotations
renderer round-trip and conservative Lower_Q
```

The renderer must preserve relation identity, bound/open ports, modes, exact versus
partial coverage, converse/preimage versus universal guarantee, and same-use
reciprocal provenance. Surface wording and root labels cannot strengthen authority.

## Phase 11 — standing

Implement:

```text
ClaimIR
SupportEnvironmentIR
independent ingress
least-fixed-point standing
```

## Phase 12 — generic separator engine

Implement:

```text
SeparatorProblem
GeneratorRegime
GeneratedInquiry
deterministic QuestionPolicy
transparent Expose / Orient / Factor / Polarize / Vary / Ground macros
answer-conditioned residual-question generation
erasable route classifications
```

Consume:

```text
return ambiguity
recovery failure
sixfold mismatch
cue ambiguity
fold reopening
ordinary residual ambiguity
typed method residuals
```

through the same interface.

Static question relations do not create dynamic route edges. `Polarize` uses the
admitted logical breaker, separator, or positive-negation relation and never acts as
a Boolean contextual-negation fallback.

Add `MethodBridgeIR` and one residual-handler/reentry vertical slice compiled through
first-order `IProg`; do not add a method-specific runtime effect or authoritative
suspension table.

## Phase 13 — cue planning

Implement exact finite and working approximate cue planning.

Admission of every candidate cue must preserve:

```text
support and exact support route
applicability and binding
semantic/execution coverage
answer and occurrence provenance
required-discharge relevance
protected productivity or separator role
declared resource preorder
```

For finite exact residual fields, expose a sufficient discriminator basis check:

```text
every protectedly distinct live pair
-> at least one supported/applicable/covered cue with different answers
```

Select `Min` under a declared resource preorder and retain incomparable minimal bases.
The joint answer signature may use the exact kernel check only when answer behavior is
total, deterministic, and exactly covered.  Under incomplete generation or coverage,
return a nondominated working frontier plus `Unknown` residuals rather than claiming a
minimum or impossibility.

A concrete unseparated protected pair returns to `SeparatorProblem`; absence from a
finite materialized cue catalog does not establish a representation gap.

## Phase 14 — representation-gap detection

Implement:

```text
MaterializationGap
ExactNoSeparatorWithinRegime
RepresentationGap
FreshWithinRegime
ProposedRegimeExtension
representation/probe unlock witness
local-IFP reopening witness
```

A lawful extension must identify which prior question was non-formable or
non-executable and which exact new capability changes that status. Mere generation or
adjacency is not `Unlock`.

## Phase 15 — binding extension and bridges

Implement:

```text
definitional extension
conservative observational extension
rebinding
BindingBridgeIR
targeted reopening
conservative question-language embedding and strict-growth witness
typed question/route transport
formability effects of extension or rebinding
```

Transport questions and learned routes only through typed bridges preserving their
relations, ports, modes, bindings, applicability, and protected interpretation. Noun
similarity or surface analogy is not a bridge.

## Phase 16 — method learning and folds

Implement:

```text
operator occurrences
method folds
derived QRouteOcc over source/event/resolution/continuation ancestry
candidate question methods
route ablation and transparent expansion
missing question/supported-answer regeneration
RecoveryContract
CompressionLicence
directional approximation soundness
extension-domain-sensitive method/fold applicability
regenerative economy frontier
optional exact linear consequence-subspace certificate for a live numeric binding
Unlock
Reopen
```

Method/route names remain provenance and authoring aids, not dispatch policy or
semantic authority. A route fold is retained only when its expansion regenerates
typing, applicability, authority, failure exits, protected behavior, provenance, and
reopening.

## Phase 17 — conservative cross-binding standing lift

Use the already established safe rule:

```text
transport old standing only at old scope/applicability/grain/horizon
reuse unaffected evidence
open liabilities for new distinctions
recompute before broader promotion
```

## Phase 18 — predecessor-judged self-revision

Implement candidate patches and locked predecessor acceptance. Reify controller,
question, frontier, root/macro, compiler, and route structures as ordinary typed
forms when they become objects of inquiry. A candidate controller revision remains
predecessor-judged and cannot use its own route policy or recurrence as its warrant.

## Phase 19 — measured breadth only

Only after witnessed need:

```text
learned question policy
learned frontier/routing policy and latent-capability masks
fair open-ended generation
vector retrieval
parallel semantic scheduling
PostgreSQL
distributed effects
```

Every Phase 19 addition requires measured protected strict gain against the
self-hosted reference controller and remains removable without changing semantics.

---

# 76. Required vertical slices

## Slice A — positive-negation reciprocal specialization

Demonstrate:

```text
typed relation
-> source determination presentation
-> positive departure witness
-> admitted negation use
-> positive-negation OpenQuery
-> actual/generated exterior
-> use-specific reverse return fiber
-> recovery check
-> seed/reorientation
-> reciprocal positive negation
-> reciprocal return
-> residual question
```

No fake semantic step.

## Slice B — actual probe and standing

Add:

```text
Prompt/BackendRequest
-> RawReturn
-> ActualEvent
-> ResolutionPath
-> whole proof-carrying supported answer
-> Claim
-> independent support
-> Standing
```

## Slice C — ambiguous return

Demonstrate one exterior whose reverse section contains two protected source classes.

The runtime must generate a separator rather than select one silently.

## Slice D — joint family recovery

Construct two negation uses where neither member signature recovers a target observation but their product does.

## Slice E — materialization versus expressibility

Demonstrate:

```text
existing lawful route but not materialized
```

versus:

```text
no separator in admitted language
-> RepresentationGap
```

## Slice F — probe invention

A sixfold/recovery residual remains inexpressible by the current probe basis.

Admit a new probe through conservative binding extension and reopen the old residual.

## Slice G — fold/reopen

Learn/fold a recurrent reciprocal path.

Introduce a new departure/signature that invalidates the fold licence.

Reopen while preserving history.

## Slice H — self-revision

Use the same inquiry language on a reified implementation/specification claim.

Candidate successor cannot change its own predecessor judge.

## Slice I — occurrence-indexed self-hosted inquiry

Demonstrate and cold-replay:

```text
protected residual / explicit discharge obligation
-> LiveQFrontier with Productive OR RequiredDischarge
-> checked Ask occurrence
-> transparent root/macro lowering
-> ActualEvent + immutable RawReturn where required
-> explicit resolution path + whole proof-carrying supported answer
-> occurrence-indexed QSucc
-> successor residual/frontier or justified local IFP
```

Use two checked continuations sharing the same semantic question and proof-carrying answer to
prove that route identity cannot collapse to `(q, S_hat)`. Replay invokes the external
provider zero additional times and regenerates the same protected continuation and
provenance identities from accepted roots.

---

# 77. What may remain algorithmically simple first

Use deliberately simple correct algorithms:

```text
determination presentation:
    claim-local support/dependency web

negation frontier:
    deterministic finite list when finite

negative-route selection:
    stable deterministic policy

return fibers:
    exact finite enumeration where possible

recovery:
    direct protected-signature constancy check

family signatures:
    explicit tuple/product in finite fixtures

standing:
    full monotone fixed-point worklist

separator selection:
    deterministic exact-first policy

cue minimization:
    brute force under threshold

retrieval:
    exact relational/index traversal

provider:
    mock + one real backend
```

Do not optimize before instrumentation.

---

# 78. What must not be mocked in a complete semantic slice

These must be real:

```text
typed relation identity
source determination presentation
departure witness
negation-use provenance
semantic negation coverage
partial binding
answer-dependent IProg
checked Ask/continuation occurrence identity
whole supported-answer preservation
occurrence-indexed QSucc
raw return preservation
actual event ancestry
resolution-path provenance
use-specific reverse return
protected recovery
residual separator generation
required-discharge frontier behavior
standing separation
binding version
fold recovery/reopening
```

External environments may be mocked only through explicit bindings.

---

# 79. Deferred architecture

Do not build yet:

```text
universal complement algebra
global object-property ontology
CombinedNegationIR
global scalar near-negation score
global semantic union of negative relations
separate sixfold runtime/history
general agent framework
vector-first memory
global learned relevance oracle
automatic semantic rebinding
universal minimal determination web
```

Each requires a witnessed protected need.

---

# 80. Engineering execution authority

`AGENTS.md` owns contributor/agent execution discipline and the enforced engineering
harness. This plan owns architecture, phase dependencies, stable obligations, and
completion criteria; it does not duplicate the engineering clock or carry a live
work queue.

The implementation must nevertheless preserve these architectural non-collapse
laws at every phase boundary:

```text
generation != actuality != resolution != checking != warrant != standing
semantic question != Ask occurrence != compiled operator != ActualEvent
semantic coverage != execution coverage
static question relation != realized route edge
same endpoint != same event/path/continuation provenance
derived trace/fold/frontier != authoritative history
candidate successor != its predecessor judge
```

---

# 81. Required project-state files

Retain:

```text
IMPLEMENTATION_FRONTIER.md
CONFORMANCE_STATUS.md
DECISIONS.jsonl
FAILURES.jsonl
```

Important decisions to record include:

```text
determination presentation admission basis
negation-use soundness route
negation semantic coverage
generator coverage
return/recovery exactness
binding extension classification
fold/reopening effects
```

---

# 82. CI gates

Every change:

```text
cargo fmt --all --check
cargo check --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
embedded migration/replay checks when affected
canonical artifact and type/IR verification
documentation-control check
git diff --check
```

The specification job compiles only `Inquiry_Calculus_v2_0.tex` and rejects fatal
errors, unresolved references, duplicate labels, and stale active-authority names.

Reciprocal-core changes additionally:

```text
departure witness fixtures
unknown-not-negative fixtures
negation soundness fixtures
coverage separation fixtures
duplicate-use provenance breaker
return-fiber ambiguity fixtures
recovery fixtures
joint-family-recovery fixture
monotone-redetermination breaker
sixfold dependency fixtures
```

History changes:

```text
cold replay
historical version locality
```

Interrogative/controller changes:

```text
occurrence-indexed QSucc breaker
partial-supported-set preservation
Productive OR RequiredDischarge frontier
static/dynamic route separation
local IFP closure and reopening
root/route annotation erasure and conservative Lower_Q
raw actuality -> resolution -> supported answer -> continuation separation
```

Binding changes:

```text
bridge preservation
affected-fold reopening
old-event immutability
```

---

# 83. Observability

Every reciprocal inquiry should expose references for:

```text
checked Ask/continuation occurrence and explicit environment
semantic question and live residual/frontier
Productive or exact RequiredDischarge reason
source determination presentation
departure witness route
negation use
semantic negation coverage
generator/execution coverage
positive exterior occurrence
raw return/event
resolution path and whole proof-carrying supported answer
occurrence-indexed successor question
return fiber
protected recovery profile
seed bridge
reciprocal negation use
reciprocal exterior
reciprocal return fiber
sixfold residuals
Gamma result
separator problem
representation-gap result
```

The system should answer:

```text
Why is this candidate exterior?
Which relation made it exterior?
Why is that relation allowed to constitute the source determination?
Is this negation field exhaustive, field-relative, partial, or working?
Has the field actually been fully explored?
Which negation use produced this exterior?
What does that use return?
Which source relations are recovered?
What remains ambiguous?
Why was this reciprocal route selected?
What historical event actualized it?
Which resolution path supports the answer set?
Which checked continuation generated the successor question?
Was the question productive, required for discharge, or both?
What residual question was generated?
```

---

# 84. Performance policy

Instrument:

```text
determination web construction
departure witness checking
negation-frontier size
negation section generation
return-fiber solve time
recovery check time
family signature size
separator generation
standing iterations
memory crawl
prompt context
provider latency/cost
replay
binding bridge checks
fold reopen cost
```

Only cross architecture boundaries after measured evidence.

---

# 85. Production migration path

Preserve:

\[
\boxed{
Rust\ monolith
+
SQLite
+
content\ store
}
\]

until measured scale requires:

- stronger relational DB;
- object storage;
- parallel effect execution;
- distributed workers.

Physical migration must not change semantic identity or history.

---

# 86. Deferred and conditional implementation gates

## 86.1 Determination-presentation admission

The system requires a lawful \(W_D(x)\) that defines the live determination for
departure judgment without importing unrelated facts or omitting constitutive
support.

Unsafe extremes:

```text
all standing facts about x
    -> incidental differences become false identity criteria

too-small hand-selected web
    -> genuine departures disappear
```

Admissible implementation candidates include:

- all standing related constraints;
- minimal regeneratively sufficient webs;
- claim/support-local determination presentations;
- possibly a family of admissible presentations when minima are nonunique.

Every admitted presentation must be explicit, supported, versioned, indexed by
scope/applicability/grain/horizon, and exactly traceable to its standing source
determination. A claim-local support/dependency presentation is the reversible
baseline. Regenerative minimization is a separate fold with recovery and reopening,
not a prerequisite for reciprocal execution. A stronger canonical admission or
unique-minimum rule requires an executable breaker and accepted theorem.

## 86.2 Cross-binding standing lift

Retain the Phase 17 conservative rule until a stronger accepted theorem preserves
old scope, applicability, grain, horizon, support, and historical locality.

## 86.3 Open-ended generator completeness

Do not claim a universal finite negative, separator, root, or question basis.
Resource-bounded exhaustion remains `Unknown` or `ResourceBounded` outside declared
exact coverage.

## 86.4 Learned policy

Question, root, negation-route, and frontier policy is Phase 19 optimization only.
It cannot become semantic authority, suppress required discharge, or warrant itself.

## 86.5 Retrieval basis

Add vectors or approximate retrieval only after an exact-route protected miss and a
measured strict-gain fixture. Preserve exact references, replay, and `Unknown`.

## 86.6 General transition authority and protected erasure

Do not promote operational crashes, attempts, route labels, or state transitions into
new semantic actuality or authority species. A generalized transition authority or
protected-erasure binding remains deferred until an accepted semantic/binding
contract supplies its types, evidence route, non-collapse laws, and predecessor
judgment.

Cold replay must satisfy the stable dependency:

```text
completed effect token
-> rechecked BackendRequest / ActualEvent / RawReturn after restart
-> exact source Ask occurrence linkage when source-derived
-> ResolutionPath / FiniteDecoder
-> Supported | ExactEmpty | Undefined | Unsupported | Unknown
   Supported -> exact Probe observation support and least-fixed-point standing
             -> whole proof-carrying supported answer
             -> reloaded source Ask and capture-safe binding
             -> regenerated ProgramIR / ProbeSuspension / ContinuationLowering
             -> admitted resumption -> next Ask or Return
   ExactEmpty | Undefined | Unsupported | Unknown
             -> constructor-specific typed residual or justified stop
             -> never the source continuation
```

No pre-crash derived object or provider redispatch may supply the replay. Regenerate lowering from
accepted source/compiler identities and versions first. A persistable compile/replay recipe is
authorized only if a fresh-process breaker proves that an exact mapping is otherwise unrecoverable.

---

# 87. Stable architectural laws

The following laws constrain every phase and completion level.

\[
\boxed{
\text{relation is the semantic primitive}.
}
\]

\[
\boxed{
\text{reciprocal inquiry begins from a live determination presentation}.
}
\]

\[
\boxed{
\text{positive exteriority is witnessed by determination-relevant positive incompatibility}.
}
\]

\[
\boxed{
\text{exteriority is not protected non-equivalence}.
}
\]

\[
\boxed{
\text{unknown is neither interior nor exterior}.
}
\]

\[
\boxed{
\text{typed negation is a supported oriented relation role}.
}
\]

\[
\boxed{
\text{typed negation is not Boolean complement}.
}
\]

\[
\boxed{
\text{soundness does not require exhaustive complement}.
}
\]

\[
\boxed{
\text{semantic negation coverage is explicit}.
}
\]

\[
\boxed{
\text{semantic coverage}
\neq
\text{execution coverage}.
}
\]

\[
\boxed{
\text{multiple negation relations form a tagged family}.
}
\]

\[
\boxed{
\text{untagged semantic union loses reciprocal provenance}.
}
\]

\[
\boxed{
\text{joint information accumulates through return-signature products}.
}
\]

\[
\boxed{
\text{positive negation is a section filling}.
}
\]

\[
\boxed{
\text{return is reverse section}.
}
\]

\[
\boxed{
\text{return fiber}
\neq
\text{selected return filling}.
}
\]

\[
\boxed{
\text{recovery is protected determination by the return fiber}.
}
\]

\[
\boxed{
\text{local recovery}
\neq
\text{family schema recovery}.
}
\]

\[
\boxed{
\text{pure reciprocal return}
\neq
\text{semantic reconciliation}.
}
\]

\[
\boxed{
\text{compatible monotone constraint addition cannot change an already determined source class}.
}
\]

\[
\boxed{
\text{sixfold roles are generated dependently}.
}
\]

\[
\boxed{
\Gamma_D\text{ is downstream compatibility, not role generation}.
}
\]

\[
\boxed{
\text{sixfold is a derived view over ordinary programs, fibers, support, and events}.
}
\]

\[
\boxed{
\text{reciprocal residuals feed the generic separator engine}.
}
\]

\[
\boxed{
\text{separator failure may generate representation/probe/binding growth}.
}
\]

\[
\boxed{
\text{new representation may enlarge the future question universe}.
}
\]

\[
\boxed{
\text{standing, history, folds, bridges, and predecessor judgment remain unchanged in authority}.
}
\]

\[
\boxed{
\text{dynamic question succession is indexed by the checked Ask/continuation occurrence}.
}
\]

\[
\boxed{
\text{live question frontier}
=
\text{productive questions}
\cup
\text{required discharge}.
}
\]

\[
\boxed{
\text{raw actuality}
\neq
\text{resolution}
\neq
\text{supported answer}
\neq
\text{control succession}.
}
\]

\[
\boxed{
\text{roots are derived operators; the regenerative recurrence is control}.
}
\]

\[
\boxed{
\text{there is one authoritative event/program/artifact ancestry and only derived trace views}.
}
\]

---

# 88. Completion dependency and cold-replay closure

The four completion levels in Section 0.1 are cumulative except that Phase 19 remains
conditional experimentation. `REFERENCE_CALCULUS_COMPLETE` requires one repository
to execute and cold-replay the following dependency without hidden semantic state:

\[
\boxed{
\begin{aligned}
&\text{typed forms and relations}\\
&\to
\text{derived roots/macros conservatively lowered to an occurrence-indexed Ask}\\
&\to
\text{standing source determination presentation}\\
&\to
\text{positive departure witness}\\
&\to
\text{coverage-indexed typed negation use}\\
&\to
\text{tagged negative frontier}\\
&\to
\text{positive-negation question}\\
&\to
\text{generated or actual exterior occurrence}\\
&\to
\text{use-specific reverse return fiber}\\
&\to
\text{protected recovery}\\
&\to
\text{seed/reorientation}\\
&\to
\text{reciprocal negation and return}\\
&\to
\text{sixfold residuals}\\
&\to
\Gamma_D\text{ compatibility}\\
&\to
\text{generic separator problem}\\
&\to
\text{representation/probe extension if required}\\
&\to
\text{raw actuality / resolution / whole supported answer / QSucc}\\
&\to
\text{successor residual and live question frontier}\\
&\to
\text{derived paired actuality and route views over one history}\\
&\to
\text{standing}\\
&\to
\text{fold / recovery / reopening}\\
&\to
\text{binding extension / bridge}\\
&\to
\text{predecessor-judged self-revision}.
\end{aligned}
}
\]

No step may be supplied by:

- Boolean fallback;
- hidden exterior predicate;
- untyped "other" selector;
- opaque LLM judgment;
- mutable provider session;
- raw-return bytes selecting a semantic continuation;
- `(q, S_hat)` used as dynamic route identity without its checked occurrence;
- productivity used to suppress an explicit required discharge;
- a root or route label treated as a semantic opcode;
- a derived trace, present, route graph, or fold treated as another history;
- semantic union that loses negation-use provenance;
- state-changing return without revision evidence.

`SELF_HOSTED_INQUIRY_CONTROLLER_COMPLETE` additionally requires the implemented
calculus to construct and close or reopen this successor frontier itself. Thesis
evaluation begins only after that controller is executable; adaptive routing remains
conditional on measured benefit.

---

# 89. Stable phase dependency

Section 75 is the single build map. The frontier selects one executable obligation
from that map based on repository actuality; this plan does not record the cursor.
The stable dependency is:

```text
Phases 0--2
  repository, identity, typed forms, relations, OpenQuery

Phase 3
  first-order IProg and occurrence-indexed question succession

Phases 4--7 and 11
  reciprocal semantics, runtime, actuality, resolution, and standing

Phases 8--10
  one-history occurrence projections, live frontier, and controlled compiler/provider

Phases 12--16
  separator recurrence, cue basis, representation gaps, bridges, and transparent folds

Phase 17
  conservative cross-binding standing lift

Phase 18
  replayable predecessor-judged revision

reference-calculus closure
  complete successor chain, cold replay, ablation, and acceptance reconciliation

self-hosted controller closure
  residual -> LiveQFrontier -> occurrence -> supported answer -> QSucc -> residual

thesis evaluation
  controlled empirical comparison at comparable resources

Phase 19
  measured optional optimization only after self-hosted evidence
```

Phases may be exercised by vertical slices before every neighboring phase is globally
complete, but a later slice never implies completion of an earlier phase or of the
whole project. `ic-machine`, new authoritative storage, new opcodes, provider
frameworks, and distributed scheduling remain deferred until an executable breaker
establishes irreducible need.

---

# 90. Compact v2.0 implementation invariant

The implementation may use the following as the compressed statement of the v2.0 architecture:

\[
\boxed{
\begin{minipage}{0.92\linewidth}
Inquiry Calculus v2.0 is a well-typed relational inquiry programming language.
A question is an open typed relation, and a source program is a first-order
answer-dependent `Return`/`Ask` structure. Dynamic question succession is derived
from the checked `Ask` occurrence, its explicit environment, the whole supported
answer set, and its continuation; semantic `(q,S)` alone is not route identity.
The live frontier retains both protectedly productive questions and exact required
discharge obligations. Roots and operational interrogatives are transparent derived
operators that lower to the ordinary language.

An actual inquiry occurrence preserves the checked occurrence, immutable raw
return/event, resolution path, supported semantic answer set, and continuation as
distinct identities. Resume is not replay, endpoint equality is not path identity,
and derived traces, sufficient presents, route graphs, cues, and folds remain views
over one accepted artifact/source-program/event ancestry rather than new histories.

A live determination is represented by an explicit, supported relational
presentation.  Its exterior is not presumed from a boundary projection and is
not obtained by Boolean complement.  A candidate is positively established as
outside the determination when a standing determination-relevant discriminator
places source and candidate in incompatible typed cells.  An oriented relation
whose admitted incidences have such departure witnesses may serve as a typed
negation use, with semantic coverage stated independently of execution
coverage.

A determination may admit multiple negation uses.  They remain a tagged family:
the relation use that licensed an exterior remains part of the reciprocal
occurrence because different uses can return different source fields.  Inquiry
partially binds one such relation to the source and positively determines an
exterior filling.  Pure return is the reverse section of that same relation.
The return fiber recovers a protected source relation exactly when every source
still possible through that fiber agrees on the protected consequence of the
relation.  Multiple negation uses accumulate reusable information through the
product of their return signatures, not through an untagged semantic union.  The
product remains a derived information view unless a supported jointness certificate
licenses its interpretation as one actual composite return.

The exterior filling is then taken as a new center of determination through an
explicit seed/reorientation relation, and the same operation is repeated in the
reciprocal orientation.  The six dependent roles
\((S_X,O_X,R_X;S_Y,O_Y,R_Y)\) are therefore a derived reciprocal trace, not six
independent openings and not a separate history.  Exact reciprocal closure is
fiber- and coverage-relative.  Gamma checks joint compatibility only after the
dependent roles have been generated.

A generated exterior does not revise standing semantics.  If warranted
actuality invalidates the standing source web, reconciliation/revision produces
a successor web; compatible monotone constraint addition alone cannot transform
an already determined source into a protectedly different source.  Every
surviving return ambiguity, recovery failure, seed mismatch, reciprocal
mismatch, or compatibility failure becomes an ordinary protected residual and
feeds the generic separator engine.

When the required separator is absent only from current materialization,
generation continues within the admitted language.  When no admitted
question/probe/representation can express a witnessed protected distinction,
the result is a representation or binding gap.  Recurrent separators may be
reified as attributes, methods, probes, or representation coordinates; admitted
extensions enlarge the later question language.  Actual question--return
occurrences remain reconstructible from authoritative ancestry, folds remain
reopenable, standing remains independently warranted, and self-revision remains
predecessor-judged. All control returns through the one derived
`BIND -> OPEN -> VARY -> RETURN -> DETERMINE -> REFACTOR` recurrence.
\end{minipage}
}
\]

---

# 91. Conditional determination-presentation contract

Departure judgment requires an explicit answer to:

\[
\boxed{
\textbf{WHAT EXACT RELATIONAL PRESENTATION IS LICENSED TO COUNT AS
THE LIVE DETERMINATION \(W_D(x)\) FOR DEPARTURE JUDGMENT?}
}
\]

The implementation boundary must be capable of representing:

1. one minimal regeneratively sufficient web;
2. a family of incomparable sufficient webs;
3. a claim-local support/dependency presentation;
4. a quotient/fold over such a family.

A claim-local support/dependency presentation with full provenance is the smallest
reversible baseline. Do not absorb every standing fact merely because it mentions the
source, and do not require an unproved unique global minimum. Any minimization or
quotient is a separately licensed fold that preserves support, applicability, scope,
grain, horizon, recovery, and reopening.

The accepted decision ledger selects among these representations for a concrete
implementation scope. A future theorem may strengthen the contract only through the
ordinary breaker, evidence, and predecessor-judgment path; the live selection itself
does not belong in this plan.

---

# 92. Final implementation directive

Implement v2.0 from the smallest structure that regenerates its protected behavior:

\[
\boxed{
\textbf{REPRESENT THE LIVE DETERMINATION;
OPEN THE SMALLEST CONSEQUENTIAL QUESTION OR REQUIRED DISCHARGE;
RETAIN THE CHECKED ASK/CONTINUATION OCCURRENCE;
WITNESS DEPARTURE POSITIVELY;
USE COVERAGE-INDEXED TYPED OPPOSITION;
RETAIN THE OPPOSITION ROUTE;
RETURN THROUGH ITS REVERSE SECTION;
MEASURE WHAT THE RETURN ACTUALLY RECOVERS;
REORIENT AND REPEAT;
PRESERVE RAW ACTUALITY BEFORE RESOLUTION;
BIND THE WHOLE PROOF-CARRYING SUPPORTED ANSWER RECORD;
DERIVE THE SUCCESSOR FRONTIER THROUGH OCCURRENCE-INDEXED QSUCC;
TURN SURVIVING DIFFERENCE INTO A SEPARATOR;
EXTEND REPRESENTATION ONLY WHEN THE CURRENT LANGUAGE CANNOT EXPRESS IT;
FOLD ONLY WHAT REGENERATES AND REOPEN ON A NEW DISTINCTION;
AND NEVER LET GENERATION, HISTORY, OR REVISION CLAIM MORE WARRANT THAN
THEIR EXPLICIT ROUTES PROVIDE.}
}
\]

Every step instantiates the one `BIND -> OPEN -> VARY -> RETURN -> DETERMINE ->
REFACTOR` recurrence. If a proposed component cannot lower to, instantiate, or
constrain that architecture—and its removal loses no protected capability—it does
not belong in the reference implementation.
