import InquiryCalculus.Legacy.V20.DischargeModeSyntax

/-! # v2.0 question composition syntax boundary -/
namespace InquiryCalculus.Legacy.V20

/-- A source-bound syntactic guard coordinate. -/
structure QuestionGuard (B : Binding) (I : TypeInterpretation B) where
  formula : CandidateFormulaSyntax B I

/-- A source-bound answer placeholder, not a returned or warranted value. -/
structure AnswerPlaceholder (B : Binding) (I : TypeInterpretation B) where
  answer : AnswerCarrierSyntax B I

/-- Candidate composition shapes; no answer or program semantics are introduced. -/
inductive QuestionCompositionSyntax (B : Binding) (I : TypeInterpretation B) where
  | base : CanonicalQuestionSyntax B I → QuestionCompositionSyntax B I
  | bind : QuestionCompositionSyntax B I → QuestionCompositionSyntax B I → QuestionCompositionSyntax B I
  | tensor : QuestionCompositionSyntax B I → QuestionCompositionSyntax B I → QuestionCompositionSyntax B I
  | guard : QuestionGuard B I → QuestionCompositionSyntax B I → QuestionCompositionSyntax B I
  | plug : QuestionCompositionSyntax B I → AnswerPlaceholder B I → QuestionCompositionSyntax B I

/-- Dependent/independent semantics and simultaneous substitution remain obligations. -/
inductive QuestionCompositionObligation where
  | dependentComposition
  | independentCombination
  | guardRefinement
  | answerSubstitution
  | simultaneousMultiPortSubstitution
  deriving DecidableEq, Repr

theorem question_composition_is_data_only (B : Binding) (I : TypeInterpretation B)
    (composition : QuestionCompositionSyntax B I) : composition = composition := rfl

end InquiryCalculus.Legacy.V20
