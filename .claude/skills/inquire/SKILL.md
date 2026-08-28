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
         should_change=... invariants=... discriminator=... wrong_impl=... coverage=...

8. **UPDATE** from the preserved return, not the pre-return story.
9. **CHANGE** with an authorized reversible candidate sufficiently strong to cross the localized
   boundary. After success, subtract excess before retaining the production change.
10. **VERIFY** with targeted checks, then required broad gates.
11. **CHALLENGE** sufficiency and necessity.
12. **MINIMIZE** by safe ablation.
13. **RATCHET** only durable evidence and replace the live Frontier residual when it changes.

Record a consequential answer with all recurrence coordinates:

    source_digest="$(sha256sum formal-successor/Questions.txt | cut -d' ' -f1)"
    program_manifest_digest="$(sha256sum formal-successor/ENGINEERING_QUESTION_PROGRAMS.json | cut -d' ' -f1)"
    .claude/hooks/ic-trace question \
      q=... mode=Pure \
      answer=... branch=... \
      occurrence=... continuation=... \
      bindings=... horizon=... coverage=... \
      authority=... evidence=... \
      program=QP-PREFORMAL-RESIDUAL-RATCHET \
      rhythm=<manifest-scheduled-principal-rhythm> \
      residual_class=<declared-residual-class> \
      compiled_questions=<exact-required-CQ-ids> \
      question_families=<derived-family-ids> \
      coding_questions=<derived-comma-separated-source-lines> \
      coverage_dimensions=<derived-relational-dimensions> \
      root_spans=<derived-erasable-root-lowerings> \
      rhythm_positions=<derived-preformal-positions> \
      reciprocal_status=represented \
      reciprocal_challenges=<required-RCP-ids> \
      blocked_reciprocals=none \
      reciprocal_pairs=<derived-left:right-pairs-separated-by-semicolons> \
      reciprocal_axes=<derived-central-axes-or-none> \
      reciprocal_reason=... \
      parent_residual=<stable-residual-id-or-none> \
      condition_ids=<relevant-condition-ids-or-none> \
      breaker_ids=<relevant-breaker-ids-or-none> \
      reciprocal_obligation=<represented|blocked|not_applicable> \
      question_disposition=<Answered|Productive|Required|Redundant|Inapplicable|Blocked|Unknown> \
      residual_shape=<manifest-residual-shape> \
      method_frontier=<exact-manifest-method-frontier> \
      condition_keys=<schema@roles@scope@applicability@grain@orientation-or-none> \
      source_digest="$source_digest" \
      program_manifest_digest="$program_manifest_digest"

The occurrence identifies the checked Ask/engineering question use; continuation identifies the
answer-dependent branch program. Derive the fields from the residual-selected rhythm in
`formal-successor/ENGINEERING_QUESTION_PROGRAMS.json`; never invent coverage. Every required
reciprocal challenge must appear in `reciprocal_challenges` or `blocked_reciprocals`, never both. If
all are blocked, use `reciprocal_status=blocked`, `reciprocal_challenges=none`,
`reciprocal_pairs=none`, and `reciprocal_axes=none`, with a typed reason naming the unavailable
capability. New traces pin both input digests in their first record, and the harness refuses a
residual until the active cycle contains a policy-accepted question after every raw return.

An optional derived route occurrence may be recorded:

    .claude/hooks/ic-trace route \
      source_occurrence=... answer=... \
      successor_occurrence=... provenance=...

This projects the trace; it does not create a second semantic history.

## Preserve raw returns

For a safe raw return:

    command > temporary-file 2>&1
    .claude/hooks/ic-trace raw cmd=... file=temporary-file sensitive=false

The trace copies the bytes into ignored digest-addressed storage before interpretation.

For credentials, personal data, or another sensitive return, never copy the bytes:

    .claude/hooks/ic-trace raw \
      cmd=... digest=<safe-sha256> sensitive=true

Then record the independent interpretation:

    .claude/hooks/ic-trace check verdict=... coverage=...
    .claude/hooks/ic-trace residual \
      class=... next=... parent_residual=... open_relation=... \
      condition_ids=... condition_keys=... blocker_ids=... breaker_ids=... \
      separator_ids=... survived_contrast_ids=... conflict_ids=... gap_ids=... \
      failed_fold_ids=... reopen_condition_ids=... overlap_ids=... coverage=... \
      resolution_class=... residual_shape=... method_frontier=... next_question_family=...

The enforced order is `seal -> one or more raw returns -> one or more checks -> residual`.
No residual can close a prediction before an actual return and check, and a second seal cannot
replace an open cycle.

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

## Close

Update only the record that owns the resulting fact:

- demonstrated behavior -> CONFORMANCE_STATUS.md;
- accepted choice -> append DECISIONS.jsonl;
- actual durable constraint -> append FAILURES.jsonl;
- next residual -> replace IMPLEMENTATION_FRONTIER.md live block.

Then, after the checked residual:

    .claude/hooks/ic-trace stop state=<state> warrant=...

Use only Satisfied, Equivalent, Impossible, Blocked, Unknown, or ResourceBounded. Equivalent closes
one branch; only Satisfied closes the task.
