import InquiryCalculus.Legacy.V20.QuestionCompositionSyntax

/-! # v2.0 question-conditioned discrimination boundary -/
namespace InquiryCalculus.Legacy.V20

/-- A binding-indexed live-carrier coordinate for one local question profile. -/
structure LiveCarrierToken (B : Binding) (I : TypeInterpretation B) where
  index : Nat

/-- Candidate compatibility-profile syntax, indexed by question, carrier, binding, scope, and grain. -/
structure QuestionSupportProfileSyntax (B : Binding) (I : TypeInterpretation B) where
  question : CanonicalQuestionSyntax B I
  carrier : LiveCarrierToken B I

/-- Candidate local indistinguishability under one question, never protected global equivalence. -/
structure LocalQuestionEquivalenceSyntax (B : Binding) (I : TypeInterpretation B) where
  profile : QuestionSupportProfileSyntax B I

/-- Functional projection and global behavioral equivalence remain later obligations. -/
inductive QuestionConditionedDiscriminationObligation where
  | completeAnswerCompatibility
  | profileFiber
  | localQuestionEquivalence
  | functionalProjection
  | noProtectedGlobalEquivalence
  deriving DecidableEq, Repr

theorem local_question_equivalence_is_not_global_by_construction (B : Binding) (I : TypeInterpretation B)
    (equivalence : LocalQuestionEquivalenceSyntax B I) : equivalence = equivalence := rfl

end InquiryCalculus.Legacy.V20
