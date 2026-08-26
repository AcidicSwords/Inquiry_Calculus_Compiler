# AGENTS.md — Inquiry Calculus Software Engineering Clock

## Purpose and scope

This file governs how consequential work is conducted in this repository, including code,
tests, specifications, migrations, research integration, authority records, and delivery.
It does not define Inquiry Calculus semantics, duplicate the implementation roadmap, or own
moving project state.

An accepted instruction is a cached answer. Apply it directly while its scope, evidence, and
reopen condition remain unchanged. Questioning reopens an answer only when a live protected
distinction warrants it; it is not permission to ignore settled instructions.

## Authority by question

```text
explicit user task
    -> task scope, authorization, and delivery

Inquiry_Calculus_v2_0.tex
    -> semantic meaning

Inquiry_Calculus_v2_0_Comprehensive_Implementation_Plan.md
    -> project completion contract, architecture, and phase obligations

IMPLEMENTATION_FRONTIER.md
    -> the single strongest live executable residual

CONFORMANCE_STATUS.md
    -> demonstrated executable evidence at declared coverage

DECISIONS.jsonl
    -> accepted implementation choices and reopen conditions

FAILURES.jsonl
    -> actual observed failures and environmental constraints

Git + code + tests + raw tool/runtime/provider returns
    -> repository actuality and ancestry

research/
    -> derived breaker ancestry, consulted only when a live residual makes it relevant
```

No remembered summary outranks a fresh probe of the repository. A more local `AGENTS.md` may
narrow work in its scope but may not silently violate a repository-wide invariant. If two
sources answer the same question in the same scope incompatibly, stop that branch as `Unknown`
until the governing authority is established; do not code through the conflict.

## Default autonomous objective

When no narrower user task is active:

> Autonomously advance the strongest live residual in `IMPLEMENTATION_FRONTIER.md` toward the
> plan's smallest complete, executable, cold-replayable v2.0 reference implementation. For each
> ratchet: inspect actuality, construct the smallest decisive breaker, make the smallest reversible
> change, verify, challenge, minimize, update project state, deliver when authorized, and recur.
> Stop only at demonstrated completion or a lawful stop state.

A narrower user task temporarily supersedes the Frontier. Resume from the then-current Frontier
when it closes unless the user changed or revoked this objective.

`Ask` means discharge a typed question through its declared route. It does not mean ask the user
by default. Ask the user only when repository sources, lawful probes, and standing authority
cannot decide an answer whose supported alternatives would change the continuation.

## Operational contract

Every consequential engineering question must correspond to an ordinary typed open relation,
and every answer-conditioned continuation must depend on its supported return. Continuations are
inspectable first-order program data with explicit environments and capture-safe answer bindings,
never opaque host closures, hidden model policy, or an after-the-fact prose choice.

Keep the discharge modes distinct:

```text
Pure      exact derivation from admitted structure
Generate  provisional question, hypothesis, design, test, patch, or method
Probe     actual repository, tool, runtime, environment, or provider interaction
Check     independently admitted discriminator relation
Warrant   authorized acceptance or promotion
```

Only a `Probe` establishes new actuality. A pure derivation may propagate already established
actuality but does not create an event. Preserve partial and mixed-mode supported answers,
per-component support, unanswered `Unknown` components, and exact provenance. Never fabricate a
total answer or choose a convenient singleton from a supported set.

Never collapse:

```text
generated possibility
!= actual probe
!= raw return
!= decoded or interpreted return
!= checked claim
!= warranted standing result
```

## The engineering clock

```text
SPECIFY
-> INSPECT
-> CONTRAST
-> TRACE
-> EXPERIMENT
-> UPDATE
-> CHANGE
-> VERIFY
-> CHALLENGE
-> MINIMIZE
-> RATCHET
```

This is a clock, not a checklist. A position is active only when its underlying relation is open.
Skip positions already discharged or irrelevant to the protected horizon. A substantive return
may reopen an earlier position. `RATCHET` constructs the residual that selects the next live
position.

### SPECIFY — establish what counts as correct

Ask only what remains open:

- What behavior or result is required?
- What must remain unchanged or become impossible?
- Which source has authority, and why is the requirement in force?
- What scope, applicability, version, environment, grain, horizon, and constraints apply?
- What independent evidence can decide acceptance?

Do not manufacture requirements from architectural preference. Exit when the affected contract
is explicit enough to discriminate a successor.

### INSPECT — interrogate actuality

Use the real repository, compiler, tests, runtime, provider, migration, or tool whenever it can
answer more reliably than generation. Establish what is present, what occurred at the relevant
boundary, which input/version/environment produced it, and what ties the return to that
occurrence. Preserve raw returns before interpretation when safe.

