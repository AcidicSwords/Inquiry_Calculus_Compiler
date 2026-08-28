# Formal successor control plane

This directory keeps successor construction state separate from predecessor evidence while both
remain in one repository.

## Authority and inputs

- `FORMAL_CALCULUS_CONSTRUCTION_SPEC.md` controls how the successor is built and accepted; it is not
  itself the successor semantics.
- `Questions.txt` is an external inquiry corpus; its wording does not create primitives.
- `../Inquiry_Calculus_v2_0.tex` is predecessor semantic authority.
- `../formal/` contains candidate successor definitions and proofs at their demonstrated coverage.
- `../crates/` and predecessor ledgers establish implementation actuality, not semantic truth.

`ACTIVE_INPUTS.json` pins the imported inputs and branch point. `PREDECESSOR_BASELINE.md` records
the deferred Rust cursor. Successor evidence, decisions, failures, inventories, and propagation
reports live here and never overwrite predecessor state merely because the successor differs.

`ENGINEERING_QUESTION_PROGRAMS.json` makes the pinned corpus operational. Autonomous cycles select
line-addressed questions from the coding section and compose them with paired, direction-aware
questions from `Reciprocal why:` after actual returns. These programs govern inquiry sequencing;
they do not make the question wording semantic authority. New traces pin both files, validate cited
source lines against the declared programs, and cannot close a sealed cycle without a validated
coding/reciprocal composition or explicit typed reciprocal inapplicability.

The root `IMPLEMENTATION_FRONTIER.md` is intentionally the one live cursor so the existing inquiry
harness can autonomously recur on this branch without being redirected to the old Rust residual.
`AUTONOMOUS_ITERATION.md` is the restart-safe operational runbook for one finite ratchet and for
Phase A-N progression; it does not own moving state or semantic authority.

## Propagation back to Rust

Before Formal Gate F, Rust semantic expansion is frozen. After Gate F, an accepted formal change
may reach Rust only through a report that identifies the changed formal declarations, complete
dependency closure, preservation/correction classification, proof evidence, generated
successor-to-Rust delta, affected conformance checks, and reopening condition.
