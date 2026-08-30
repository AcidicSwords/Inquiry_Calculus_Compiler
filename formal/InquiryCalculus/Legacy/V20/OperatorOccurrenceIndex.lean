import InquiryCalculus.Legacy.V20.StateWorkingPresentation

/-! # v2.0 operator-occurrence-index boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The predecessor records a compact occurrence lookup over existing history without copying snapshots. -/
structure OperatorOccurrenceIndexSyntax (StateRef : Type u) (OperatorRef : Type u)
    (ReturnRef : Type u) (BoundaryRef : Type u) (EventRef : Type u) (Operator : Type u)
    (History : Type u) where
  preStateRef : StateRef
  operatorRef : OperatorRef
  returnRef : ReturnRef
  postStateRef : StateRef
  boundaryRef : BoundaryRef
  eventRef : EventRef
  operator : Operator
  history : History
  compactOperatorOccurrenceTupleShape : Prop
  operatorIndexedEventReferenceSetShape : Prop
  indexIsLookupOverExistingHistoryUnproved : Prop
  indexDoesNotDuplicateStateSnapshotsUnproved : Prop
  noIndexConstruction : Prop
  noHistoryLookupExecution : Prop
  noSnapshotDuplication : Prop
  noOccurrenceEvaluation : Prop
  noSemanticAuthorityPromotion : Prop

/-- Source obligations retained until indexing and lookup behavior are separately checked. -/
inductive OperatorOccurrenceIndexObligation where
  | compactOperatorOccurrenceTupleShape
  | operatorIndexedEventReferenceSetShape
  | indexIsLookupOverExistingHistoryUnproved
  | indexDoesNotDuplicateStateSnapshotsUnproved
  | noIndexConstruction
  | noHistoryLookupExecution
  | noSnapshotDuplication
  | noOccurrenceEvaluation
  | noSemanticAuthorityPromotion
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
