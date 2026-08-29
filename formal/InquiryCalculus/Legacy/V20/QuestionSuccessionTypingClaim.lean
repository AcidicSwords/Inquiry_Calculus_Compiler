import InquiryCalculus.Legacy.V20.OccurrenceIndexedQuestionSuccession

/-! # v2.0 question-succession-typing claim boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y z

/-- The typed premises and intended conclusion of the source question-succession claim. -/
structure QuestionSuccessionTypingClaimSyntax (AskReference : Type u) (Question : Type v)
    (SupportedAnswer : Question → Type w) (Environment : Type x) (askQuestion : AskReference → Question)
    (nextQuestion : Question) where
  checkedAskOccurrence : AskReference
  wholeSupportedAnswer : SupportedAnswer (askQuestion checkedAskOccurrence)
  occurrenceIndexedSuccession : Prop
  lawfulFirstOrderEnvironmentAfterCaptureSafeSubstitution : Environment
  nextQuestionWellTypedInLawfulEnvironment : Prop

/-- The source proof requires these independently formalized preservation routes. -/
inductive QuestionSuccessionTypingClaimObligation where
  | checkedAskOccurrencePremise
  | wholeSupportedAnswerToExactQuestionPremise
  | occurrenceIndexedSuccessionPremise
  | askRuleTypesSubstitutedContinuation
  | explicitContinuationEnvironment
  | captureSafeSubstitutionPreservesTyping
  | pureNormalizationPreservesSourceProgramTyping
  | firstOpenQuestionTypedInLawfulEnvironment
  | noAdditionalTransitionOrTypingRule
  | sourceClaimUnprovedUntilDependenciesChecked
  | noAxiomOrPlaceholderProof
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
