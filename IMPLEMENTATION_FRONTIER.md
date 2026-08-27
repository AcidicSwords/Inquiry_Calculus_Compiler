# Implementation Frontier

This file is the sole live implementation cursor. Historical work belongs in Git, demonstrated behavior in `CONFORMANCE_STATUS.md`, and accepted choices or failures in the JSONL ledgers.

Repository actuality must be probed before acting. The current residual was derived from demonstrated implementation behavior through predecessor coordinate `f0506ce`; that coordinate is ancestry, not a claim that a tracked document can name its own final commit or worktree state.

## Immediate demonstrated predecessor

`QASK-MIXED-RESOLVE-005` established the whole-question resolution gate over a checked mixed-mode view. Reaching it required a prerequisite the inspection uncovered: finite decode and the five-way classifier were single-open-port specializations, so a multi-port question had no lawful classification at all. Both now carry a shared answer-port scope whose arity-one entry points are unchanged and whose port-indexed entry points type a route against the named port's own schema carrier. Over that, `resolve_mixed_mode_question` requires every open port to be accounted for exactly once, takes the whole-question kind from the first non-Supported port with its residual intact, and admits a joint `Supported` only when every decoded completion agrees with each non-Probe port's checked typed result. `admit_mixed_mode_continuation` binds the exact occurrence continuation from that answer alone. Successor derivation was deliberately left out. A later audit against the canonical supported-answer type established that the joint record projects its per-component contributions but not the nonempty member set of whole completions that `SuppAns(q)` also carries, and that the agreement check enforces authority preservation rather than the canonical cross-port relation obligation (D-0162).

## Strongest live obligation

Bring the whole-question supported answer into the canonical `SuppAns(q)` shape — one route, one evidence environment, one nonempty member set of whole completions, and one component-indexed support witness — so that it can be supplied whole to a continuation and, after that, to the occurrence-indexed successor.

<!-- LIVE_FRONTIER_BEGIN -->
id: QSUPPANS-WHOLE-SHAPE-006
plan_phase: 8
goal: Give the whole-question supported answer its canonical SuppAns shape - one checked route, one evidence environment, one nonempty member set of whole completions, and one component-indexed support witness - so the record can be supplied whole to a continuation, and only then derive the occurrence-indexed successor from it.
protected_difference: Canonical SuppAns(q) carries a nonempty member set S of completions over the whole port field, and equality of supported answers is equality of the whole proof-carrying record, not of its member projection. The implemented joint answer exposes only the component-indexed contribution map and never projects S, so a consumer must reach into one port's answer set to find the members. That is recoverable at one Probe port and ambiguous at two. QSucc and its capture-safe binding are separately keyed to one single-event Probe-only AdmittedFiniteAnswerSet, so a mixed occurrence reaches its continuation but cannot yet produce NextSourceControl, and projecting the joint answer down to one port to reuse them would drop every other port's contribution, declared mode, and authority route.
discriminator: Take one finite mixed-mode occurrence whose joint Supported answer is already admitted at its continuation. Require the record to project exactly one nonempty member set of whole completions alongside its per-component route and provenance, and derive exactly one NextSourceControl from that whole record. Reject a member set assembled from a proper subset of the ports, a record whose per-component map and member set disagree about a port, a successor derived by projecting to one port's answer set, two occurrences sharing a question and every per-port answer yet requiring different successors being collapsed, and any derivation that dispatches, re-decodes, or executes the continuation.
horizon: one finite mixed-mode occurrence, its existing checked contributions, caller-declared finite completions, and one linear successor; no second successor relation or history, no general executor, resolver procedure artifact, compiler procedure artifact, event for non-Probe modes, cross-binding bridge, scheduler, controller, table, or opcode
relevant_decisions: D-0150, D-0156, D-0158, D-0159, D-0160, D-0161, D-0162
relevant_failures: F-0001, F-0002
if_pass: rederive the strongest remaining Phase 7/8 residual from the stable plan and pending conformance; do not reopen passed QASK/QSUCC/QREADY/QACTUAL/QRESOLUTION fixtures without a new breaker
if_fail: reopen the earliest member-set coverage, component-provenance retention, route or evidence-environment identity, successor identity, or continuation-binding relation; do not build a second successor authority, a general executor, a dispatcher, a scheduler, a controller, a table, or an opcode
<!-- LIVE_FRONTIER_END -->

## Smallest decisive fixture

Reuse the admitted mixed-mode continuation. Project its member set and its per-component route and provenance, show the two agree at every port, then derive one successor from the whole record. Ablate the field: a member set taken from a proper subset of the ports, or an answer stripped of a non-Probe port's mode or route, must not reconstruct the same successor. Retain the existing single-port successor contract unchanged for single-port occurrences.

The smallest wrong implementation projects the whole-question answer down to one port's `AdmittedFiniteAnswerSet` and calls the existing successor, treats the per-component map as the member set, drops the non-Probe contributions, collapses two occurrences that share every per-port answer but differ in continuation, or normalizes by executing the continuation.

## Prohibitions at this boundary

- Do not add a second successor relation, question history, or route authority.
- Do not introduce a general executor, compiler, resolver procedure artifact, dispatcher, scheduler, controller, table, or opcode.
- Do not create an event for a Pure, Generate, Check, or Warrant port merely because a successor is derived.
- Do not erase exact source occurrence, port/mode, compiler, runtime-program, Probe event/route/decoder/resolution, or non-Probe typed-result/authority identities.

## Exit

On pass, update conformance and project state, then rederive the next strongest residual. On failure, preserve the actual return and reopen the earliest implicated mixed-port relation rather than broadening the architecture.
