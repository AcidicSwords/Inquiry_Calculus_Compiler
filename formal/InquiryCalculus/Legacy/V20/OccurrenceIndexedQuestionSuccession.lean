import InquiryCalculus.Legacy.V20.HeadQuestion

/-! # v2.0 occurrence-indexed-question-succession boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y

/-- QSucc projects the HeadQ of the same Ask continuation after whole supported-answer substitution. -/
def occurrenceIndexedQuestionSuccession {AskReference : Type u} {Question : Type v}
    {SupportedAnswer : Question → Type w} {SourceProgram : Type x}
    (askQuestion : AskReference → Question)
    (substitutedContinuation : (askReference : AskReference) → SupportedAnswer (askQuestion askReference) → SourceProgram)
    (headQuestion : SourceProgram → Question → Prop)
    (askReference : AskReference) (answer : SupportedAnswer (askQuestion askReference)) (nextQuestion : Question) : Prop :=
  headQuestion (substitutedContinuation askReference answer) nextQuestion

/-- The protected successor presentation retains the occurrence, whole answer, and pure reconstruction boundary. -/
structure QuestionStepPresentation (SourceConfiguration : Type u) (AskReference : Type v) (Question : Type w)
    (SupportedAnswer : Question → Type x) (askQuestion : AskReference → Question) where
  sourceConfiguration : SourceConfiguration
  askReference : AskReference
  wholeSupportedAnswer : SupportedAnswer (askQuestion askReference)
  sameAnswerSubstitution : Prop
  pureCanonicalReconstructionBeforeNextOpenEffect : Prop

/-- Source obligations retained until substitution, reconstruction, and successor-typing are separately formalized. -/
inductive OccurrenceIndexedQuestionSuccessionObligation where
  | exactAskOccurrence
  | wholeSupportedAnswerToExactQuestion
  | headQuestionOfSameSubstitutedContinuation
  | protectedSuccessorPresentation
  | sameAnswerSubstitution
  | pureCanonicalReconstructionBeforeNextOpenEffect
  | dependentOccurrenceAnswerQuestionIndexes
  | fixedVersions
  | noNewTransitionPrimitive
  | noEventRawReturnOrSelectedAnswerCollapse
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
