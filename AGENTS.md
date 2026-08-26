# AGENTS.md

## Status and purpose

This file governs all consequential work in this repository: code, tests, specifications, research, migrations, authority records, and delivery operations.

It presents one protocol in two equivalent forms:

1. an engineering protocol that a contributor can apply directly; and
2. a normative first-order `IProg` whose questions select answer-dependent continuations.

The engineering rules are the operational reading of the program. The program is the compilation target of the rules. If a consequential instruction cannot be represented by the program without losing a protected distinction, treat that as a representation problem; do not hide the choice in prose or host-language control flow.

This repository is built by resolving live questions, not by running a fixed checklist regardless of the returns. The governing recurrence is:

```text
current determination
-> live question whose supported answers lead to different continuations
-> answer discharged through the required route
-> smallest action justified by that answer
-> preserved actual return
-> independent check and residual
-> next question or justified stop
```

An instruction is a cached answer to an earlier question. Apply it directly while its scope, evidence, and reopen condition remain unchanged. Questioning over instruction means reopening an answer when a live distinction warrants it; it does not mean ignoring settled instructions or re-asking them ritualistically.

Do not ask a question when every supported answer leads to the same continuation.

---

## Companion control documents

This repository uses three deliberately separate control documents.

### `AGENTS.md` — repository-wide execution protocol

This file governs **how consequential work is performed**:

- authority by question;
- `Pure / Generate / Probe / Check / Warrant`;
- actual-return preservation;
- typed answer-dependent continuations;
- reciprocal challenge when consequential;
- separator, subtraction, regeneration, stopping, recurrence;
- test/change gates;
- no-self-warrant;
- Git/filesystem safety.

Its detailed first-order `IProg` is the normative compilation target of the engineering protocol.

### `PERSISTENT_CODEX_GOAL.md` — standing autonomous objective

This file governs **what the implementation agent is persistently trying to accomplish** when no narrower user instruction supersedes it.

It supplies:

- the long-horizon implementation goal;
- autonomous continuation rules;
- the software-native perceptual reasoning aperture;
- sufficient-present development-state discipline;
- the rule for generating the next implementation obligation from the current residual.

A new user request may narrow or temporarily redirect the active task. When that request is satisfied, resume the persistent goal unless the user changed or revoked it.

### `PROJECT_RESEARCH_IMPLEMENTATION_HANDOFF.md` — research context and implementation deltas

This file is a **self-contained checkpoint of what the separate, still-running research/domain-crawl recursion has established so far** and how those results bear on implementation. The implementation agent is not expected to possess the research conversation, its working ledgers, or its in-progress documents. Do not search for missing research files before using the handoff; everything currently required from research is restated there.

It is not independent semantic authority, and it is intentionally revisable as the separate research process advances.

Use it to distinguish:

- research-derived reasoning discipline that can be used immediately;
- fixture-ready breakers;
- architecture constraints for later phases;
- implementation-ready derived macros/contracts;
- rejected overclaims;
- research-only open questions.

If this handoff conflicts with the canonical specification, the canonical specification governs meaning. If it conflicts with an accepted implementation decision in the decision's recorded scope, reopen the decision only through its recorded reopen condition or a newly witnessed protected breaker.

A later handoff revision may supersede an earlier research conclusion without implying a semantic-authority change. Treat the changed research statement as a new candidate constraint: map it to the canonical calculus, locate the earliest implementation boundary it could affect, and require an executable breaker before changing accepted implementation behavior.

### Default autonomous invocation

When invoked without a narrower explicit task:

1. read `PERSISTENT_CODEX_GOAL.md`;
2. reconstruct the current implementation state from repository actuality;
3. read `IMPLEMENTATION_FRONTIER.md`;
4. consult `PROJECT_RESEARCH_IMPLEMENTATION_HANDOFF.md` for applicable research-derived constraints;
5. execute the principal development program in this file on the strongest live executable obligation;
6. after a warranted local ratchet, update the project-state files and continue to the next residual without waiting for a new user instruction.

Do not ask the user to choose a continuation that the repository, authority files, tests, or lawful probes can determine.


# I. Engineering protocol

## 1. Authority is indexed by the question

Authority is not one total ordering for every kind of claim. First identify the question, then use the source authorized to answer that kind of question.

| Question being answered | Governing source |
|---|---|
| What did the user authorize, and what delivery is in scope? | The explicit user request and subsequently accepted task contract. |
| What does Inquiry Calculus v1.1 mean? | `Inquiry_Calculus_Unified_Canonical_Specification_v1_1.tex` (Positive-Negation Successor Edition, 25 August 2026). |
| How should that meaning be implemented, and in what order? | `Inquiry_Calculus_v1_1_Comprehensive_Implementation_Plan.md`. |
| What is the standing autonomous project objective when no narrower user task supersedes it? | `PERSISTENT_CODEX_GOAL.md`, constrained by the semantic authority and implementation plan. |
| What is the strongest unresolved implementation obligation now? | `IMPLEMENTATION_FRONTIER.md`. |
| What research-derived implementation constraints or later-phase deltas are currently known? | `PROJECT_RESEARCH_IMPLEMENTATION_HANDOFF.md`, as derived guidance only; it cannot silently amend canonical semantics. |
| Which local implementation choices have been accepted? | `DECISIONS.jsonl`, within each decision's recorded scope and reopen condition. |
| What does the repository or an external system actually contain or do? | Files, Git state, builds, tests, runtime traces, tools, providers, and their preserved raw returns. |
| What behavior has been demonstrated under admitted checks? | Tests and `CONFORMANCE_STATUS.md`, limited to the distinctions and coverage those checks actually establish. |
| Which observed failures constrain future work? | `FAILURES.jsonl`; a failure is historical evidence, not semantic authority. |

Prefer a current accepted source to superseded detail. Plans, comments, generated text, earlier commits, and chats may provide ancestry or candidates, but cannot silently override a current semantic contract. Conversely, the specification does not establish that code conforms: code and returns establish actuality, and independent checks establish only their declared coverage.

A more local `AGENTS.md` may settle narrower questions. It may not silently invalidate a repository-wide invariant. If sources appear to conflict, determine whether they answer the same question in the same scope. If they do, make the incompatibility the next live question and do not code through it by guessing.

### Evidence and discharge modes

Use these modes exactly:

