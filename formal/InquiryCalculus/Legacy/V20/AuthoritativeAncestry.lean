import InquiryCalculus.Legacy.V20.ThreeDistinctOrders

/-! # v2.0 authoritative-ancestry and projection boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The predecessor records a single provenance structure and non-competing history projections. -/
structure AuthoritativeAncestrySyntax (AcceptedRoots : Type u) (EventHistory : Type u)
    (RevisionHistory : Type u) (Event : Type u) (Revision : Type u) (ImmutableReference : Type u)
    (DerivedView : Type u) (Interpretation : Type u) where
  acceptedRoots : AcceptedRoots
  eventHistory : EventHistory
  acceptedRevisionHistory : RevisionHistory
  event : Event
  revision : Revision
  immutableTypedReference : ImmutableReference
  derivedView : DerivedView
  interpretation : Interpretation
  authoritativeAncestryShape : Prop
  eventHistoryAppendOnlyShape : Prop
  acceptedRevisionHistoryShape : Prop
  eventAndRevisionProjectionsHaveIndependentEdgeSemanticsUnproved : Prop
  derivedViewsCanPointIntoButCannotAppendActualityOrAcceptanceUnproved : Prop
  interpretationMayReviseWithoutRewritingExternalEventUnproved : Prop
  noAncestryConstruction : Prop
  noActualityAppend : Prop
  noAcceptanceDecision : Prop
  noExternalEventRewrite : Prop
  noEdgeSemanticsEvaluator : Prop
  noSemanticAuthorityPromotion : Prop

/-- Source obligations retained until ancestry, append, revision, and edge behavior are separately checked. -/
inductive AuthoritativeAncestryObligation where
  | authoritativeAncestryShape
  | eventHistoryAppendOnlyShape
  | acceptedRevisionHistoryShape
  | independentEventAndRevisionEdgeSemanticsUnproved
  | derivedViewsCannotAppendActualityOrAcceptanceUnproved
  | interpretationMayReviseWithoutExternalEventRewriteUnproved
  | noAncestryConstruction
  | noActualityAppend
  | noAcceptanceDecision
  | noExternalEventRewrite
  | noEdgeSemanticsEvaluator
  | noSemanticAuthorityPromotion
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
