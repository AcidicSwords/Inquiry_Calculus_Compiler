# AGENTS.md

## Purpose

This repository is built by resolving software-development questions, not by following a fixed workflow regardless of what the repository returns.

A consequential action should be the answer-dependent continuation of a question that distinguished the state before the action.

Use this shape:

```text
current state
-> question whose possible answers would change what should happen next
-> supported answer from the right source
-> smallest action justified by that answer
-> actual result
-> next unresolved question
```

An instruction in this repository is a previously settled answer. Apply it directly while the question it answered still has the same answer in the current scope. If new code, requirements, evidence, or operating conditions make another answer possible, reopen that question rather than blindly following or silently overriding the old instruction.

Do not re-ask settled questions when nothing relevant changed.

---

## How to read questions

A question has an answer type and an answer source.

Use these answer sources:

- **Derive** — compute the answer exactly from already established repository facts.
- **Propose** — generate a candidate; the answer is provisional until checked where checking matters.
- **Observe** — inspect files, history, tools, builds, tests, runtime behavior, or external systems and preserve what actually returned.
- **Verify** — use an independent test, type checker, linter, compiler, comparison, proof, benchmark, or other discriminator.
- **Accept** — apply the repository's current acceptance authority when a change affects an accepted contract.

`Ask` means resolve the question from the strongest available source. It does **not** mean ask the user by default.

Ask the user only when:
1. the answer cannot be established from the repository, tools, current specifications, or other available evidence; and
2. different supported answers would lead to materially different implementation paths.

If all plausible answers lead to the same action, the question is not currently useful. Take the common justified action instead.

---

## Repository authority

For any task, first establish which current sources govern the behavior being changed.

Prefer current accepted sources over historical detail. Earlier plans, code, comments, chats, and superseded documents are evidence and ancestry; they do not override a newer accepted contract merely because they are more detailed or easier to find.

For this project, use these sources when present:

1. `IMPLEMENTATION_FRONTIER.md` — current unresolved implementation boundary.
2. `Inquiry_Calculus_v1_1_Comprehensive_Implementation_Plan.md` — current implementation-facing architecture.
3. the current canonical v1.1 specification and accepted additions — semantic contract.
4. `DECISIONS.jsonl` — accepted local implementation decisions and reopen conditions.
5. `FAILURES.jsonl` — observed failures and their current status.
6. `CONFORMANCE_STATUS.md` and tests — executable evidence of implemented behavior.
7. code and actual runtime/build/test returns — current implementation actuality.

A more local `AGENTS.md` may add narrower conventions for its directory. It may not silently invalidate a repository-wide invariant; if the two appear to conflict, resolve which question each file answers and whether their scopes actually overlap.

---

# Settled project answers

These are current answers to already-resolved project questions. Use them without reopening unless the stated condition becomes live.

| Question | Current answer | Reopen when |
|---|---|---|
| What language should the reference implementation use? | Rust with Cargo. | A protected capability cannot be implemented credibly in Rust, or measured constraints make the choice consequential. |
| What is the initial persistence architecture? | SQLite with one authoritative writer plus immutable content-addressed artifacts. | Measured throughput, durability, deployment, or concurrency requirements exceed it. |
| Should the initial system be distributed? | No. Start as a small single-process reference runtime. | A protected requirement or measured boundary requires distribution. |
| Should the project begin with a graph database or vector database? | No. Use exact relational/reference access first. | Logged misses demonstrate protected strict gain from another retrieval route. |
| Should a generic agent framework or workflow engine control the system? | No. Keep control in the project's own typed program/runtime. | A concrete protected behavior cannot be represented or executed without stronger machinery. |
| How should semantic/core values cross boundaries? | Through explicit typed forms and references, not generic untyped payloads. | A successor specification explicitly changes the type contract. |
| How should answer-dependent continuations be represented? | As first-order, inspectable, persistable program data; not opaque host-language closures. | A breaker shows the representation cannot preserve required behavior. |
| What is the executable runtime core? | Pure return, internal branching, and actual probe/effect. | A required effect cannot be represented as one of these without losing a protected distinction. |
| When is an external/model/tool result part of history? | Preserve the raw actual return before decoding or interpretation. | Do not reopen casually; changing this requires an explicit successor to the current authority model. |
| May generated output count as a check or acceptance merely because it is structured or confident? | No. Proposal, actuality, verification, and acceptance remain separate. | Only an explicit accepted authority change can alter this. |
| How much architecture should be added ahead of use? | The smallest reversible structure that realizes the current protected behavior. | A passing fixture exposes a new protected distinction requiring more structure. |
| What should happen when a current representation cannot express an independently witnessed difference? | Open a representation/binding-extension problem; do not search forever inside the same language or declare equivalence. | A separator is later found inside the existing admitted language. |

