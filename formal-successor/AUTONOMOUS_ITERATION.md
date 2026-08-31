# Autonomous formal-successor iteration

This runbook makes the branch objective resumable. It operationalizes, but does not replace,
`FORMAL_CALCULUS_CONSTRUCTION_SPEC.md`, `AGENTS.md`, or the one live block in
`../IMPLEMENTATION_FRONTIER.md`.

## Persistent objective

Advance the strongest live formal-successor residual toward one exact, well-typed,
machine-checked Inquiry Calculus that supersedes v2.0. The successor must make its notation,
prose, relations, question and program grammar, composition, methods, evidence, transformation,
compression, regeneration, and implementation correspondence precise enough to check and
regenerate.

The Rust workspace is predecessor evidence and, after Formal Gate F, a downstream implementation
target. It is not the objective. Before Gate F, do not change Rust semantics. After Gate F, touch
Rust semantics only through a checked successor-to-Rust correspondence delta and rerun both
conformance surfaces.

## Resume coordinate

Every autonomous run reconstructs its state from repository evidence instead of remembered prose:

1. Confirm branch `codex/formal-successor` and inspect the complete worktree without discarding
   unrelated changes.
2. Read the construction specification, this runbook, the exact live Frontier block, successor
   conformance, decisions, failures, and `ACTIVE_INPUTS.json`.
3. Verify the pinned input digests and run `node tools/successor_control_check.js`.
4. Read the current formal modules and the source surfaces named by the live residual.
5. Resume the active trace and its remaining fuel using `ic-trace state`. Initialize
   a new finite trace only after lawful task closure and Stop. Its first record pins
   `Questions.txt`, `ENGINEERING_QUESTION_PROGRAMS.json`, and the closed predecessor
   trace digest when present. A checkpoint does not silently reset fuel. When its
   canonical 24-Ask budget is exactly exhausted while a clean regenerated field still
   contains a live Required/Productive executable occurrence, continue the same task with:

       .claude/hooks/ic-trace resume reason="continue persistent autonomous task"

   This appends a policy-compatible `note/event=checkpoint_resume` bound to the latest
   checkpoint and field, then grants exactly one new 24-Ask ratchet. It requires current
   user-authorized harness control, cannot repeat for the same checkpoint, cannot operate
   before fuel reaches zero, and neither changes question priority nor claims closure.
6. Continue only the strongest live residual. Do not revive the deferred Rust frontier or invent a
   second moving cursor.

If repository evidence conflicts with remembered state, repository evidence wins. If governing
authorities conflict in the same scope, stop that branch as `Unknown` until authority is resolved.

## Persistent recursive ratchet

Each finite iteration is a replayable checkpoint in a persistent recursive process:

```text
RECONSTRUCT CURRENT STATE
-> SELECT ONE LIVE RESIDUAL
-> REBUILD THE DERIVED RELATIONAL SURFACE
-> MATERIALIZE A FINITE LIVE QUESTION FIELD
-> ASK ONE REPRESENTED EXECUTABLE OCCURRENCE
-> CONSTRUCT A DECISIVE ADMISSIBLE CONTRAST
-> FOR EFFECTFUL DISCHARGE: SEAL -> OPERATE -> RAW -> INTERPRET -> CHECK
-> ANSWER WITHOUT EXCEEDING THE RETURN'S AUTHORITY
-> REIFY EXPLICIT PRODUCTS, DEPENDENCIES, BREAKERS, HOLES, FOLDS, AND REOPENINGS
-> REGENERATE THE QUESTION FIELD
-> LOCALIZE BY PARTITION / SUBTRACTION / BACKOFF
-> BUILD THE SMALLEST TYPED SURVIVOR
-> COMPUTE AND RECHECK DEPENDENCY CLOSURE
-> CHALLENGE WITH A STRONGER BREAKER
-> ABLATE SINGLY AND JOINTLY
-> RERUN AFFECTED CORPUS AND REPRESENTATION CHECKS
-> RERUN SUCCESSOR REGENERATION AT DECLARED COVERAGE
-> RATCHET EVIDENCE AND THE NEXT SINGLE RESIDUAL
-> RECORD A CHECKPOINT
-> RECUR
```

