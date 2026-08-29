import InquiryCalculus.Legacy.V20.InquirySourceGrammar

/-! # v2.0 residual-question-generator boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y z

/-- The data retained by a residual-sensitive next-question relation. -/
structure ResidualQuestionInput (State : Type u) (AskReference : Type v) (Question : Type w)
    (SupportedAnswer : Question → Type x) (Residual : Type y) (Continuation : Type z)
    (askQuestion : AskReference → Question) where
  state : State
  checkedAskReference : AskReference
  wholeSupportedAnswer : SupportedAnswer (askQuestion checkedAskReference)
  residual : Residual
  retainedContinuation : Continuation
  retainedEnvironment : Prop

/-- A canonical next question either distinguishes the residual or is explicitly required. -/
def residualQuestion (State : Type u) (AskReference : Type v) (Question : Type w)
    (SupportedAnswer : Question → Type x) (Residual : Type y) (Continuation : Type z)
    (askQuestion : AskReference → Question) (distinguishesResidual : Residual → Question → Prop)
    (explicitlyRequired : Question → Prop)
    (input : ResidualQuestionInput State AskReference Question SupportedAnswer Residual Continuation askQuestion)
    (nextQuestion : Question) : Prop :=
  distinguishesResidual input.residual nextQuestion ∨ explicitlyRequired nextQuestion

/-- The residual field preserves the source classes that can select consequential next questions. -/
inductive ResidualQuestionField where
  | survivingFillerClasses
  | openDependencies
  | brokenExclusions
  | reopenedFolds
  | representationDefects
  | otherConsequentialUnresolvedStructure
  deriving DecidableEq, Repr

/-- Source obligations retained until productivity, standing-program, and occurrence semantics are separately formalized. -/
inductive ResidualQuestionGeneratorObligation where
  | higherOrderContinuationRepresentation
  | wholeSupportedAnswerAtAskQuestion
  | checkedSourceAskOccurrence
  | retainedContinuationAndEnvironment
  | residualField
  | survivingFillerClassesOpenDependenciesBrokenExclusionsReopenedFoldsRepresentationDefects
  | discriminativePossibleSupportedReturns
  | explicitlyRequiredStandingProgramDischarge
  | noPartialAnswerInput
  | noStaticTextualChecklistOnly
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
