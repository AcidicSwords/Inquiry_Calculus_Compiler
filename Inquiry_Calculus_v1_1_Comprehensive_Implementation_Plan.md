# Inquiry Calculus v1.1 — Comprehensive Implementation Plan
## Successor Reciprocal-Boundary / Positive-Negation Edition

**Status:** Current implementation-facing specification

**Standing semantic authority:** Inquiry Calculus v1.1 together with its canonical additions, modified only by the successor reciprocal-boundary semantics established in the present implementation inquiry

**Predecessor:** `Inquiry_Calculus_v1_1_Comprehensive_Implementation_Plan.md` as recorded by Git commit `49dc381ac230326aa28be6c157ece0d21a31eaa2`

**Supersession rule:** This stable-path document supersedes the predecessor as the current implementation-facing plan. The predecessor remains retrievable from Git as ancestry and regression evidence; no competing live predecessor-plan path is retained.

**Purpose:** Specify the smallest complete implementation architecture that realizes the standing calculus after the reciprocal-boundary correction without introducing semantic machinery not required by the calculus.

---

# 0. Executive statement

The implementation architecture remains fundamentally the one established for Inquiry Calculus v1.1:

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

The major successor correction is concentrated in the reciprocal-boundary core.

The predecessor architecture began from reciprocal relations or boundary projections and then compiled path-preserving round trips. That preserved answer dependence and branch provenance, but it still treated "the other side" as already supplied by the reciprocal distinction.

The successor semantics makes the missing dependency explicit:

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

The implementation recursion further established several corrections required to make this conception exact:

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

These corrections do not replace the rest of the implementation plan. They give the existing separator, representation-growth, probe-invention, actuality, history, folding, binding-extension, and self-revision mechanisms their canonical reciprocal generator.

The current implementation center is therefore:

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

---

# 1. Authority and relationship to v1.1

This document is not a replacement calculus.

It is the implementation successor obtained by correcting the generative interpretation of reciprocal distinction while preserving every v1.1 structure not broken by that correction.

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

The successor gives this criterion a stronger reciprocal interpretation: a web determines \(x\) exactly when its remaining lawful fillings contain only one protected source class. Positive-negation inquiry explores forms outside the current determination and asks what source structure survives the reciprocal return.

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

The positive-negation successor adds a second implementation discipline:

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

Every component belongs to one of three classes.

## 5.1 Constitutional machinery

Required for semantics:

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

## 5.3 Research/optimization gates

Current remaining gates include:

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

No dynamically typed semantic escape hatch.

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
        value: TermExprRef,
    },

    Ask {
        question: QuestionExprRef,
        continuation: IProgRef,
    },
}
```

No arbitrary Rust closures.

The actual answer may construct the later question.

This remains essential to the reciprocal sixfold.

---

# 14. The source determination presentation

The successor semantics requires an explicit source determination presentation:

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

# 38. Successor sixfold semantics

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

A schematic source program is:

\[
\boxed{
\begin{aligned}
\mathsf{Recip}(D,S_X)
={}&
\mathsf{Ask}
(
?O_X[N_i^X(S_X,O_X)],
\lambda O_X.\\
&
\mathsf{Ask}
(
?R_X[
R_X\in Ret_i^X(O_X)
],
\lambda R_X.\\
&
\mathsf{Ask}
(
?S_Y[
Seed_Y(O_X,S_Y)
],
\lambda S_Y.\\
&
\mathsf{Ask}
(
?O_Y[
N_j^Y(S_Y,O_Y)
],
\lambda O_Y.\\
&
\mathsf{Ask}
(
?R_Y[
R_Y\in Ret_j^Y(O_Y)
],
\lambda R_Y.\\
&
\mathsf{Return}_I(
\Xi_D,
Residuals(\Xi_D)
)))))).
\end{aligned}
}
\]

In practice the generator emits candidate programs for applicable \(N_i,N_j\) rather than embedding one global relation.

No new runtime opcode is required.

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

A reciprocal role or negation route may be skipped only when it cannot change the current protected residual or discharge another live obligation.

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

Positive-negation inquiry is now one canonical source of such witnessed gaps.

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
\iota_Q:
Q^\infty(\Sigma_t)
\hookrightarrow
Q^\infty(\Sigma_{t+1})
}
\]

Strict inclusion requires a well-typed successor question outside the image of
\(\iota_Q\).  Definitional replacement or rebinding need not preserve old question
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

---

# 59. Actuality and paired history

One authoritative occurrence record:

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

A generated exterior and an actualized exterior are not interchangeable.

The paired history remains:

\[
Q\xrightarrow{\alpha}R\xrightarrow{\kappa}Q.
\]

The positive-negation successor gives this an additional interpretation:

\[
\boxed{
\text{open distinction}
\to
\text{positive determination}
\to
\text{new distinction}.
}
\]

No new history species.

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
```