| Engineering verb | Mode | What it can establish |
|---|---|---|
| Derive | `Pure` | A consequence computed from already established inputs. |
| Propose | `Generate` | A candidate, hypothesis, separator, design, or patch; never actuality or acceptance by itself. |
| Observe or execute | `Probe` | A raw return from files, history, a command, a runtime, a provider, a person, or another external boundary. |
| Verify | `Check` | An independently discriminated relation between evidence and a declared claim. |
| Accept | `Warrant` | Acceptance under the source currently authorized to change the affected contract. |

Only a `Probe` return establishes new semantic actuality. A `Pure` derivation may propagate already established actuality but does not create a new actual event. A test execution has a raw `Probe` return; interpreting that return against a prediction is a `Check`. A `Warrant` may accept a checked change but does not manufacture the event it accepts.

No route may self-promote. In particular:

- generated structure is not actual because it is detailed or confident;
- an actual return is not verified merely because it parses;
- a passing check is not accepted outside its declared authority or coverage;
- an agent, patch, test, cache, summary, or derived view cannot warrant itself;
- failure to retrieve or activate a value is not evidence that the value is absent or irrelevant.

`Ask` means discharge a typed question through its declared route. It does not mean ask the user by default. Ask the user only when the answer cannot be established from the repository, tools, specifications, accepted authority, or other available evidence, and distinct supported answers would materially change the continuation.

Partial supported answers are valid. Preserve which components were answered, their support, and which components remain `Unknown`; never invent totality to make control flow convenient.

## 2. Settled project answers

The following are cached answers. Apply them without reopening unless the stated condition becomes live.

| Question | Current answer | Reopen when |
|---|---|---|
| What language should the reference implementation use? | Rust with Cargo. | A protected capability cannot be implemented credibly in Rust, or measured constraints make the choice consequential. |
| What is the initial persistence architecture? | SQLite with one authoritative writer plus immutable content-addressed artifacts. | Measured throughput, durability, deployment, or concurrency requirements exceed it. |
| Should the initial system be distributed? | No. Begin as a small single-process reference runtime. | A protected requirement or measured boundary requires distribution. |
| Should the project begin with a graph or vector database? | No. Use exact relational/reference access first. | Logged misses demonstrate protected strict gain from another retrieval route. |
| Should a generic agent framework or workflow engine control the system? | No. Keep control in the project's typed program/runtime. | A concrete protected behavior cannot be represented or executed without stronger machinery. |
| How should semantic values cross boundaries? | Through explicit typed forms and references, not generic untyped payloads. | An accepted successor changes the type contract. |
| How are answer-dependent continuations represented? | As first-order, inspectable, persistable program data, never opaque host-language closures. | A breaker shows that representation cannot preserve required behavior. |
| What is the executable runtime core? | Pure return, internal branching, and actual probe/effect. | A required effect cannot be represented by those forms without losing a protected distinction. |
| When is an external, model, or tool result part of history? | Preserve the raw actual return before decoding, interpretation, checking, or acceptance. | Only an explicit successor to the authority model may change this. |
| May generated output count as a check or acceptance? | No. Generation, actuality, verification, and acceptance remain separate. | Only an explicit accepted authority change. |
| How much architecture should be added ahead of use? | The smallest reversible structure that realizes current protected behavior. | A witnessed fixture exposes an independent variation requiring more. |
| What happens when the admitted representation cannot express an independently witnessed distinction? | Open a representation or binding-extension problem. Do not search forever inside the same language or declare equivalence. | A separator is found within the existing admitted language. |
| What does a typed distinction schema provide? | `D = (X, Y, B_D, pi_X, pi_Y, Gamma_D)` provides typed candidate carriers, boundary incidence, projections, and a downstream compatibility predicate. Projection does not create exteriority. | An accepted semantic successor changes the distinction schema. |
| What is the standing source determination for reciprocal work? | A supported claim-local `DeterminationPresentation`, indexed by scope, applicability, grain, horizon, and provenance; it is not an assertion of all facts. | Phase 4 breakers require a broader or narrower admitted presentation. |
| How is support partitioned at the current departure boundary? | The determination presentation resolves through its own exact claim-targeted support environment; source-observation, candidate-observation, and incompatibility relation uses each retain their own exact relation-targeted support environment. Resolving those routes is structural provenance, not standing/admission of the evidence uses. | A typed aggregate/bridge is accepted, a support-closure law independently evaluates relation-targeted evidence, or the canonical departure-support contract changes. |
| What establishes departure? | A positive, relevant, non-circular `DepartureWitness` against the standing presentation. Failed equality, search, retrieval, projection, or mere non-equivalence is insufficient. | An accepted semantic successor changes the witness obligation. |
| What does incomplete departure or negation coverage mean? | `Unknown`, not negative evidence and not an exterior. | Coverage is completed by an admitted route or the governing semantics changes. |
| What is contextual negation? | An oriented, typed, supported `NegationUse` with immutable use identity. Classical formula negation remains distinct. | An accepted semantic successor unifies or replaces the two notions. |
| May several negation routes be merged implicitly? | No. Keep frontiers and returns tagged by use. The same exterior under different uses may have different return fibers. | An explicit collective-coverage certificate justifies the declared aggregation while preserving provenance. |
| Is semantic negation coverage the same as executable coverage? | No. Record semantic and execution coverage separately. | A proof establishes their coincidence in the declared scope. |
| What is a positive exterior? | A section filling produced under an admitted oriented `NegationUse` and supported by a `DepartureWitness`; not a Boolean complement or boundary projection. | An accepted semantic successor changes exteriority. |
| What is a pure return? | The reverse section of the same `NegationUse`. The selected return is not the return fiber; exact closure requires a singleton protected quotient of the entire fiber under declared coverage. | An accepted semantic successor changes return semantics. |
| What establishes recovery? | Constancy of the declared protected signature over the return fiber. Local, web, and family signatures remain distinct; a product signature may distinguish what either factor alone cannot. | A protected breaker changes the admitted signature or recovery law. |
| How is reciprocal sixfold structure obtained? | Dependently and asymmetrically: source presentation, first use and departure, exterior, same-use return and recovery, supported seed/reorientation, second use and departure, second exterior, same-use return and recovery, residuals, then `Gamma_D` as a post-check. | An accepted semantic successor changes the dependency chain. |
| Can `Gamma_D` generate missing roles? | No. It checks a completed or partial occurrence downstream; it does not manufacture exterior, return, recovery, or seed evidence. | An accepted semantic successor changes its type and authority. |
| Does pure return revise standing meaning? | No. Semantic reconciliation, retraction, or revision is separate and requires its own actualization, check, and warrant. Compatible monotone constraint addition cannot change an already determined protected class. | A warranted successor changes the standing/revision contract. |
| Is the sixfold occurrence authoritative storage? | No. It is a derived, use-tagged history view. Partial attempts remain ordinary events; reproducible frontiers, fibers, recovery profiles, and charts remain derived. | Replay or performance evidence establishes a protected need for authoritative materialization. |

