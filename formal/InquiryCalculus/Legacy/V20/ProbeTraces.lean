import InquiryCalculus.Legacy.V20.RecurrentProbes

/-! # v2.0 probe-trace boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The predecessor records ordered comparable trace and question coordinates without detecting or warranting a pattern. -/
structure ProbeTracesSyntax (ProbeContract : Type u) (Event : Type u) (OrderedTrace : Type u)
    (Persistence : Type u) (Change : Type u) (ChangePoint : Type u) (Recurrence : Type u)
    (Covariation : Type u) (Dissociation : Type u) (PathDependence : Type u)
    (BoundaryMovement : Type u) (ComparatorFailure : Type u) (Pattern : Type u) where
  probeContract : ProbeContract
  event : Event
  orderedComparableTrace : OrderedTrace
  persistence : Persistence
  change : Change
  changePoint : ChangePoint
  recurrence : Recurrence
  covariation : Covariation
  dissociation : Dissociation
  pathDependence : PathDependence
  boundaryMovement : BoundaryMovement
  comparatorFailure : ComparatorFailure
  perceivedPattern : Pattern
  orderedComparableTraceShape : Prop
  everyComparedPairSatisfiesContractOrExplicitBridgeUnproved : Prop
  traceIsOrdinaryRelationalOperand : Prop
  persistenceQuestionShape : Prop
  changeQuestionShape : Prop
  changePointQuestionShape : Prop
  recurrenceQuestionShape : Prop
  covariationQuestionShape : Prop
  dissociationQuestionShape : Prop
  pathDependenceQuestionShape : Prop
  boundaryMovementQuestionShape : Prop
  comparatorFailureQuestionShape : Prop
  salienceDoesNotWarrantPerceivedPatternUnproved : Prop
  noTraceConstruction : Prop
  noPairwiseComparabilityDecision : Prop
  noPatternDetection : Prop
  noTraceQuestionResolution : Prop
  noSalienceWarrant : Prop
  noSemanticAuthorityPromotion : Prop

/-- Source obligations retained until trace construction, comparison, question resolution, and warrant are separately checked. -/
inductive ProbeTracesObligation where
  | orderedComparableTraceShape
  | everyComparedPairSatisfiesContractOrExplicitBridgeUnproved
  | traceIsOrdinaryRelationalOperand
  | persistenceQuestionShape
  | changeQuestionShape
  | changePointQuestionShape
  | recurrenceQuestionShape
  | covariationQuestionShape
  | dissociationQuestionShape
  | pathDependenceQuestionShape
  | boundaryMovementQuestionShape
  | comparatorFailureQuestionShape
  | salienceDoesNotWarrantPerceivedPatternUnproved
  | noTraceConstruction
  | noPairwiseComparabilityDecision
  | noPatternDetection
  | noTraceQuestionResolution
  | noSalienceWarrant
  | noSemanticAuthorityPromotion
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
