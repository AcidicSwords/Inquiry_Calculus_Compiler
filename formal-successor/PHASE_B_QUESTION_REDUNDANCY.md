# Phase B question-redundancy boundary

This pass types redundancy as factorization of a question profile through the current local
representation. It proves the factorization-to-kernel direction and constructs the reverse only
when the current representation covers its advertised type. The predecessor's unconditioned
equivalence therefore remains unaccepted.

```text
node tools/phase_b_question_redundancy.js check
node tools/phase_b_question_redundancy_check.js --compile
```

The independent checker rejects lost coverage, global improvement or representation promotion,
program leakage, source changes, and axioms. The next residual is precision-not-improvement.
