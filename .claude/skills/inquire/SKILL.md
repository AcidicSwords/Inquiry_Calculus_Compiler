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
4. **TRACE** the smallest responsible relational path. Several live loci require a separator.
5. **EXPERIMENT** by naming the smallest wrong implementation and sealing a prediction:

       .claude/hooks/ic-trace seal \
         should_change=... invariants=... discriminator=... wrong_impl=...

6. **UPDATE** from the preserved return, not the pre-return story.
7. **CHANGE** only the smallest authorized reversible region.
8. **VERIFY** with targeted checks, then required broad gates.
9. **CHALLENGE** sufficiency and necessity.
10. **MINIMIZE** by safe ablation.
11. **RATCHET** only durable evidence and replace the live Frontier residual when it changes.

Record a consequential answer with all recurrence coordinates:

    .claude/hooks/ic-trace question \
      q=... mode=Pure \
      answer=... branch=... \
      occurrence=... continuation=... \
      bindings=... horizon=... coverage=... \
      authority=... evidence=...

The occurrence identifies the checked Ask/engineering question use; continuation identifies the
answer-dependent branch program. Use explicit values such as none only when that coordinate truly
does not apply.

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
    .claude/hooks/ic-trace residual class=... next=...

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