Partial answers remain partial.

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

Question-conditioned retrieval remains recurrent:

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

No `NegationMemory` subsystem.

---

# 63. Cue reconstruction and reciprocal inquiry

Cue reconstruction:

\[
q_t
\to
\square
\to
q_{t+1}
\]

solves a missing historical filling from retained relational constraints.

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

Do not identify them, but implement them through the same generic solvers and separator engine.

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

# 73. Recurrent inquiry controller

One bounded iteration:

```text
1. reconstruct accepted presentation
2. refresh standing
3. identify current source question/determination
4. materialize the source DeterminationPresentation
5. construct active view / retrieve relevant structure
6. derive applicable NegationUse family
7. derive tagged NegationFrontier
8. construct candidate positive-negation IProgs
9. select one lawfully
10. lower through Return/Branch/Probe
11. actualize if required
12. preserve raw return
13. resolve supported exterior filling
14. derive use-specific reverse return fiber
15. compute local recovery / residual ambiguity
16. seed/reorient if reciprocal continuation remains live
17. repeat positive-negation/return on reciprocal side
18. construct sixfold residuals
19. apply Gamma compatibility
20. send unresolved protected differences to SeparatorProblem
21. detect representation/probe gaps when separators are inexpressible
22. update claims/support
23. reconcile standing semantics only if warranted actuality requires it
24. fire unlocks/reopen folds
25. generate next residual question
26. decrement fuel
```

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

# 74. Revised conformance suite

The successor requires all predecessor constitutional fixtures plus the following reciprocal-boundary suite.

## 74.1 Determination and departure

1. A source determination presentation is explicit and versioned.
2. Unrelated retained facts are not automatically constitutive.
3. Same-carrier exact cell exclusion has a constitutive separator witness.
4. Exact finite cell exclusion and constitutive separator existence coincide.
5. The 65,536-case finite feature fixture has zero mismatches.
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
45. The three-state joint-recovery witness passes.
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

These are additional later-phase obligations, not new reciprocal roles and not Phase 0
passes:

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

All predecessor typing, compiler, actuality, history, standing, fold, binding, and self-revision fixtures remain required.

---

# 75. Revised build phases

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
```

## Phase 4 — determination, departure, typed negation, and reciprocal return

This phase replaces the predecessor's simple reciprocal-compilation phase.

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

Phase 4 gates are the reciprocal conformance fixtures above.

## Phase 5 — runtime machine

Implement:

```text
ProgramIR
Return
Branch
Probe
verifier
suspension/resume
```

## Phase 6 — persistence and actuality

Implement:

```text
SQLite journal
content-addressed store
ActualEvent
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
HoleIR
FiberIR
ProtectedCompletionFieldIR
```

## Phase 8 — paired actuality

Implement:

```text
question trace
return trace
missing return fiber
missing question reconstruction
event-path provenance
```

## Phase 9 — active views and recurrent memory access

Implement:

```text
ActiveView
reserve
occlusion licences
access routes
activation witnesses
CrawlState
```

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
```

Add reciprocal renderer fixtures ensuring coverage/authority are not silently strengthened.

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

