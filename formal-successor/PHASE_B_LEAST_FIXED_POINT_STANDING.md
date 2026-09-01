# Phase B least-fixed-point standing boundary

## Status

`FORMAL-B-LEAST-FIXED-POINT-STANDING-001` is a checked predecessor reconstruction at
the standing boundary. It does not promote Formal Gate B or choose successor primitives.

The reconstruction binds six exact source records at v2.0 lines 4579–4607. Of these,
one remains `Unproved`, four remain `Ambiguous`, and one is a `FormalTheorem`. The Lean
theorems below discharge a precise typed reading; they do not silently change the stored
classification of the surrounding prose and displays.

## Retained relations

The module keeps separate:

- an explicit grounded-ingress predicate;
- route targets and positive prerequisite incidence;
- a supplied closed-support relation indexed by the current standing predicate;
- monotonicity of that supplied relation;
- pre-fixed points, fixed points, and the intersection of all pre-fixed points;
- mathematical fixed-point existence and separately defined finite iteration.

`Standing` is the least fixed point of the operator that adds explicit ingress and targets
with a closed support route. The proof derives both fixed-point directions and leastness
without an axiom, choice principle, algorithm, or greatest-fixed-point selection.

## Decisive contrast and localization

The finite model contains five candidates:

- an explicit ingress root;
- a child supported by that root;
- two nodes supported only by one another;
- an orphan with no ingress and no support route.

The least fixed point contains exactly the root and child. The two-node rootless positive
cycle is excluded. A larger set containing the rooted chain and cycle is also a fixed point,
so fixed-point equality alone is decisively insufficient; leastness carries the distinction.
The orphan shows that having no route dependencies is not grounded ingress.

The general rootless-region theorem retains a finite enumeration, absence of ingress, and a
positive same-region prerequisite for every closed route. Its complement is a pre-fixed point,
which excludes the region from least standing. This is the exact boundary of the predecessor
theorem, not a claim about negative, mixed, or externally discharged cycles.

## Effective-convergence boundary

`Iterate` is defined independently. The concrete finite model stabilizes at step two, and
every iteration is included in least standing. No theorem in this ratchet claims termination,
computability, or convergence for an arbitrary carrier or dynamically changing operator.

## Machine checks

`node tools/least_fixed_point_standing_check.js --compile` checks exact source identities and
statuses, builds the module with warnings rejected, audits 17 proof declarations for axioms,
and rejects 17 source ablations. The broad Lean/kernel and Rust-preservation suites remain
required before the ratchet is recorded.

## Coverage and reopening

Coverage is limited to the supplied monotone predicate operator and the finite contrast above.
It provides no route search, standing decision procedure, execution semantics, warrant,
successor promotion, Gate-B passage, or Rust semantic change. Reopen if source classification,
operator indices, monotonicity, leastness, grounding, positive-cycle assumptions, proof
dependencies, or the mathematical/effective-convergence distinction changes.
