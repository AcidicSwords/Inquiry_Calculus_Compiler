# Implementation Frontier

This file is the sole live implementation cursor. Historical work belongs in Git, demonstrated behavior in `CONFORMANCE_STATUS.md`, and accepted choices or failures in the JSONL ledgers.

Repository actuality must be probed before acting. The current residual was derived from demonstrated implementation behavior through predecessor coordinate `669b790`; that coordinate is ancestry, not a claim that a tracked document can name its own final commit or worktree state.

## Immediate demonstrated predecessor

`QASK-DISCHARGE-BUNDLE-003` established one derived finite all-Probe pairing of the regenerated `SourceAskLowering` with the existing checked port-indexed discharge bundle. It rechecks both sides, retains exact occurrence and port authority, and rejects an otherwise compatible bundle for another source occurrence. Event, route, decoder, and resolution provenance remain owned by the existing bundle. No second bundle, compiler, dispatcher, table, or opcode was added.

## Strongest live obligation

Determine the smallest typed mixed-mode source-Ask execution view needed when one source occurrence has both Probe and non-Probe ports, without treating a non-Probe result as an event or introducing a general executor.

<!-- LIVE_FRONTIER_BEGIN -->
id: QASK-MIXED-MODE-004
plan_phase: 5
goal: Derive the narrow typed view that keeps a Probe port's checked event/route evidence distinct from a non-Probe port's typed result and its own authority, while retaining one exact source occurrence and complete port/mode field.
protected_difference: A mixed-mode source Ask can share a result type or endpoint across ports while only Probe creates an event. Treating every port as Probe manufactures actuality; flattening non-Probe output into an event or an untyped value erases its declared mode and authority route.
discriminator: Construct one finite source Ask with one Probe and one non-Probe port. Accept an exact occurrence-indexed mixed view only when every port appears once with its declared mode and the Probe side has existing checked bundle evidence. Reject missing/foreign/duplicate/mode-changed ports, an event assigned to the non-Probe port, untyped non-Probe output, and a source/bundle substitution. Verification must not dispatch.
horizon: one finite mixed-mode source Ask, one existing Probe evidence chain, one typed non-Probe result with explicit route/authority coordinate, and no dispatch; no general executor, compiler procedure artifact, resolver procedure artifact, event for non-Probe modes, cross-binding bridge, scheduler, controller, table, or opcode
relevant_decisions: D-0125, D-0150, D-0156, D-0157, D-0158
relevant_failures: F-0001, F-0002
if_pass: rederive the strongest remaining Phase 5/6 residual from the stable plan and pending conformance; do not reopen passed QASK/QSUCC/QREADY/QACTUAL fixtures without a new breaker
if_fail: reopen the earliest mixed-port occurrence, port/mode, typed-result, authority-route, or event separation relation; do not build a general compiler, dispatcher, scheduler, controller, table, or opcode
<!-- LIVE_FRONTIER_END -->

## Smallest decisive fixture

Construct one source `Ask` whose open ports have distinct modes. Rewalk the exact occurrence and require a complete typed tagged field: existing checked `Probe` evidence only for the Probe port, and a separately typed, route-indexed non-Probe result only for the non-Probe port. No event can substitute for the latter.

The smallest wrong implementation makes every port a Probe, stores a non-Probe output as a raw/event return, drops its route/authority coordinate, selects one port, accepts an untyped result, or dispatches while verifying.

## Prohibitions at this boundary

- Do not introduce a general executor, compiler, resolver, dispatcher, scheduler, controller, table, or opcode.
- Do not create an event for a Pure, Generate, Check, or Warrant port merely because the mixed view verifies.
- Do not erase exact source occurrence, port/mode, compiler, runtime-program, Probe event/route/decoder/resolution, or non-Probe typed-result/authority identities.

## Exit

On pass, update conformance and project state, then rederive the next strongest residual. On failure, preserve the actual return and reopen the earliest implicated mixed-port relation rather than broadening the architecture.
