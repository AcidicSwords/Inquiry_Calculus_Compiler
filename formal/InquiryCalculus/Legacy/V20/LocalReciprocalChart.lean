import InquiryCalculus.Legacy.V20.NormalizeOpenRelation

/-! # v2.0 Stage 2 local reciprocal-chart boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- A selected or constructed local reciprocal chart retains every displayed field as typed data. -/
structure LocalReciprocalChartSyntax (Determination : Type u) (Presentation : Type u)
    (CandidateBoundary : Type u) (PositiveUseX : Type u) (PositiveUseY : Type u)
    (NegationUseX : Type u) (NegationUseY : Type u) (SemanticCoverage : Type u)
    (ExecutionCoverage : Type u) (Seed : Type u) (ReturnFiber : Type u) (Recovery : Type u)
    (Gamma : Type u) (TimeIndex : Type u) (Grain : Type u) (Horizon : Type u) where
  determination : Determination
  sourcePresentation : Presentation
  candidateBoundary : CandidateBoundary
  positiveUseX : PositiveUseX
  positiveUseY : PositiveUseY
  negationUseX : NegationUseX
  negationUseY : NegationUseY
  semanticCoverage : SemanticCoverage
  executionCoverage : ExecutionCoverage
  seed : Seed
  returnFiber : ReturnFiber
  recovery : Recovery
  gamma : Gamma
  timeIndex : TimeIndex
  grain : Grain
  horizon : Horizon
  constructionOrSelectionUnproved : Prop
  presentationIsVersionedSourceDetermination : Prop
  boundaryRetainsCandidateIncidenceAndStatus : Prop
  usesRetainTypedIdentitiesAndTaggedFrontiers : Prop
  fiberAndRecoveryRetainUseSpecificResults : Prop
  seedAndReciprocalFieldsMayRemainOpen : Prop
  multiAnswerMayHaveOverlappingChartFamily : Prop
  noGlobalBinaryPartitionInvention : Prop
  noReverseNegationInvention : Prop
  noExteriorFromProjection : Prop
  noCompletedSixfoldWithoutSemanticAnswerStructure : Prop

/-- Source obligations retained until chart construction, field semantics, and completeness are separately checked. -/
inductive LocalReciprocalChartObligation where
  | constructionOrSelectionUnproved
  | allSixteenDisplayedFieldsRetained
  | versionedSourceDeterminationPresentation
  | candidateBoundaryIncidenceAndStatus
  | typedUseIdentitiesAndTaggedFrontiers
  | semanticAndExecutionCoverageRemainDistinct
  | useSpecificReturnFibersAndRecovery
  | seedAndReciprocalFieldsMayRemainOpen
  | multiAnswerOverlappingChartFamily
  | noGlobalBinaryPartitionInvention
  | noReverseNegationInvention
  | noExteriorFromProjection
  | noCompletedSixfoldWithoutSemanticAnswerStructure
  | noChartConstructionAlgorithm
  | noSemanticCompletionOrAuthorityPromotion
  | noAnswerSelectionOrTotalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
