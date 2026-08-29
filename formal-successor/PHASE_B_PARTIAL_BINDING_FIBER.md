# Phase B partial binding and completion fiber boundary

This local Phase B pass reconstructs the two exact v2.0 definitions as typed candidate data.
`OpenPortSet` retains nonempty named open coordinates; `TypedPortAssignment` retains its
binding-indexed port type; `PartialBindingSyntax` separates supplied coordinates from open ones;
and `CompletionFiberSyntax` is a carrier rather than a satisfaction-defined set or total function.

Relation satisfaction, membership/validity, complete answer carriers, refinement, and canonical
question formation are explicit later obligations. Gate B remains pending.

```text
node tools/phase_b_partial_binding_fiber.js check
node tools/phase_b_partial_binding_fiber_check.js --compile
```

The independent checker rejects ten mutations: source loss, Gate B promotion, missing carriers or
obligations, raw strings, satisfaction collapse, semantic-question leakage, and axiomatic gaps.
The next residual is canonical question syntax.
