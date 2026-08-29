import InquiryCalculus.Legacy.V20.CheckedAskOccurrence

/-! # v2.0 head-question boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x

/-- A normalization result distinguishes a reached checked question from return-before-question. -/
inductive HeadQuestionResult (Question : Type u) where
  | reachesQuestion : Question → HeadQuestionResult Question
  | returnsBeforeQuestion : HeadQuestionResult Question

/-- HeadQ is the partial relation induced by fixed deterministic pure normalization. -/
def headQuestion {SourceProgram : Type u} {Question : Type v}
    (pureNormalization : SourceProgram → HeadQuestionResult Question)
    (sourceProgram : SourceProgram) (question : Question) : Prop :=
  pureNormalization sourceProgram = .reachesQuestion question

/-- The head-question boundary retains its source configuration and normalization constraints. -/
structure HeadQuestionSyntax (SourceProgram : Type u) (Question : Type v) (NormalizationVersion : Type w)
    (SourceConfiguration : Type x) where
  sourceConfiguration : SourceConfiguration
  sourceProgram : SourceProgram
  question : Question
  fixedNormalizationVersion : NormalizationVersion
  pureNormalization : Prop
  deterministicNormalization : Prop
  captureSafeAnswerExtension : Prop
  checkedSemanticQuestion : Prop

/-- Source obligations retained until normalization and source-typing semantics are separately formalized. -/
inductive HeadQuestionObligation where
  | pureNormalizationFirstReachesCheckedSemanticQuestion
  | returnBeforeAnotherQuestionIsEmpty
  | partialFunctionalRelation
  | fixedDeterministicNormalizationVersion
  | restrictedToSourceConfigurationPrograms
  | restrictedToCaptureSafeAnswerExtensions
  | sourceConfigurationInference
  | noAnswerSelection
  | noExecutionOrSolverAuthority
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
