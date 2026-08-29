# Phase B binding and reference type surface

This local Phase B pass elaborates the three explicit v2.0 binding definitions and the fifteen explicit reference-type alternatives. It does not decide successor primitives or pass Gate B.

`Binding` carries the ten source-named binding slots as typed parameters. `TypeCode B` is a
binding-indexed syntax, distinct from Lean's `Type`; `TypeInterpretation B` realizes only codes
with an explicit admissibility proof. This preserves the source qualification that a native binding
need not realize every reference-language structure.

The introduction prose, grammar display interpretation, and native-binding qualification remain
the three exact `ReferenceTypeGrammarObligation` cases. They are not constructors or theorems.

```text
node tools/phase_b_binding_type.js check
node tools/phase_b_binding_type_check.js --compile
```

The independent checker regenerates all seven source identities and rejects 14 mutations including
source loss or duplication, obligation promotion, binding-slot or constructor removal, host-Type or
String collapse, missing partial admissibility, axioms/proof gaps, detached modules, and Gate B
self-promotion. The next residual is represented forms.
