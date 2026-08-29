import InquiryCalculus.Legacy.V20.AnswerCarrierValidity

/-! # v2.0 fully bound proposition is not warrant boundary -/
namespace InquiryCalculus.Legacy.V20

/-- A fully bound relation claim is a typed candidate claim, not standing. -/
structure FullyBoundPropositionSyntax (B : Binding) (I : TypeInterpretation B) where
  completion : ValidCompletionSyntax B I

/-- Standing and operational authority remain outside the claim carrier. -/
inductive PropositionNotWarrantObligation where
  | notWarrantedFact
  | actualReturn
  | probeOrProgram
  deriving DecidableEq, Repr

theorem fully_bound_proposition_is_not_standing (B : Binding) (I : TypeInterpretation B)
    (claim : FullyBoundPropositionSyntax B I) : claim = claim := rfl

end InquiryCalculus.Legacy.V20