A checkpoint is not task closure. Task closure is a separate, adversarially challenged event and is
lawful only when no Ask, unreified Answer, dirty field, open actual cycle, or newly materialized
required/productive executable question remains. Blocked, Unknown, ResourceBounded, and reopening
conditions remain explicit rather than disappearing.

The exploration and commitment scales are different. Cast a wide admissible net of contrasts,
alien cases, reversals, removals, paths, joint variations, contradictions, and blockers; commit
only the smallest relation forced by checked returns. A local ratchet removes only resolved regions
from the broad residual topology. Preserve remaining active, blocked, latent, and reopened
obligations with ancestry, propagate through every obligation sharing the changed conditions or
roles, factor genuine recurrences, and then expand again.

### 1. Reconstruct and select

Establish the current formal candidate, the active phase and gate, demonstrated evidence, open
obligations, and predecessor surfaces in scope. Select the Frontier residual whose supported
answers can cause the greatest protected difference. Do not select work because it is convenient
for Rust or because a nearby Lean declaration is easy to add.

### 2. Frame the inquiry

Reconstruct the explicit engineering surface, then materialize applicable question occurrences
from the schema-4 generator registry and pinned source forms. Bind form, rendering, occurrence,
path, environment, scope, horizon, coverage, authority, provenance, and evidence route. Identify:

```text
current observable
decisive contrasting observable
protected difference
decisively wrong admissible foil or overstrong candidate
independent discriminator
invariants
```

The question field constrains process; it does not supply an Answer or warrant. There is no
semantic next-question oracle, fixed residual schedule, or universal score. Select from the live
field only through explicit required-discharge, dependency, resource, effect, risk, coverage, or
Frontier relations. When lawful occurrences remain incomparable, execution policy may choose one;
the others remain live. The Q1--Q14 families and roots classify ordinary relational generators and
must remain erasable; they are not a fixed semantic wheel.

#### Bounded relation-instance construction

The implemented constructor is `.claude/hooks/ic-question-instance.js`. Reified products may
declare an `inquiry_carrier`, an `inquiry_relation` with ordered `{name, carrier}` roles, or an
`inquiry_seed`. These are provisional engineering declarations, not checked successor types.
A seed names a corpus `question_form`, a previously reified `relation_product`, a role-to-reference
`bindings` map, a nonempty ordered `open_roles` tuple, and an occurrence `path`. Every role must be
exactly bound or open. Product references use `{kind: "product", id: "..."}`; previously
materialized ordinary questions use `{kind: "question", id: "..."}` with carrier `Question`.
The latter need not have an Answer. There is no separate MetaQuestion constructor.

The reified seed must retain every dependency read by generation, including the dependencies of a
question used as a subject. Its applicability, horizon, and coverage are retained in the generated
instance. `ic-relational-surface.js json ROOT` exposes `generated_questions`: deterministic
corpus-anchored prompts with their exact relation, role bindings, open tuple, ancestry, and path.
Merge these into the next field without dropping unchosen occurrences. The append gate refuses
field regeneration that omits a newly reified seed, forged instance meaning, dangling references,
wrong carrier declarations, overlapping bound/open roles, or an executable invalidated dependency.
An Ask must preserve the instance's canonical binding JSON, dependency list, horizon, and coverage.

Generation defaults to `Unknown` and non-executable. A declared discharge route or actual required
obligation must separately justify readiness. Querying a relation does not assert that it holds or
that its open tuple has a witness. A seed with another path or ancestry is not deduplicated, and a
changed corpus rendering receives a different occurrence identity. Generated candidates and blocked
generation failures are visible projections, not new authoritative history.

This is only role opening/binding and recursive subject construction. It does **not** yet implement
the full family of reciprocal, composition, transport, local-permutation, fold-license, or basin-escape
generators. The corpus's opposed questions remain distinct obligations; swapping two same-carrier
bindings does not prove a converse, inverse, reciprocal return, or complete relational coverage.