These answers are repository rules because the questions that generated them are presently settled.

---

# The development program

For a consequential task, resolve the following program. Do not mechanically ask every question: skip a question when its answer is already established and still applicable, or when all supported answers lead to the same continuation.

## 1. Establish the task contract

**Question — `TaskContract`**

> What exactly must be true when this task is complete, what existing behavior must remain true, and which current repository sources establish those requirements?

**Answer from:** Derive / Observe / Verify.

A valid answer identifies:
- requested behavior;
- preserved behavior;
- scope;
- affected public/internal contracts;
- relevant tests or acceptance criteria;
- unresolved ambiguity that could change implementation.

If two current sources appear to require protected-different behavior, that conflict is the next question. Do not code through it by guessing.

---

## 2. Establish current actuality

**Question — `ObservedState`**

> What does the repository actually contain and do at the boundary relevant to this task?

**Answer from:** Observe.

Use the smallest sufficient evidence:
- relevant files and symbols;
- callers/callees or data flow when consequential;
- existing tests;
- `git status` / current diff;
- build/type/lint results where relevant;
- reproduced runtime behavior where relevant;
- schema/migration state where relevant.

Do not substitute remembered architecture or plausible behavior for inspection.

Repository search produces candidate locations. Confirm the forward path from a candidate location to the requested behavior before treating it as responsible.

---

## 3. Establish a witnessed gap

**Question — `Gap`**

> What exact observable difference remains between the current implementation and the task contract?

**Answer from:** Propose, then Verify when the distinction is consequential.

A useful gap states both sides:

```text
current observable:
required observable:
protected difference:
```

If no protected before/after difference can be stated, do not invent a patch yet.

If no gap appears, resolve the closure question near the end of this file rather than assuming success from failure to find one.

---

## 4. Establish that the gap is real

**Question — `GapEvidence`**

> What independent inspection, executable fixture, type relation, failing test, trace, or other evidence distinguishes the proposed gap from an already-correct implementation?

**Answer from:** Observe / Verify.

Prefer a small discriminator.

For a bug or behavior change, when feasible create or identify a check that:
- fails on the relevant predecessor behavior; and
- passes on the intended successor behavior.

Do not write a test that can pass both the broken and intended behaviors and then treat it as evidence.

A passing test establishes only what that test can distinguish.

---

## 5. Relate requirement and implementation when the mapping is not obvious

Use this subprogram only when the requirement/implementation relation remains ambiguous.

### Requirement side

1. **What does the requirement establish on its own, without adapting it to the current code?**
2. **Given that requirement, what must the implementation therefore realize?**
3. **After accounting for real implementation constraints, what remains genuinely required and what was merely presentation, convenience, or an unsupported assumption?**

### Implementation side

4. **What does the implementation actually establish on its own, before interpreting it through the intended requirement?**
5. **Given that implementation, what contract does it actually realize?**
6. **After imposing the standing requirement, what must the implementation change, preserve, or expose?**

### Compatibility

7. **Can those answers all be true in the same scope? If not, what exact incompatibility remains?**

The incompatibility, not the prose disagreement, becomes the next implementation problem.

---

## 6. Localize responsibility

**Question — `Locus`**

