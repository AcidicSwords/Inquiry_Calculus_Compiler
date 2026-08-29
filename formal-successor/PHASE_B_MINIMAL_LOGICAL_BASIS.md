# Phase B minimal logical basis boundary

This local Phase B pass preserves the v2.0 classical reference-dialect basis and its three
displayed derivations as candidate syntax shapes. All seven selected records remain `Ambiguous`
`LegacyObligation` entries. This is not a choice of successor primitives, a proof of classical
logic, a guarantee that every binding natively provides complement, or a bridge to oriented
negation. Gate B remains pending.

`ReferenceLogicalBasisToken` records the five named reference tokens. `deriveOr`, `deriveForall`,
and `deriveImplies` preserve only the displayed candidate formula shapes. `NativeComplementBoundary`
makes native complement explicit and binding-dependent. A derivation is not a semantic equivalence theorem, and logical negation remains separate from later negation-use evidence.

```text
node tools/phase_b_minimal_logical_basis.js check
node tools/phase_b_minimal_logical_basis_check.js --compile
```

The independent checker regenerates all seven source identities and rejects fourteen mutations:
source loss/promotion, ambiguity erasure, Gate B promotion, basis/member/boundary loss, each
derivation loss, obligation loss, global-complement collapse, oriented-negation leakage, and
axiomatic completion. The next residual is data-only relation-expression IR.