### 3. Cross and localize

Use a safe pure countermodel, isolated fixture, overcomplete representation, maximal removal, or
deliberately overstrong candidate to cross the boundary clearly. Preserve the actual proof,
countermodel, compiler, corpus, runtime, checker, or domain return. If no crossing occurs, change
the contrast rather than accumulating microscopic edits.

After a crossing, partition and subtract the successful contrast. Retain all incomparable minimal
survivors when uniqueness is not established. Do not infer a cause from the size or location of the
exploratory change.

### 4. Answer, reify, reciprocate, and regenerate

The Ask exists before its Answer. For effectful discharge, preserve the exact order
`Ask -> Seal -> Raw -> Interpret -> Check -> Answer`; Pure and Generate do not fabricate actuality.
After every consequential Answer, reify explicit products with their status, provenance,
dependencies, applicability, coverage, and horizon. Generated products become queryable but not
Standing. Then regenerate the field and materialize every newly formable reciprocal challenge as
either:

- represented by its declared opposed corpus pair; or
- individually typed but blocked, with the unavailable capability or inapplicability recorded.

The two central axes—constrain/release and distinguish/coarsen—remain distinct. Regenerate path,
direction, support, fold/reopen, propagation, and Answer/question-succession relations from the
enlarged surface. A one-way paraphrase, convenient singleton, or green build does not close this
obligation. Unchosen materialized questions remain live unless an Answer, typed inapplicability, or
evidenced fold licenses their removal.

The current machine retirement path resolves `Answered` to the matching complete Answer
occurrence. Partial/Unknown answers do not retire the question. An inapplicability assertion
alone is not an implemented retirement license: retain that occurrence with its disposition.
Reopening restores every omitted fold member before another Ask or checkpoint. Fold and
invalidation events likewise require a refreshed field; their evidence adequacy remains a
separate construction obligation rather than being inferred from a nonempty reference.

Active fold-evidence policy 2 resolves `protected_equivalence_evidence` and
`regeneration` to distinct, previously reified checked products. Each must originate
in a completed checked Probe Answer whose Check and Answer cite its immutable Raw
JSON report. Merely writing `checker:...` or marking a candidate `checked` is not
enough. The fold supplies `protected_continuations` as a JSON list of independently
admitted continuation-product identities. Evidence must match the exact members,
their occurrence/rendering/path digests, representative, horizon, coverage, and
continuation list. All currently admitted targeted continuations must be covered and
each continuation must be applicable to every proposed fold member.

Reports use schema 2. A checked continuation product carries
`inquiry_protection: {schema: 2, targets, execution, raw_digest}`. `execution` is
inspectable first-order data with the exact form
`{schema: 1, language: "question_identity_projection", field}`; `field` must be an
immutable question-occurrence identity coordinate. Its Raw `protected_continuation`
report repeats the exact `targets` and `execution` claim and supplies one Supported
observation per target. Admission independently re-executes the projection over the
represented question occurrences and rejects any different reported value. It does
not evaluate opaque code, host closures, tools, or model behavior.

A schema-2 `fold_check` report contains `claim` with `schema`, `relation` (`protected_equivalence` or
`regeneration`), `members`, `member_identities`, `representative`, `continuations`,
`horizon`, and `coverage`. The checked product carries that claim as `fold_evidence`,
plus `raw_digest`, and declares its continuation/support dependencies. Equivalence
reports contain one Supported `observations` cell (`member`, `continuation`, `value`)
for every declared member/continuation pair; each value must equal the independently
re-executed continuation and then agree across the fold. Regeneration
reports contain `regenerated_members`, preserving every exact occurrence identity.
The executable positive examples are in `tools/harness_fold_evidence_check.js`;
`.claude/hooks/ic-fold-evidence.js` owns admission and identity projection.

