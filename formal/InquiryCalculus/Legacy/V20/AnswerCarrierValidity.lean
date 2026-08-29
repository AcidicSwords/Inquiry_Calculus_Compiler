import InquiryCalculus.Legacy.V20.CanonicalQuestionSyntax

/-! # v2.0 answer-carrier and valid-completion boundary -/
namespace InquiryCalculus.Legacy.V20

/-- The typed shape of a complete candidate answer for one canonical question. -/
structure AnswerCarrierSyntax (B : Binding) (I : TypeInterpretation B) where
  question : CanonicalQuestionSyntax B I
  coordinates : List (TypedPortAssignment B I)

/-- A candidate valid-completion occurrence, not an actual return or a warrant. -/
structure ValidCompletionSyntax (B : Binding) (I : TypeInterpretation B) where
  answer : AnswerCarrierSyntax B I

/-- Actual return, warrant, probe, and program claims remain outside this boundary. -/
inductive AnswerValidityObligation where
  | actualReturn
  | validityWarrant
  | probeOrProgram
  deriving DecidableEq, Repr

theorem answer_validity_syntax_is_data_only (B : Binding) (I : TypeInterpretation B)
    (completion : ValidCompletionSyntax B I) : completion = completion := rfl

end InquiryCalculus.Legacy.V20
