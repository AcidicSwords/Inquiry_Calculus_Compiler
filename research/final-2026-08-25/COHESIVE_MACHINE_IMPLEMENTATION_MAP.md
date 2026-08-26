
# Inquiry Calculus — Cohesive Machine Implementation Map

Status: derived implementation guidance, not semantic authority  
Repository actuality: `AcidicSwords/Inquiry_Calculus_Compiler`  
Semantic authority: current Inquiry Calculus v1.1 Revised Again canonical specification  
Implementation authority: repository `IMPLEMENTATION_FRONTIER.md`, implementation plan, conformance fixtures, code, and actual tool/build/test returns

---

## 1. Governing implementation law

The repository should converge to one machine without creating one monolithic semantic object.

The cohesive machine is a **typed orchestration of already distinct canonical relations**:

\[
\boxed{
\text{SEMANTIC QUESTION}
\to
\text{COMPILED PROGRAM}
\to
\text{ACTUAL OCCURRENCE}
\to
\text{PRESERVED RAW RETURN}
\to
\text{RESOLUTION}
\to
\text{ADMISSION}
\to
\text{STANDING}
\to
\text{DETERMINATION / RESIDUAL}
\to
\text{NEXT QUESTION}
\to
\text{COMPRESSION / REOPENING}.
}
\]

The orchestrator owns sequence and transaction boundaries.  
It does **not** own the truth of the relations it composes.

No stage may silently strengthen semantic authority.

---

## 2. Current repository actuality

The repository already contains most of the canonical nouns as executable structural artifacts.

### `ic-core`

Current modules include:

- artifact identity / envelopes;
- types and typed forms;
- formula and relation schema;
- relation expressions and scoped relation uses;
- open queries and completion candidates;
- first-order `IProg`;
- boundaries;
- positive-negation declarations;
- determination presentations;
- departure witnesses;
- exterior views;
- seed/reorientation;
- dependent sixfold views;
- finite separator/cue machinery;
- bridge/factorization/fiber machinery;
- raw returns;
- actual events;
- operator occurrences;
- resolution paths and finite decoders;
- claims;
- support environments and least-fixed-point standing;
- method contracts;
- compression licences and recovery.

### `ic-runtime`

Current runtime is deliberately structural:

\[
\mathsf{Return}
\mid
\mathsf{Branch}
\mid
\mathsf{Probe}.
\]

It verifies typed control flow, rejects unguarded branch-only recurrence, and turns a probe block into a suspension. A separately preserved raw-return identity may resume the program.

It does not yet:

- dispatch a backend;
- append an actuality event;
- execute a decoder;
- choose an answer-dependent continuation from a decoded return;
- perform support admission;
- recompute standing.

### `ic-store`

Current store supplies:

- immutable content-addressed artifact persistence;
- explicit dependency checking;
- one authoritative SQLite writer;
- append-only actual-event ledger;
- stale-parent/fork rejection;
- event/question/boundary/operator/raw-return revalidation;
- restart persistence witness.

It records actuality; it does not interpret it.

### `ic-cli`

Currently has no semantic or orchestration machinery.

---

## 3. The current gap is vertical composition

The repository is not missing a large taxonomy of semantic structures.

The current missing capability is a verified vertical path through already implemented structures.

The strongest live implementation seam is:

\[
\boxed{
\text{ACTUAL DECODED EVIDENCE}
\to
\text{INDEPENDENT SUPPORT ADMISSION}
\to
\text{STANDING}
\to
\text{DETERMINATION RELEVANCE}.
}
\]

Today the pieces on each side already exist:

\[
\begin{aligned}
&\mathsf{ActualEvent}
\to
\mathsf{OperatorOccurrence}
\to
\mathsf{ActualDecodeResult}
\to
\mathsf{DecodedObservationUse},\\[1mm]
&\mathsf{SupportEnvironmentArtifact}
\to
\mathsf{DeclaredSupportClosure}
\to
\mu T
\to
\mathsf{Standing}.
\end{aligned}
\]

But the current standing helper still accepts declared booleans for applicability/check discharge/invalidation. Those declarations are intentionally not independent warrant.

The cohesive machine must replace **declaration-as-runtime-input** with **derived admission from preserved evidence/checker occurrences**, while retaining `DeclaredSupportClosure` as a bounded fixture/reference mechanism where useful.

---

## 4. Canonical end-to-end machine spine

### 4.1 Open

Input:

\[
q=?_I R[\beta].
\]

Repository forms:

- `RelationSchema`
- `RelationUse`
- `OpenQuery`
- `IProgArtifact`

Output:

- checked source inquiry program;
- explicit open ports;
- scope/applicability/grain/horizon/discharge modes.

### 4.2 Differentiate / chart

Construct/select the positive-negation dependent reciprocal chart.

Repository forms already include:

- boundary structures;
- determination presentation;
- negation use;
- tagged exterior;
- seed/reorientation;
- sixfold derived view.

No sixfold runtime species is required.

### 4.3 Active view / fold

Build a question-conditioned active view from retained structure.

Occlude only through a valid `CompressionLicense`.

Required invariant:

\[
\mathsf{Active}
\neq
\mathsf{FoldedButRegenerable}
\neq
\mathsf{Lost}.
\]

### 4.4 Compile

Lower:

\[
\mathsf{IProg}
\to
\mathsf{ProgramIR}
\]

and compile actualizable ports to `ProbeOperator`.

Preserve:

- question identity;
- boundary identity;
- active-view reference;
- probe contract;
- decoder contract;
- backend/code identity;
- compiler version.

### 4.5 Suspend

`ProgramIR::step` reaches:

\[
\mathsf{MachineStep::Suspended}.
\]

At this point nothing actual has happened.

### 4.6 Actualize

A backend adapter performs the actual operation.

This adapter is binding-supplied implementation machinery.

Its output is opaque external bytes or another exact raw carrier.

### 4.7 Preserve before interpretation

Create and persist:

\[
\mathsf{RawReturn}.
\]

Then append:

\[
\mathsf{ActualEvent}
\]

to the authoritative ledger.

This transaction must precede semantic interpretation.

### 4.8 Derive occurrence

Derive:

\[
\mathsf{OperatorOccurrence}
\]

from the actual event.

Occurrence is a view over actuality, never a second history.

### 4.9 Resolve

Apply an admitted resolution path:

\[
\mathsf{RawReturn}
\to
\mathsf{ActualDecodeResult}.
\]

Current finite decoder already preserves:

\[
\mathsf{Decoded}
\neq
\mathsf{Undefined}
\neq
\mathsf{Unknown}.
\]

A decoded candidate remains distinct from relation truth and standing.

### 4.10 Match semantic occurrence

Relate a decoded candidate to the exact declared relation use:

\[
\mathsf{DecodedCandidateSet}
+
\mathsf{RelationUse}
\to
\mathsf{DecodedObservationUse}.
\]

This establishes structural correspondence only.

### 4.11 Admit evidence — current implementation frontier

Introduce a **derived admission service**, not a new semantic primitive.

Working name:

\[
\boxed{
\mathsf{AssessSupportDischarge}.
}
\]

It consumes already canonical artifacts and actual occurrences and returns a typed assessment.

It must independently establish, where required:

1. the claimed discharge route actually occurred;
2. applicability is standing for the occurrence context;
3. referenced checkers actually discharged and their results are preserved;
4. the support environment still has no unresolved protected dependency;
5. no explicit invalidation/nogood defeats the route;
6. the evidence path is non-self-licensing;
7. semantic coverage and execution coverage remain separate;
8. `Unknown`, failed materialization, and absent coverage remain non-positive.

Suggested derived result:

```text
SupportDischargeAssessment =
    Closed {
        environment_ref,
        evidence_refs[],
        checker_result_refs[],
        applicability_evidence_ref,
        occurrence_refs[],
        coverage_refs[],
    }
  | Open {
        environment_ref,
        dependencies[],
    }
  | Inapplicable { ... }
  | Invalidated { ... }
  | CoverageGap { ... }
  | CheckFailed { ... }
  | Unknown { ... }
```

