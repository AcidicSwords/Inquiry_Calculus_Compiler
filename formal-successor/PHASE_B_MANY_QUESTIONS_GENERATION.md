# Phase B one-relation-many-questions generation boundary

This pass preserves typed opening variation over one relation occurrence. It does not introduce a
factor-question primitive or semantic answer-generation, probe, or program law. All three source
records remain `Ambiguous` obligations and Gate B remains pending.

```text
node tools/phase_b_many_questions_generation.js check
node tools/phase_b_many_questions_generation_check.js --compile
```

The independent checker rejects ten mutations including source promotion, a fabricated factor
primitive, semantic generation, program leakage, and axioms. The next residual is discharge-mode
syntax.
