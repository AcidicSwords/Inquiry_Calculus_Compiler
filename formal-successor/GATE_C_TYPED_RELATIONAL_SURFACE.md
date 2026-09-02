# Gate C Typed Relational Surface

## Result

**Partial and binding-conditional.** The successor now has its own binding forms, contextual forms,
free regular formula syntax, structurally derived reindexing, and predicate denotation. Lean proves
that denotation commutes with reindexing. The protected predecessor surface is not yet regenerated:
the type/form bridge requires explicit binding data, and arbitrary predecessor relations still need
a typed representability and soundness witness.

## Current formal meaning

- `BindingForm` is the dependent pair of a binding `TyCode` and a value in `El(code)`. The type tag
  is retained.
- `ContextualForm(Γ)` is the dependent pair of an ambient type in context and a term of that type.
- `RegularFormula(Γ)` contains contextual binding atoms, equality, top, meet, existential hiding,
  and binding-declared logical extensions.
- Formula reindexing is derived by structural recursion. It is not a raw syntax constructor.
- Denotation maps formulas into the `RegPred` fibre. The theorem `denote_reindex` uses functorial
  predicate reindexing, equality naturality, top/meet preservation, Beck–Chevalley, and naturality
  of binding-declared logical extensions.

Thus syntax is not denotation, binding-local `TyCode/El` is not contextual `Ty/Tm`, and binding
atoms are not arbitrary host propositions.

## Breaker

The type-tag foil has two distinct codes with the same one-element value carrier. Its two forms are
distinct, while erasing the code maps both to the same value. Lean proves that this erasure is not
injective. A predecessor bridge must therefore carry an injective type-code map rather than only a
value conversion.

## Predecessor correspondence

`TypeInterpretationBridge` explicitly maps predecessor type codes and values into a successor
binding, and `mapForm` preserves the mapped type code. This is binding-conditional; it is not
manufactured from file ancestry or model agreement.

The predecessor `Relation` carrier admits arbitrary `Prop`-valued relations. The successor regular
language admits only binding atoms and regular composites. `RelationRepresentabilityBoundary`
therefore records representability as a supplied condition. `Representability.lean` now constructs
the dependent domain/codomain telescope and encoded assignments, and defines representation as
pointwise equivalence with satisfaction (entailment from top). Its invariant theorem supplies a
generic obstruction to representation, not yet a concrete separating model.

`equal_refl` derives assertion on the diagonal. Identity is represented under the separately supplied
`FaithfulEncoding.reflect` condition. Existential introduction follows from adjunction, while
`ExistentialReflection` supplies contextual witness extraction. Neither establishes composition
closure: a three-port formula construction and coverage of relevant contextual witnesses by encoded
predecessor mediators remain necessary. An extra contextual mediator can otherwise satisfy both
formulas without corresponding to any predecessor mediator.

The binding-extension route also requires an interpreted atom with the intended relation; adding an
atom name alone supplies no denotation. Arbitrary predecessor relations are not silently atomized,
and the `typed-relational-surface` protected target remains not yet regenerated.