This result need not be an authoritative artifact immediately. Start as a pure checked derived value if that is sufficient for the first fixture.

The decisive rule is:

\[
\boxed{
\text{a boolean supplied by the caller is not evidence that the
corresponding discharge happened.}
}
\]

### 4.12 Compute standing

Translate only independently discharged support routes into the existing standing problem.

Then compute:

\[
\mathsf{Stand}=\mu T.
\]

The least-fixed-point engine remains the authority for support closure from admitted roots/routes.

No rootless positive cycle enters standing.

### 4.13 Establish determination relevance

A standing claim is not automatically a determination of a particular typed source.

Require an explicit standing interpretation/relevance relation connecting:

\[
\mathsf{Claim}
\leftrightarrow
\mathsf{DeterminationPresentation}
\]

through the claim-local relational web.

This should preferably reuse an ordinary `RelationUse` / support route rather than add an opaque `claim_denotes_form` boolean.

The current structural determination-support link proves identity/context and standing membership; the next semantic seam must establish the missing relevance relation without self-warrant.

### 4.14 Positive departure / reciprocal inquiry

Only after source determination and positive evidence stand may:

\[
\mathsf{DepartureWitness}
\]

be admitted for a negation use.

Then continue through:

\[
S_X
\to
O_X
\to
R_X
\to
S_Y
\to
O_Y
\to
R_Y
\]

while preserving use identity, semantic/execution coverage, and return fibers.

### 4.15 Produce residual

Every unsuccessful stage returns a typed residual rather than a generic false/null.

Examples:

- representation gap;
- materialization gap;
- applicability gap;
- execution coverage gap;
- decoder unknown;
- support dependency open;
- checker failure;
- warrant gap;
- relevance gap;
- recovery gap;
- binding gap.

### 4.16 Select next method relationally

The current `MethodContract` tuple is a good canonical implementation encoding.

Semantically, treat it as a projection of a method relational web.

Selection is:

\[
\Delta
\to
?M,\beta[
\mathsf{Addresses}(M,\beta(\Delta))
\land
\mathsf{Applicable}(M)
].
\]

Do not schedule by method name.

Fields such as relation/applicability/law/coverage/authority/backend/checker/cost/failure/provenance remain protected role identities.

### 4.17 Recenter / next question

Use answer-dependent continuation to construct the next question from the actual resolved/admitted return.

Do not continue from stale pre-return state unless independence is proved.

This is the point where source `IProg::Ask` and runtime `ProgramIR` must eventually meet end-to-end.

### 4.18 Compress / reopen

After standing/reconciliation:

1. derive a candidate active presentation;
2. validate any `CompressionLicense`;
3. retain recovery/residual/unlock obligations;
4. write accepted patch/version ancestry;
5. regenerate the current active view/frontier.

A compression licence record is not permission by itself; the machine must evaluate the named preservation/recovery/unlock obligations.

---

## 5. One authoritative-owner rule

The machine should not create another database-shaped "machine state" that independently owns semantic truth.

Authoritative owners:

- raw actuality → immutable artifacts + actual-event ledger;
- accepted semantic revisions → accepted patch/version ancestry when implemented;
- canonical schemas/programs/contracts → immutable content-addressed artifacts.

Derived views:

- operator occurrences;
- decoded result views;
- standing result;
- active question-conditioned view;
- sixfold occurrence projection;
- method applicability frontier;
- compression/reopening frontier;
- sufficient-present/checkpoint.

Therefore:

\[
\boxed{
\text{ONE AUTHORITATIVE OWNER PER SEMANTIC FACT;
EVERY OTHER REPRESENTATION IS A REGENERABLE VIEW OR REFERENCE.}
}
\]

---

## 6. Crate-level cohesion

Current dependency structure:

```text
ic-core
  ↑      ↑
runtime  store

ic-cli (currently empty)
```

Do not make `ic-core` depend on runtime/store.

Two viable implementation arrangements:

### Preferred while the machine is still small

Add an orchestration/application crate later, e.g.:

```text
ic-machine
    depends on:
        ic-core
        ic-runtime
        ic-store
```

It owns:
- backend adapter invocation;
- event transaction choreography;
- decoder/checker runner calls;
- standing/reconciliation loop;
- cold replay/materialization;
- next-question execution.

It owns **no new semantics**.

`ic-cli` then becomes an interface to `ic-machine`.

### Acceptable smaller first ratchet

Before adding a crate, keep the next admission boundary as pure `ic-core` derived functions and fixtures.

Only create `ic-machine` when the first real end-to-end actualization transaction requires runtime + store simultaneously.

This follows "smallest reversible structure first."

---

## 7. Implementation order from the current frontier

### Ratchet A — evidence route actuality

Fixture:

A relation use declares `Probe`, but no matching actual event/operator occurrence exists.

Expected:

\[
\boxed{
\text{must not close support}.
}
\]

Then add a matching actual occurrence and decoded-observation link.

Expected:

route-actuality obligation may be discharged, but standing still requires the remaining checks.

### Ratchet B — checker actuality

Replace caller boolean `checks_discharged=true` in one vertical fixture with preserved checker-result evidence.

Expected:

a checker declaration without an actual/check-authorized result cannot close the route.

### Ratchet C — applicability admission

Resolve applicability through an explicit standing/checked relation rather than caller boolean.

Expected:

applicability failure deactivates the route without erasing historical warrant.

### Ratchet D — determination relevance

Construct two standing claims with the same scope/applicability but only one explicitly linked to the source determination web.

Expected:

context equality alone cannot make the unrelated claim establish the determination.

### Ratchet E — non-circular departure

Create a candidate departure whose only positive incompatibility/support route depends on the departure/negation claim it is meant to license.

Expected:

least-fixed-point admission refuses it.

Then add independent ingress/check support.

Expected:

the route may close.

### Ratchet F — first complete vertical reciprocal slice

Execute:

```text
Standing source determination
→ positive-negation OpenQuery
→ compile/suspend Probe
→ actual raw return
→ append ActualEvent
→ derive OperatorOccurrence
→ decode
→ match observation use
→ independently admit support
→ recompute Standing
→ admit DepartureWitness
→ TaggedExteriorClaim
→ SeedReorientation
→ reciprocal next question
```

No step may be faked by construction of a downstream artifact.

### Ratchet G — residual-driven method route

Take one failed admission outcome, e.g. `CoverageGap`.

Use the typed residual to discover a compatible `MethodContract`.

Prove that method selection occurs from the residual relation rather than a hard-coded failure-to-method switch.

### Ratchet H — first compression/reopening vertical slice

After a stable repeated subpath:

1. build a `CompressionLicense`;
2. independently validate its exact/approximate obligations;
3. construct active folded view;
4. introduce a new question whose protected continuation breaks the fold;
5. detect unlock;
6. recover;
7. refine;
8. regenerate the same standing/relevant next inquiry as an unfurled execution.

### Ratchet I — cold replay

Close the process.

From a fresh process with only canonical artifacts, event history, accepted patches/versions, compiler/binding versions, and declared nondeterministic choices:

\[
\boxed{
\mathsf{ColdReplay}
\to
\text{same protected standing/frontier/next-question behavior}.
}
\]

This is the first strong implementation witness that the sufficient-present architecture is real rather than a conversational description.

---

## 8. Research network → repository mapping

The domain crawl should no longer create core Rust types merely because a useful method was discovered.

Instead:

| Research result | Repository role |
|---|---|
| `ResolveOpenWeb` | compiler/orchestrator macro over existing query/separator/residual artifacts |
| `RefineFoldByQuestion` | fold/recovery service + regression fixture |
| `ProtectedCoherenceFactorization` | generalized bridge/factorization validation law; higher witnesses remain reified forms |
| `TwoSpeedIncorporation` | ingestion/promotion strategy over immutable ancestry + accepted semantic revisions |
| `ProtectedAbstractionTower` | compiler/dialect conformance fixtures, not a new runtime species |
| `CoupledRepresentationRefinement` | method-network/index rebuild strategy |
| `FrameProbeCoDesign` | representation-gap → probe-basis extension loop |
| `RevisionBySupportDelta` | standing/support recomputation + successor patch macro |
| `ProofCarryingFoldSearch` | compression-validation strategy |
| `CapabilityPreorder` | derived method-index/view over implementation/bridge relations |
| `EvidenceShape` | method-applicability metadata/derived relation |
| `RegenerativeInformationState` | checkpoint/replay conformance target |

Research-derived structures become one of:
- fixture;
- checker;
- derived view;
- compiler macro;
- method contract;
- adapter;
- residual;
- future optimization.

They become a new semantic artifact kind only if existing composition provably cannot regenerate a protected behavior.

---

## 9. The cohesive machine as one recurrence

The whole implementation can be compressed to:

\[
\boxed{
\begin{aligned}
X_t
&\xrightarrow{\mathsf{Open/Compile}}
P_t\\
&\xrightarrow{\mathsf{Run}}
\mathsf{Suspension}\\
&\xrightarrow{\mathsf{Actualize}}
r_t\\
&\xrightarrow{\mathsf{Preserve}}
E_t\\
&\xrightarrow{\mathsf{Resolve}}
A_t\\
&\xrightarrow{\mathsf{Admit}}
W_t\\
&\xrightarrow{\mu T}
\Stand_t\\
&\xrightarrow{\mathsf{Reconcile}}
\Delta_t\\
&\xrightarrow{\mathsf{Ask}}
P_{t+1}\\
&\xrightarrow{\mathsf{Fold/Reopen}}
X_{t+1}.
\end{aligned}
}
\]

Where:
- \(r_t\) is raw actuality, not interpretation;
- \(A_t\) is decoded/resolved possibility, not standing;
- \(W_t\) is admitted support structure;
- \(\Stand_t\) is least-fixed-point current standing;
- \(\Delta_t\) is the live residual;
- \(X_{t+1}\) is a derived successor presentation over authoritative ancestry.

This is one machine because every transition is linked.

It is not one object because every boundary remains semantically distinct.

---

## 10. Current implementation center

The next executable implementation center remains the repository's own frontier:

\[
\boxed{
\text{smallest claim-targeted admission boundary connecting
actual evidence to standing determination relevance and non-circularity}.
}
\]

Do not jump ahead to:
- general backend dispatch;
- LLM prompt engines;
- autonomous method schedulers;
- aggressive compression;
- domain dialect proliferation;
- full cold replay

until this seam can be exercised through a discriminating fixture.

Once this seam is real, the repository can begin closing the first true vertical loop.

---

## 11. Definition of progress

A new module/type is not progress by itself.

A repository ratchet is progress when it shortens the unimplemented path between:

\[
\boxed{
\text{OPEN QUESTION}
\quad\text{and}\quad
\text{NEXT WARRANTED QUESTION}
}
\]

while preserving all currently demonstrated separations.

The implementation target is therefore:

\[
\boxed{
\textbf{THE SMALLEST COLD-REPLAYABLE MACHINE IN WHICH
EVERY CONSEQUENTIAL TRANSITION IS A CHECKED COMPOSITION
OF CANONICAL RELATIONS AND EVERY LOSSY FOLD CAN EXPLAIN
HOW AND WHEN IT REOPENS.}
}


---

# 145. AdmissionKernel — smallest vertical reconciliation boundary

The immediately executed next question was:

\[
\boxed{
\text{What is the smallest boundary that can replace caller-declared
support closure without duplicating the standing engine?}
}
\]

## 145.1 Maximal candidate rejected

Do **not** create a second "admitted standing" engine that:
- evaluates support graphs;
- decides fixed points;
- owns claim truth;
- duplicates `Standing`.

That would create two authorities.

## 145.2 Smaller surviving boundary

Define an implementation-only service:

\[
\boxed{
\mathsf{AdmissionKernel}
}
\]

whose sole semantic output is a checked set of **admissible inputs** to the existing least-fixed-point standing computation.

It has two products:

\[
\boxed{
\mathsf{AdmittedIngress}
}
\]

and:

\[
\boxed{
\mathsf{AdmittedSupportRoute}.
}
\]

Then:

\[
\boxed{
\mathsf{AdmissionKernel}
\to
\mathsf{StandingProblem}
\to
\mu T.
}
\]

`Standing` remains unchanged as the fixed-point evaluator.

## 145.3 Admitted ingress

A claim may enter ingress only through a binding-authorized independently grounded route, such as a properly represented:
- preserved actual-return fact plus admitted interpretation/check;
- trusted configuration fact;
- accepted predecessor relation/patch;
- checker axiom/certificate whose authority is independently declared by the binding.

Suggested derived record:

```text
AdmittedIngress {
    claim_ref,
    authority_route,
    evidence_refs[],
    scope,
    applicability,
}
```

The first vertical implementation should make this a checked derived value rather than a new stored artifact unless canonical identity becomes consequential.

A raw return alone is not ingress.

A claim declaring itself `Standing` is not ingress.

## 145.4 Admitted support route

For canonical `SupportEnvironmentArtifact E`, derive:

```text
AdmittedSupportRoute {
    environment_ref,
    target_claim,
    standing_premises[],
    discharge_evidence[],
    applicability_evidence,
    checker_result_evidence[],
    invalidation_assessment,
    execution_coverage,
}
```

Only after each role is independently discharged can the route be translated to the existing `SupportEnvironment` used by `StandingProblem`.

Open dependency references remain open; they are not changed to `false` by omission.

## 145.5 Route actuality

For evidence whose declared `DischargeMode` is `Probe`, require a preserved actual chain:

\[
\boxed{
\mathsf{ActualEvent}
\to
\mathsf{OperatorOccurrence}
\to
\mathsf{ActualDecodeResult}
\to
\mathsf{DecodedObservationUse}
\to
\mathsf{RelationUse}.
}
\]

A declared route tag is only a requirement until that chain exists.

For `Pure`, require a derivation from already admitted/standing inputs under an admitted pure relation/checker.

For `Check`, require an actual checked result/certificate.

For `Generate`, positive epistemic discharge is prohibited unless a separate independent route validates the generated candidate.

For `Warrant`, use the binding's standing/warrant route rather than treating the mode label as warrant.

## 145.6 Non-circularity without a second cycle checker

Do not add a generic "acyclic support graph" rule.

Instead:

1. independently validate ingress roots;
2. make every derived support dependency an explicit standing premise/open dependency;
3. pass only those routes to \(\mu T\).

Then the existing least fixed point enforces:

\[
\boxed{
\text{rootless positive support recurrence does not self-discharge}.
}
\]

This is smaller and more exact than a blanket DAG requirement.

A separate cycle analysis is needed only for a binding-specific proof/checker semantics whose cycles have their own soundness law.

## 145.7 Determination relevance remains a separate gate

Even after:

\[
\mathsf{Claim}\in\Stand,
\]

the machine must still establish:

\[
\boxed{
\mathsf{RelevantTo}
(\mathsf{Claim},\mathsf{DeterminationPresentation})
}
\]

through an explicit standing relation/interpretation contract.

Do not put `relevant=true` into `AdmissionKernel`.

Therefore:

\[
\boxed{
\mathsf{Admission}
\neq
\mathsf{DeterminationInterpretation}.
}
\]

## 145.8 Minimal cohesive composition

The first semantically meaningful full pipeline becomes:

\[
\boxed{
\begin{aligned}
&\mathsf{ActualOccurrence}\\
&\to \mathsf{Resolution}\\
&\to \mathsf{AdmissionKernel}\\
&\to \mathsf{StandingProblem}\\
&\to \mu T\\
&\to \mathsf{DeterminationRelevance}\\
&\to \mathsf{Departure/PositiveNegation}.
\end{aligned}
}
\]

This is the smallest currently justified bridge between the repository's implemented actuality half and its implemented standing/reciprocal half.

## 145.9 First implementation fixture

The strongest smallest fixture is:

`DECLARED_PROBE_ROUTE_CANNOT_CLOSE_SUPPORT_WITHOUT_OCCURRENCE`

Construct:
- one structurally valid claim;
- one structurally valid support environment;
- one relation use declaring `Probe`;
- a `DeclaredSupportClosure` that would currently be able to mark checks/applicability as true;
- **no** actual event/occurrence/decoded observation corresponding to the route.

The new `AdmissionKernel` must return a typed open/unknown actuality residual and refuse to emit an `AdmittedSupportRoute`.

Then add:
- matching `ActualEvent`;
- derived `OperatorOccurrence`;
- matching finite decoder/path result;
- matching `DecodedObservationUse`.

Now route actuality may discharge, while checker/applicability/standing relevance remain independently unresolved.

This fixture advances implementation without pretending the entire support/warrant problem is already solved.


---

# 16. Self-modifying cohesive machine

The cohesive machine should eventually support self-revision by constructing a successor machine as another object of inquiry.

Do not mutate the currently accepted executable before its successor is admitted.

Preferred architecture:

\[
\boxed{
\begin{aligned}
V_t
&\to \mathsf{ReifyImplementation}\\
&\to \mathsf{Residual}\\
&\to \mathsf{CandidatePatch}\\
&\to \mathsf{SeparateBuild}\\
&\to \mathsf{PredecessorValidation}\\
&\to \mathsf{IndependentValidation}\\
&\to \mathsf{AcceptedVersion}_{t+1}\\
&\to \mathsf{Restart/Reconstruct}\\
&\to \mathsf{ProtectedRegression}.
\end{aligned}
}
\]

This design exploits the repository's immutable artifact/event architecture.

## 16.1 Cold replay limitation

Add the protected distinction:

\[
\boxed{
\mathsf{ColdReplayEquality}
\neq
\mathsf{CompilerOrSemanticIntegrity}.
}
\]

If the candidate changes the compiler/checker used by replay, same-path replay cannot be the only warrant.

Use a preserved predecessor or diverse/verified checking route when that integrity is protected.

## 16.2 Judge migration

When a patch changes the acceptance/checking/protection relation itself, route through:

\[
\boxed{
\mathsf{GovernedJudgeMigration}.
}
\]

A moving root may be lawful without an immutable meta-root when adjacent versions have an independently authorized overlap/bridge.

If overlap authority is lost:

\[
\boxed{
\mathsf{TransitionAuthorityGap}
}
\]

and internal self-revision must stop pending an external/bootstrap route.

## 16.3 Hot update boundary

Do not implement live in-process self-replacement unless availability becomes protected.

If needed, require:
- safe update point;
- state transformer;
- old/new semantic bridge;
- transition theorem;
- rollback/recovery.

Otherwise, accepted successor + restart/reconstruct is the smaller machine.


---

# 17. Occurrence-local validation reduces the trusted self-modification surface

When a generator/compiler can emit an independently checkable certificate for each produced artifact, the cohesive machine need not globally trust the producer.

Preferred boundary:

\[
\boxed{
\mathsf{CandidateProducerOccurrence}
\to
\mathsf{CandidateArtifact}
\to
\mathsf{IndependentValidation}
\to
\mathsf{Admission}.
}
\]

This should eventually be used for any implementation layer where a practical independent validator exists.

Do not infer global compiler correctness from one validated occurrence.

Do not infer occurrence validity merely from a globally trusted compiler when the occurrence itself remains unexamined and that distinction is protected.

If the validator changes, treat the validator as the object of the next revision and invoke `GovernedJudgeMigration`.


---

# 18. Self-stabilizing reconstruction and generalized transition authority

## 18.1 Transition authority is not predecessor identity

Generalize all judge/protection upgrades to consume:

```text
TransitionAuthorityRoute
```

rather than assuming the route is always `PredecessorAuthority`.

Accepted bindings may include:
- predecessor overlap;
- stable meta-checker;
- diverse validator;
- accepted institutional/external authority;
- explicit bootstrap/recovery root.

The candidate itself cannot be the sole source of the route.

## 18.2 Derived-state self-stabilization

Make the eventual machine resilient to arbitrary corruption/loss of **derived** state.

Authoritative inputs:
- immutable artifacts;
- event ledger;
- accepted patch/version chain;
- versioned binding/checker/compiler identities;
- persisted nondeterministic choices.

Rebuildable:
- standing cache;
- active/folded view;
- residual frontier;
- method indexes;
- compiled caches.

Conformance target:

```text
destroy derived state
→ fresh process
→ rebuild from authoritative roots
→ same protected standing/frontier/next-question behavior
```

This should be a stronger variant of cold replay.

## 18.3 Authority-root breaker

If authoritative history/checker/version trust is damaged, do not apply the same self-stabilization claim.

Return an explicit:

```text
AuthorityRecoveryGap
AncestryGap
```

until an independently authorized bootstrap route is supplied.

## 18.4 Generator-patch second-order regression

When a self-modification changes the candidate/question/method generator, include future-distribution consequences in regression if future self-correction is protected.

Immediate performance equality/improvement is insufficient.


---

# 19. Distributed authority realization

Do not encode accepted self-revision authority as a single root/key field.

Separate:

```text
AuthorityRelation
AuthorityBasis
CredentialRealization
TransitionBridge
```

The ordinary standing/admission machinery determines whether a concrete authority relation stands.

The binding defines what that authority means.

## 19.1 Authority support families

Support multiple minimal support environments for one transition-authority claim.

Quorum/threshold bindings may impose an additional coherence/intersection theorem before local support routes imply a unique global authorization.

## 19.2 Credential lifecycle

Allow:
- share refresh;
- holder rotation;
- credential rotation;
- dealerless generation

without conflating any of them with semantic authority migration.

A fresh credential is not authoritative until a standing authority relation binds it to the protected role.

## 19.3 New self-modification fixtures

Add future fixtures SM-014 through SM-019:

- quorum authority without one root object;
- incoherent locally closed quorums fail global authorization;
- below-threshold credential recovery gap;
- DKG key generation without authority constitution;
- joint configuration transition;
- credential refresh preserving authority semantics.

## 19.4 Implementation compression

`TransitionAuthorityRoute` should eventually be implemented as ordinary claim admission/standing for an authority-typed relation plus binding-native composition/bridge checks.

Do not build a separate authority-consensus engine into core semantics.


---

# 20. Delegation, support provenance, and versioned representation migration

## 20.1 AdmissionKernel target generalization

The repository's newest standing implementation handles:

```text
SupportSubjectRef::Claim
SupportSubjectRef::Relation
```

in one typed fixed point.

Therefore the future admission service must be generic:

```text
AdmittedSupportRoute {
    subject: SupportSubjectRef,
    environment_ref,
    evidence,
    applicability_evidence,
    checker_evidence,
    invalidation_state,
    coverage,
}
```

Do not build a claim-only admission path.

## 20.2 Delegated authority fixtures

Future conformance fixtures:

```text
AUTH-DEL-001 exact path support
AUTH-DEL-002 route-specific revocation
AUTH-DEL-003 exercise vs redelegation
AUTH-DEL-004 attenuation cannot widen scope
AUTH-DEL-005 equal endpoint permission, distinct revocation provenance
```

A delegated authority relation is simply a relation subject whose standing depends on its exact support path and binding-native authority composition law.

## 20.3 Support provenance optimization

Explicit canonical `SupportEnvironmentArtifact` records should remain the initial authority.

A later derived `SupportProvenanceCircuit` may compress many alternative/joint routes if exact round-trip and invalidation behavior are demonstrated.

Use it first as:
- cache;
- explanation index;
- dependency invalidation index;
- incremental standing aid.

Do not make it canonical until it regenerates the protected support-environment family.

## 20.4 Artifact/schema self-migration

Never rewrite old canonical artifacts/events in place.

Preferred pattern:

```text
old versioned artifact
→ migration occurrence + bridge version
→ new versioned artifact
```

Retain old artifact identity and migration provenance.

If a migration loses protected information, require:
- retained complement/provenance; or
- explicit `LossyMigrationGap`.

Future fixtures:

```text
MIG-001 dropped protected field requires complement
MIG-002 old-view compatibility != exact physical rollback
MIG-003 semantic change escalates patch role
MIG-004 migration-chain round-trip
MIG-005 generated migration requires occurrence-local validation
```

## 20.5 Implementation learning rule

Current repository standing provenance is already exact per closing environment.

Do not flatten it to a bare `standing=true` cache.

If a cache is added, it must retain or regenerate the exact support-route provenance needed for:
- revocation;
- invalidation;
- explanation;
- reopening;
- authority/path-sensitive self-revision.


---

# 21. Control occurrence, causal order, and active-liveness integration

## 21.1 Runtime continuation boundary

Do not identify source continuation/program data with a captured runtime continuation occurrence.

If richer control is implemented later, preserve:

```text
ContinuationProgramRef
ContinuationOccurrenceRef
CapturedState/SnapshotRef
ContinuationUsePolicy   // one-shot, multi-shot, binding-specific
```

only if an executable fixture requires runtime continuation occurrences.

Prefer compiling multi-shot/effect-handler semantics into existing first-order `ProgramIR`/branching where a validated CPS/explicit-state lowering suffices.

## 21.2 Journal order is storage order

The append-only event ledger may totally serialize records for integrity/replay.

Do not infer semantic causality from the parent/index chain unless the binding explicitly establishes that law.

Future concurrent-event support should carry explicit causal/dependency references or an admitted causal projection.

Suggested fixture:

```text
CAUSAL-001:
journal e1 < e2
but no native dependency relation
=> no inferred Causes/Precedes relation
```

## 21.3 Sufficient present for concurrent bindings

A current view may need to be a causally consistent region/cut rather than a simple journal prefix.

Keep the storage ledger linear if operationally convenient.

Represent native partial order in typed relations.

## 21.4 Active liveness engine

Add a future derived active-view service:

```text
ProtectedLivenessClosure {
    roots,
    dependency_rules,
    conditional_rules,
}
```

Outputs only a derived active set.

Authoritative artifacts/events remain unaffected.

Root classes may include:
- current query/program;
- live residual;
- protected continuation;
- required support route;
- current authority;
- recovery/unlock pointer;
- current regression breaker.

Do not equate active with standing.

## 21.5 Ephemeron-style derived caches

Caches/indexes should be discardable when the authoritative inputs they depend on are no longer active.

Examples:
- decoder result cache;
- factorization result cache;
- method capability index;
- support provenance circuit;
- compiled active view.

This reduces active representation without deleting source ancestry.

## 21.6 LeastClosureEvaluator

The current standing engine suggests a reusable implementation kernel for finite monotone closure problems.

Do not generalize by erasing types.

A future internal utility may compute:

```text
least_closure(seed, rule_step)
```

for different typed wrappers:
- Standing;
- causal predecessor closure;
- active liveness.

Each caller must supply its own validation and semantic rule contract.

Same evaluator implementation is not semantic identity.


---

# 22. Nonmonotonic native semantics and approximation-oriented verification

## 22.1 Keep core standing positive

Do not extend the repository's `Standing` fixed point into a universal default/stable-model engine.

For nonmonotonic bindings, represent:

```text
NativeTheoryRef
CandidateExtensionRef
IsExtension relation/checker
Extension-local membership relation
Coverage/counterextension certificate
```

Native solvers generate candidate extensions/models.

Independent checkers admit extension-validity relations.

Core standing tracks those positive checked relations.

## 22.2 Global consequence modes

Provide derived query modes only when needed:

```text
Credulous   // exists checked extension
Skeptical  // all checked/admissible extensions, requiring complete coverage
```

Never infer skeptical consequence from solver timeout or incomplete enumeration.

## 22.3 Assumption-indexed standing view

A future index may map a subject to its minimal admitted support/assumption environments and nogoods.

This can accelerate hypothetical/context reasoning without changing canonical standing semantics.

## 22.4 CounterextensionSearch

Generalize breaker execution to typed outcomes:

```text
Witness
ExhaustivelyEmpty
Unknown
Blocked
ResourceBounded
```

This method can serve:
- skeptical consequence;
- contract attack;
- invariant breaker search;
- compression breaker search;
- method regression.

## 22.5 CEGAR integration

A future verification/refinement service may carry:

```text
ConcreteDomainRef
AbstractDomainRef
AbstractionBridgeRef
Concretization/FeasibilityChecker
ProtectedPropertyRef
PrecisionRef
```

If an abstract breaker cannot be concretized:
emit a representation/precision residual and refine.

Do not admit the breaker semantically.

## 22.6 Directional approximation metadata

Approximate compression/analysis needs a typed orientation:

```text
Exact
OverApprox
UnderApprox
```

plus:
- protected query family;
- soundness theorem/checker;
- coverage;
- residual/distortion.

Do not use an under-approximation to prove universal absence.
Do not use an over-approximation's raw witness as a concrete breaker without feasibility.

## 22.7 Bidirectional approximation engine

For suitable bindings maintain:

```text
LowerApproxRef
ConcreteContractRef
UpperApproxRef
```

with:

```text
Lower ⊆ Concrete ⊆ Upper
```

Decisive exits:
- lower-side breaker;
- upper-side proof.

Refinement exits:
- spurious upper breaker -> refine upper;
- lower search failure -> widen lower.

This is a derived verification strategy, not semantic authority.

## 22.8 Repository fixtures

Add future conformance fixtures:

```text
NM-001 multiple checked native extensions
NM-002 skeptical consequence requires exhaustive/certified counterextension absence
NM-003 extension generator != extension checker

CEGAR-001 spurious abstract breaker refines representation
CEGAR-002 feasible breaker is admitted

APPROX-001 under-approx failure remains unknown
APPROX-002 sound over-approx absence proves universal property
APPROX-003 exact/over/under folds cannot be interchanged
```

## 22.9 Self-application to repository regression

Treat the finite fixture suite as an under-approximation of possible implementation breakers.

Passing all current fixtures is not global correctness.

A stronger claim requires:
- a completeness theorem over the protected implementation contract; or
- a sound upper abstraction with no breaker.

This should remain explicit in release/conformance metadata.


---

# 23. State-transforming probes and minimal sufficient-present quotients

## 23.1 ActualEvent already has the correct transition spine

Current repository actuality already records:

```text
state_before
operator
raw_return
state_after
```

along with question/boundary/grain/route/binding/backend/provenance.

Do not add a second "measurement effect event."

When a binding protects the state transition semantics, admit/check an ordinary
relation such as:

```text
ProbeEffect(operator, state_before, raw_return, state_after)
```

before downstream use.

Structural event identity and semantic state-transition validity remain separate.

## 23.2 Alternative program branches are not automatically forkable actuality

The runtime/compiler may represent several alternative probe branches.

A binding must separately state whether the probed state/resource may be:
- cloned;
- reset;
- independently reprepared;
- consumed;
- measured only once;
- sequentially reused.

Do not let source IProg branching imply physical/runtime copyability.

## 23.3 Context-local results

Permit context-indexed standing results when a binding does not admit global
joint assignment.

A global completion should be an explicit open question/check, not a default merge.

Potential derived residual:

```text
GlobalizationObstruction
```

## 23.4 Protected discriminator quotient

The active-view compressor should eventually use the exact relation:

```text
h1 ~ h2
iff
every currently protected continuation gives the same protected observation
```

rather than comparing all stored fields.

This is the precise mathematical target for active-context compression.

## 23.5 ContinuationQuotientMinimize

Potential implementation service:

```text
ContinuationQuotientMinimize {
    candidate_states,
    protected_continuations,
    successor_relations,
    comparison_checker,
    coverage,
}
```

Algorithmically:
- identify candidate-equal states;
- find a protected continuation that separates them;
- split;
- repeat to stable congruence;
- emit quotient + separator provenance + reopening contract.

Use finite DFA-style partition refinement where applicable.

Use CEGAR/directional approximations when exact quotient construction is
infeasible.

## 23.6 Exact bounded-state criterion

Do not promise a constant-size finite sufficient present universally.

For finite-state implementations, exact bounded compression requires finite index of
the protected continuation equivalence.

If index is infinite:
- retain more state;
- use symbolic representation;
- narrow horizon/grain;
- or use explicit approximation.

## 23.7 New fixtures

```text
PROBE-001 same immediate outcome, different post-state behavior
PROBE-002 state-before/after identity != semantic transition proof
PROBE-003 represented branching != forkable actuality

CTX-001 local standings with no global section

INFO-001 same current observation != same decision-sufficient information state
INFO-002 hidden distinction cannot drive observation-based action

QUOT-001 new continuation reopens quotient
QUOT-002 finite continuation quotient regenerates minimal protected machine
QUOT-003 infinite-index witness blocks finite-state sufficiency claim
```


---

# 24. Protected simulation order and open-ended learning

## 24.1 CompressionLicense already has the right structural boundary

Current `CompressionLicense` retains:

```text
folded
kind = Exact | Approximate { distortion_contract }
horizon
protected continuations
scope
evidence
residual
recovery
unlock conditions
```

This is sufficient structurally for the new Blackwell/Le-Cam/continuation
factorization results.

Do not add another compression artifact kind.

## 24.2 Derived ProtectedSimulation relation

Future check/evaluation code may represent an ordinary relation:

```text
ProtectedSimulation {
    source_presentation,
    target_presentation,
    protected_continuation_family,
    horizon,
    direction,
    native_comparison_contract,
    evidence,
}
```

This is not canonical ontology.

It is a checked relation useful for:
- fold licensing;
- cross-version migration;
- bridge checking;
- predictive-state minimization;
- approximate decision preservation.

## 24.3 Future continuation expansion is an unlock

A new protected continuation must be treated as a possible unlock on every fold
whose licence did not cover it.

Algorithm:

```text
new continuation K*
→ find relevant active/folded presentations
→ test factorization through current quotient
→ if factorization holds, keep fold
→ else recover smallest supported ancestry
→ split/refine
→ rerun old protected continuations
→ refold
```

If no recovery/reacquisition route survives:

```text
FutureTaskRecoveryGap
```

## 24.4 Two-level memory architecture

Keep the implementation separation:

```text
authoritative / recoverable ancestry
           ↕
question-conditioned replay/recovery
           ↕
economical active semantic network
```

This has strong computational and cognitive precedent.

Do not identify replayed/reconstructed records with original actual events.

## 24.5 Generative replay policy

If a future backend generates synthetic historical samples:

```text
GeneratedReplayOccurrence
```

must remain distinct from:

```text
HistoricalActualEvent
```

unless an independently checked exact bridge is available.

Use approximate recovery semantics otherwise.

## 24.6 Developmental/accessibility implication

A representation may support current execution while preventing later explanation,
inspection, strategic modification, or cross-domain use.

Therefore protected continuation families must include metalevel uses when those
uses are part of the machine's self-revision requirements.

Do not compress solely on current external task performance.

## 24.7 New fixtures

```text
COMP-DEC-001 current-task equality != universal decision sufficiency
COMP-DEC-002 one-way protected simulation != exact equivalence
COMP-DEC-003 approximation contract is query-family-relative

PSR-001 stochastic future-test separator

LEARN-001 new protected continuation reopens old quotient
LEARN-002 unrecoverable new distinction => FutureTaskRecoveryGap

MEM-001 semantic fold survives only while episode recovery route remains
MEM-002 generated replay != original actuality

DEV-001 same task behavior, different metarepresentational accessibility
```

## 24.8 Release/conformance implication

A release should not claim:

```text
"this active representation is sufficient for all future inquiry"
```

unless a standing completeness envelope for future protected continuations exists.

The ordinary claim should be:

```text
sufficient for declared continuation family/horizon,
with declared recovery and unlock conditions.
```


---

# 25. Protected capability disposition: regeneration, restriction, privacy, and erasure

## 25.1 Do not universalize append-only payload recovery

The current repository's immutable/content-addressed artifacts are appropriate for
the present reference implementation and project binding.

Future bindings may protect non-recoverability.

Do not equate:

```text
immutable artifact/event identity
```

with:

```text
perpetually recoverable plaintext payload
```

at the architecture level.

## 25.2 Derived protected-use boundary

Future policy/admission code may reason over ordinary relations describing:

```text
RequiredUse(subject, operation, purpose, authority)
ForbiddenUse(subject, operation, purpose, observer)
ConditionalUse(subject, operation, purpose, authority)
RecoveryPermitted(...)
ErasureRequired(...)
```

No core enum is required.

## 25.3 Potential payload disposition architecture

Only if a protected fixture requires erasure/restriction, separate:

```text
ArtifactIdentityRef
PayloadRef / EncryptedPayloadRef
DispositionRelationRef
AccessAuthorityRef
ErasureEvidenceRef
AntiRecoveryCoverageRef
```

Identity/provenance may remain while the protected payload becomes inaccessible,
provided even the surviving metadata is allowed under the binding.

## 25.4 Anti-recovery verification

Do not implement:

```text
recovery_search_failed => erased
```

Correct status is `Unknown` unless a binding-native complete anti-recovery
certificate exists.

Cryptographic erase, secure deletion, differential privacy, and machine unlearning
all have different native guarantees/checkers.

## 25.5 Compression ordering correction

A "richer" presentation may violate privacy.

Fold/representation comparison must evaluate the whole protected use boundary:

```text
required uses preserved
forbidden uses not newly enabled
conditional uses remain authority-scoped
approximation distortion remains within contract
```

not raw information quantity.

## 25.6 Unlearning implication

If model/data unlearning is introduced, keep separate:

```text
DeleteRequestOccurrence
UnlearningProcedureOccurrence
PostDeletionModelRef
Deletion/UnlearningCheckerEvidence
ResidualCache/DerivedStateRefs
ObserverCoverage
```

Endpoint equality to a retrained reference is not sufficient proof of deletion.

## 25.7 New fixtures

```text
ERASE-001 intentional erase != accidental loss
ERASE-002 bounded recovery failure != erasure proof
ERASE-003 cryptographic erase is coverage-relative

PRIV-001 more informative can violate protected privacy
PRIV-002 privacy post-processing uses native theorem

LAW-001 stored but processing-restricted

UNLEARN-001 endpoint equivalence != deletion provenance

SIDE-001 direct deletion with surviving indirect channel fails anti-recovery
```

## 25.8 Release-language correction

Never claim:

```text
"all retained state is regenerable"
```

as a universal capability requirement.

Prefer:

```text
all required recoverable state has a standing recovery route;
all protected non-recoverable state has a standing erasure/privacy relation;
all conditional state is accessible only under its standing authority/purpose.
```


---

# 26. Versioned interpretation and semantic rehydration

## 26.1 Artifact identity is not interpretation identity

Content-addressed artifact identity must remain independent of:

```text
interpretation binding
schema / ABI / decoder version
authority
context
time/version
```

when a protected semantic relation depends on those coordinates.

Do not bake changing semantic interpretation into artifact identity unless the
artifact definition itself canonically includes it.

## 26.2 Opaque carry

Future storage/runtime code may preserve payloads that are not currently
interpretable.

Required rule:

```text
uninterpreted != invalid
uninterpreted != irrelevant
uninterpreted != standing semantic claim
```

Preserve identity/provenance/raw payload where authorized, and reopen when a new
decoder/binding becomes available.

This is directly analogous to protobuf unknown-field preservation.

## 26.3 SemanticRehydration service

Potential derived service:

```text
SemanticRehydration {
    artifact_ref,
    historical_binding_ref,
    decoder/schema/abi_ref,
    context_refs,
    authority_ref,
    protected_interpretation_family,
}
```

Possible outputs:

```text
Rehydrated(meaning/relation refs)
InterpreterRecoveryGap
SchemaAbiGap
InterpretationContextGap
AuthorityResolutionGap
SemanticMigrationUnknown
```

This service does not create semantic authority.
All outputs still need the normal checking/warrant route.

## 26.4 Migration correctness

A representation migration must be judged through protected behavior, not format
identity.

Use a relation such as:

```text
ProtectedSemanticTransport(source, target, bridge, horizon, evidence)
```

whose native checker may be:
- compiler semantic preservation proof;
- schema compatibility theorem/check;
- explicit migration validation;
- other binding-native method.

Directional preservation is permitted when the protected contract is directional.

## 26.5 Contextual instantiation

Do not identify a reusable description with one occurrence:

```text
Program != ExecutionOccurrence
ContinuationProgram != CapturedContinuationOccurrence
Expression != UtteranceOccurrence
Schema != DecodedInterpretationOccurrence
```

A shared internal helper may track:

```text
ContextualInstantiation {
    description_ref,
    context/binding refs,
    occurrence_ref,
}
```

only if a protected fixture requires a persisted canonical representation.

Prefer derived relations first.

## 26.6 Latest repository position

Current inspected head during this crawl:

```text
57b27da36312229b5c17d65fb28683a39a940ee6
feat: prove reciprocal occurrence vertical slice
```

Do not divert current Phase-4/Phase-5 implementation sequencing for semantic
migration work.

Keep the following as future regression fixtures.

## 26.7 Fixtures

```text
SEMVER-001 same artifact, different binding interpretation
SEMVER-002 payload recovery without historical interpreter => semantic recovery gap

ABI-001 same name/source form != compatible runtime ABI

PROTO-001 parseable migration can still be lossy
PROTO-002 preserve opaque unknown field without semantic promotion
PROTO-003 dropping unknown field breaks future exact rehydration

LING-001 etymological ancestry != current meaning
PRAG-001 same sentence/content != same utterance force
LAW-001B unchanged text + authority succession changes current standing interpretation

MIGRATE-001 physical representation changes while protected semantics commute
```

## 26.8 Release-language correction

Do not claim:

```text
"artifact is recoverable, therefore its semantics are recoverable"
```

Prefer:

```text
payload is recoverable;
semantic rehydration is licensed only under the retained/reconstructed
interpretation dependencies and declared horizon.
```


---

# 27. Governed self-modification admission and trust-root discipline

## 27.1 Do not trust self-regeneration

A future self-hosting compiler/interpreter implementation must not infer:

```text
recompiled successfully / reproduced lineage
=> executable corresponds to represented source
```

Trusting-trust supplies a concrete breaker.

If source/executable correspondence is protected, use an independently grounded
verification path such as:
- proof-grounded compilation;
- diverse double-compiling;
- translation validation;
- another binding-native certificate.

## 27.2 Generator and admission kernel remain separate

Future self-modification architecture:

```text
mutable proposal/search/generator
        ↓
candidate successor + explicit certificate/evidence
        ↓
authoritative admission checker
        ↓
standing successor transition
```

Generator code need not itself carry semantic authority.

## 27.3 Proof-carrying successor package

Possible derived package:

```text
SuccessorProposal {
    predecessor,
    candidate_successor,
    declared_delta,
    protected_use_boundary,
    proof_or_certificate_refs,
    actuality/build_refs,
    migration_refs,
    recovery_or_erasure_refs,
    breaker_results,
}
```

Keep this derived until concrete repository fixtures force canonical persistence.

## 27.4 Checker succession

A candidate must not:
1. replace the checker;
2. invoke the replacement checker;
3. use that check as the sole authorization for the same replacement.

Checker changes require a separate standing route:

```text
CheckerMigration(old_checker, new_checker, evidence, authority)
```

authorized by:
- the predecessor checker;
- an independent verifier;
- or another standing authority root.

## 27.5 Occurrence-specific validation

For rapidly evolving generators prefer, where appropriate:

```text
source occurrence
+ actual transform occurrence
+ target occurrence
+ certificate
+ independent checker
```

over global trust in the whole generator.

This is translation-validation style admission.

## 27.6 Root boundary

Measured-boot style chains reinforce:

```text
verified downstream chain != root verified by same chain
```

Make root assumptions explicit.

Attestation identity alone must not imply semantic correctness.

## 27.7 Trust-root minimization

The engineering objective is:

```text
minimize trusted checking/authority surface
while preserving semantic coverage and actuality linkage
```

not:

```text
recursively verify everything until no root remains
```

The latter is not established.

## 27.8 Latest repository position

Current inspected head:

```text
3dd7cef1f9e1a590d118dacb3d27b253bdde6012
feat: admit finite supported answers
```

The commit preserves a multi-completion supported answer and exact event/raw-return
support provenance but leaves answer-slot binding/continuation choice open.

Do not disrupt that implementation sequence.

## 27.9 Future fixtures

```text
SELFTRUST-001 self-regeneration != source correspondence
SELFTRUST-002 independent regeneration cross-check

SELFCERT-001 checker rewrite cannot self-authorize
SELFCERT-002 predecessor-authorized checker migration

PCC-001 untrusted generator + valid certificate can admit
PCC-002 generator assertion cannot override checker rejection

ATTEST-001 measured identity != safety
ROOT-001 trust chain cannot discharge its own root

TV-001 occurrence-specific validated rewrite
TV-002 certificate/actual-artifact mismatch rejects
```


---

# 28. Clockwork implementation discipline

The cohesive machine should be engineered as a narrow pipeline, not a framework
of frameworks.

Current target shape:

```text
canonical data/relations
    ↓
small pure/derived admission functions
    ↓
verified first-order runtime transition
    ↓
single append-only actuality path
    ↓
derived decode/support/standing views
    ↓
next explicit continuation
```

Rules:

1. No duplicate authority owner.
2. No duplicate source of truth.
3. No persisted derived object unless identity/replay requires persistence.
4. No trait/service/registry until there are concrete interchangeable
   implementations or an authority boundary requires one.
5. No new crate until an actual cross-crate transaction cannot be cleanly owned.
6. No generic self-modification subsystem before ordinary answer-dependent
   execution is complete.
7. No boolean "validated"/"relevant"/"safe" fields when the evidence relation can
   be retained directly.
8. No automatic method dispatcher; residuals generate explicit next questions.
9. Prefer exhaustive small finite fixtures before generic machinery.
10. Every abstraction must have a deletion test:
    name the protected behavior that fails when it is removed.

The current repository frontier is appropriately narrow:
lower one `BoundFiniteAskContinuation` into the existing runtime while preserving
the exact supported answer and raw-return/event provenance.

Complete that gear before adding another gear.


---

# 29. Clockwork execution architecture

The machine is now constrained to three implementation categories.

## 29.1 Authoritative state

Persist only what must independently survive:

```text
canonical referenced semantic identity
actual external occurrence / raw return
standing authority input
required recovery/provenance
```

## 29.2 Derived transitions

Default form:

```text
fn explicit_inputs -> Result<typed_output, typed_residual>
```

No hidden mutable semantic state.

Examples include decode, support admission, standing computation, finite answer
binding, and lowering.

## 29.3 External boundary

External effects occur only through the explicit probe/actuality path.

Replay consumes preserved actuality; it does not repeat external work.

## 29.4 Core-placement test

A new component enters a privileged/core layer only if:

```text
moving it outward breaks a required invariant
OR it owns unique authority/actuality/canonical identity
OR repeated concrete logic already demonstrates a shared invariant
```

Otherwise keep it local.

## 29.5 Current exact implementation target

Repository head inspected:

```text
38cfd2ca06b7984c8e3eb64ea77942000a58c3fc
feat: bind finite inquiry answers
```

The current live gear is:

```text
BoundFiniteAskContinuation
+
verified ProgramIR Probe suspension
+
known source-continuation -> runtime-block correspondence
+
admitted event/raw-return provenance
-----------------------------------------
derived exact resume binding
```

The implementation must reject mismatched:

```text
source continuation
runtime resume target
probe operator
event/raw return
question
answer binding
```

and retain the complete partial finite answer set.

## 29.6 Placement

Prefer a narrow pure function in `ic-runtime`, since runtime already consumes
`ic-core` types.

Do not:
- make `ic-core` depend on runtime;
- add a new crate;
- persist the derived lowering unless later identity/replay proves persistence
  necessary.

## 29.7 Fixture-first sequence

```text
LOWER-001 exact named continuation -> fixed resume succeeds
LOWER-002 wrong continuation rejects
LOWER-003 wrong resume target rejects
LOWER-004 wrong operator/event/raw return rejects
LOWER-005 multi-completion answer remains whole
LOWER-006 replay from same event derives same lowering
```

Only after these pass ask whether a more general environment/substitution
mechanism is forced.

## 29.8 Anti-overengineering invariant

```text
one authority owner
one actuality path
one canonical identity per semantic fact
derived views recomputed by default
typed local failures
no framework before repeated need
```


---

# 30. Current lowering gear: derived runtime answer binding

Canonical source environment remains:

```text
TypeSymbol -> TypedFormRef
```

Do not revise it to hold finite answer sets yet.

Current derived lowering should carry:

```text
source continuation identity
runtime resume target
existing explicit ProgramBinding environment
answer-slot symbol
whole AdmittedFiniteAnswerSet
raw-return/event provenance
```

as noncanonical runtime data.

Possible implementation forms, in order of preference:

```text
(Resumption, BoundFiniteAskContinuation)
```

if sufficient;

otherwise one private derived wrapper containing exactly those fields.

Do not add a general `RuntimeValue` enum until another concrete path requires it.

Do not perform substitution until an executable continuation operator actually
needs to consume the answer value.

This keeps the current gear one responsibility wide.


---

# 31. Repository convergence — continuation gear closed, Phase 6 actualization gear open

Current inspected head:

```text
7eee9ee5438baf8628d93817a1bc57737508d0c1
feat: resume admitted inquiry continuations
```

The repository now closes:

```text
admitted finite answer
-> capture-safe source binding
-> exact source-continuation/runtime-target lowering
-> admitted event/raw-return-bearing resumption
```

The old map sections treating this lowering as current are ancestry.

## 31.1 Current machine boundary

`ic-runtime`:
- structural Return/Branch/Probe;
- verified suspension;
- raw-only resumption;
- admitted answer resumption;
- no external dispatch;
- no event persistence.

`ic-store`:
- immutable content-addressed artifacts;
- one append-only checked actual-event ledger;
- idempotent append of the identical event;
- restart verification;
- no external dispatch.

Current migrations contain no request/attempt table.

The current missing transaction is:

```text
ProbeSuspension
-> durable pre-dispatch intent
-> external effect
-> raw-return/event commit
-> existing replay/resumption
```

## 31.2 Exactly-once boundary

Do not claim:

```text
one event row == one real-world side effect
```

SQLite cannot atomically commit an arbitrary remote effect.

After a crash with a prepared request but no committed event:

```text
UnknownActuality
```

is the generic result.

Only a binding-native idempotency/status/reconciliation contract licenses retry or
completion.

## 31.3 Minimal operational record

Working implementation name:

```text
PreparedProbe
```

Noncanonical, persistence-only.

Purpose:
- prove request intent existed before dispatch;
- retain a stable recovery/request token;
- bind to exact ProbeOperator;
- retain only the non-derivable replay context needed to reconstruct the suspension;
- link to one event_ref after successful local completion.

Do not duplicate query/boundary/backend/compiler data already reachable from
ProbeOperator/ProbeContract.

Do not finalize the exact field layout until the first crash/restart fixture is
ablated.

## 31.4 Local completion transaction

After the backend actually returns, prefer one SQLite transaction that commits:

```text
RawReturn artifact
ActualEvent artifact
event_ledger append
PreparedProbe completion -> event_ref
```

before any decoding.

This minimizes crash states.

It preserves the canonical rule that interpretation never precedes raw actuality
persistence.

## 31.5 `ic-machine` is now earned

Earlier rule:

```text
create ic-machine only when runtime + store are required by one true
actualization transaction
```

Current Phase 6 now satisfies that condition.

Add only:

```text
crates/ic-machine
```

with dependencies:

```text
ic-core
ic-runtime
ic-store
```

and Tokio only at the external-effect boundary as already planned.

Responsibilities:
- persist PreparedProbe;
- invoke one backend/fixture adapter;
- perform local completion transaction;
- select recovery route after restart;
- call existing deterministic replay/resumption functions.

Not responsibilities:
- new semantic objects;
- standing logic;
- decoder semantics;
- method scheduling;
- self-modification policy;
- generic workflow framework;
- CLI behavior.

`ic-cli` remains a thin interface.

## 31.6 Phase 6 fixture order

```text
P6-ACT-001 prepared intent precedes dispatch
P6-ACT-002 prepared != actual
P6-ACT-003 ambiguous non-idempotent crash -> UnknownActuality
P6-ACT-004 idempotent retry reuses same token
P6-ACT-005 raw return + event atomically local-complete before decode
P6-ACT-006 committed event cold-replays same admitted resumption
P6-ACT-007 one prepared request cannot finalize to two event refs
```

No additional framework before these fixtures force it.


---

# 32. PreparedProbe ablation target

Do not start Phase 6 by designing a rich request table.

Current hypothesis:

```text
PreparedProbe
    recovery_token
    operator_ref
    source_iprog_ref
    minimal_pre_event_anchor
```

Potential `resume_target` / lowering data is derived by default.

A field is admitted to persistence only if removing it prevents exact fresh-process
reconstruction of:
- the verified suspension;
- the exact source Ask/continuation;
- the eventual ActualEvent context;
- the same event-to-admitted-resumption suffix.

This is the next fixture-driven schema decision.


---

# 33. Phase 6 implementation feedback — remove the unearned machine layer

Repository head `0c55aff` implemented durable pre-dispatch preparation and atomic
raw/event completion in `ic-store`.

The workspace still has:

```text
ic-core
ic-runtime
ic-store
ic-cli
```

This is evidence against the previous claim that Phase 6 had already forced an
`ic-machine` crate.

Update:

```text
ic-machine: DEFERRED
```

Do not add it unless a later concrete reusable orchestration invariant cannot be
owned by existing boundaries.

# 34. Current exact gear — identity mock dispatch

The current store journal preserves `request_ref` and `operator_ref` independently,
but only proves request existence.

First fixture should impose:

```text
request_ref == checked_operator.executable_code_ref
```

for the mock provider binding.

This makes the existing executable-code artifact occupy the BackendRequest role
without introducing a behaviorless one-to-one wrapper artifact.

# 35. Dispatch authority is linear, recovery state is not

Do not let:

```text
ExternalEffectState::Pending
```

serve as a provider-dispatch capability.

Use one private, nonclone, move-consumed derived `DispatchPermit`, available only
when a new durable preparation is inserted.

Exact repeated prepare / recovered pending returns state only.

This preserves:

```text
recovery inspection != effect permission
```

and prevents the crash-retry ambiguity from reappearing at the provider boundary.

# 36. First mock provider

Keep it fixture-local/narrow:

```text
checked identity request
+
fresh DispatchPermit
-> opaque bytes
```

Then:

```text
opaque bytes
-> RawReturn
-> construct checked ActualEvent
-> complete_external_effect
```

No decode inside provider.

No provider registry.

No SurfacePlan.

No generic BackendRequest artifact until a protected transport-envelope difference
appears.

# 37. Next exact implementation question

Field-ablate `ActualEvent` construction.

Ask which event fields are:
- already determined by prepared operator/history/binding;
- actual provider/world outputs;
- fixture constants;
- truly new occurrence metadata.

Only the last two categories that cannot be regenerated should cross the provider
actualization interface.