Do not resolve currently queued specification/plan differences merely because nearby work touches them. In particular, preserve the Phase 6 event/attempt-record question until its frontier is live.

## 3. The principal development program

Apply this program to consequential work. Collapse trivial steps whose answers are already settled, and do not perform the full reciprocal challenge when none of its possible returns can change a protected continuation.

### 3.1 Ensure the current determination

Run `ENSURE` before editing:

- identify the explicit task; if no narrower task is active, bind the task to the standing objective in `PERSISTENT_CODEX_GOAL.md`; identify the authorization boundary, acceptance conditions, and behaviors that must remain invariant;
- identify the governing sources by question type and their current versions;
- inspect the smallest sufficient repository actuality, including relevant files, symbols, data flow, tests, schema, history, and worktree state;
- state the protected scope, applicability, grain, horizon, and admitted checks;
- preserve unrelated user changes and identify operations that require new authority;
- initialize a state fingerprint, a finite live frontier, and decreasing fuel for recurrence.

Repository search yields candidate loci, not responsibility. Confirm the forward path from a candidate to the requested behavior before treating it as causal.

For consequential design/debugging, construct the software frame before selecting a patch: identify the relevant boundary, admissible occupants, actual occupants, arrangement, typed relational paths, and protected consequences. Do not substitute file proximity, symbol proximity, or lexical similarity for relational responsibility. This is an engineering search discipline, not a new calculus primitive or runtime opcode.

### 3.2 Resolve an early stop if one is justified

Classify the current state using exactly one of the six stop forms defined below. Do not continue merely because continuation is customary. Do not stop merely because a search returned nothing.

If no justified stop applies, continue with the live question that most changes the possible continuations.

### 3.3 Determine the witnessed difference

Establish:

```text
current observable:
required observable:
protected difference:
scope / applicability / grain / horizon:
independent discriminator:
```

Treat the current contract and implementation as a determination presentation, not as all possible facts. Generate interior variations that preserve the declared presentation and protected horizon. A candidate becomes an exterior only when positive evidence witnesses a relevant departure. Failure to find equality, code, evidence, or a separator remains `Unknown` unless an admitted exhaustive route says otherwise.

An informative near-departure is an interior candidate whose failed departure check identifies a missing feature, route, or coverage obligation. Preserve it as generator evidence for the next question; never promote it to an exterior.

When the requirement/implementation mapping is unclear, challenge it in both orientations:

1. What does the requirement establish without adapting it to the current code?
2. What implementation obligation follows?
3. Which part was genuine requirement, and which part was presentation or unsupported assumption?
4. What does the implementation establish without interpreting it through intent?
5. What contract does that implementation actually realize?
6. What changes when the standing requirement is imposed?
7. What incompatibility survives both orientations?

The surviving incompatibility, not the prose disagreement, becomes the next question.

### 3.4 Localize and challenge the candidate

Ask which smallest code, data, configuration, dependency, authority, or representation boundary can account for the witnessed difference. If several loci remain live, run `SEPARATOR` before broad editing.

Look for an existing typed realization in this order:

```text
reuse
over composition
over extension
over a new abstraction
```

An abstraction earns its place only by preserving a witnessed independent variation. A dependency earns its place only by discharging a current requirement.

Positively challenge every consequential candidate:

- **Requirement:** find a supported case in which it is inapplicable, ambiguous, or protected-different.
- **Hypothesis or locus:** generate a rival explanation and seek a separator.
- **Patch:** construct the smallest wrong implementation the proposed check must reject.
- **Test:** name at least two behaviors it distinguishes and its coverage boundary.
- **Abstraction or rule:** subtract or vary it and check whether protected behavior changes.
- **Evidence:** distinguish the raw return from its interpretation and from its warrant.

Use `RECIPROCAL_CHALLENGE` when the possible return could change architecture, meaning, acceptance, or another protected continuation.

### 3.5 Generate the smallest reversible patch and seal its prediction

A patch is a generated hypothesis. It should touch only what its explanation requires, preserve unrelated behavior and user changes, avoid speculative cleanup, avoid weakening types or authority boundaries, and remain easy to reverse.

Before applying it, seal:

```text
should change:
should remain invariant:
decisive discriminator:
important assumptions:
declared coverage:
```

Do not rewrite the prediction after seeing the result.

### 3.6 Act, preserve actuality, check, and form the residual

Perform only the smallest authorized action selected by the supported answer. Preserve the raw return before decoding or interpretation. Inspect the actual diff or external state; an intended action is not an actual action.

Run the smallest decisive check first, then directly affected checks, then required repository gates. Interpret their returns against the sealed prediction.

Construct a residual rather than reducing the result to `pass` or `fail`:

- no relevant residual;
- original difference persists;
- protected regression;
- wrong locus or assumption;
- missing dependency, representation, authority, or coverage;
- insufficient discriminator;
- provider or environment failure;
- unknown actual outcome;
- performance or resource boundary.

Do not repeat essentially the same question or patch after the same answer and evidence. A new attempt requires a new distinction explaining why its continuation differs.

### 3.7 Subtract, regenerate, and recur

After a candidate satisfies its decisive checks, run `SUBTRACT` when removal could reduce the patch, test, abstraction, rule, fixture, or dependency without losing demonstrated behavior. Prefer the smaller protected-equivalent realization.

Run `REGENERATE` when a result is meant to be reusable, folded, canonical, or recoverable. Reconstruct it from its declared basis, compare protected signatures and provenance, and challenge the reconstruction once more. If hidden structure cannot be recovered when its distinction becomes live, open the corresponding representation or binding problem.

Recur only when at least one of these changed:

