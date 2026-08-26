# Implementation Frontier

This file is the sole live implementation cursor. Historical work belongs in Git, demonstrated behavior in `CONFORMANCE_STATUS.md`, and accepted choices or failures in the JSONL ledgers.

Repository actuality must be probed before acting. The current residual was derived from demonstrated implementation behavior through predecessor coordinate `a936ebf`; that coordinate is ancestry, not a claim that a tracked document can name its own final commit or worktree state.

## Immediate demonstrated predecessor

`METHOD-BRIDGE-001` established a cold-replayable, content-addressed `MethodBridge` whose source-declared residual selects an exact first-order continuation, typed guard, and input-reconstruction program. It rejects undeclared residuals and rival transports, preserves method names only as provenance, and adds no dispatch policy, opcode, table, warrant, or second history.

## Strongest live obligation

Close the witnessed parallel test-database alias before relying on the workspace gate as repeatable evidence.

<!-- LIVE_FRONTIER_BEGIN -->
id: TEST-ISOLATION-001
plan_phase: cross-phase acceptance
goal: Give every parallel file-backed replay fixture a collision-proof temporary SQLite path without changing runtime persistence or migration semantics.
protected_difference: A migration idempotence failure caused by two tests aliasing one temporary database is not an implementation migration failure and cannot be dismissed as a green-rerun environment accident.
discriminator: Concurrent calls to persisted_cold_replay_fixture obtain distinct paths even when the platform clock returns the same instant, and repeated parallel admitted-resume plus workspace suites pass with exact cleanup.
horizon: ic-runtime admitted-resume test fixture allocation and cleanup only; no production store, schema, migration, artifact, event, or replay behavior
relevant_decisions: D-0131
relevant_failures: F-0005
if_pass: restore CUE-PLANNING-001 unchanged as the strongest semantic implementation obligation
if_fail: preserve the colliding paths and reopen exclusive temporary-resource allocation rather than weakening migration checks
<!-- LIVE_FRONTIER_END -->

## Smallest decisive fixture

Start from the three tests that call `persisted_cold_replay_fixture` concurrently. Force allocation through one process-local sequence in addition to process and clock identity, then establish that a generated batch of paths is duplicate-free and the existing parallel suite no longer aliases a database.

The smallest wrong implementation merely retries migration, deletes a database another test owns, serializes the whole suite, or suppresses the `table already exists` result.

## Prohibitions at this boundary

- Do not change production SQLite opening, migrations, schema, or replay behavior.
- Do not delete or reuse a path owned by another test.
- Do not serialize parallel tests to hide the collision.
- Do not add a dependency when standard process-local uniqueness is sufficient.

## Exit

On pass, append the resolution evidence, restore `CUE-PLANNING-001` byte-for-byte as the live block, and continue. On failure, preserve the actual paths and reopen allocation rather than relaxing migration or replay checks.