> What is the smallest code, data, configuration, interface, or dependency boundary whose behavior can account for the witnessed gap?

**Answer from:** Observe / Derive / Verify.

Trace enough context to establish responsibility, not merely textual proximity.

If several candidate loci remain live:

> What observation or check would separate these candidate loci with respect to the witnessed behavior?

Resolve that separator before broad editing when the answer would materially change the patch.

---

## 7. Check for an existing realization

**Question — `ExistingMechanism`**

> Does the repository already contain a typed mechanism, abstraction, helper, test utility, storage path, or runtime path that can realize the required behavior without adding a new architectural concept?

**Answer from:** Observe / Derive.

When protected behavior is equivalent, prefer:

```text
reuse
over composition
over extension
over a new abstraction
```

A new abstraction is justified by a real independent variation or protected distinction, not by the possibility that it may become useful later.

---

## 8. Generate the smallest candidate change

**Question — `Patch`**

> What is the smallest reversible patch at the established locus that would remove the witnessed gap while preserving the task contract?

**Answer from:** Propose.

The candidate should:
- touch only what the explanation requires;
- preserve unrelated behavior;
- preserve user changes already in the worktree;
- avoid speculative cleanup;
- avoid new dependencies unless they discharge a real requirement;
- avoid weakening types, tests, or authority boundaries for convenience.

Generated code is a hypothesis, not proof that the task is solved.

---

## 9. Seal the prediction before checking the patch

**Question — `Prediction`**

> If this patch is the correct explanation of the gap, what should change, what should remain unchanged, and which actual check will distinguish success from failure?

**Answer from:** Propose / Derive.

Keep the prediction concise:

```text
should change:
should not change:
discriminator:
important assumptions:
```

Do not rewrite the prediction after seeing the result.

For trivial mechanical edits, the task contract and an obvious targeted check may already supply the prediction.

---

## 10. Apply the candidate and observe what actually happened

**Question — `EditResult`**

> What actually changed when the candidate patch was applied?

**Answer from:** Observe.

Inspect the actual diff and repository state. An intended edit is not an actual edit.

Do not discard or overwrite unrelated user changes.

Do not use destructive Git operations (`reset --hard`, broad `clean`, rewriting unrelated history) unless the task explicitly requires them and their consequences are understood.

---

## 11. Run the smallest decisive check, then the relevant regression gates

**Question — `CheckResult`**

> What do the relevant independent checks actually return for the predicted changed behavior and the behavior that was supposed to remain invariant?

**Answer from:** Observe / Verify.

Prefer this order:
1. smallest targeted discriminator;
2. directly affected unit/integration/property tests;
3. relevant type/lint/build checks;
4. required repository-wide gates.

For the current Rust reference implementation, when applicable, the expected baseline gates are:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Run migration, replay, compiler, standing, binding, or fold-specific gates when the change touches those contracts.

Do not run expensive unrelated suites merely as ritual unless repository CI or the task contract requires them.

Do not weaken an existing valid test to make a candidate patch pass. If the current contract genuinely changed, establish why the old test no longer represents the accepted behavior before changing it.

---

## 12. Interpret the check as a residual, not as a verdict word

**Question — `Residual`**

> Given the pre-edit prediction and the actual edit/check returns, what consequential difference remains unresolved?

**Answer from:** Derive / Verify, or Propose followed by Verify when classification itself is uncertain.

Typical answers include:
- no relevant residual;
- original gap persists;
- protected regression appeared;
- wrong locus;
- wrong assumption;
- missing dependency;
- missing representation;
- insufficient test;
- environment/provider failure;
- unknown actual outcome;
- performance/resource boundary.

The failure result is evidence. Let it change the next question.

Do not repeat essentially the same patch after the same failure unless a new answer explains why the next attempt is different.

---

## 13. Continue from the residual

**Question — `NextQuestion`**

> What unresolved question is now forced by the residual, such that different supported answers would lead to materially different implementation continuations?

**Answer from:** Derive / Propose.

