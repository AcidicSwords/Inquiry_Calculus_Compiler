import InquiryCalculus.Legacy.V20.LocalReciprocalChart

/-! # v2.0 Stage 3 active-view boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- Stage 3 retains a view expression and its inclusion, reserve, occlusion, and reopening conditions. -/
structure ActiveViewSyntax (State : Type u) (View : Type u) (Question : Type u) (Grain : Type u)
    (Inclusion : Type u) (Reserve : Type u) (Occlusion : Type u) (Licence : Type u)
    (ReopeningPredicate : Type u) where
  state : State
  view : View
  question : Question
  grain : Grain
  inclusion : Inclusion
  reachableReserve : Reserve
  occlusion : Occlusion
  licence : Licence
  reopeningPredicate : ReopeningPredicate
  viewExpressionShape : Prop
  includesLiveQuestionOrProtectedFutureStructure : Prop
  preservesRelevanceUndeterminedAsReachableReserveUnproved : Prop
  occludesOnlyWithApplicableExactOrApproximateLicence : Prop
  recordsEveryReopeningPredicateUsedByOcclusion : Prop
  noViewBuilder : Prop
  noRelevanceDecisionProcedure : Prop
  noLicenceDecisionProcedure : Prop

/-- Source obligations retained until view construction and licence behavior are separately checked. -/
inductive ActiveViewObligation where
  | viewExpressionShape
  | includeLiveQuestionOrProtectedFutureStructure
  | preserveRelevanceUndeterminedReachableReserveUnproved
  | occludeOnlyWithApplicableExactOrApproximateLicence
  | recordEveryReopeningPredicateUsedByOcclusion
  | noAutomaticViewBuilder
  | noAutomaticRelevanceDecision
  | noReachableReserveErasure
  | noUnlicensedOcclusion
  | noOmittedReopeningPredicate
  | noLicenceDecisionProcedure
  | noAnswerSelectionOrTotalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
