---
name: inquire
description: Run consequential work in this repository through the inquiry loop - probe actuality, name the live obligation, seal a prediction, act, check against the seal, classify the residual, and stop in a named state. Use for any change to crates/, migrations/, fixtures/, or the authority documents; for debugging; and for design decisions. The PreToolUse gate refuses mutations until a prediction is sealed, so start here.
---

# Inquire

The loop this repository's own calculus specifies, applied to working in it.

Read `references/calculus-engineering-map.md` before any consequential design or
debugging decision. It is the reasoning content; this file is the sequence.

## Ledger first

```bash
.claude/hooks/ic-trace init <slug>
.claude/hooks/ic-trace status
```

Every record is append-only and parent-linked. Nothing is ever rewritten — that
is what makes a sealed prediction a prediction rather than a story told
afterwards.

## The loop

**1. ENSURE — probe, don't recall.**
Read the files, run `git status`, check the actual state. Remembered state is
not actuality, and your own earlier summary is not the source.

```bash
.claude/hooks/ic-trace ensure task=... authority=... invariants=...
```

**2. FRONTIER — name the one obligation this serves.**
From `IMPLEMENTATION_FRONTIER.md`. If the work serves none, that is the finding:
stop and say the scope is unjustified rather than proceeding.

**3. STOP? — is an ending already justified?**
`Satisfied | Equivalent | Impossible | Blocked | Unknown | ResourceBounded`.
Do not continue merely because continuing is customary.

**4. DIFFERENCE — state it in five lines.**

```text
current observable:
required observable:
protected difference:
scope / applicability / grain / horizon:
independent discriminator:
```

**5. LOCUS — the smallest boundary that accounts for it.**
If several candidates remain live, build a separator *before* editing anything.
A repository search yields candidates, not culprits: confirm the forward path
from candidate to behavior.

**6. CHALLENGE — the smallest wrong implementation your check must reject.**
Name it explicitly. If you cannot, the check is decoration.

**7. CONSEQUENTIAL? — gate the deep pass.**
Run the full reciprocal challenge only when the answer could change
architecture, meaning, or acceptance. The economy law forbids ceremony whose
every outcome leads to the same continuation.

**8. SEAL — required before any mutation.**

```bash
.claude/hooks/ic-trace seal \
  should_change=... invariants=... discriminator=... wrong_impl=...
```

The gate refuses Edit, Write, and mutating Bash until this exists. Seal what you
actually expect; the ledger will not let you revise it afterwards.

**9. ACT, then preserve the return before reading it.**

```bash
cargo test --locked --workspace --all-features > /tmp/ic-raw.txt 2>&1
.claude/hooks/ic-trace raw cmd='cargo test ...' file=/tmp/ic-raw.txt
```

**10. CHECK — the return against the seal, not against your intent.**

```bash
.claude/hooks/ic-trace check verdict=... coverage=...
```

State what the check distinguished. It establishes exactly that and no more.

**11. RESIDUAL — one of nine, never pass/fail.**

```bash
.claude/hooks/ic-trace residual class=<none|persists|regression|wrong_locus|
  missing_dep|weak_discriminator|env_failure|unknown|resource> next=...
```

**12. SUBTRACT — is there a smaller protected-equivalent realization?**
Remove a part; rerun the discriminator. Keep it only when a check witnesses its
necessity.

**13. RECORDS.** `DECISIONS.jsonl` for accepted choices with a reopen condition.
`FAILURES.jsonl` for observed failures that constrain later work.
`IMPLEMENTATION_FRONTIER.md` when the strongest obligation moves.
`CONFORMANCE_STATUS.md` only to the extent an executable fixture demonstrates.

**14. STOP — name the state.**

```bash
.claude/hooks/ic-trace stop state=<one of six> warrant=...
```

`Satisfied` requires a warrant from whoever owns the affected contract — for
anything touching an accepted contract, that is the user, not me. `Unknown` is a
lawful, reportable result.

## Standing prohibitions

- Departure must be **positive**: a concrete incompatible pair. Failed search,
  failed equality, and boundary projection are not witnesses.
- `Unknown ≠ Negative`. Not found is not absent.
- `coverage_sem ≠ coverage_exec`. Handled is not exercised.
- Return **fiber** ≠ selected return. Ask what else the same evidence admits.
- No self-warrant. I am not the independent check on my own change.
- Generation ≠ actuality ≠ check ≠ warrant.
- Never weaken a valid test to make a candidate pass.
- Never edit the harness to get past the harness.

## When the gate refuses

It is doing its job. The refusal names the missing record. Supply it — do not
look for a route around it. Routes around it are what the self-protection rule
exists to close, and the ledger makes any detour legible in the diff.

## Scope

Agent tooling lives in `.claude/` only. Nothing here enters `crates/`: `ic-cli`
is a reserved semantic boundary, and a trace linter is not semantics.