Then resolve that question from the strongest available source and let its answer determine the continuation.

This is the default continuation after unexpected results. There is no universal "debug again" instruction.

Examples:

- If two loci remain possible:
  > What result would distinguish which locus actually controls the behavior?

- If a dependency may be missing:
  > Which dependency must be present for the expected path to be executable, and is it actually present?

- If the representation cannot express a witnessed distinction:
  > What representation, type, interface, probe, or binding would make the difference expressible and testable?

- If a new abstraction seems necessary:
  > What protected behavior can the new abstraction express that the existing structure cannot?

- If a provider/library choice becomes consequential:
  > Which protected behavior differs between the available choices under this task's actual requirements?

---

# Question selection

When several unresolved questions are available, prefer a question whose plausible supported answers most reduce the live implementation alternatives without unacceptable cost, risk, or irreversibility.

In plain terms, ask:

> Which question would most change what I should do next?

Prefer:
- exact checks over speculative discussion;
- local discriminators over broad exploratory refactors;
- repository evidence over user interruption;
- reversible probes over irreversible changes;
- questions that separate several live alternatives at once.

Do not ask questions merely because they are customary.

---

# Tests as questions

Treat a test as an executable question about the implementation.

A good test has at least two implementations it can distinguish: one that should pass and one that should fail.

Before adding or relying on a test, ask:

> What wrong behavior would this test reject that the current evidence does not already reject?

If the answer is "none", the test adds little discrimination.

For behavior fixes, prefer tests that witness the actual requested difference rather than tests coupled to the chosen internal implementation.

For refactors with no intended behavior change, existing externally meaningful tests may be the correct discriminator.

Mocks are useful only when they preserve the behavior being tested. Do not mock away the very boundary whose behavior the test is supposed to establish.

---

# Code review as a final question pass

Before considering the task complete, inspect the actual diff and resolve:

1. **What required behavior does each consequential changed region serve?**
2. **Did anything change only because it was nearby?**
3. **Can any changed region be removed while preserving the demonstrated result?**
4. **Did the patch weaken a type, check, error boundary, durability rule, or authority boundary to make implementation easier?**
5. **Are public APIs, schemas, migrations, serialization, replay, or documentation affected?**
6. **If concurrency, external effects, persistence, security boundaries, or untrusted input are involved, what new failure modes became possible and which check distinguishes them?**
7. **Does the final repository state contain accidental formatting churn, generated-file edits, dead code, stale comments, or unused abstractions?**
8. **Do the targeted and required regression checks pass on the actual final state?**

Prefer the smaller patch when two patches have the same protected behavior.

---

# Closure

A task is not complete merely because:
- the model believes the patch is correct;
- no more problems were noticed;
- search found no other relevant code;
- one test passed;
- the changed code compiles.

Resolve:

**Question — `TaskClosed`**

> Under the current task contract and the checks available in this repository, is there any remaining implementation distinction whose different resolution would change whether this task should be accepted?

Possible answers:

- **Closed** — no live acceptance-changing distinction remains under the current admitted checks.
- **Open** — a specific residual remains; continue from it.
- **Blocked** — a required capability or external return is unavailable.
- **Unknown** — current evidence cannot distinguish the remaining alternatives.
- **Impossible** — only when impossibility is actually established within the declared scope.

`Unknown` is not success.  
`Not found` is not impossible.  
`Tests pass` is not stronger evidence than the tests provide.

---

# Durable rules are cached answers

When a development answer becomes repeatedly useful and stable, it may become a repository rule or decision.

Store enough to recover why it is a rule:

```text
question:
answer:
scope:
evidence:
reopen when:
```

Use `DECISIONS.jsonl` for consequential implementation choices.

Use `FAILURES.jsonl` for failures whose return constrains future work.

Update `IMPLEMENTATION_FRONTIER.md` when the strongest unresolved implementation question changes.

Do not turn one incidental workaround into a permanent rule without establishing the question it answers and its applicability.

Do not keep a rule after evidence shows the question now has a different answer. Preserve the old decision as history and record what reopened it.

