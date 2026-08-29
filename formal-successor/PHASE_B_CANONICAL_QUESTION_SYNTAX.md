# Phase B canonical-question syntax boundary

This local Phase B pass reconstructs the exact v2.0 canonical-question definition as a typed
occurrence over a partial binding. It does not define its answer carrier, valid-completion relation,
probe, program, or successor question semantics; those remain explicit obligations. Gate B remains
pending.

```text
node tools/phase_b_canonical_question_syntax.js check
node tools/phase_b_canonical_question_syntax_check.js --compile
```

The independent checker rejects nine mutations: source loss, Gate B promotion, carrier and typing
loss, obligation loss, answer-carrier leakage, probe/program leakage, and axiomatic completion.
The next residual is the answer-carrier and validity boundary.
