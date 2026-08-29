import InquiryCalculus.Legacy.V20.QuestionConditionedDiscrimination

/-! # v2.0 question refinement preorder boundary -/
namespace InquiryCalculus.Legacy.V20

/-- A local candidate precision relation between questions on a carrier. -/
structure PrecisionPreorderSyntax (B : Binding) (I : TypeInterpretation B) where
  coarser : CanonicalQuestionSyntax B I
  finer : CanonicalQuestionSyntax B I

/-- A local candidate joint refinement coordinate. -/
structure JointRefinementSyntax (B : Binding) (I : TypeInterpretation B) where
  left : CanonicalQuestionSyntax B I
  right : CanonicalQuestionSyntax B I

/-- A candidate active representation extension by one question coordinate. -/
structure ActiveRepresentationRefinementSyntax (B : Binding) (I : TypeInterpretation B) where
  current : LiveCarrierToken B I
  added : CanonicalQuestionSyntax B I

/-- Factorization, kernel, nonredundancy, and active representation semantics remain later obligations. -/
inductive QuestionRefinementObligation where
  | profileDetermination
  | functionalFactorization
  | kernelInclusion
  | jointKernelIntersection
  | nonredundantCoordinate
  | activeRepresentationExtension
  deriving DecidableEq, Repr

theorem question_refinement_is_local_syntax_only (B : Binding) (I : TypeInterpretation B)
    (precision : PrecisionPreorderSyntax B I) : precision = precision := rfl

end InquiryCalculus.Legacy.V20
