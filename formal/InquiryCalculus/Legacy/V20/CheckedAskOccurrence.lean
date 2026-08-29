import InquiryCalculus.Legacy.V20.ResidualQuestionGenerator

/-! # v2.0 checked-Ask-occurrence boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y z

/-- A first-order source configuration retains source program, environment, version, and provenance records. -/
structure SourceConfigSyntax (SourceProgram : Type u) (Environment : Type v) (BindingVersion : Type w)
    (CompilerVersion : Type x) (Provenance : Type y) (ResultType : Type z) where
  sourceProgram : SourceProgram
  environment : Environment
  bindingVersion : BindingVersion
  compilerVersion : CompilerVersion
  provenance : Provenance
  resultType : ResultType
  sourceProgramWellTyped : Prop
  checkedSourceVersions : Prop

/-- A checked Ask occurrence is reconstructed from a structural position in one source configuration. -/
structure CheckedAskOccurrenceSyntax (Position : Type u) (Question : Type v) (AnswerSlot : Type w)
    (Continuation : Type x) (Environment : Type y) (BindingVersion : Type z)
    (CompilerVersion : Type (max u v w x y z)) (Provenance : Type (max u v w x y z)) where
  structuralPosition : Position
  question : Question
  answerSlot : AnswerSlot
  continuation : Continuation
  environmentAtPosition : Environment
  bindingVersionAtPosition : BindingVersion
  compilerVersionAtPosition : CompilerVersion
  provenanceAtPosition : Provenance
  sourceAskSyntaxAtPosition : Prop
  rewalkedStructuralContext : Prop
  questionWellFormed : Prop
  continuationWellTypedUnderAnswerExtension : Prop
  captureSafe : Prop

/-- The source projections expose question and continuation from an already checked occurrence. -/
def askQuestion {Position : Type u} {Question : Type v} {AnswerSlot : Type w} {Continuation : Type x}
    {Environment : Type y} {BindingVersion : Type z} {CompilerVersion : Type (max u v w x y z)}
    {Provenance : Type (max u v w x y z)} :
    CheckedAskOccurrenceSyntax Position Question AnswerSlot Continuation Environment BindingVersion CompilerVersion Provenance → Question :=
  fun occurrence => occurrence.question

def askContinuation {Position : Type u} {Question : Type v} {AnswerSlot : Type w} {Continuation : Type x}
    {Environment : Type y} {BindingVersion : Type z} {CompilerVersion : Type (max u v w x y z)}
    {Provenance : Type (max u v w x y z)} :
    CheckedAskOccurrenceSyntax Position Question AnswerSlot Continuation Environment BindingVersion CompilerVersion Provenance → Continuation :=
  fun occurrence => occurrence.continuation

/-- Source obligations retained until replay, typing, normalization, and occurrence semantics are separately formalized. -/
inductive CheckedAskOccurrenceObligation where
  | firstOrderDependentSourceConfig
  | mutuallyApplicableEnvironmentBindingCompilerAndAcceptedProvenance
  | structuralPositionsAndSubterms
  | captureSafeEnvironmentAndStructuralProvenanceByReplay
  | sourceAskSyntaxAtPosition
  | rewalkingChecksEveryDisplayedField
  | copiedFieldsDoNotFormOccurrence
  | questionDependentCarrier
  | continuationDependentType
  | sameNormalizedQuestionMayHaveDifferentContinuations
  | noRawReturnEventOrSelectedAnswerCollapse
  | noHostClosureOrHiddenModelPolicy
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