Repository search yields candidate loci, not responsibility. Confirm the forward path from a
candidate to the protected behavior before treating it as causal.

### CONTRAST — construct a consequential difference

Create a strong protected foil. Name:

```text
current observable:
required observable:
protected difference:
scope / applicability / grain / horizon:
independent discriminator:
```

Vary removal, reversal, strengthening, weakening, ordering, environment, or representation only
where the resulting difference can affect protected consequence. Prefer strong contrast first;
subtract later.

### TRACE — expose the responsible relation

Use whichever relation is actually open: cause, mechanism, dependency, support, prerequisite,
path, provenance, applicability, composition, ordering, interface, resource, or concurrency.

Ask where the divergence first becomes observable, through which typed path it propagates, what
precondition or blocker is active, and what actual evidence separates mechanism from narrative
correlation. Ledger order, domain succession, causal order, and inquiry traversal remain distinct.

### EXPERIMENT — ask the discriminator of the system

Identify the smallest actual, pure, or check question that separates the live alternatives.
Name the smallest wrong implementation it must reject. If no wrong behavior is distinguished,
the check is decoration.

Before mutation, seal:

```text
should change:
should remain invariant:
decisive discriminator:
smallest wrong implementation:
declared coverage:
```

Do not revise the prediction after seeing the return.

### UPDATE — rebuild from the return

Separate the raw return from its interpretation. State what it establishes, which alternatives it
eliminates, which survive, whether the frame/representation/coverage failed, and which residual is
now live. Do not continue from the pre-return story.

### CHANGE — make the smallest responsible transformation

Prefer, when protected behavior is equal:

```text
reuse
> transparent composition
> conservative extension
> new abstraction
```

Touch only what the explanation requires. Preserve unrelated work, types, authority boundaries,
compatibility surfaces, and user changes. Keep the transformation reversible and avoid
speculative cleanup.

### VERIFY — establish what the realized successor satisfies

Use actual probes and independent checkers. Establish whether the realized successor satisfies
the declared contract, whether protected invariants remain true, what exact distinction each
checker establishes, and what lies outside its coverage. Generated prose cannot fill `Check` or
`Warrant`.

### CHALLENGE — attack apparent success

Attack sufficiency:

```text
Can every assumed condition hold while the protected requirement fails?
```

Attack necessity:

```text
Can the protected requirement hold without one assumed component?
```

Vary input, ordering, environment, concurrency, failure mode, integration boundary, and resource
pressure when relevant. Use property, fuzz, metamorphic, differential, stress, fault-injection,
security, or formal checks only when they discriminate the residual.

For a consequential reciprocal-boundary claim, preserve this dependency:

```text
source determination
-> admitted tagged NegationUse + positive DepartureWitness
-> exterior
-> reverse section of that same use and its whole return fiber
-> protected recovery
-> supported seed/reorientation
-> independently admitted reciprocal use, departure, exterior, fiber, and recovery
-> residuals
-> Gamma_D downstream check
```

One-way success is not reciprocal success. Projection cannot create exteriority, a selected return
is not the fiber, incomplete coverage remains `Unknown`, and pure return cannot revise standing.

### MINIMIZE — subtract successful excess

For every added line, type, module, service, dependency, check, or configuration ask:

> What protected consequence is lost if this is removed?

Try the safe ablation. If removal changes no protected consequence under declared coverage and
regeneration, provenance, recovery, and reopening obligations survive, remove or fold it. Prefer
the smallest protected deformation, not mechanically the fewest lines.

### RATCHET — retain the discriminator, not the investigation

Retain only durable structure: regression test, property, type/contract, assertion, interface
invariant, stable probe, accepted decision with a reopen condition, actual failure constraint,
conformance evidence, or new live frontier.

Record what is established, its evidence and coverage, what can leave active context, what would
reopen it, and the smallest consequential residual. Do not preserve research chronology or moving
status in stable authority documents.

## Question selection

Do not ask every available question. A discretionary question is productive when supported
answers can lead to protected-different continuations. A question may also remain live because an
explicit program or standing dependency must discharge a `Probe`, `Check`, `Warrant`, support,
reconstruction, or another typed obligation.

Select from questions that are:

- well typed and formable;
- applicable;
- executable where execution is required; and
- productive **or** required by an explicit live discharge obligation.

If exact `Pure` computation settles the relation, compute it. If a tool or environment can answer
more reliably than generation, probe it. If useful questions remain incomparable under a declared
resource/risk preorder, retain a nondominated frontier instead of inventing a scalar score.

## Standing non-collapse laws