This establishes report-to-execution correspondence only for the declared finite
question-identity projection—not the truth of an arbitrary external/model
observation or the sufficiency of the selected horizon. Those remain independent
inquiry obligations. A new protected continuation
outside the old coverage, withdrawn transitive support, or migration of an old
label-only fold requires explicit `reopen` and restoration. Reopening may occur
after reification and before the required field refresh; another Ask may not.
Historical policy-0 and policy-1 reports retain their occurrence-time rules. A
controlled transition to policy 2 reopens their active folds rather than silently
granting the stronger correspondence. Evidence policy cannot be downgraded. Raw
evidence must remain available with its original bytes.

The append-only trace is the ancestry source. `RESIDUAL_OBLIGATIONS.json` supplies stable project
obligation seeds, while the broad residual index is rebuilt from those seeds, trace coordinates,
and the one live Frontier selection. Never hand-maintain a second moving topology.

### 5. Construct only the typed survivor

Implement the smallest retained formal or process change justified by localization. The successor
may correct v2.0, but the correction must be explicitly classified; it may not be disguised as
preservation. No `sorry`, custom semantic axiom, ambiguous overload, untyped context application,
or manually authoritative prose may fill an open mathematical position.

### 6. Propagate and reprove

Compute the full affected dependency closure (S^*\). Recheck every affected definition, theorem,
countermodel, question elaboration, notation/prose round trip, binding, compiler relation,
preservation/correction correspondence, and regeneration claim that is applicable at the current
phase. Record uncovered components as `Unknown`; never promote semantic coverage from an
unexercised checker.

### 7. Challenge and minimize

Seek a stronger breaker capable of defeating the apparent boundary. Test the reverse implication
independently. Remove large groups before leaves, then test joint removals so interacting excess is
not mistaken for necessity. Retain only structure whose removal loses a named protected
consequence at declared coverage.

### 8. Ratchet durable state

On an accepted local change, update only the owning records:

- demonstrated successor behavior -> `CONFORMANCE_STATUS.md`;
- accepted successor choice -> append `DECISIONS.jsonl`;
- actual durable failure or constraint -> append `FAILURES.jsonl`;
- propagation closure -> a machine-readable report under `reports/`;
- strongest remaining obligation -> replace the one live Frontier block.

The one live Frontier is an operational selection from the broader residual topology, not the
whole unresolved universe. A local `Satisfied` result remains indexed by obligation, binding,
horizon, and coverage. Preserve unsearched breaker families, blocked capabilities, latent phase
obligations, reopening conditions, and newly reachable residuals rather than collapsing the state
to the local theorem.

The report is evidence, never theorem authority. Git owns chronology. README files remain static
orientation.

### 9. Checkpoint, recur, or close

A local pass reaches a checkpoint when its Answer is reified, the field is regenerated, the
affected dependency closure stabilizes, and durable evidence/residuals are recorded. Continue from
that regenerated field without treating the checkpoint as task termination.

Task-level closure additionally requires no unresolved Ask, no Answer awaiting reification, no
dirty surface, no open actual cycle, no newly materialized required/productive executable question,
and explicit retention of all Blocked, Unknown, ResourceBounded, and reopening conditions. Challenge
that state with a final adversarial question, then record `closure` and one lawful Stop status:
`Satisfied`, `Equivalent`, `Impossible`, `Blocked`, `Unknown`, or `ResourceBounded`. Never repeat the
same occurrence in the same represented state.

Use actual occurrence IDs for `closure.adversarial_question` and
`closure.adversarial_answer`. The Answer must be checked through Probe/Check, complete,
reified, and responsible for the closing field regeneration. The harness checks that ancestry,
not the semantic sufficiency of the challenge or the truth of an asserted warrant. Any later
consequential transition invalidates that closure. `Satisfied` requires no unresolved field
members; an explicitly blocked/unknown field cannot be relabeled success.
If the final challenge itself is unresolved, declare `closure.state` as `Unknown`,
`Blocked`, or `ResourceBounded`, matching its retained Answer (or a resource-bounded
Partial Answer), and use the same Stop state. That is an explicit non-successful
closure, not an exemption from Ask/Answer/reification/field ancestry.

