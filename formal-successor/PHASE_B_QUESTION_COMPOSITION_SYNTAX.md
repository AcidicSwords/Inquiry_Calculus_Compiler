# Phase B question-composition syntax boundary

This pass preserves dependent bind, independent tensor, guard refinement, answer plug, and
simultaneous multi-port substitution as typed candidate syntax. It does not evaluate conjunction,
substitution, answers, or programs. All nine source records remain `Ambiguous` obligations and Gate
B remains pending.

```text
node tools/phase_b_question_composition_syntax.js check
node tools/phase_b_question_composition_syntax_check.js --compile
```

The independent checker rejects 11 mutations including source promotion, missing grammar forms,
semantic answer evaluation, program leakage, and axioms. The next residual is
question-conditioned discrimination.