Add `MethodBridgeIR` and one residual-handler/reentry vertical slice compiled through
first-order `IProg`; do not add a method-specific runtime effect or authoritative
suspension table.

## Phase 13 — cue planning

Implement exact finite and working approximate cue planning.

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

## Phase 14 — representation-gap detection

Implement:

```text
MaterializationGap
ExactNoSeparatorWithinRegime
RepresentationGap
FreshWithinRegime
ProposedRegimeExtension
```

## Phase 15 — binding extension and bridges

Implement:

```text
definitional extension
conservative observational extension
rebinding
BindingBridgeIR
targeted reopening
conservative question-language embedding and strict-growth witness
```

## Phase 16 — method learning and folds

Implement:

```text
operator occurrences
method folds
RecoveryContract
CompressionLicence
directional approximation soundness
extension-domain-sensitive method/fold applicability
regenerative economy frontier
optional exact linear consequence-subspace certificate for a live numeric binding
Unlock
Reopen
```

## Phase 17 — conservative cross-binding standing lift

Use the already established safe rule:

```text
transport old standing only at old scope/applicability/grain/horizon
reuse unaffected evidence
open liabilities for new distinctions
recompute before broader promotion
```

## Phase 18 — predecessor-judged self-revision

Implement candidate patches and locked predecessor acceptance.

## Phase 19 — measured breadth only

Only after witnessed need:

```text
learned question policy
fair open-ended generation
vector retrieval
parallel semantic scheduling
PostgreSQL
distributed effects
```

---

# 76. Revised vertical slices

## Slice A — positive-negation reciprocal recurrence

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
-> Resolution
-> ActualEvent
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
raw return preservation
actual event ancestry
use-specific reverse return
protected recovery
residual separator generation
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

# 80. Implementation-agent protocol

Add to `AGENTS.md`:

```text
FOR EVERY CONSEQUENTIAL CHANGE:

1. Reconstruct current accepted semantic state.
2. Read IMPLEMENTATION_FRONTIER.md.
3. Identify the strongest live obligation.
4. State the protected observable consequence.
5. State the smallest executable fixture.
6. Check whether existing typed relations/operators already regenerate it.
7. Preserve the relation/generation/actuality/warrant boundaries.
8. For reciprocal work, identify:
   - source determination presentation
   - departure witness
   - negation-use identity
   - semantic coverage
   - execution coverage
   - return fiber
   - protected recovery
9. Never infer exteriority from failed equality or failed search.
10. Never infer exhaustive negation from a partial sound relation.
11. Never erase NegationUse provenance from an exterior occurrence.
12. Never mutate standing semantics from a merely generated exterior.
13. Prefer generic SeparatorProblem over subsystem-specific residual logic.
14. Preserve raw returns and actual history.
15. Classify failure before changing architecture.
16. Run targeted fixture.
17. Run completed phase gates.
18. Cold-replay semantic/history changes.
19. Remove redundant machinery.
20. Update decisions/failures/frontier.
21. Commit code, fixtures, and evidence together.

NEVER:

- add Boolean complement as a default negation rule;
- treat unknown as negative;
- treat boundary projection as proof of reciprocal exteriority;
- treat one stable return sample as exact return closure;
- use a source relation to construct the return fiber it is supposed to test;
- use a new negation relation to warrant its own exteriority;
- collapse multiple negation uses into an untagged union;
- reinterpret historical events using later negation/frontier knowledge;
- let a candidate patch define its own acceptance criteria.
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
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
migration test
canonical artifact test
type/IR verifier
```

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
source determination presentation
departure witness route
negation use
semantic negation coverage
generator/execution coverage
positive exterior occurrence
raw return/event
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

# 86. Current research gates

## 86.1 Determination-presentation admission

This is now the principal unresolved reciprocal-semantic gate.

The system needs the smallest lawful \(W_D(x)\) that may define the current determination for departure judgment.

Unsafe extremes:

```text
all standing facts about x
    -> incidental differences become false identity criteria

too-small hand-selected web
    -> genuine departures disappear
```