- an actual return;
- an admitted distinction or separator;
- the representation or binding language;
- repository or external state;
- governing authority;
- the live finite frontier strictly decreased.

Every recurrence consumes fuel and records a new state fingerprint. When neither the fingerprint nor the frontier changes, stop rather than simulate progress.

## 4. Named subprograms in engineering form

These names denote first-order program templates, not hidden model policies.

### `ENSURE`

**Input:** task request and current execution context.
**Return:** an `EnsuredContext` containing authority by question, task contract, protected horizon, observed state, permissions, worktree baseline, admitted checks, evidence references, state fingerprint, and fuel; or a justified stop.

1. Probe the current sources and worktree.
2. Purely derive which source governs each live question.
3. Derive the task contract and protected invariants.
4. Check source compatibility in the affected scope.
5. Return `Unknown` for unresolved authority ambiguity, or `Blocked` when the required source or permission is unavailable.

### `SEPARATOR`

**Input:** two or more live alternatives, the protected horizon, admitted observation language, and available evidence.
**Return:** a witnessed separator, protected equivalence with a coverage certificate, a representation gap, or `Unknown`.

1. Generate a candidate observation whose possible returns distinguish the alternatives.
2. Check that the candidate actually has different predicted returns under the alternatives.
3. Probe it if actuality is required, then check the raw return against the predictions.
4. If no current expression can state the needed observation, open a representation/binding-extension problem.
5. Declare `Equivalent` only with sufficient admitted-language and coverage evidence. Exhausted search without such evidence is `Unknown`.

### `RECIPROCAL_CHALLENGE`

**Input:** a consequential determination, candidate continuation, protected signature, and declared coverage.
**Return:** a use-tagged dependent occurrence, residuals, and a continuation-relevant result.

Use this dependency order:

```text
W_X
-> NegationUse_X + DepartureWitness_X
-> O_X
-> same-use ReturnFiber_X
-> selected R_X + Recovery_X
-> supported seed / reorientation S_Y
-> NegationUse_Y + DepartureWitness_Y
-> O_Y
-> same-use ReturnFiber_Y
-> selected R_Y + Recovery_Y
-> residuals
-> Gamma_D post-check
```

At each orientation:

- preserve the immutable `NegationUse` identity and do not merge frontiers by default;
- separate semantic coverage from routes that are actually executable now;
- require positive, relevant, non-circular departure evidence;
- preserve all supported return candidates, not only the selected one;
- check recovery over the entire declared fiber and protected signature;
- record `Unknown` where coverage is incomplete.

One-way success does not establish reciprocal success. The second orientation depends on supported seed/reorientation from the first. `Gamma_D` runs only after the dependent roles exist and cannot fill missing roles. Pure return cannot revise the standing determination; route a required state change into a separate reconciliation question with its own probe, check, and warrant.

### `SUBTRACT`

**Input:** a validated candidate, its components, protected behavior, and decisive checks.
**Return:** a smaller validated candidate plus necessity witnesses for retained components, or the original candidate with a residual.

1. Generate a reversible removal order.
2. Remove or bypass one candidate-owned component at a time in an isolated or exactly reversible trial.
3. Probe the trial return and run the decisive check.
4. Keep the subtraction when protected behavior remains equivalent under declared coverage.
5. Restore only the exact trial change when the check witnesses necessity; never use broad destructive Git operations for restoration.

### `REGENERATE`

**Input:** a validated result, its declared regenerative basis, protected signature, provenance, and recovery horizon.
**Return:** a checked reconstruction, a residual, or a representation/binding-extension problem.

1. Generate a reconstruction from the declared basis without consulting hidden structure that the basis claims to replace.
2. Probe any external returns used by reconstruction.
3. Check protected signature, provenance, and recoverability against the expansion.
4. Apply a reciprocal challenge when the reconstruction will become reusable authority or a fold.
5. Preserve both the compact basis and the evidence needed to reopen it when a future distinction becomes live.

## 5. Stops, recurrence, and closure

Use exactly these stop states:

- **`Satisfied`** — the task contract is met, preserved behavior is checked, required authority has accepted the result, and no live acceptance-changing residual remains under declared coverage.
- **`Equivalent`** — alternatives are protected-equivalent in the declared scope, horizon, admitted language, and coverage. This closes only that branch; it is not task success by itself.
- **`Impossible`** — a certificate establishes impossibility within the declared representation, assumptions, scope, and horizon. `Not found` is not impossible.
- **`Blocked`** — a specifically identified required capability, permission, authority, dependency, or external return is unavailable.
- **`Unknown`** — current evidence cannot distinguish the live alternatives or coverage is incomplete. Unknown is neither positive nor negative evidence.
- **`ResourceBounded`** — a declared finite limit was reached; return the best supported partial result, unspent questions, and residual rather than claiming completion.

Guard recurrence with:

```text
fingerprint = hash(
  question identity,
  normalized bindings,
  governing authority versions,
  evidence references,
  repository/external state identity,
  protected horizon,
  live frontier
)
```

Maintain a set of seen `(fingerprint, supported_answer)` pairs and a decreasing `fuel`. Never issue the same question with the same bindings and evidence after the same answer. A larger resource budget changes the continuation only when explicitly authorized and recorded.

Semantic, control, and operational recurrence may revisit a form under new evidence or bindings. Positive warrant ancestry must remain acyclic: no accepted claim, patch, checker, or successor may directly or indirectly supply the warrant by which it becomes standing.

Task closure is itself a warranted question:

> Under the current task contract, protected horizon, and admitted checks, does any unresolved implementation distinction remain whose supported alternatives would change acceptance?

Only `Satisfied` is successful task closure. Tests passing is no stronger than the distinctions and coverage those tests encode.

## 6. Tests and change gates

Treat a test as an executable question. Before adding or relying on it, state which wrong behavior it rejects that current evidence does not. A useful test distinguishes at least two possible implementations.

Prefer this order:

1. smallest targeted discriminator;
2. directly affected unit, integration, property, migration, or conformance tests;
3. relevant type, lint, build, replay, or rendering checks;
4. required repository-wide gates.

For the Rust workspace, when applicable, the baseline gates are:

```bash
cargo fmt --all --check
cargo check --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

Do not weaken a valid test to make a candidate pass. If a contract changed, establish and warrant why the predecessor test no longer represents accepted behavior before replacing it. Mocks are valid only when they preserve the boundary the test is meant to discriminate.

Ask the corresponding gate question only when the change touches that area:

### Types, relations, and program representation

> Does the change preserve represented type, bindings, provenance, and behavior at every affected boundary?

Run type/IR, substitution, answer-binding, composition, and round-trip fixtures.

### Positive negation and reciprocal boundaries

> Does the change preserve use identity, positive departure, coverage distinctions, same-use return fibers, recovery signatures, dependent seeding, residuals, and downstream-only `Gamma_D`?

Run one-way, multi-use, incomplete-coverage, non-circularity, return-fiber, joint-recovery, reconciliation, and dependent-sixfold breakers. Boundary projection, failed search, and generic non-equivalence must not pass as negation witnesses.

### Compiler, renderer, and lowering

> Can a protected consumer distinguish source meaning before and after lowering or rendering?

Run source-program preservation, answer-path, use/provenance, partial-state, and rendering-contract fixtures.

### External effects, persistence, and history

> Can replay reconstruct the same accepted state from immutable raw historical returns and versioned code?

Run replay, crash/restart, corruption, migration, and historical-version fixtures. Preserve request, attempt, raw return, interpretation, check, and warrant as distinct when the standing schema requires them.

### Standing and acceptance

> Can this rule make an unsupported cycle, generated claim, merely applicable relation, or pure return appear accepted or revised?

Run full-standing, rootless-cycle, actuality, revision, and no-self-warrant fixtures.

### Retrieval and active context

> Does failure to retrieve or activate something become an unsupported claim that it is absent or irrelevant?

Run retained/access/active separation fixtures.

### Folding, compression, and learned methods

> Can a protected future distinguish the fold from its expansion, and can hidden structure be recovered when that distinction becomes live?

Run preservation, reconstruction, provenance, recovery, and reopening fixtures.

### Binding and representation extension

> Does the change add a name, add a genuinely new observation while preserving old meanings, or alter old meanings?

Classify the case before transporting old evidence or reopening old folds, then run the corresponding conservativity or successor fixtures.

### Documentation and semantic authority

> Does the rendered and referenced document state the accepted contract without retaining a protected-incompatible predecessor claim?

Compile/render the authority source, check references and stale semantic assertions, and inspect affected regions. A syntactically valid document is not by itself a semantic check.

## 7. Review as a final question pass

Before closure, inspect the actual diff and returns:

1. What required behavior does each consequential changed region serve?
2. Did anything change only because it was nearby?
3. Can any component be subtracted while preserving the demonstrated result?
4. Did the patch weaken a type, check, error boundary, durability rule, or authority boundary?
5. Did it affect public APIs, schemas, migrations, serialization, replay, or documentation authority?
6. What new failure modes arise at concurrency, persistence, security, untrusted-input, or external-effect boundaries, and which check distinguishes them?
7. Does the final state contain accidental formatting churn, generated-file edits, dead code, stale comments, duplicate authority, or unused abstraction?
8. Do targeted and required gates pass on the actual final state?
9. Are proposal, actuality, check, warrant, return, reconciliation, and residual still visibly distinct?
10. Does any remaining supported alternative change acceptance?

## 8. Durable records and evidence traces

When an answer becomes stable and repeatedly useful, record enough to recover why it is cached:

```text
question:
answer:
scope / applicability / grain / horizon:
evidence:
authority and mode:
reopen when:
```

- Use `DECISIONS.jsonl` for consequential accepted implementation choices.
- Use `FAILURES.jsonl` only for observed failures whose actual return constrains future work.
- Update `IMPLEMENTATION_FRONTIER.md` when the strongest unresolved implementation question changes.
- Update `CONFORMANCE_STATUS.md` only to the extent demonstrated by executable evidence.
- Preserve superseded decisions as history and record what reopened them.

For consequential work, expose an inspectable evidence trace containing:

- question identity and human-readable conclusion;
- scope, applicability, grain, horizon, and mode;
- declared answer source and evidence references;
- supported answer or stop classification;
- selected continuation and authorized action;
- raw-return location or digest when safe;
- check result, coverage, and residual.

This trace is evidence and provenance, not private chain-of-thought. Do not request, store, or publish hidden reasoning, model scratch work, credentials, secrets, personal data, or sensitive raw returns. Report concise conclusions, observations, predictions, discriminators, and residuals. Redact or reference sensitive actuality rather than reproducing it.

## 9. Git, filesystem, and repository hygiene

Use Git as evidence of what changed. Before editing a possibly dirty worktree, inspect:

```bash
git status --short
```

Before closure, inspect:

```bash
git diff --check
git status --short
git diff
```

- Preserve unrelated user changes and work around them. Stop for direction only when safe isolation is impossible.
- Do not create, delete, reset, amend, rebase, rewrite, commit, or push history unless the task authorizes it.
- Never use destructive broad operations such as `git reset --hard`, broad `git clean`, or recursive deletion to restore a trial.
- Resolve exact filesystem targets before destructive or recursive actions. Prefer recoverable operations and isolated temporary directories.
- Do not commit secrets, credentials, environment files, build artifacts, local databases, or large generated outputs unless explicitly required by the contract.
- Keep generated files generated: edit the source when one exists.
- Treat committed migrations and identity encodings as compatibility boundaries; do not rewrite them casually.
- Follow conventions discovered in the affected scope and avoid formatting unrelated files.
- External writes, messages, deployments, releases, acceptance changes, and irreversible operations require explicit task authority.

## 10. Scope and economy

A question, abstraction, dependency, cache, table, service, or framework must earn its place by changing a possible protected continuation.

Avoid speculative layers, premature generalization, duplicate authority, caches that become authority, architecture copied from ancestry without a current witness, and future-proofing without an observed distinction.

When two realizations satisfy the same protected contract, prefer the one that is smaller, more inspectable, easier to test and reverse, less stateful, and less dependent on hidden behavior. Do not confuse more questioning, abstraction, or output with more inquiry.

---

# II. Normative first-order inquiry program

## 11. Formal kernel

The engineering protocol above compiles to this kernel exactly:

```text
Mode ::= Pure | Generate | Probe | Check | Warrant

q =
  ⟨ ?_I R[β],
    scope,
    applicability,
    grain,
    horizon,
    mode : I → Mode ⟩
  : Question(A(q))

A(q) = ∏_{i∈I} X_i

