# Phase B question-refinement preorder boundary

This pass preserves only typed local candidate shapes for precision, joint refinement, and an
active-representation extension. It does not prove profile determination, functional
factorization, kernel inclusions, joint-kernel equality, nonredundancy, or representation
semantics. Gate B remains pending.

```text
node tools/phase_b_question_refinement_preorder.js check
node tools/phase_b_question_refinement_preorder_check.js --compile
```

The independent checker rejects source promotion, carrier loss, semantic/kernel leakage,
program leakage, and axioms. The next residual is question-refinement semantics.
