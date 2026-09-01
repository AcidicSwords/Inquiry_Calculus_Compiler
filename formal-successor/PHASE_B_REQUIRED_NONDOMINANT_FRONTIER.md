# Phase B required-protected nondominant frontier

`FORMAL-B-REQUIRED-NONDOMINANT-FRONTIER-001` binds four exact `LegacyObligation` records at v2.0
lines 4981–5001, retaining three `Ambiguous` and one `Unproved` classification. The finite model
retains the required set, ordinary strict dominance, their union frontier, the no-preorder fallback,
and a typed same-dependency removal licence.

The decisive contrast makes a required occurrence ordinarily dominated by an alpha substitute while
the union still retains it. It requires two retained substitutions—one for each dependency—before
the required occurrence can be removed. The required occurrence nonetheless dominates an optional
occurrence, which is excluded from the union; the special protection belongs only to the required
bearer. With no supplied preorder, the complete field remains.

This does not define resource costs, a universal preorder, executable selection, a scheduler, or
successor semantics. `node tools/required_nondominant_frontier_check.js --compile` verifies source
binding, rejects eight targeted ablations, and audits eight axiom-free proofs.
