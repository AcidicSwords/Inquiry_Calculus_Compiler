# Implementation Frontier

This file is the sole live implementation cursor. Historical work belongs in Git, demonstrated behavior in `CONFORMANCE_STATUS.md`, and accepted choices or failures in the JSONL ledgers.

Repository actuality must be probed before acting. The current residual was derived from demonstrated implementation behavior through predecessor coordinate `0318e97`; that coordinate is ancestry, not a claim that a tracked document can name its own final commit or worktree state.

## Immediate demonstrated predecessor

`QASK-MIXED-MODE-004` established `MixedModeSourceAskDischarge`, the derived nonexecuting view of one source `Ask` whose open ports do not share a discharge mode. It partitions the exact re-walked occurrence's declared port field by mode, admits the Probe subset through the unchanged existing bundle checker, and requires each non-Probe port to carry an exact typed result, declared route, rechecked resolution path, and the occurrence's own versions and provenance in a record with no event field. The all-Probe entry point keeps its exact contract. No dispatch, event for a non-Probe mode, compiler procedure, resolver procedure, table, or opcode was added.

## Strongest live obligation

Determine the smallest whole-question resolution gate over that mixed view: the point at which a mixed-mode occurrence's complete port evidence becomes exactly one `ResolutionOutcome`, and only `Supported` may reach the checked source continuation.

<!-- LIVE_FRONTIER_BEGIN -->
id: QASK-MIXED-RESOLVE-005
plan_phase: 8
goal: Derive the smallest whole-question resolution gate over one checked mixed-mode discharge view, so that Probe and non-Probe port evidence jointly determine exactly one of the five resolution outcomes and only Supported reaches the exact checked source continuation.
protected_difference: The existing finite gate consumes an all-Probe bundle only, so a mixed occurrence currently has complete port evidence and no lawful way to become one outcome. Resolving port-by-port, or resolving from the Probe side alone, would let a Probe return select a continuation while a declared non-Probe port is still undischarged, and would manufacture a Cartesian answer the relation does not support. ExactEmpty, Undefined, Unsupported, and Unknown must stay separate from Supported and must never invoke the continuation.
discriminator: Take one finite mixed-mode source Ask with one Probe and one non-Probe port and its existing checked mixed view. Accept exactly one whole-question outcome derived from every port's evidence together. Reject an outcome derived from a proper subset of the ports, a non-Supported outcome reaching the continuation, a Supported answer that omits the non-Probe port's contribution or its authority route, a foreign source program at the equal question, and any resolution that dispatches or re-decodes an event.
horizon: one finite mixed-mode source Ask, its existing checked mixed view, caller-declared finite resolution leaves, and one linear continuation; no general resolver procedure artifact, executor, compiler procedure artifact, event for non-Probe modes, cross-binding bridge, scheduler, controller, table, or opcode
relevant_decisions: D-0150, D-0156, D-0158, D-0159
relevant_failures: F-0001, F-0002
if_pass: rederive the strongest remaining Phase 7/8 residual from the stable plan and pending conformance; do not reopen passed QASK/QSUCC/QREADY/QACTUAL/QRESOLUTION fixtures without a new breaker
if_fail: reopen the earliest whole-question coverage, outcome separation, supported-answer completeness, or continuation-admission relation; do not build a general resolver, executor, dispatcher, scheduler, controller, table, or opcode
<!-- LIVE_FRONTIER_END -->

## Smallest decisive fixture

Reuse the existing checked mixed-mode view over one Probe and one non-Probe port. Derive one whole-question outcome from both ports' evidence, admit the exact source continuation only from `Supported`, and hold the other four outcomes to a typed residual or justified stop. Vary the port field, the outcome, and the source program to show that each is load-bearing.

The smallest wrong implementation resolves the Probe port alone, unions per-port outcomes instead of checking them jointly, lets a non-Supported outcome invoke the continuation, drops the non-Probe port's contribution or authority route from the supported answer, accepts an equal-question foreign source program, or redispatches while resolving.

## Prohibitions at this boundary

- Do not introduce a general executor, compiler, resolver procedure artifact, dispatcher, scheduler, controller, table, or opcode.
- Do not create an event for a Pure, Generate, Check, or Warrant port merely because the whole-question gate resolves.
- Do not let any of `ExactEmpty`, `Undefined`, `Unsupported`, or `Unknown` reach the checked continuation, and do not collapse them into one another.
- Do not erase exact source occurrence, port/mode, compiler, runtime-program, Probe event/route/decoder/resolution, or non-Probe typed-result/authority identities.

## Exit

On pass, update conformance and project state, then rederive the next strongest residual. On failure, preserve the actual return and reopen the earliest implicated mixed-port relation rather than broadening the architecture.
