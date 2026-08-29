# Phase B represented-form surface

This local Phase B pass reconstructs the explicit v2.0 typed represented-form carrier,
reification boundary, and partial operational interpretation against the checked binding/type
surface. It does not choose successor primitives, relation schemas, formulas, questions, grain
semantics, or program execution, and it does not pass Gate B.

`Form B I` is a dependent carrier: its value is realized only under its retained binding-indexed
admitted `TypeCode`. `ReificationBoundary` accepts a typed operator token and returns a form;
`OperationalInterpretationBoundary` requires an explicit definedness proof before it can return a
typed operator role. Neither boundary is asserted to be total or inverse.

The three nearby v2.0 claims concerning role closure, construction ancestry/recomposition, and
grain-relative opening remain `RepresentedFormObligation` cases. They are not definitions,
constructors, or theorems in this layer.

```text
node tools/phase_b_forms.js check
node tools/phase_b_forms_check.js --compile
```

The independent checker regenerates the seven exact source identities and rejects nine mutations:
source loss, obligation promotion, Gate B promotion, missing form carrier, erased type admission,
totalized interpretation, relation-layer leakage, raw string collapse, and axiomatic completion.
The next residual is typed relations.
