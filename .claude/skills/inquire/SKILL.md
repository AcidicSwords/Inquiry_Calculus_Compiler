---
name: inquire
description: Run consequential repository work through the Inquiry Calculus engineering clock. Use before changes to code, tests, migrations, authority documents, control files, delivery, consequential debugging, or design.
---

# Inquire

Read references/software-engineering-binding.md before a consequential design or debugging
decision. AGENTS.md owns the full repository protocol; this skill gives the mechanically recorded
sequence.

Use an actual POSIX Bash. On Windows, prefer Git Bash; do not rely on an unavailable WSL alias.

## Open a collision-safe trace

    .claude/hooks/ic-trace init <slug>
    .claude/hooks/ic-trace ensure \
      task=... authority=... invariants=...

The ignored trace is derived engineering evidence, not canonical event history.

If the explicit user task authorizes a revision to AGENTS.md, the canonical TeX, the stable plan,
the inquiry harness, or the CI acceptance/checker surface, record that predecessor authority before
mutation:

    .claude/hooks/ic-trace control \
      authority=... residual=... predecessor=... scope=...

An autonomous candidate may not create this authority for itself.
For more than one protected group, use an exact comma-separated set drawn from
`canonical,plan,agents,harness,ci`; near matches and negative names do not authorize a group.
Routine README orientation, Frontier, conformance, decision, and failure ratchets remain ordinary
consequential changes: they require an open sealed cycle, but not a separate control grant.
`init` cannot abandon an open sealed cycle or a checked residual without its terminal stop; close
the trace lawfully first.

## Run the clock

1. **SPECIFY** the task contract, authority, protected horizon, invariants, and discriminator.
2. **INSPECT** fresh repository and external actuality. An explicit task may temporarily supersede
   the persistent Frontier; otherwise use the one delimited live block.
3. **CONTRAST** current and required observables.
4. **CROSS** the protected boundary with a decisive admissible contrast.
5. **LOCALIZE** inside the demonstrated crossing by partition, subtraction, or backoff.
6. **TRACE** a sufficient responsible relational path through roles, direction, composition, and
   order. Several live loci require a separator; localize only after a decisive path contrast.
7. **EXPERIMENT** by naming a decisively wrong admissible implementation or foil that can reject a
   large alternative region, then seal the prediction:

       .claude/hooks/ic-trace seal \
         ask_occurrence=... \
         should_change=... invariants=... discriminator=... wrong_impl=... coverage=...

8. **UPDATE** from the preserved return, not the pre-return story.
9. **CHANGE** with an authorized reversible candidate sufficiently strong to cross the localized
   boundary. After success, subtract excess before retaining the production change.
10. **VERIFY** with targeted checks, then required broad gates.
11. **CHALLENGE** sufficiency and necessity.
12. **MINIMIZE** by safe ablation.
13. **RATCHET** only durable evidence and replace the live Frontier residual when it changes.

Materialize a finite live field before asking. `members` is a JSON array of occurrence records;
each includes `occurrence`, `question_form`, `rendering`, exact `prompt`, `source_lines`,
`generator_ids`, `path`, `disposition`, `executable`, and `dependencies`:

    .claude/hooks/ic-trace field \
      field_id=... members='[...]' basis=... coverage=... \
      regenerated_from=bootstrap dispositions='{}' removal_evidence='{}'

Select one represented executable occurrence. Source and manifest digests are pinned automatically:

    .claude/hooks/ic-trace ask \
      q=... mode=<Pure|Generate|Probe|Check|Warrant> occurrence=... field_id=... \
      question_form=... rendering=... source_lines=... generator_ids=... \
      reciprocal_relations=... path=... bindings=... horizon=... coverage=... \
      authority=... evidence=... dependencies=...

Record its Answer separately, then reify explicit products without raising authority:

    .claude/hooks/ic-trace answer \
      occurrence=... ask_occurrence=... answer=... \
      resolution_class=<Supported|Partial|Plural|ExactEmpty|Unsupported|Unknown|Blocked|ResourceBounded> \
      status=<provisional|supported|checked|warranted> polarity=<Positive|Negative|Mixed|None> \
      residual=... evidence=... coverage=... authority=...

    .claude/hooks/ic-trace reify \
      answer_occurrence=... status=... products='[...]' new_questions=... coverage=...