Current safe implementation:

> use the claim-local support/dependency web of the standing determination occupying the source role; retain exact provenance; permit later regenerative minimization only as a reversible compression.

The unresolved comparison is among:

- all standing related constraints;
- minimal regeneratively sufficient webs;
- claim/support-local determination presentations;
- possibly a family of admissible presentations when minima are nonunique.

This does not block implementation of the rest of the successor core.

## 86.2 Cross-binding standing lift

Retain conservative rule until stronger theorem.

## 86.3 Open-ended generator completeness

Do not claim universal finite negative/question basis.

## 86.4 Learned policy

Question/negation-route policy is optimization only.

## 86.5 Retrieval basis

Add vectors only after measured protected miss.

## 86.6 Final post-research integration

The research crawl closed at a local fixed point on 25 August 2026. Its final corpus is preserved
under `research/final-2026-08-25/` and compacted for implementation by
`PROJECT_RESEARCH_IMPLEMENTATION_HANDOFF.md`. It is derived breaker and regression ancestry, not
semantic authority, a competing plan, or a chronology to implement.

Classify its surviving implementation pressure as follows:

| Class | Forward consequence |
|---|---|
| `ALREADY_DEMONSTRATED` | Typed semantic kernel, finite departure and reciprocal slice, immutable artifacts/events, typed request-before-dispatch, exact backend-request checking, and one crash-safe injected provider call remain established by current tests. |
| `CURRENT_FIXTURE` | `RPL-001`--`RPL-005`, `CYCLE-001`, `TRACE-001`, and `PRESENT-001` are demonstrated. The local Ollama `qwen3.5:9b` transport/decode prefix of `PROVIDER-001` is demonstrated. The current fixture completes its typed multi-answer admission, resumption, and zero-redispatch cold replay. |
| `PHASE_LOCAL_CONSTRAINT` | Apply research breakers only at their responsible Phase 6--18 boundary: actuality, resolution, paired provenance, sufficient presents, provider separation, shared standing, transparent separator/method/fold/bridge compositions, conservative lift, and predecessor-judged revision. |
| `FUTURE_OPTIMIZATION` | Learned routing, latent masks, vectors, parallelism, distribution, and other breadth remain Phase 19 experiments requiring measured protected gain. |
| `SEMANTIC_REOPEN_CANDIDATE` | General transition authority and protected-erasure bindings require explicit semantic or binding authority before implementation. |
| `REJECTED_OVERCLAIM` | Research names do not authorize new opcodes, semantic crash attempts, a universal parallel/composition operator, a memory database, an `M001..M066` hierarchy, separate authority/sixfold engines, or an immediate `ic-machine` crate. |

The demonstrated cold-replay sequence is:

```text
completed effect token
-> rechecked BackendRequest / ActualEvent / RawReturn after restart
-> ResolutionPath / FiniteDecoder
-> Decoded | Undefined | Unknown
-> exact Probe observation support and least-fixed-point standing
-> complete AdmittedFiniteAnswerSet
-> reloaded source Ask and capture-safe binding
-> regenerated ProgramIR / ProbeSuspension / ContinuationLowering
-> admitted resumption
-> next Ask or Return
```

No pre-crash derived object or provider redispatch may supply the replay. Regenerate lowering from
accepted source/compiler identities and versions first. A persistable compile/replay recipe is
authorized only if a fresh-process breaker proves that an exact mapping is otherwise unrecoverable.

The current provider sequence is:

```text
checked content-addressed local Ollama request
-> narrow ProbeProvider transport using installed qwen3.5:9b
-> versioned Ollama HTTP status + exact body RawReturn
-> ordinary ActualEvent committed before interpretation
-> schema-response decoder preserving every completion
-> exact independent support routes and standing reconstruction
-> complete supported answer, source binding, and admitted resumption
-> file-backed restart and zero-redispatch regeneration
```

