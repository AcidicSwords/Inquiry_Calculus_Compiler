# Phase B blocked and Unknown residuals

`FORMAL-B-BLOCKED-UNKNOWN-RESIDUAL-001` binds three exact `LegacyObligation` records at v2.0
lines 5055–5068, retaining one `Ambiguous` and two `Unproved` classifications. A required
nonexecutable undischarged occurrence remains a `blocked`, `unknown`, or resource residual outside
the executable frontier; a resolvedness claim does not suppress it.

The finite model distinguishes required blocked work from idle executable work. It separately
records failed separator, path, counterexample, proof, and useful-question searches; each carries
no established equivalence, impossibility, necessity, or irrelevance conclusion.

This does not define a solver, scheduler, negative proof rule, support, warrant, or successor
semantics. `node tools/blocked_unknown_residual_check.js --compile` verifies source binding,
rejects six targeted ablations, and audits eight axiom-free proofs.
