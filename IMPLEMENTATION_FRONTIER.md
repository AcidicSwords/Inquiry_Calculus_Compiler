# Implementation Frontier

This file is the sole live implementation cursor. Historical work belongs in Git, demonstrated behavior in `CONFORMANCE_STATUS.md`, and accepted choices or failures in the JSONL ledgers.

Repository actuality must be probed before acting. The current residual was derived from demonstrated implementation behavior through predecessor coordinate `22df490`; that coordinate is ancestry, not a claim that a tracked document can name its own final commit or worktree state.

## Immediate demonstrated predecessor

`QASK-LOWERING-001` established a derived, nonexecuting source-Ask lowering over one exact occurrence, each declared source port and mode, its compiler coordinate, and one checked runtime-program identity. The checker rewalks source and runtime and a consuming reconstruction must separately match both expected identities. The finite breaker rejects swapped occurrences, changed runtime/compiler, and empty, duplicate, foreign, or mode-changed port fields. It neither dispatches nor creates an event, compiler, resolver, table, or opcode.

## Strongest live obligation

Demonstrate that the checked `SourceAskLowering` relation regenerates after a file-backed restart from persisted ordinary roots, without reusing an in-memory lowering or dispatching a provider.

<!-- LIVE_FRONTIER_BEGIN -->
id: QASK-COLD-REGEN-002
plan_phase: 5
goal: Reconstruct one checked source-Ask-to-runtime lowering after reopening only persisted ordinary source/configuration/runtime roots, preserving exact occurrence, port/mode field, compiler coordinate, and runtime identity.
protected_difference: A lowering that passes only through a warm catalog can silently depend on stale source, query, type, or runtime objects. A fresh reconstruction must reject a substituted occurrence or runtime even where question/result/endpoint projections agree.
discriminator: Persist one finite source Ask/configuration/occurrence and one runtime-program artifact, close and reopen the store, rebuild a fresh catalog, decode/recheck both sides, regenerate the derived lowering, and compare its expected occurrence/runtime identities. Reject a swapped occurrence and a changed runtime with zero provider calls.
horizon: one file-backed source Ask and runtime artifact, fresh catalog reconstruction, exact port/mode/compiler/runtime identities, and no dispatch; no persisted lowering recipe, general compiler, resolver procedure artifact, port execution, event, cross-binding bridge, scheduler, controller, table, or opcode
relevant_decisions: D-0125, D-0150, D-0154, D-0155, D-0156
relevant_failures: F-0001, F-0002
if_pass: rederive the strongest remaining Phase 5/6 residual from the stable plan and pending conformance; do not reopen passed QASK/QSUCC/QREADY/QACTUAL fixtures without a new breaker
if_fail: reopen the earliest persisted root, catalog reconstruction, occurrence, port/mode, compiler, or runtime-program identity relation; do not serialize the derived lowering or introduce a general compiler, dispatcher, scheduler, controller, table, or opcode
<!-- LIVE_FRONTIER_END -->

## Smallest decisive fixture

Persist one source configuration/Ask occurrence and one `RuntimeProgramArtifact`; after restart, decode them and rewalk the same source query to regenerate, rather than reload, the derived lowering. The reconstructed lowering must carry exactly one matching port/mode entry for each source port and must compare equal only at the expected source occurrence/runtime coordinates.

The smallest wrong implementation caches a warm lowering, reloads only a compatible question/endpoint, serializes the derived lowering as authority, accepts another occurrence/runtime program, or dispatches while regenerating.

## Prohibitions at this boundary

- Do not persist the derived lowering as a new authority or generalize it into a compiler, resolver, dispatcher, scheduler, controller, table, or opcode.
- Do not execute or create an event merely because a lowering is reconstructed.
- Do not erase exact occurrence, source port/mode, compiler, runtime-program, or fresh-catalog reconstruction identities.

## Exit

On pass, update conformance and project state, then rederive the next strongest residual. On failure, preserve the actual return and reopen the earliest implicated persisted-root or lowering relation rather than broadening the architecture.
