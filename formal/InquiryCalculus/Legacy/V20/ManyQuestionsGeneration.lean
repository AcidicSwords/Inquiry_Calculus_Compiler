import InquiryCalculus.Legacy.V20.PropositionNotWarrant

/-! # v2.0 one relation generates many questions boundary -/
namespace InquiryCalculus.Legacy.V20

/-- An opening position is a typed coordinate of one relation occurrence. -/
structure QuestionOpening (B : Binding) (I : TypeInterpretation B) where
  question : CanonicalQuestionSyntax B I
  port : PortName

/-- Candidate variation over one relation occurrence; no factor-question primitive is introduced. -/
structure ManyQuestionsGenerationSyntax (B : Binding) (I : TypeInterpretation B) where
  openings : List (QuestionOpening B I)

/-- Concrete compositions, answer semantics, and programs remain outside the grammar. -/
inductive ManyQuestionsGenerationObligation where
  | sameRelationVariation
  | noIndependentFactorPrimitive
  | semanticGeneration
  | probeOrProgram
  deriving DecidableEq, Repr

theorem many_questions_generation_is_data_only (B : Binding) (I : TypeInterpretation B)
    (generation : ManyQuestionsGenerationSyntax B I) : generation = generation := rfl

end InquiryCalculus.Legacy.V20