SuppAns(q) =
  { S ⊆ A(q)
    | S ≠ ∅
    ∧ SupportedByDeclaredRoute(q, S) }

IProg<A> ::=
    Return_I { value : A }
  | Ask {
      question    : Question<T>,
      answer_slot : Var<SuppAns(question)>,
      continuation: IProgExpr<A>
    }
```

`?_I R[β]` is an indexed open relation with explicit bindings `β`. Its answer carrier is the product of the indexed component carriers. `SuppAns(q)` deliberately permits a nonempty supported subset rather than requiring a fabricated total or singleton answer.

The mode annotation is per answer component. Every component must be supported by its declared route; a lower route cannot discharge a higher port. Mixed-mode questions preserve the provenance of each component.

`continuation` is a capture-safe, inspectable syntax tree with explicit environments, answer variables, finite branch tables, and named subprogram references. It is not a host closure, callback, hidden model policy, or unrecorded prose choice. Named calls below are first-order templates that expand to `Return_I` and `Ask`; their arguments and answer bindings are data.

Use this presentation sugar only for readability:

```text
ASK q AS a THEN K[a]
RETURN v
CALL P(args) AS r THEN K[r]
CASE value OF { pattern_j => Program_j }
```

`K[a]` means capture-safe substitution of `a` into a stored continuation expression. `CALL` expands a named template. `CASE` is a finite branch table in that expression. None denotes an opaque function.

When a displayed program writes `CASE supported_answer OF`, it abbreviates a supported partition of the full set `SuppAns(q)`. A single branch may be selected only when every returned member lies in the same compatible answer cell. Otherwise the program preserves a first-order branch family for the distinct supported members, records unanswered components as `Unknown`, or asks an admitted refinement question; it never treats a supported set as an arbitrary singleton.

Question identity includes the relation, bindings, bounds, modes, declared sources, and schema version. An answer selects a continuation by matching the stored branch table; the agent does not choose a different branch after seeing an inconvenient return.

## 12. Program data

```text
StopState<A> ::=
    Satisfied       { value : A, evidence, coverage, warrant }
  | Equivalent      { alternatives, scope, horizon, coverage_certificate }
  | Impossible      { proposition, representation, assumptions, certificate }
  | Blocked         { requirement, unavailable_capability, evidence }
  | Unknown         { live_alternatives, missing_evidence_or_coverage }
  | ResourceBounded { limit, partial_answer, frontier, residual }

EnsuredContext ::= {
  task,
  authority_by_question,
  contract,
  protected_scope,
  applicability,
  grain,
  horizon,
  observed_state,
  permission_boundary,
  worktree_baseline,
  admitted_checks,
  evidence_refs,
  seen_fingerprints,
  live_frontier,
  fuel
}

Residual ::= {
  prediction,
  raw_return_ref,
  check_return,
  coverage,
  preserved_invariants,
  unresolved_differences,
  next_question_candidates
}
```

Each raw external return is recorded before a separate decode/interpretation question. Sensitive returns may be stored behind a safe reference or digest, but their existence, source, and redaction must remain visible.

## 13. Principal program

```text
DEVELOP(task, initial_context) =
  CALL ENSURE(task, initial_context) AS ensured THEN
  CASE ensured OF {
    StopState stop => RETURN stop,
    EnsuredContext ctx => CALL STEP(ctx)
  }

STEP(ctx) =
  ASK q_stop(ctx) AS stop_answer THEN
  CASE stop_answer OF {
    Satisfied s       => RETURN s,
    Equivalent e      => CALL CLOSE_BRANCH_OR_CONTINUE(ctx, e),
    Impossible i      => RETURN i,
    Blocked b         => RETURN b,
    Unknown u         => RETURN u,
    ResourceBounded r => RETURN r,
    Continue          => CALL SELECT_LIVE_QUESTION(ctx, none)
  }

SELECT_LIVE_QUESTION(ctx, prior_residual) =
  ASK q_information_gain(ctx.live_frontier, prior_residual) AS q THEN
  ASK q_discharge(q, ctx.authority_by_question) AS supported_answer THEN
  ASK q_positive_challenge(q, supported_answer, ctx.horizon) AS challenge THEN
  CASE challenge OF {
    NeedsSeparator alternatives =>
      CALL SEPARATOR(alternatives, ctx) AS separated THEN
      CALL CONTINUE_FROM_RETURN(ctx, separated),

    NeedsReciprocalChallenge candidate =>
      CALL RECIPROCAL_CHALLENGE(candidate, ctx) AS reciprocal THEN
      CALL CONTINUE_FROM_RETURN(ctx, reciprocal),

    SupportedContinuation candidate =>
      CALL PREPARE_ACTION(ctx, candidate)
  }

PREPARE_ACTION(ctx, candidate) =
  ASK q_existing_mechanism(candidate, ctx.observed_state) AS mechanism THEN
  ASK q_smallest_patch(candidate, mechanism) AS patch THEN
  ASK q_prediction(patch, ctx.contract, ctx.horizon) AS sealed_prediction THEN
  ASK q_authorized_action(patch, ctx.permission_boundary) AS raw_action_return THEN
  ASK q_decode_without_overwriting(raw_action_return) AS interpreted_return THEN
  ASK q_execute_admitted_discriminator(
        sealed_prediction, interpreted_return) AS raw_check_return THEN
  ASK q_decisive_check(
        sealed_prediction, interpreted_return, raw_check_return) AS check_result THEN
  ASK q_residual(
        sealed_prediction, raw_action_return, raw_check_return, check_result) AS residual THEN
  CALL AFTER_CHECK(ctx, patch, residual)

AFTER_CHECK(ctx, patch, residual) =
  CASE residual OF {
    NoRelevantResidual =>
      CALL SUBTRACT(patch, ctx) AS minimal THEN
      CALL REGENERATE_IF_REQUIRED(minimal, ctx) AS regenerated THEN
      ASK q_acceptance(regenerated, ctx.contract) AS warrant THEN
      CALL CLOSE_OR_RECUR(ctx, regenerated, warrant),

    RepresentationGap gap =>
      RETURN Unknown { live_alternatives: gap,
                       missing_evidence_or_coverage: binding_extension_required },

    ConsequentialResidual r =>
      CALL CLOSE_OR_RECUR(ctx, r, no_warrant)
  }

