# Phase B ambient boundary and predecessor spine

This ratchet closes one local Phase B relation: which forms Lean supplies as metalanguage before
the v2.0 calculus is elaborated. It does not close the predecessor core and does not pass Formal
Gate B.

`PHASE_B_PREDECESSOR_SPINE_SCHEMA.json` selects 41 exact reviewed TeX identities and orders them
into 13 elaboration layers. `PHASE_B_PREDECESSOR_SPINE.json` is the exactly regenerated projection.
The first layer is checked; the other 12 remain open and select
`FORMAL-B-BINDING-TYPE-SURFACE` as the next residual.

## Retained boundary

Lean supplies `Type`, `Prop`, equality, ordinary and dependent functions, dependent pairs, and
universal and existential quantification. `InquiryCalculus.Meta.Ambient` checks that these forms
are available without defining calculus-specific copies. Conversely, v2.0 binding, type codes,
represented forms, relations, relation-expression syntax, questions, fibers, programs, runtime
syntax, occurrences, protection, support, and standing remain predecessor structures that must be
elaborated rather than treated as host primitives.

The v2.0 sentence at `PRED-TEX-PROSE-983F2B30F7C1C1D2` describes its metalanguage as classical
set-theoretic. The Lean boundary does not silently install global classical reasoning. The exact
uses and required strength of that predecessor statement remain an explicit obligation.

## Machine checks

```text
node tools/phase_b_predecessor_spine.js check
node tools/phase_b_predecessor_spine_check.js
formal: lake build --wfail
```

The independent checker reconstructs every selected source excerpt from the canonical TeX,
requires an acyclic backward dependency order, compiles the ambient module, and rejects 17
mutations. Its breakers include layer/source omission, duplicate or foreign ancestry, forward
dependency, movement across the ambient/calculus boundary, silent classicality, Rust meaning,
custom declarations or proof gaps in the ambient module, missing theorems/modules, detached
digests, and Gate B self-promotion.

This establishes an elaboration boundary and dependency topology only. It neither proves the 41
predecessor claims nor chooses successor primitives. Formal Gate B remains `PENDING` until the
complete predecessor core compiles without proof gaps and every unresolved claim is represented
as an explicit obligation.
