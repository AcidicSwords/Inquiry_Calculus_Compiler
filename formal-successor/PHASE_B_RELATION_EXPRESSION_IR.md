# Phase B data-only relation-expression IR boundary

This local Phase B pass preserves the v2.0 data-only relation-expression display as typed candidate
syntax. All three selected source records remain `Ambiguous` `LegacyObligation` entries. The layer
does not define denotation, relational join/substitution/hiding/renaming/guard semantics, or a
concrete semantic question; Gate B remains pending.

`RelationExpressionIR` retains only the seven displayed constructor shapes: relation, bind, join,
expose, hide, rename, and guard. Binding tokens, port selections, renamings, and guards are syntax
coordinates. In particular, an exposed port selection does not by itself create a semantic question.

```text
node tools/phase_b_relation_expression_ir.js check
node tools/phase_b_relation_expression_ir_check.js --compile
```

The independent checker regenerates all three source identities and rejects thirteen mutations:
source loss/promotion, ambiguity erasure, Gate B promotion, grammar/constructor/obligation loss,
denotation collapse, semantic-question leakage, string syntax, and axiomatic completion. The next
residual is relation schemas and named ports.
