# Implementation Frontier

This file is the sole live implementation cursor. Historical work belongs in Git, demonstrated behavior in `CONFORMANCE_STATUS.md`, and accepted choices or failures in the JSONL ledgers.

Repository actuality must be probed before acting. The current residual was derived from demonstrated implementation behavior through predecessor coordinate `69a4ec3`; that coordinate is ancestry, not a claim that a tracked document can name its own final commit or worktree state.

## Immediate demonstrated predecessor

`QSUPPANS-WHOLE-SHAPE-006` gave the whole-question answer two separate projections: the nonempty member set of rechecked whole completions, and the component-indexed route and provenance map. It also extracted `derive_successor_position` in `ic-core` as the one successor relation, which reads only the occurrence; `derive_question_successor` delegates to it unchanged, and `derive_mixed_mode_successor` supplies the whole-question answer as a second carrier for that same relation. Both carriers reach one next position for one occurrence. Its predecessors established the mixed-mode port-evidence separation, the port-indexed decode and resolution gates, and the whole-question resolution gate; D-0162 narrowed the agreement check's canonical hook to authority preservation and recorded that no cross-port relation-membership test exists.

## Strongest live obligation

Establish whether the canonical route and evidence-environment projections of a supported answer are recoverable from the component map or are irreducible fields, and give `SuppAns` record equality its canonical meaning — equality of the whole proof-carrying record rather than of its member projection or of an ad hoc struct comparison.

<!-- LIVE_FRONTIER_BEGIN -->
id: QSUPPANS-RECORD-IDENTITY-007
plan_phase: 8
goal: Determine whether a supported answer's canonical route and evidence-environment projections are recoverable from the component-indexed map or are irreducible fields, and give supported-answer equality its canonical meaning as equality of the whole proof-carrying record.
protected_difference: Canonical supported-answer equality is equality of the whole record - route, evidence environment, member set, and support witness - explicitly not equality of the member projection. The implemented joint answer derives its equality from a structural comparison of whatever fields it happens to hold, so two answers with equal members and equal contributions compare equal even if they were reached by different declared routes or evidence environments, and nothing establishes that those two projections are in fact recoverable from the contributions rather than missing. Until that is settled, a fold, cache, or reopening decision keyed on answer equality could silently merge two answers the calculus keeps apart.
discriminator: Construct two whole-question answers over one occurrence that agree on every member and on every port's contributed result, but differ in a declared route or evidence coordinate. Either exhibit that difference as a difference in the record - establishing the projections as irreducible - or exhibit an exact derivation of route and evidence from the component map and show the two answers are then genuinely the same record. Reject an equality that ignores a protected route or evidence difference, an equality resting on the member projection alone, and any answer whose claimed route or evidence coordinate is not rechecked against its own port evidence.
horizon: one finite mixed-mode occurrence, its existing checked contributions and member set, and finite caller-declared route and evidence coordinates; no canonical supported-answer artifact or envelope, no second successor relation, no general executor, resolver procedure artifact, compiler procedure artifact, event for non-Probe modes, cross-binding bridge, scheduler, controller, table, or opcode
relevant_decisions: D-0150, D-0156, D-0158, D-0159, D-0160, D-0161, D-0162, D-0163
relevant_failures: F-0001, F-0002
if_pass: rederive the strongest remaining Phase 7/8 residual from the stable plan and pending conformance; do not reopen passed QASK/QSUCC/QREADY/QACTUAL/QRESOLUTION/QSUPPANS fixtures without a new breaker
if_fail: reopen the earliest route-projection, evidence-environment, member-set, or record-equality relation; do not introduce a canonical supported-answer artifact, a second successor authority, a general executor, a dispatcher, a scheduler, a controller, a table, or an opcode
<!-- LIVE_FRONTIER_END -->

## Smallest decisive fixture

Take the existing joint answer and build a second one over the same occurrence with the same members and the same per-port results, but reached through a different declared route or evidence coordinate. Either the record distinguishes them, or an exact derivation shows the coordinate was never independent. Whichever holds, state it as the answer rather than leaving equality unexamined.

The smallest wrong implementation compares answers by member projection, compares whatever struct fields happen to exist and calls that canonical equality, accepts a declared route or evidence coordinate without rechecking it against the port evidence it claims to summarize, or introduces a canonical supported-answer artifact to force equality by content address.

## Prohibitions at this boundary

- Do not introduce a canonical supported-answer artifact, envelope, or content-addressed identity for a derived record.
- Do not add a second successor relation, question history, or route authority.
- Do not introduce a general executor, compiler, resolver procedure artifact, dispatcher, scheduler, controller, table, or opcode.
- Do not create an event for a Pure, Generate, Check, or Warrant port merely because a record is compared.
- Do not erase exact source occurrence, port/mode, compiler, runtime-program, Probe event/route/decoder/resolution, or non-Probe typed-result/authority identities.

## Exit

On pass, update conformance and project state, then rederive the next strongest residual. On failure, preserve the actual return and reopen the earliest implicated mixed-port relation rather than broadening the architecture.