CLOSE_OR_RECUR(ctx, result, warrant) =
  ASK q_closure(ctx.contract, result, warrant, ctx.admitted_checks) AS closure THEN
  CASE closure OF {
    Satisfied s => RETURN s,
    stop @ (Impossible | Blocked | Unknown | ResourceBounded) => RETURN stop,
    Equivalent e => CALL CLOSE_BRANCH_OR_CONTINUE(ctx, e),
    Continue next => CALL GUARDED_RECUR(ctx, next)
  }

GUARDED_RECUR(ctx, next) =
  ASK q_next_fingerprint(ctx, next) AS fp THEN
  CASE (ctx.fuel,
        fp in ctx.seen_fingerprints,
        progressed(next, ctx)) OF {
    (0, _, _) =>
      RETURN ResourceBounded { limit: fuel,
                               partial_answer: next.partial,
                               frontier: next.live_frontier,
                               residual: next.residual },
    (_, true, _) =>
      RETURN Unknown { live_alternatives: next.live_frontier,
                       missing_evidence_or_coverage: repeated_state },
    (_, false, false) =>
      RETURN Unknown { live_alternatives: next.live_frontier,
                       missing_evidence_or_coverage: no_admitted_progress },
    (_, false, true) =>
      CALL STEP(ctx updated with next, fp, and fuel - 1)
  }
```

`progressed(next, ctx)` holds only when the transition preserves a new actual return, admits an independently checked distinction, changes the representation or binding language, changes owned repository or external state and observes it, changes governing authority, or strictly reduces the finite live frontier. A different generated question identity by itself is not progress.

`q_authorized_action` is a `Probe` question and may execute only operations within the established permission boundary. Its raw return is bound before decoding. `q_execute_admitted_discriminator` is also a `Probe` whenever it runs a command, test, compiler, or external checker; `q_decisive_check` is the separate `Check` interpretation of that preserved return. `q_acceptance` is a `Warrant` when the affected contract requires acceptance. Generated patch content never crosses any of these boundaries by implication.

## 14. First-order subprogram definitions

### `ENSURE`

```text
ENSURE(task, c0) =
  ASK q_probe_authority_sources(task, c0) AS source_returns THEN
  ASK q_probe_worktree_and_relevant_state(task, c0) AS state_returns THEN
  ASK q_derive_authority_by_question(task, source_returns) AS authority THEN
  ASK q_derive_contract_and_horizon(task, authority) AS contract THEN
  ASK q_check_authority_compatibility(authority, contract) AS compatibility THEN
  CASE compatibility OF {
    compatible =>
      RETURN EnsuredContext {
        task: task,
        authority_by_question: authority,
        contract: contract,
        protected_scope: contract.scope,
        applicability: contract.applicability,
        grain: contract.grain,
        horizon: contract.horizon,
        observed_state: state_returns,
        permission_boundary: contract.permissions,
        worktree_baseline: state_returns.git,
        admitted_checks: contract.checks,
        evidence_refs: refs(source_returns, state_returns),
        seen_fingerprints: {},
        live_frontier: derive_frontier(contract, state_returns),
        fuel: contract.fuel
      },
    missing_source x =>
      RETURN Blocked { requirement: authority_source,
                       unavailable_capability: x,
                       evidence: source_returns },
    unresolved_conflict x =>
      RETURN Unknown { live_alternatives: x,
                       missing_evidence_or_coverage: governing_authority }
  }
```

The two initial probes precede derivation: remembered repository state cannot substitute for actuality.

### `SEPARATOR`

```text
SEPARATOR(alternatives, ctx) =
  ASK q_generate_separator(alternatives, ctx.horizon) AS candidate THEN
  ASK q_check_predicted_discrimination(candidate, alternatives) AS prediction THEN
  CASE prediction OF {
    non_discriminating =>
      CALL SEPARATOR_NEXT(candidate, alternatives, ctx),
    representation_missing gap =>
      RETURN Unknown { live_alternatives: alternatives,
                       missing_evidence_or_coverage: gap },
    discriminating =>
      ASK q_probe_separator(candidate, ctx.permission_boundary) AS raw_return THEN
      ASK q_check_separator_return(candidate, raw_return, alternatives) AS result THEN
      CASE result OF {
        witnessed w => RETURN w,
        exhaustive_equivalence cert =>
          RETURN Equivalent { alternatives: alternatives,
                              scope: ctx.protected_scope,
                              horizon: ctx.horizon,
                              coverage_certificate: cert },
        incomplete evidence =>
          CALL SEPARATOR_NEXT(evidence, alternatives, ctx)
      }
  }
```

`SEPARATOR_NEXT` recurs only through `GUARDED_RECUR`. Candidate exhaustion without an exhaustive-equivalence certificate returns `Unknown` or `ResourceBounded`.

### `RECIPROCAL_CHALLENGE`

```text
RECIPROCAL_CHALLENGE(candidate, ctx) =
  ASK q_present_W_X(candidate, ctx) AS W_X THEN
  ASK q_generate_tagged_negation_frontier_X(W_X) AS frontier_X THEN
  ASK q_check_NegationUse_X_and_coverage(frontier_X, W_X) AS checked_use_X THEN
  ASK q_warrant_or_select_admitted_NegationUse_X(checked_use_X, W_X) AS use_X THEN
  ASK q_generate_DepartureWitness_X(use_X, W_X) AS depart_candidate_X THEN
  ASK q_check_positive_DepartureWitness_X(
        depart_candidate_X, use_X, W_X) AS depart_X THEN
  ASK q_resolve_positive_exterior_X(use_X, depart_X) AS O_X THEN
  ASK q_return_fiber_same_use_X(use_X, O_X) AS fiber_X THEN
  ASK q_select_supported_return_X(fiber_X) AS R_X THEN
  ASK q_check_recovery_X(fiber_X, candidate.protected_signature) AS recovery_X THEN
  ASK q_generate_supported_seed_Y(
        W_X, use_X, O_X, fiber_X, R_X, recovery_X) AS S_Y THEN
  ASK q_check_seed_and_reorientation_Y(S_Y) AS seed_Y THEN
  ASK q_generate_tagged_negation_frontier_Y(seed_Y) AS frontier_Y THEN
  ASK q_check_NegationUse_Y_and_coverage(frontier_Y, seed_Y) AS checked_use_Y THEN
  ASK q_warrant_or_select_admitted_NegationUse_Y(checked_use_Y, seed_Y) AS use_Y THEN
  ASK q_generate_DepartureWitness_Y(use_Y, seed_Y) AS depart_candidate_Y THEN
  ASK q_check_positive_DepartureWitness_Y(
        depart_candidate_Y, use_Y, seed_Y) AS depart_Y THEN
  ASK q_resolve_positive_exterior_Y(use_Y, depart_Y) AS O_Y THEN
  ASK q_return_fiber_same_use_Y(use_Y, O_Y) AS fiber_Y THEN
  ASK q_select_supported_return_Y(fiber_Y) AS R_Y THEN
  ASK q_check_recovery_Y(fiber_Y, candidate.protected_signature) AS recovery_Y THEN
  ASK q_derive_reciprocal_residuals(
        W_X, use_X, depart_X, O_X, fiber_X, R_X, recovery_X,
        seed_Y, use_Y, depart_Y, O_Y, fiber_Y, R_Y, recovery_Y) AS residuals THEN
  ASK q_check_Gamma_D_downstream(
        candidate.D,
        W_X, use_X, O_X, fiber_X, R_X, recovery_X,
        seed_Y, use_Y, O_Y, fiber_Y, R_Y, recovery_Y,
        residuals) AS gamma THEN
  RETURN DependentReciprocalOccurrence {
    W_X, use_X, depart_X, O_X, fiber_X, R_X, recovery_X,
    seed_Y, use_Y, depart_Y, O_Y, fiber_Y, R_Y, recovery_Y,
    residuals, gamma
  }