- `Unknown != Negative`; not found is not absent or impossible.
- `coverage_sem != coverage_exec`; handled is not exercised.
- Return fiber != selected return.
- Converse != inverse.
- Generic backward relation != same-use reciprocal return.
- Applicability != support.
- Method occurrence != semantic discharge.
- Successful endpoint != same occurrence, provenance, or route.
- Current equivalence != future protected equivalence.
- Ledger order != causal order.
- Generation may propose a breaker but cannot defeat standing without sufficient authority.
- A candidate change cannot define or relax the criteria by which it is judged.
- An agent, patch, test, checker, summary, cache, fold, or route cannot warrant itself.

## Mutation and evidence discipline

For every consequential mutation:

1. inspect repository actuality and the worktree;
2. name the explicit task or live Frontier obligation it serves;
3. state the current/required observables and protected difference;
4. identify the smallest responsible locus and wrong implementation;
5. seal the expected protected deformation;
6. mutate only within authorization;
7. preserve the actual return;
8. check against the seal;
9. classify the residual;
10. challenge and subtract;
11. update only the records that own the changed fact.

Expose inspectable question/evidence provenance, concise conclusions, predictions, checks,
coverage, and residuals. Never request, store, or publish private chain-of-thought, model scratch
work, credentials, secrets, personal data, or sensitive raw returns. Redact or retain only a safe
digest/reference when raw evidence is sensitive.

## Project-state ownership

After a successful ratchet:

```text
new demonstrated behavior
    -> CONFORMANCE_STATUS.md

accepted implementation decision
    -> append DECISIONS.jsonl

actual durable failure or environment constraint
    -> append FAILURES.jsonl

new strongest executable residual
    -> replace the single live block in IMPLEMENTATION_FRONTIER.md
```

README is static orientation. The implementation plan owns requirements and phase/completion
architecture, not PASS/PENDING rows, current commits, provider availability, or recent fixture
history. Git owns chronology. Research remains derived breaker ancestry.

## Tests and change gates

Run the smallest decisive discriminator first, then directly affected tests, then broad required
gates. A valid test may be replaced only after the predecessor contract is shown inapplicable or
superseded under the proper authority; never weaken it to make a candidate pass.

A test occurrence is evidence for a declared consequence boundary, not a self-standing proof of
correctness. Each new or materially changed conformance claim must identify:

```text
F      protected failure or consequence
C      proposed delimiting condition
Omega  admitted scope and protected horizon
M      relevant system or transition semantics
P      actual or pure probe used to seek the breaker
V      independent checker
E      evidence and coverage
U      reopening condition
```

For a proposed necessary boundary, explicitly seek the breaker
`Reachable_(Omega,M)(x) AND F(x) AND NOT C(x)`. A found witness reopens or rejects the boundary.
Only an independently established empty breaker field licenses `F => C` at the declared scope;
an empty search under incomplete coverage remains `Unknown`. Establish the reverse breaker
separately before claiming `F <=> C`. A green runner result is an occurrence, not an exact
boundary theorem. For a repair, test the changed reachability relation or an invariant excluding
the admitted failure region, rather than only re-running the original sample. Promote discovered
conditions into ordinary objects for variation, ablation, control, composition, and reopening.

### Failure-formation testing specialization

When a test concerns failure, treat it as a recursively constructed inquiry into how the failure
forms, not merely a predicate expected to pass. Bind the investigation as:

```text
FailureFormation = (failure, succession, scope, horizon, grain, effectivity, binding)
```

Use only binding-supplied succession/reachability semantics. Never silently manufacture temporal
or transitive closure. Preserve these distinctions:

```text
failure manifestation != failure formation
CanFail != MustFail != manifested failure
condition first appears != condition causes failure != condition is necessary
path-local necessity != global necessity
earliest demonstrated formation != absolutely earliest formation
```

For each proposed failure-forming edge `C -R-> D`, represent and discharge the productive or
required questions for actual path support, guards/preconditions, bypass (`D without C`), escape
(`C without D`), alternate routes, and independent grounding. A bypass witness defeats global
necessity; an escape witness defeats sufficiency. One actual trace may support occurrence-local
dependence but cannot establish a global path theorem without admitted route coverage.

When grain and evidence allow it, retain a derived formation profile identifying when each
surviving consequential condition first became actual and through which transition. Prefer the
first protected divergence from a comparable acceptable occurrence over the usually trivial
question of first failure reachability along an already failing trace. Derive a commitment frontier
only when the binding supplies universal continuation semantics; otherwise it remains `Unknown`.

Any failure-path family, formation profile, divergence/commitment frontier, escape set, coverage,
and residual bundle is a rebuildable view over ordinary relations, questions, events, answers, and
provenance. Do not add an authoritative failure graph, controller, opcode, history, or discharge
mode. Recur through:

```text
BIND -> OPEN -> VARY -> RETURN -> DETERMINE -> REFACTOR
```

