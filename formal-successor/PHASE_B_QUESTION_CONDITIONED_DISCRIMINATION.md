# Phase B question-conditioned discrimination boundary

This pass preserves typed local support-profile and local-equivalence carriers for a question on a
live carrier. It does not define compatibility, profile fibers, projections, answer semantics, or
protected global behavioral equivalence. The local/global distinction is mandatory. Gate B remains
pending.

```text
node tools/phase_b_question_conditioned_discrimination.js check
node tools/phase_b_question_conditioned_discrimination_check.js --compile
```

The independent checker rejects 10 mutations, including global-equivalence collapse, profile
semantics, program leakage, source promotion, and axioms. The next residual is the question
refinement preorder.
