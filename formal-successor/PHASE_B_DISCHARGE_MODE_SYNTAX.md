# Phase B discharge-mode syntax boundary

This pass preserves the five typed route classifiers `Pure`, `Generate`, `Probe`, `Check`, and
`Warrant` for an open port. A classification is not execution, an actual return, independent
admission, or standing policy. In particular, a generative completion cannot self-discharge a
Probe, Check, or Warrant port. Gate B remains pending.

```text
node tools/phase_b_discharge_mode_syntax.js check
node tools/phase_b_discharge_mode_syntax_check.js --compile
```

The independent checker rejects ten mutations including mode loss, source/status promotion,
self-execution, return/program leakage, and axioms. The next residual is question composition.
