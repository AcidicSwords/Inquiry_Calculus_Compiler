# Phase B — relational operator descent

## Boundary

This pass recovers the v2.0 continuation-descent definition as a typed relational
square. It distinguishes the source's existential next representation from descent
relative to a supplied protected next representation. It compares whole relational
images and introduces no executor, selected-return semantics, totality assumption,
Rust behavior, successor primitive, or Gate-B promotion.

## Source correspondence

| Source identity | Lines | Classification | Retained role |
|---|---:|---|---|
| PRED-TEX-PROSE-FB082A57E7A83268 | 4110–4110 | Unproved | Present-output quotient is not automatically recurrent state. |
| PRED-TEX-PROSE-403D87CD771923C0-02 | 4112–4112 | Ambiguous | Introduces the representation-quotient frame. |
| PRED-TEX-DISPLAY-D85F60F3BD86A29F | 4113–4115 | Ambiguous | Supplies the typed current representation map `q`. |
| PRED-TEX-PROSE-DA48910D06B9AE1A | 4116–4116 | Ambiguous | Introduces the continuation frame. |
| PRED-TEX-DISPLAY-E062943987A89C30 | 4117–4119 | Ambiguous | Supplies the typed continuation relation `a`. |
| PRED-TEX-PROSE-1BD1A5A82DED8417 | 4120–4120 | Ambiguous | Calls the continuation protected without defining that protection here. |
| PRED-TEX-DECL-60B874576BE3C6DC | 4122–4139 | FormalDefinition | Existential next carrier/map and descended relation with the relationally commuting square. |
| PRED-TEX-PROSE-51B5522AD4A8222A | 4141–4141 | Unproved | Requires descent for each protected continuation in executable retained state. |
| PRED-TEX-DECL-0C77C2F920F7005A | 4143–4151 | Unproved | Current-output sufficiency does not imply continuation sufficiency. |

The canonical source bytes and all classifications remain unchanged.

## Checked relational boundary

`formal/InquiryCalculus/Legacy/V20/OperatorDescent.lean` defines:

- typed relations, exact function graphs, serial composition, and extensional
  relational equality;
- `ContinuationDescentWitness`, preserving the source existential shape;
- `DescendsTo`, relative to a supplied next representation;
- `FiberStable`, requiring equal whole represented continuation images for every
  pair identified by the current representation;
- a canonical descended relation and the constructive equivalence
  `DescendsTo q a q' ↔ FiberStable q a q'`.

The two-state countermodel merges `left` and `right` at the present quotient while
the continuation and identity next representation expose `false` and `true`.
Therefore the protected fixed next representation does not descend. A constant next
representation does descend and satisfies the bare existential source shape. That
contrast preserves, rather than silently repairs, the source's missing protection
condition: an unconstrained existential `q'` can erase the future distinction.

## Independent evidence and limits

`node tools/operator_descent_check.js --compile` regenerates all nine source
identities, rejects source/classification mutations, independently checks the
pointwise whole-image formulas, enumerates every finite descended relation in the
countermodel, rejects selected-return/orientation/typing foils and source ablations,
and audits every named theorem against its exact standard `propext`/`Quot.sound`
dependency boundary; no theorem uses `Classical.choice` or a custom axiom.

This proves a conditional mathematical interface only. It does not prove the three
Unproved source claims, supply the meaning of “protected,” decide descent, execute a
continuation, authorize an engineering fold, change Rust, select successor
primitives, or pass Gate B. The next source boundary must determine regenerative
sufficiency without treating the constant-next-representation witness as protected.
