# Hom-wise quotient and conditional composition

This is a downstream realization of v2.0 lines 4061–4106, not a successor
primitive or a promotion of the predecessor source classifications. The source
signature supplies endpoint-indexed terms, observation contexts, consequences,
and a protected horizon for each hom. The quotient identifies exactly those
parallel terms with equal consequences throughout their hom's horizon.

## Source correspondence

The checker regenerates these eight references from the canonical TeX and the
independently checked predecessor classification. No second source manifest or
generated history is required. Ambiguous source prose/displays remain Ambiguous
even though the supplied typed interpretation supports conditional proofs.

| Source identity | Lines | Source classification | Typed correspondence |
| --- | --- | --- | --- |
| PRED-TEX-PROSE-BBCC9442AAAE4320 | 4063–4063 | Ambiguous | `HomQuotient S H A B` preserves the hom |
| PRED-TEX-DISPLAY-2B3AD19A08A4881B | 4064–4071 | Ambiguous | Quotient of `protectedSetoid S H A B` |
| PRED-TEX-PROSE-403D87CD771923C0 | 4073–4073 | Ambiguous | Introduces the following map |
| PRED-TEX-DISPLAY-4A6F7E465E68AE70 | 4074–4079 | Ambiguous | `quotientMap : Term A B → HomQuotient S H A B` |
| PRED-TEX-PROSE-C359B4BAAFC28DB3 | 4080–4080 | Ambiguous | `quotientMapSound`, `quotientMapExact`, `quotientMapEqIff` |
| PRED-TEX-DECL-2DCEA31A4E4CED7D | 4082–4097 | FormalDefinition | `Congruent S H compose` |
| PRED-TEX-PROSE-04C6F935934FDBCF | 4099–4099 | Ambiguous | `descendedComposition` requires congruence |
| PRED-TEX-DISPLAY-D96A01F40D99D46A | 4100–4106 | Ambiguous | `descendedCompositionOnRepresentatives` |

## Exact scope and reciprocal boundary

Supply a total operation `Term B C → Term A B → Term A C` on one composable
triple. This is a particular admitted regime, not an assertion that every
calculus composition or continuation is total. `Congruent` requires both input
equivalences to imply output equivalence, at the respective three horizons.

`descentIffCongruence` proves both directions: such a quotient operation
commuting with the quotient maps exists if and only if the supplied operation
is congruent. The forward construction and reverse necessity proof are separate.
No algorithm deciding congruence, choice of representatives, or general context
executor is introduced. No associativity or identity law is assumed or proved.

The contracted countermodel has three terms: ordinary, hidden, visible.
Observation merges ordinary and hidden; composition with the same left term
sends them to ordinary and visible. Its carrier is closed, its representatives
are equivalent, and its composites are separated. `notCongruent` and `noDescent`
reject unconditional descent. The independent finite probe counts all ten
breaking quadruples. Removing the protected observation erases the breaker;
factoring the operation through the observation restores congruence. These
ablations localize this failure, not a global minimal-carrier theorem.

## Verification and limits

Run `node tools/hom_wise_quotient_check.js --compile`. Without `--compile` the
tool reports only source/static/finite-data checks, never a Lean proof pass.
The independent contracts check full formulas, endpoint/horizon typing,
representative computation, both descent directions, and the finite model.
Seven rejected contracts and four source ablations challenge these boundaries;
eight source mutations challenge identity, classification, range and provenance.
Every theorem has an exact dependency audit. Quotient proofs use the declared
standard `Quot.sound` and/or `propext` foundations; none uses `Classical.choice`.
The three direct countermodel proofs use no axioms. Full-library kernel recheck
is separate from successful elaboration.

F: identifying representatives whose composites are protected-distinct.
C: congruence of the supplied operation at the three hom horizons.
Omega: typed parallel terms and a supplied total composable triple.
M: supplied observation functions and operation; not Rust or recurrent execution.
P: finite countermodel, complete finite quadruple search and ablations.
V: independent Lean statement/type probes and kernel recheck.
E: exact source regeneration and scoped proof/dependency returns.
U: changed horizons, typing, composition regime, source interpretation or proof
dependencies reopen the affected claim.

Partial relational regimes, recurrent operator descent, full harness acceptance,
and Formal Gates B–N remain separate obligations. A present-output quotient is
not thereby executable retained state. Rust semantics remain frozen before Gate F.
