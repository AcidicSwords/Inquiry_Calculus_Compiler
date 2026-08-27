# Implementation Frontier

This file is the sole live implementation cursor. Historical work belongs in Git, demonstrated behavior in `CONFORMANCE_STATUS.md`, and accepted choices or failures in the JSONL ledgers.

Repository actuality must be probed before acting. The current residual was derived from demonstrated implementation behavior through predecessor coordinate `9739b1f`; that coordinate is ancestry, not a claim that a tracked document can name its own final commit or worktree state.

## Immediate demonstrated predecessor

`QPURE-REGISTRY-001` established that the present schema has no formable pure operation: the only accepted source tags are `Return` and `Ask`, both with direct nonexecuting heads. The compiler-version-indexed identity normalizer is therefore complete only at this grammar; an unknown operation tag rejects. This is an `Equivalent` no-addition result, adding no source syntax, registry, evaluator, table, opcode, scheduler, controller, or persistence schema.

## Strongest live obligation

Derive the smallest checked `SourceAskLowering` relation that binds one exact source `Ask` occurrence, every declared open port and discharge mode, the compiler coordinate, and one verified runtime program without adding a new runtime opcode or dispatching a provider.

<!-- LIVE_FRONTIER_BEGIN -->
id: QASK-LOWERING-001
plan_phase: 5
goal: Derive a checked source-Ask-to-runtime lowering that preserves exact occurrence, per-port mode, compiler coordinate, and runtime program identity.
protected_difference: A runtime program can have a compatible result/endpoint while lowering another source occurrence, omitting a source port, changing a declared discharge mode, or using another compiler coordinate. Such substitution changes the actual continuation/authority route even before dispatch.
discriminator: Construct one checked finite source `Ask` with every open port named once and a verified `ProgramIR`. Accept only the exact source occurrence, port/mode list, compiler coordinate, and runtime identity; reject swapped occurrence, missing/duplicate/foreign port, changed mode, compiler, or runtime program. Prove verification does not dispatch or create an event.
horizon: one finite checked source Ask, its exact open-port field, one verified runtime program, and no dispatch; no general compiler, port execution, provider call, event, resolver procedure artifact, cross-binding bridge, scheduler, controller, table, or opcode
relevant_decisions: D-0125, D-0150, D-0154, D-0155
relevant_failures: F-0001, F-0002
if_pass: rederive the strongest remaining Phase 5/6 residual from the stable plan and pending conformance; do not reopen passed QSUCC/QREADY/QACTUAL fixtures without a new breaker
if_fail: reopen the earliest source occurrence, port/mode, compiler, runtime-program identity, or execution-separation relation; do not introduce a general compiler, dispatcher, scheduler, controller, table, or opcode
<!-- LIVE_FRONTIER_END -->

## Smallest decisive fixture

Construct one checked source `Ask` with its exact finite open-port field and a verified runtime program. Its lowering must rewalk both source and runtime, require exactly one matching lowering per port and declared mode, and retain the source/compiler/runtime coordinates. Every structural substitution must reject before any provider boundary.

The smallest wrong implementation names only the question or endpoint, treats every port as `Probe`, omits a port, accepts another compiler/runtime program, or dispatches while verifying.

## Prohibitions at this boundary

- Do not generalize the source lowering into a compiler, resolver, dispatcher, scheduler, controller, table, or opcode.
- Do not execute or create an event merely because a source lowering verifies.
- Do not erase exact occurrence, source port/mode, compiler, or runtime-program identities.

## Exit

On pass, update conformance and project state, then rederive the next strongest residual. On failure, preserve the actual return and reopen the earliest implicated lowering relation rather than broadening the architecture.
