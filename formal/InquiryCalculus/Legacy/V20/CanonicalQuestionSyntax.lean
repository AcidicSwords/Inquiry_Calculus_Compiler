import InquiryCalculus.Legacy.V20.PartialBindingFiber

/-! # v2.0 canonical question syntax boundary -/
namespace InquiryCalculus.Legacy.V20

/-- A typed canonical question occurrence generated from a partial relation binding. -/
structure CanonicalQuestionSyntax (B : Binding) (I : TypeInterpretation B) where
  binding : PartialBindingSyntax B I

/-- Answer-carrier, completion, and operational claims remain outside syntax. -/
inductive CanonicalQuestionObligation where
  | answerCarrier
  | validCompletion
  | probeOrProgram
  deriving DecidableEq, Repr

theorem canonical_question_syntax_is_data_only (B : Binding) (I : TypeInterpretation B)
    (question : CanonicalQuestionSyntax B I) : question = question := rfl

end InquiryCalculus.Legacy.V20