---

# Project-specific change gates

Ask the corresponding question only when the change touches that area.

### Types / relations / program representation

> Does the change preserve the represented type and behavior at every affected boundary?

Run the relevant type/IR and composition fixtures.

### Compiler / renderer / lowering

> Can a protected consumer distinguish the source meaning before and after lowering or rendering?

Run source-program preservation, path/provenance, and rendering-contract fixtures.

### External effects / persistence / history

> Can replay reconstruct the same accepted state from immutable historical returns and versioned code?

Run replay, crash/restart, and historical-version fixtures.

### Standing / acceptance logic

> Can the new rule make an unsupported cycle, generated claim, or merely applicable relation appear accepted?

Run full standing and rootless-cycle fixtures.

### Retrieval / active context

> Does failure to retrieve or activate something become an unsupported claim that it is absent or irrelevant?

Run retained/access/active separation fixtures.

### Folding / compression / learned methods

> Can any protected future distinguish the folded behavior from its expansion, and can the hidden structure be recovered when that distinction becomes live?

Run preservation, recovery, provenance, and reopening fixtures.

### Binding / representation extension

> Does the change merely add a new name, add a genuinely new observation while preserving old meanings, or change old meanings?

Classify and test the appropriate case before transporting old evidence or reopening old folds.

---

# Git and repository hygiene

Use Git as evidence of what changed.

Before editing when the worktree may already contain changes:

```bash
git status --short
```

Before completion:

```bash
git diff --check
git status --short
git diff
```

Do not overwrite unrelated changes.

Do not create, delete, reset, amend, or rewrite commits unless the task or repository workflow explicitly requires it.

Do not commit secrets, credentials, local environment files, build artifacts, or large generated outputs unless they are explicitly part of the repository contract.

Keep generated files generated: modify their source when one exists.

Follow repository formatting and naming conventions discovered in the affected scope rather than imposing unrelated preferences.

---

# Scope and economy

A consequential question or abstraction must earn its place by changing a possible continuation.

Avoid:
- speculative framework layers;
- premature generalization;
- unrelated refactors;
- architecture copied from an older plan without a current use;
- duplicate sources of truth;
- caches that become authority;
- "future-proofing" with no witnessed future distinction.

When two implementations satisfy the same current contract, prefer the one that is:
- smaller;
- easier to inspect;
- easier to test;
- easier to reverse;
- less stateful;
- less dependent on hidden behavior.

Do not confuse more abstraction with better design.

---

# Compact program

For reference, the development loop is:

```text
DEVELOP(task):

  contract <-
    "What exactly must be true, what must remain true,
     and which current sources establish it?"

  observed <-
    "What does the repository actually contain and do
     at the relevant boundary?"

  gap <-
    "What protected observable difference remains between
     the observed implementation and the contract?"

  evidence <-
    "What independent observation or check establishes
     that this gap is real?"

  if requirement <-> implementation mapping is ambiguous:
      reciprocal <-
        "What does each side imply about the other,
         what changes when that implication is admitted,
         and what incompatibility survives both directions?"

  locus <-
    "What smallest locus can account for the witnessed gap?"

  existing <-
    "Can existing repository machinery already realize
     the required behavior?"

  patch <-
    "What smallest reversible patch removes the gap
     while preserving the contract?"

  prediction <-
    "If this patch is right, what changes, what stays invariant,
     and what check separates success from failure?"

  edit_result <-
    "What actually changed when the patch was applied?"

  check_result <-
    "What do the relevant independent checks actually return?"

  residual <-
    "Given the prediction and the actual returns,
     what consequential difference remains unresolved?"

  if residual is empty:
      closure <-
        "Is there any remaining acceptance-changing distinction?"
      return closure

  next_question <-
    "What question is forced by this residual such that
     different supported answers change what should happen next?"

  answer <- resolve(next_question)

  continue from answer
```

The loop is successful when the next action is explained by the answer that precedes it, and the answer was obtained from a source with enough authority to support that action.
