# Phase B root frontier

`FORMAL-B-ROOT-FRONTIER-001` binds the two `Ambiguous` and one `Unproved` `LegacyObligation`
records at v2.0 lines 5163–5183. The root frontier is reconstructed by composition, not as another
schema: admitted root production is conjoined with the already checked executable-candidate
relation and projected through the already checked required-safe nondominant relation.

The finite discriminator field contains productive optional, required-but-ordinarily-dominated,
dominated optional, non-root, unadmitted, nonformable, inapplicable, nonexecutable, and idle
occurrences. Exactly the productive, required, and dominated-optional candidates are eligible; the
dominated optional candidate is removed while the dominated required bearer survives. Missing
preorder data preserves the eligible field unchanged.

The finite discriminator is Boolean so it tests the composed boundary without duplicating the
older Prop-valued availability and nondominance proof records. The semantic definition continues
to reference those existing relations directly. It adds no root wheel, selection, scheduler,
execution, or closure authority.

`node tools/root_frontier_check.js --compile` checks all three exact source anchors, rejects fifteen
structural ablations, and audits fourteen axiom-free proofs. The universal source claim and every
construction, selection, execution, successor-promotion, and Gate-B claim remain open.
