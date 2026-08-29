# Phase B question-refinement semantic boundary

This pass machine-checks the sufficient direction from a factorization witness to local
profile-kernel inclusion. It constructs the reverse direction only under explicit surjective
coverage of the finer profile. It does not make the v2.0 displayed equivalence unconditional,
prove joint/active kernel laws, or establish global representation semantics.

```text
node tools/phase_b_question_refinement_semantics.js check
node tools/phase_b_question_refinement_semantics_check.js --compile
```

The independent checker rejects removal of the coverage precondition, global-order promotion,
program leakage, source changes, and axioms. The next residual is joint and active refinement.