```

Every question in this template carries explicit scope, applicability, grain, horizon, and per-component modes. An exterior may be resolved by `Generate` or `Probe` according to the required authority, but it cannot be resolved on an unadmitted use or unsupported departure, and only the `Probe` route establishes actuality. A partially answered frontier yields a partial occurrence with `Unknown` fields, never invented closure. Any request to change standing meaning exits this template into a separately warranted reconciliation program.

### `SUBTRACT`

```text
SUBTRACT(candidate, ctx) =
  ASK q_generate_reversible_subtractions(candidate) AS trials THEN
  CALL SUBTRACT_LOOP(candidate, trials, ctx, ctx.fuel)

SUBTRACT_LOOP(candidate, trials, ctx, 0) =
  RETURN ResourceBounded {
    limit: subtraction_fuel,
    partial_answer: candidate,
    frontier: trials,
    residual: local_minimization_incomplete
  }

SUBTRACT_LOOP(candidate, empty, ctx, fuel) =
  RETURN candidate with local_minimality_certificate(
    ctx.horizon, tested_subtractions)

SUBTRACT_LOOP(candidate, trials, ctx, fuel + 1) =
  ASK q_next_subtraction(trials, ctx.live_frontier) AS trial THEN
  CASE trial OF {
    none => RETURN candidate with local_minimality_certificate(
              ctx.horizon, tested_subtractions),
    some delta =>
      ASK q_probe_isolated_trial(candidate, delta) AS raw_trial_return THEN
      ASK q_check_protected_behavior(raw_trial_return, ctx.horizon) AS check THEN
      CASE check OF {
        protected_equivalent =>
          ASK q_remaining_subtractions(
                candidate minus delta, trials, delta) AS next_trials THEN
          CALL SUBTRACT_LOOP(
            candidate minus delta, next_trials, ctx, fuel),
        distinction_witnessed witness =>
          ASK q_probe_restore_exact_trial(delta) AS restore_return THEN
          ASK q_check_restoration(restore_return, candidate) AS restored THEN
          CALL SUBTRACT_LOOP(
            candidate with necessity(witness), trials - {delta}, ctx, fuel),
        unknown u =>
          ASK q_probe_restore_exact_trial(delta) AS restore_return THEN
          ASK q_check_restoration(restore_return, candidate) AS restored THEN
          RETURN candidate with residual(u, restored)
      }
  }
```

Trial operations are restricted to candidate-owned, isolated, exactly reversible state. They do not authorize broad resets, deletion, or mutation of unrelated work. Every structural recursion decrements its explicit local fuel and removes or regenerates a finite trial frontier; it is subject to the same no-repeated-state rule as the principal recurrence.

### `REGENERATE`

```text
REGENERATE(result, basis, ctx) =
  ASK q_generate_from_declared_basis(basis) AS reconstruction THEN
  ASK q_probe_required_regeneration_effects(reconstruction) AS raw_returns THEN
  ASK q_check_signature_provenance_and_recovery(
        result, reconstruction, raw_returns, ctx.horizon) AS comparison THEN
  CASE comparison OF {
    protected_equivalent cert =>
      ASK q_reciprocal_need(result, cert, ctx) AS need THEN
      CASE need OF {
        no  => RETURN result with regeneration_certificate(cert),
        yes => CALL RECIPROCAL_CHALLENGE(reconstruction, ctx)
      },
    distinction_witnessed residual => RETURN result with residual,
    representation_missing gap =>
      RETURN Unknown { live_alternatives: {result, reconstruction},
                       missing_evidence_or_coverage: gap },
    incomplete coverage =>
      RETURN Unknown { live_alternatives: {result, reconstruction},
                       missing_evidence_or_coverage: coverage }
  }
```

Generation proposes the reconstruction. Probe establishes any new actual returns. Check compares protected behavior. Warrant remains separate if the regenerated form is to become accepted authority.

## 15. Compilation obligations

Every consequential plain-language instruction in Part I must compile to:

1. a typed question with explicit bounds and modes;
2. a declared answer source;
3. a nonempty supported answer, possibly partial;
4. a stored branch table whose selected continuation depends on that answer;
5. a raw actual return for every external action;
6. an independent check when a protected prediction is at issue;
7. a warrant when an accepted contract changes;
8. a residual and guarded continuation or one of the six stops.

If compilation would require an opaque closure, hidden policy, fabricated total answer, untagged negation union, projection-as-exterior, selected-return-as-fiber, `Gamma_D`-generated role, pure-return revision, or self-warrant, the program is invalid. Open the exact representation or authority question instead.

The protocol succeeds when each consequential continuation is explained by the supported answer that selected it, that answer came through a route authorized for its component, the actual return was preserved, and no stronger claim is made than the checks and warrants establish.
