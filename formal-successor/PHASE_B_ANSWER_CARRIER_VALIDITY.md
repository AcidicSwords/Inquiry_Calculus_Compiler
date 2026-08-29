# Phase B answer-carrier and valid-completion boundary

This pass preserves the displayed answer-carrier and valid-completion shapes as typed candidates.
Neither candidate is an actual return, standing warrant, probe, or program. Gate B remains pending.

```text
node tools/phase_b_answer_carrier_validity.js check
node tools/phase_b_answer_carrier_validity_check.js --compile
```

The independent checker rejects nine mutations including actual-return, warrant, probe, and axiom
leakage. The next residual is the proposition-not-warrant boundary.