The local transport prefix passes deterministic success/failure tests and one live schema-constrained
probe. The installed model returned two distinct candidates, and `dispatch_probe` committed the raw
frame and event before the decoder inspected either candidate. The OpenAI transport/decoder and
`ic.openai-decoded-text/v1` replay fixtures remain valid secondary evidence; its live HTTP 401 is
recorded as `F-0001` but no longer blocks the active provider sequence. Local candidates now also
regenerate as `ic.ollama-decoded-text/v1` values from only raw return and decoder-version roots.
Post-return observation
support remains separate from pre-dispatch query support, while exact standing closure and
raw-return coverage remain mandatory.

The deterministic `PROVIDER-001D` fixture now takes a committed Ollama-shaped event through
post-return values, typed forms, complete candidates, distinct support, exact standing, binding,
resumption, and file-backed zero-redispatch replay. `PROVIDER-001E` adds the required fresh local
Ollama call and creates none of those semantic artifacts before its raw event is committed.

`PROVIDER-001E` now passes against the installed `qwen3.5:9b`: one fresh dispatch commits the
raw return/event before the entire typed answer/resumption chain is built, and a file-backed restart
replays it with no second provider call. The next phase-local work is `TRACE-002`, a finite
multi-event paired-actuality and sufficient-present/reopening traversal.

`TRACE-002A` is its completed derived-order prerequisite: ledger membership must cover each paired
event exactly once, and causal order remains explicit `Unknown` unless a separate, acyclic declared
candidate edge set is supplied. Ledger append order never generates that candidate edge set or a
second authoritative history.

`TRACE-002` now demonstrates the finite two-event vertical slice. It appends and cold-replays a
second ordinary event, regenerates separate paired source/return traces, retains causal order as
`Unknown`, folds the event identities under a current protected continuation, and exposes an exact
event-sensitive reopen witness. The next Phase 8/9 residual is a derived sufficient-present
update/recovery view over an explicit new ledger event, with no mutable memory or causal store.

That Phase 8/9 update/recovery view now passes. It preserves every old presentation and protected
observation row under a strict history extension, regenerates an updated present when possible,
and returns a positive reopen separator when the appended history splits it. The next executable
step is `SEPARATOR-001`: one admitted/replayed residual must select a typed answer-dependent
continuation using the existing generic separator boundary rather than a method-name switch.

Later phase constraints are deliberately local:

- Phase 8 preserves ledger order versus causal order, resume versus replay, and endpoint versus
  event/path provenance.
- Phase 9 derives question-conditioned sufficient presents without adding a second mutable memory;
  consequence sufficiency, recursive update, recoverability, bounded active size, loss, and standing
  remain distinct.
- Phase 10 keeps question, surface plan, backend request, provider return, decoded answer, and
  standing claim separate; an LLM remains a provider/generator and cannot self-warrant.
- Phase 11 keeps one typed least-fixed-point standing engine; authority admission is an ordinary
  typed standing problem, not a second authority subsystem.
- Phases 12--16 compile transparent `OpenQuery`/`IProg` methods and supported residual-to-method
  relations; method names are provenance, not dispatch, and no research macro becomes an opcode.
- Phase 15 distinguishes inquiry-control composition from binding-native semantic composition; no
  universal `Parallel` is introduced.
- Phase 17 retains the conservative cross-binding lift already stated by this plan.
- Phase 18 remains predecessor-judged and restart/reconstructive until accepted semantic authority
  promotes a broader transition-authority rule.
- Phase 19 alone hosts measured learned routing, vector, parallel, distributed, and latent-mask
  experiments.

---

# 87. Current relative fixed point

The following are now sufficiently settled for implementation.

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

---

# 88. Definition of the first complete successor implementation

The successor reference implementation is complete when one repository can execute and cold-replay:

