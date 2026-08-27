# Implementation Frontier

This file is the sole live implementation cursor. Historical work belongs in Git, demonstrated behavior in `CONFORMANCE_STATUS.md`, and accepted choices or failures in the JSONL ledgers.

Repository actuality must be probed before acting. The current residual was derived from demonstrated implementation behavior through predecessor coordinate `f0506ce`; that coordinate is ancestry, not a claim that a tracked document can name its own final commit or worktree state.

## Immediate demonstrated predecessor

`QASK-MIXED-RESOLVE-005` established the whole-question resolution gate over a checked mixed-mode view. Reaching it required a prerequisite the inspection uncovered: finite decode and the five-way classifier were single-open-port specializations, so a multi-port question had no lawful classification at all. Both now carry a shared answer-port scope whose arity-one entry points are unchanged and whose port-indexed entry points type a route against the named port's own schema carrier. Over that, `resolve_mixed_mode_question` requires every open port to be accounted for exactly once, takes the whole-question kind from the first non-Supported port with its residual intact, and admits a joint `Supported` only when every decoded completion agrees with each non-Probe port's checked typed result. `admit_mixed_mode_continuation` binds the exact occurrence continuation from that answer alone. Successor derivation was deliberately left out.

## Strongest live obligation

Determine how a whole-question supported answer over several ports reaches the occurrence-indexed question successor, given that `QSucc` and its continuation binding are currently keyed to one single-event `AdmittedFiniteAnswerSet`.

<!-- LIVE_FRONTIER_BEGIN -->
id: QSUCC-WHOLE-ANSWER-006
plan_phase: 8
goal: Derive how one whole-question supported answer over several ports reaches the exact occurrence-indexed question successor, without collapsing it into a single port's answer set and without adding a second successor authority.
protected_difference: QSucc, its capture-safe binding, and every passing successor fixture are keyed to one AdmittedFiniteAnswerSet, which is single-event and Probe-only by construction. A mixed-mode occurrence therefore reaches its checked continuation but cannot yet produce NextSourceControl. Passing one port's answer set into the existing successor would silently drop every other port's contribution and its declared mode and authority route, while equal questions and equal per-port answers must still reconstruct protected-different successors.
discriminator: Take one finite mixed-mode occurrence whose whole-question answer is already admitted at its continuation. Derive exactly one NextSourceControl from the complete port field. Reject a successor derived from any proper subset of the ports, a successor that drops a non-Probe port's mode or authority route, two occurrences that share a question and every per-port answer yet require different successors being collapsed, and any derivation that dispatches, re-decodes, or executes the continuation.
horizon: one finite mixed-mode occurrence, its existing whole-question supported answer and admitted continuation, and one linear successor; no second successor relation or history, no general executor, resolver procedure artifact, compiler procedure artifact, event for non-Probe modes, cross-binding bridge, scheduler, controller, table, or opcode
relevant_decisions: D-0150, D-0156, D-0158, D-0159, D-0160, D-0161
relevant_failures: F-0001, F-0002
if_pass: rederive the strongest remaining Phase 7/8 residual from the stable plan and pending conformance; do not reopen passed QASK/QSUCC/QREADY/QACTUAL/QRESOLUTION fixtures without a new breaker
if_fail: reopen the earliest whole-answer coverage, per-port mode/route retention, successor identity, or continuation-binding relation; do not build a second successor authority, a general executor, a dispatcher, a scheduler, a controller, a table, or an opcode
<!-- LIVE_FRONTIER_END -->

## Smallest decisive fixture

Reuse the admitted mixed-mode continuation. Derive one successor from the whole port field, then ablate the field: a successor built from the Probe port alone, or from an answer stripped of a non-Probe port's mode or route, must not be accepted as the same successor. Retain the existing single-port successor contract unchanged for single-port occurrences.

The smallest wrong implementation projects the whole-question answer down to one port's `AdmittedFiniteAnswerSet` and calls the existing successor, drops the non-Probe contributions, collapses two occurrences that share every per-port answer but differ in continuation, or normalizes by executing the continuation.

## Prohibitions at this boundary

- Do not add a second successor relation, question history, or route authority.
- Do not introduce a general executor, compiler, resolver procedure artifact, dispatcher, scheduler, controller, table, or opcode.
- Do not create an event for a Pure, Generate, Check, or Warrant port merely because a successor is derived.
- Do not erase exact source occurrence, port/mode, compiler, runtime-program, Probe event/route/decoder/resolution, or non-Probe typed-result/authority identities.

## Exit

On pass, update conformance and project state, then rederive the next strongest residual. On failure, preserve the actual return and reopen the earliest implicated mixed-port relation rather than broadening the architecture.
