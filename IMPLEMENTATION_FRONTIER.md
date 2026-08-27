# Implementation Frontier

This file is the sole live implementation cursor. Historical work belongs in Git, demonstrated behavior in `CONFORMANCE_STATUS.md`, and accepted choices or failures in the JSONL ledgers.

Repository actuality must be probed before acting. The current residual was derived from demonstrated implementation behavior through predecessor coordinate `69a4ec3`; that coordinate is ancestry, not a claim that a tracked document can name its own final commit or worktree state.

## Immediate demonstrated predecessor

`QRESOLVE-CROSS-PORT-009` closed the one canonical obligation of this gate that was unenforced rather than merely uncovered. Probing confirmed why: a completion candidate records no relation evaluation, and relation-level standing cannot discriminate one tuple from another, so nothing could refuse a combination the relation forbids. The gate now requires a caller-declared finite membership table for the question's relation and tests every decoded member against it after the per-port checks. Exclusion under exact coverage makes the whole question `Unsupported` and names the offending completion; the same absence under partial coverage makes it `Unknown`. Membership is read, never computed. Its predecessors established the mixed-mode port-evidence separation, the `Generate` proposal carrier, the port-indexed decode and resolution gates, the whole-question resolution gate, the member-set projection, the one shared successor relation, and the route and evidence coordinates of record equality.

## Strongest live obligation

Exercise the multi-Probe-port mixed question. Several checks now standing in the gate are vacuous at one Probe port and were recorded as such: the member-set agreement across Probe ports, the choice of which non-Supported port supplies the whole-question outcome, and the shared-event admission interacting with mixed modes. Each is a policy the code takes without any breaker having forced it.

<!-- LIVE_FRONTIER_BEGIN -->
id: QRESOLVE-MULTI-PROBE-010
plan_phase: 8
goal: Exercise a mixed question with two Probe ports, so that the joint member-set agreement, the choice of which non-Supported port supplies the whole-question outcome, and the shared-event admission under mixed modes become demonstrated rather than assumed.
protected_difference: Three behaviours the gate already takes are vacuous at one Probe port and no breaker has forced any of them. First, the member set is taken from the Probe ports' witnessed completion fields and required identical across them: with one port that requirement never fires, and whether equality is the right relation rather than intersection or a jointness certificate is undecided. Second, when several ports are non-Supported the gate reports the first in canonical port order and discards the others' residuals, which is a policy chosen for determinism and never argued from the calculus. Third, an explicit shared-event admission lets several Probe ports reuse one event, and that has never met a question that also has a non-Probe port. Each is a place where the code could be wrong without any current fixture noticing.
discriminator: Construct one mixed question with two Probe ports and one non-Probe port. Require the joint answer to exist only when both Probe ports witness the same completion field, and reject a pair witnessing different fields. Make both Probe ports non-Supported with different outcome kinds and require the whole-question result to account for both rather than silently discard one. Admit both Probe ports against one shared event and require the port-indexed evidence to stay distinct. Reject a member set taken from one port alone, a discarded residual presented as absent, and a shared event that collapses two ports' route or resolution provenance.
horizon: one finite mixed-mode occurrence with two Probe ports and one non-Probe port, its caller-declared membership table, and one shared-event admission; no general relation evaluator or search, no canonical supported-answer artifact, no second successor relation, no general executor, resolver procedure artifact, compiler procedure artifact, event for non-Probe modes, cross-binding bridge, scheduler, controller, table authority, or opcode
relevant_decisions: D-0150, D-0158, D-0159, D-0161, D-0162, D-0163, D-0165, D-0166, D-0167
relevant_failures: F-0001, F-0002
if_pass: rederive the strongest remaining Phase 7/8 residual from the stable plan and pending conformance; do not reopen passed QASK/QSUCC/QREADY/QACTUAL/QRESOLUTION/QSUPPANS fixtures without a new breaker
if_fail: reopen the earliest member-set agreement, non-Supported selection, or shared-event separation relation; do not build a general relation evaluator, a second successor authority, a dispatcher, a scheduler, a controller, or an opcode
<!-- LIVE_FRONTIER_END -->

## Smallest decisive fixture

Extend the mixed question to two Probe ports and one non-Probe port. Resolve it once where both Probe ports witness the same completion field, and once where they do not. Resolve it again with both Probe ports non-Supported under different kinds. Admit both against one shared event and compare their retained route and resolution paths.

The smallest wrong implementation takes the member set from whichever Probe port it scanned first, reports one non-Supported port as though it were the only one, lets a shared event give two ports one route or one resolution path, or treats a disagreement between Probe ports as a checker failure rather than a lawful non-Supported outcome.

## Prohibitions at this boundary

- Do not build a general relation evaluator, solver, or search; membership stays decided only by a caller-declared finite table under its own declared coverage.
- Do not treat an uncovered region as excluded, or a failed membership lookup as negation.
- Do not introduce a canonical supported-answer artifact, envelope, or content-addressed identity for a derived record.
- Do not add a second successor relation, question history, or route authority.
- Do not introduce a general executor, compiler, resolver procedure artifact, dispatcher, scheduler, controller, table authority, or opcode.
- Do not create an event for a Pure, Generate, Check, or Warrant port merely because several Probe ports share one.
- Do not erase exact source occurrence, port/mode, compiler, runtime-program, Probe event/route/decoder/resolution, or non-Probe typed-result/authority identities.

## Exit

On pass, update conformance and project state, then rederive the next strongest residual. On failure, preserve the actual return and reopen the earliest implicated mixed-port relation rather than broadening the architecture.
