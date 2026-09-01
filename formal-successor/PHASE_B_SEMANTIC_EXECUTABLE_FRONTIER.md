# Phase B semantic and executable frontiers

`FORMAL-B-SEMANTIC-EXECUTABLE-FRONTIER-001` binds nine exact `LegacyObligation` records at v2.0
lines 5003–5054, retaining nine `Ambiguous` classifications. Semantic membership requires
formability, applicability, and either productive-unresolvedness or required discharge. Executable
membership additionally requires execution and either executable class `productive` or required
discharge; each candidate field is then projected through the existing required-nondominant carrier.

The finite model separates a semantic productive nonexecutable occurrence, an executable productive
occurrence, executable idle and unknown occurrences, a required idle occurrence, and an
inapplicable occurrence. Idle and unknown do not pass the productive-class branch; required idle
passes only the required branch. The membership predicates do not schedule or authorize work.

This does not define a total classifier, resource preorder, scheduler, selection policy, or
successor semantics. `node tools/semantic_executable_frontier_check.js --compile` verifies source
binding, rejects eight targeted ablations, and audits nine axiom-free proofs.