\[
\boxed{
\begin{aligned}
&\text{typed forms and relations}\\
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
\text{raw actuality and paired history}\\
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
- semantic union that loses negation-use provenance;
- state-changing return without revision evidence.

---

# 89. Remaining implementation order from the current state

The phase definitions in Section 75 remain the single build map. Repository actuality has already
demonstrated the Phase 0 foundation, substantial typed kernels across Phases 1--7 and 11--16, one
finite reciprocal vertical slice, complete finite cold replay, paired actuality, and an exact finite
sufficient-present/reopening witness. Do not replay that chronology merely because the plan retains
its full specification.

Advance the current implementation in this order:

```text
1. TRACE-002
   finite multi-event ordinary history -> paired source/return projections
   -> causal/order separator -> sufficient present -> positive reopening witness

2. Phase 8/9 generalization
   multi-event causal inquiry traces -> derived active/access/reserve views
   -> recurrent fold/reopen with exact provenance and recovery boundaries

3. Phase 11/12 executable standing and separator recurrence
   supported residual -> deterministic transparent inquiry policy
   -> bounded Unknown or next executable question

4. Phases 13--16 regenerative learning
   minimal cue basis -> materialization/representation gap
   -> lawful probe/binding extension -> reusable method/fold with reopening witness

5. Phase 17
   conservative cross-binding standing lift with history locality

6. Phase 18
   replayable predecessor-judged self-revision and cold reconstruction

7. Phase 19
   only measured optimizations that demonstrate protected strict gain

8. completion closure
   run every canonical fixture, complete successor chain, and full cold replay;
   subtract unnecessary architecture and reconcile all state documents
```

At every step, `IMPLEMENTATION_FRONTIER.md` names exactly one strongest executable obligation and
`CONFORMANCE_STATUS.md` marks only demonstrated behavior. The unsettled determination-presentation
minimization rule remains isolated behind explicit provenance/status rather than delaying unrelated
work. `ic-machine`, new authoritative storage, new opcodes, and provider frameworks remain deferred
until an executable breaker establishes their necessity.

---

# 90. Compact successor invariant

The implementation may use the following as the compressed statement of the successor architecture:

\[
\boxed{
\begin{minipage}{0.92\linewidth}
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
history remains authoritative, folds remain reopenable, standing remains
independently warranted, and self-revision remains predecessor-judged.
\end{minipage}
}
\]

---

# 91. Principal continuation

The reciprocal-boundary semantics is now sufficiently specified for implementation except for one authority question:

\[
\boxed{
\textbf{WHAT EXACT RELATIONAL PRESENTATION IS LICENSED TO COUNT AS
THE CURRENT DETERMINATION \(W_D(x)\) FOR DEPARTURE JUDGMENT?}
}
\]

The current safe implementation answer is:

> the explicitly supported claim/dependency web of the standing determination currently occupying the source role, with every constitutive relation traceable to that standing claim; do not absorb all known facts about the source. Treat regenerative minimization as a later reversible compression rather than as a precondition for constructing the first implementation.

The next research pass should test whether the eventual canonical object must be:

1. one minimal regeneratively sufficient web;
2. a family of incomparable sufficient webs;
3. a claim-local support/dependency presentation;
4. a quotient/fold over such a family.

Until that distinction is settled, the implementation can proceed using explicit claim-local determination presentations with full provenance and reopening.

---

# 92. Final implementation directive

Implement the successor from the smallest structure that regenerates its protected behavior:

\[
\boxed{
\textbf{REPRESENT THE LIVE DETERMINATION;
WITNESS DEPARTURE POSITIVELY;
USE COVERAGE-INDEXED TYPED OPPOSITION;
RETAIN THE OPPOSITION ROUTE;
RETURN THROUGH ITS REVERSE SECTION;
MEASURE WHAT THE RETURN ACTUALLY RECOVERS;
REORIENT AND REPEAT;
TURN SURVIVING DIFFERENCE INTO A SEPARATOR;
EXTEND REPRESENTATION ONLY WHEN THE CURRENT LANGUAGE CANNOT EXPRESS IT;
AND NEVER LET GENERATION, HISTORY, OR REVISION CLAIM MORE WARRANT THAN
THEIR EXPLICIT ROUTES PROVIDE.}
}
