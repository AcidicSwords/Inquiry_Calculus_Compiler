import InquiryCalculus.Legacy.V20.FormulaGrammar

/-!
# v2.0 minimal logical basis boundary

The seven source records at `PRED-TEX-PROSE-21CAA73CE59DF7BF`,
`PRED-TEX-DISPLAY-DBFEBF370C7F29E8`, `PRED-TEX-PROSE-94E6381070E1D28D`,
`PRED-TEX-DISPLAY-22810950FC89D1EE`, `PRED-TEX-DISPLAY-4932841BBEAD0158`,
`PRED-TEX-DISPLAY-5CA3330D1E75CA7C`, and `PRED-TEX-PROSE-3B076C0050261AF5`
remain Ambiguous LegacyObligations. This file preserves candidate reference-dialect tokens and
syntactic derivation shapes only. It does not select successor primitives, establish classical
logic, make complement native to a binding, or identify logical negation with oriented negation.
-/

namespace InquiryCalculus.Legacy.V20

/-- The five source-named generators in the classical reference dialect candidate basis. -/
inductive ReferenceLogicalBasisToken where
  | truth
  | equality
  | conjunction
  | existential
  | logicalNot
  deriving DecidableEq, Repr

/-- The candidate reference basis has no claim to native availability in every binding. -/
def classicalReferenceBasis : List ReferenceLogicalBasisToken :=
  [.truth, .equality, .conjunction, .existential, .logicalNot]

/-- A declared boundary for a binding that may or may not natively supply complement. -/
structure NativeComplementBoundary (B : Binding) where
  available : Prop

/-- Candidate derived disjunction; its shape is not a semantic equivalence theorem. -/
def deriveOr {B : Binding} {I : TypeInterpretation B}
    (phi psi : CandidateFormulaSyntax B I) : CandidateFormulaSyntax B I :=
  .logicalNot (.and (.logicalNot phi) (.logicalNot psi))

/-- Candidate derived universal quantification; its shape is not a semantic equivalence theorem. -/
def deriveForall {B : Binding} {I : TypeInterpretation B}
    (x : FormulaVariable B I) (phi : CandidateFormulaSyntax B I) : CandidateFormulaSyntax B I :=
  .logicalNot (.exists x (.logicalNot phi))

/-- Candidate derived implication; its shape is not a semantic equivalence theorem. -/
def deriveImplies {B : Binding} {I : TypeInterpretation B}
    (phi psi : CandidateFormulaSyntax B I) : CandidateFormulaSyntax B I :=
  deriveOr (.logicalNot phi) psi

/-- Every source claim at this boundary remains an obligation rather than a logic selection. -/
inductive MinimalLogicalBasisObligation where
  | referenceDialectQualification
  | displayedBasis
  | basisSufficiency
  | derivedDisjunction
  | derivedUniversalQuantification
  | derivedImplication
  | nativeComplementLimitation
  deriving DecidableEq, Repr

theorem derived_disjunction_has_candidate_shape {B : Binding} {I : TypeInterpretation B}
    (phi psi : CandidateFormulaSyntax B I) :
    deriveOr phi psi = .logicalNot (.and (.logicalNot phi) (.logicalNot psi)) := rfl

end InquiryCalculus.Legacy.V20
