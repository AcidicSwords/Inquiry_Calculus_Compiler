# Phase B relational sections boundary

This pass reconstructs the two asymmetric residual views of one typed ternary relation. Fixing a
`Y` coordinate yields an `X ↝ C` relation; fixing an `X` coordinate yields a distinct `Y ↝ C`
relation. Neither is promoted to total function, satisfaction semantics, program, or successor law.

```text
node tools/phase_b_relational_sections.js check
node tools/phase_b_relational_sections_check.js --compile
```

The independent checker protects the typed relation and both fixed-coordinate incidences. The next
residual is solution fibers.
