# Phase B precision-not-improvement boundary

This pass represents precision comparison and improvement assessment as distinct tagged relation
kinds. It does not define protected behavior, cost, robustness, preference, or a global ordering;
the v2.0 law remains source-bound with those criteria explicit as obligations.

```text
node tools/phase_b_precision_not_improvement.js check
node tools/phase_b_precision_not_improvement_check.js --compile
```

The independent checker rejects kind collapse, automatic improvement, source changes, program
leakage, and axioms. The next residual is relational sections.