Before a test occurrence, bind its target relation, expected distinction, scope, applicability,
grain, coverage claim, and evidence route, then seal the prediction. Interpret the return only at
its demonstrated level: one failure witnesses realizability for that occurrence; an ablation that
changes it witnesses occurrence-level relevance; a bypass witnesses non-necessity; and absence of
a bypass licenses necessity only under independently admitted exhaustive coverage. Turn every
surviving condition, relation, alternate path, and unknown region into an ordinary object available
for subsequent inquiry, control, repair, folding, and reopening.

`CONFORMANCE_STATUS.md` is the durable boundary ledger: a `PASS` row must name its executable
breaker and state its protected difference and declared coverage/reopen condition. The control
checker may accept such a pass only through explicitly registered test evidence; it cannot accept
a prose assertion or a fixture merely because the candidate changed its status.

For the Rust workspace, the baseline gates are:

```bash
cargo fmt --all --check
cargo check --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
git diff --check
```

Run documentation topology and canonical-TeX gates whenever their surfaces change. Before closure,
inspect the actual diff for accidental churn, public API/schema/serialization/replay changes,
dead code, stale comments, hidden authority, secrets, and generated artifacts.

## Git, filesystem, and delivery safety

- Inspect `git status --short` before editing and before closure.
- Preserve unrelated user changes and isolate overlapping work; ask only when safe isolation is
  impossible.
- Do not commit, push, rewrite history, delete, deploy, release, message external parties, or make
  another external write unless the explicit or standing task authorizes it.
- Never use broad destructive recovery such as `git reset --hard`, broad `git clean`, or recursive
  deletion. Resolve exact targets first and prefer recoverable operations.
- Do not commit secrets, credentials, local environment files, databases, build outputs, or large
  generated files unless the contract explicitly requires them.
- Edit a generated file's source, not the generated output, when a source exists.
- Treat committed migrations, canonical identity encodings, public schemas, and replay formats as
  compatibility boundaries.
- A control-authority change to `AGENTS.md`, the canonical specification, the stable plan, or the
  inquiry harness requires explicit predecessor authority and a named control residual. An
  autonomous implementation candidate may not relax its own acceptance path.

## Stops and guarded recurrence

Use exactly:

- `Satisfied` — the task contract is met, checks cover the declared horizon, required authority
  accepts the result, and no acceptance-changing residual remains.
- `Equivalent` — alternatives are protected-equivalent under declared scope, language, horizon,
  and coverage; this closes only that branch.
- `Impossible` — a certificate proves impossibility within the declared representation and
  assumptions.
- `Blocked` — a specifically named required authority, permission, dependency, capability, or
  external return is unavailable.
- `Unknown` — evidence or coverage cannot distinguish the live alternatives.
- `ResourceBounded` — a declared finite limit was reached; return the supported partial result,
  remaining frontier, and residual.

Only `Satisfied` closes the task successfully.

Fingerprint recurrence by question occurrence and continuation identity, normalized bindings,
Frontier identity, governing authority, evidence, repository actuality, protected horizon, and
coverage. Never repeat the same question after the same answer in the same state. Recur only across
a new actual return, admitted distinction, representation/binding change, authority change,
observed repository/external state change, or strict reduction of a finite frontier. Every loop
uses finite fuel.

## Harness contract

The `.claude` harness mechanically enforces this discipline for clients that honor
`.claude/settings.json`; it is not a universal filesystem security boundary.

Preserve:

- prediction before mutation;
- explicit user-authorized control revisions for protected authority/harness files;
- append-only, collision-safe trace records;
- the enforced `seal -> raw -> check -> residual` state transition, with no replacement seal;
- safe raw-return preservation or sensitive digest-only recording;
- occurrence/continuation/bindings/frontier/horizon/coverage/authority/evidence/actuality-aware
  fingerprints;
- exact parsing of the one live Frontier block;
- repeated-state refusal and finite fuel;
- no stop with an open sealed cycle and no `Satisfied` stop without an external warrant reference.

README orientation, Frontier, conformance, decision, and failure ratchets are ordinary project-state
mutations under a sealed cycle. They do not require the separate control grant reserved for changes
to AGENTS, canonical/plan authority, the harness, and CI acceptance/checker surfaces.

Do not edit the harness to get past the harness.

## Final law

```text
SPECIFY what correct means.
INSPECT what is actually there.
CONTRAST it with what would matter.
TRACE the relation producing the difference.
EXPERIMENT with the question that separates the live possibilities.
UPDATE from the actual return.
CHANGE only the smallest responsible region.
VERIFY what the realized successor actually satisfies.
CHALLENGE success until its boundary is visible.
MINIMIZE away what protected consequence does not need.
RATCHET the surviving discriminator into durable structure.
LET THE RESIDUAL CONSTRUCT THE NEXT QUESTION.
```
