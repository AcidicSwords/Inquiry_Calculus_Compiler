# Phase B formula grammar boundary

This local Phase B pass preserves the v2.0 formula-grammar display and its adjacent prose as a
typed candidate syntax. All six selected source records remain `LegacyObligation` entries: five
are `Ambiguous` and the logical-negation separation prose is `Unproved`. This is not a successor
logic, a binding-independent grammar, a semantic denotation, or a proof of any formula law. Gate B
remains pending.

`CandidateFormulaSyntax` retains the displayed truth/falsity, relation application with a retained
argument list, equality, binary connectives, logical negation, and typed existential/universal
forms. The candidate syntax deliberately has no denotation. The logical connective `logicalNot`
does not create an oriented negation use, a section, a filling, coverage, or departure evidence;
all of those source claims remain obligations for later layers.

```text
node tools/phase_b_formula_grammar.js check
node tools/phase_b_formula_grammar_check.js --compile
```

The independent checker regenerates all six source identities and rejects thirteen mutations:
source loss/promotion, unproved-status erasure, Gate B promotion, candidate syntax/negation/
quantifier/argument loss, obligation loss, host-`Prop` collapse, string syntax, negation-use
leakage, and axiomatic completion. The next residual is the minimal logical basis.