`ic-trace state` is the validated lifecycle projection used by status, Stop, and the active
surface. `open` includes incomplete inquiry bookkeeping; `mutation-open` is narrower and
requires the live sealed Probe. An Answer ends that mutation permission even before reification.

Compatible schema-4 policy changes require user-authorized harness control and a sealed,
pre-return Probe. Do not switch a legacy trace's state machine midway through its history.
Finish its checked legacy cycle and lawful Stop, then initialize the manifest-selected schema
with the exact predecessor trace pin. Environment variables cannot select a weaker schema.
The historical second-pass Stop escape applies only to a validated legacy trace; unreadable
ancestry cannot use it to bypass the current lifecycle.

## Phase progression

The loop repeats inside every construction phase. Promotion is monotone only through checked gates:

```text
A inventory
-> B formal predecessor surface
-> C exact profile/protection/question core
-> D permanent breakers
-> E program and effect semantics
-> F preservation/correction and full propagation
-> G law-aware compiler
-> H compression and regeneration
-> I complete question-corpus accounting
-> J reciprocal canonical notation and prose
-> K domain-binding pressure
-> L regenerative self-application
-> M generated canonical specification
-> N checked Rust successor migration
```

A later phase may expose a defect in an earlier gate and reopen it. “Lean builds” is never enough
for promotion. Each gate requires its construction-specification evidence, independent checks,
declared coverage, and reopening condition.

## Baseline checks

Run the cheapest decisive discriminator first, then affected checks, then the applicable broad
gates. The branch-wide baseline is:

```text
node tools/successor_control_check.js
node tools/harness_control_check.js
node tools/harness_lifecycle_check.js
node tools/harness_question_instance_check.js
node tools/predecessor_inventory.js check
node tools/predecessor_inventory_check.js
node tools/predecessor_tex_classification.js check
node tools/predecessor_tex_classification_check.js
node tools/predecessor_implementation_classification.js check
node tools/predecessor_implementation_classification_check.js
node tools/predecessor_fixture_classification.js check
node tools/predecessor_fixture_classification_check.js
node tools/phase_a_coverage.js check
node tools/phase_a_coverage_check.js
node tools/phase_b_predecessor_spine.js check
node tools/phase_b_predecessor_spine_check.js --compile
node tools/phase_b_binding_type.js check
node tools/phase_b_binding_type_check.js --compile
node tools/phase_b_forms.js check
node tools/phase_b_forms_check.js --compile
node tools/phase_b_relations.js check
node tools/phase_b_relations_check.js --compile
node tools/phase_b_refinement.js check
node tools/phase_b_refinement_check.js --compile
node tools/phase_b_formula_grammar.js check
node tools/phase_b_formula_grammar_check.js --compile
node tools/phase_b_minimal_logical_basis.js check
node tools/phase_b_minimal_logical_basis_check.js --compile
node tools/phase_b_relation_expression_ir.js check
node tools/phase_b_relation_expression_ir_check.js --compile
node tools/phase_b_relation_schema_ports.js check
node tools/phase_b_relation_schema_ports_check.js --compile
node tools/phase_b_partial_binding_fiber.js check
node tools/phase_b_partial_binding_fiber_check.js --compile
formal: lake build --wfail
documentation topology and canonical-TeX checks when those surfaces change
Rust format/check/clippy/test as predecessor regression evidence
git diff --check
```

Formal CI additionally performs independent kernel checking, no-sorry/custom-axiom auditing, and
the configured Lean checks. Later phases must add the breaker, corpus, round-trip, regeneration,
conservativity, correspondence, and compiler checks required by §107 rather than claiming them from
the setup scaffold.

## Autonomous safety boundary

Autonomy authorizes finite repository-local inquiry and reversible implementation toward the live
successor residual. It does not authorize weakening gates, editing the harness to pass itself,
self-warrant, silently changing semantic authority, pushing, releasing, deploying, deleting user
work, or making external writes. A required new authority or permission is a named `Blocked` state,
not a reason to redirect the objective.
