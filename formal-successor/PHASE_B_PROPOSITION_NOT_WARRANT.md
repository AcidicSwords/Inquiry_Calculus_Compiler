# Phase B proposition-not-warrant boundary

This pass preserves the v2.0 distinction between a fully bound relation proposition and a
warranted fact. `FullyBoundPropositionSyntax` is only a typed candidate claim over a completion;
actual return, standing warrant, probe, and program authority remain explicit obligations. Gate B
remains pending.

```text
node tools/phase_b_proposition_not_warrant.js check
node tools/phase_b_proposition_not_warrant_check.js --compile
```

The independent checker rejects ten mutations, including source/status promotion, standing and
actual-return leakage, probe leakage, and axioms. The next residual is many-questions generation.
