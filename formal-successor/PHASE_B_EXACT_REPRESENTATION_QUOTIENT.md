# Phase B exact representation quotient

`FORMAL-B-EXACT-REPRESENTATION-QUOTIENT-001` binds eight exact `LegacyObligation` records
at v2.0 lines 4638–4664, all retained as `Ambiguous`: three quotient records, four
consequence-sufficiency records, and one continuation-sufficiency record. It defines a proposed
quotient map and the forward consequence-sufficiency law. The coarsest biconditional and
continuation descent remain separate obligations; the latter is source-pinned here solely to
preserve the section boundary, not elaborated or discharged.

The finite contrast proves an exact map sufficient for protected equivalence, rejects a
constant overcoarse map, and shows a finer map can be sufficient without being coarsest.
It does not infer universal equivalence from tests, quotient existence, coarseness,
continuation descent, execution, Rust semantics, successor promotion, or Gate B passage.

`node tools/exact_representation_quotient_check.js --compile` verifies the exact source/classification
boundary, rejects six targeted source ablations, and audits six axiom-free proofs.