After every consequential Answer, append a regenerated `field` before another ordinary Ask. Carry
forward every unchosen live occurrence. Removal requires an evidenced Answer, typed inapplicability,
or an explicit fold that preserves ancestry, regeneration, and reopening. Generated products are
queryable but never Standing merely because they were reified.

An optional derived route occurrence may be recorded:

    .claude/hooks/ic-trace route \
      source_occurrence=... answer=... \
      successor_occurrence=... provenance=...

This projects the trace; it does not create a second semantic history.

## Preserve raw returns

For a safe raw return:

    command > temporary-file 2>&1
    .claude/hooks/ic-trace raw ask_occurrence=... cmd=... file=temporary-file sensitive=false

The trace copies the bytes into ignored digest-addressed storage before interpretation.

For credentials, personal data, or another sensitive return, never copy the bytes:

    .claude/hooks/ic-trace raw \
      ask_occurrence=... cmd=... digest=<safe-sha256> sensitive=true

Then record interpretation without mutating Raw, followed by an independent Check:

    .claude/hooks/ic-trace interpret \
      ask_occurrence=... raw_digest=... interpretation=... provenance=...
    .claude/hooks/ic-trace check \
      ask_occurrence=... verdict=... coverage=... evidence=...
    .claude/hooks/ic-trace residual \
      class=... next=... parent_residual=... open_relation=... \
      condition_ids=... condition_keys=... blocker_ids=... breaker_ids=... \
      separator_ids=... survived_contrast_ids=... conflict_ids=... gap_ids=... \
      failed_fold_ids=... reopen_condition_ids=... overlap_ids=... coverage=... \
      resolution_class=... residual_shape=... method_frontier=... next_question_family=...

The enforced effectful order is `Ask -> Seal -> Raw -> Interpret -> Check -> Answer -> Reify ->
Field`. No residual or checkpoint can bypass field regeneration, and a second seal cannot replace
an open actual cycle.

Residual classes are:

    none | persists | regression | wrong_locus | missing_dep |
    weak_discriminator | env_failure | unknown | resource

## Standing prohibitions

- Positive departure requires a supported incompatible pair; failed search or projection is not a
  witness.
- Unknown is not Negative.
- Semantic coverage is not execution coverage.
- The whole return fiber is not a selected return.
- Generation, actuality, checking, and warrant remain separate.
- No self-warrant and no fabricated singleton.
- Never weaken a valid test to make a candidate pass.
- Never edit the harness to get past the harness.
- Do not declare scope unjustified merely because an explicit user task differs from the persistent
  Frontier.

## Checkpoint, recur, or close

Update only the record that owns the resulting fact:

- demonstrated behavior -> CONFORMANCE_STATUS.md;
- accepted choice -> append DECISIONS.jsonl;
- actual durable constraint -> append FAILURES.jsonl;
- next residual -> replace IMPLEMENTATION_FRONTIER.md live block.

After reification and field regeneration, record a replayable checkpoint and continue:

    .claude/hooks/ic-trace checkpoint \
      field_id=... established=... remains_open=... \
      fold_changes=... reopen_changes=... coverage=...

If and only if Ask fuel is exactly exhausted at that clean checkpoint while the regenerated
field still has a Required/Productive executable occurrence, continue the same trace with:

    .claude/hooks/ic-trace resume reason="continue persistent autonomous task"

This is an append-only checkpoint continuation under current user-authorized harness control.
It grants one canonical finite ratchet; it does not select the next question, create standing,
or turn the checkpoint into closure.

A checkpoint is not permission to stop. Only after the task-level closure criterion is met and
adversarially challenged:

    .claude/hooks/ic-trace closure \
      field_id=... scope=... warrant=... \
      adversarial_question=... adversarial_answer=... coverage=...

    .claude/hooks/ic-trace stop state=<state> warrant=...

Use only Satisfied, Equivalent, Impossible, Blocked, Unknown, or ResourceBounded. Equivalent closes
one branch; only Satisfied closes the task.
