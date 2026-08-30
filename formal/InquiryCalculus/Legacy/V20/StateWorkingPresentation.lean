import InquiryCalculus.Legacy.V20.DerivedPairedActuality

/-! # v2.0 state and working-presentation boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The predecessor records accepted, working, and active presentation coordinates without folding or mutating state. -/
structure StateWorkingPresentationSyntax (StateSnapshot : Type u) (AcceptedPresentation : Type u)
    (WorkingPresentation : Type u) (AcceptedRevisionHistory : Type u) (EventHistory : Type u)
    (PresentationEnvironment : Type u) (Question : Type u) (Grain : Type u) (ActiveView : Type u)
    (HiddenMemory : Type u) where
  stateSnapshot : StateSnapshot
  acceptedPresentation : AcceptedPresentation
  workingPresentation : WorkingPresentation
  acceptedRevisionHistory : AcceptedRevisionHistory
  eventHistory : EventHistory
  presentationEnvironment : PresentationEnvironment
  question : Question
  grain : Grain
  activeView : ActiveView
  hiddenMemory : HiddenMemory
  noRequiredGlobalStateCompression : Prop
  acceptedPresentationVersionedFoldShape : Prop
  workingPresentationShape : Prop
  activeViewDerivedFromRetainedStateAndHistoryUnproved : Prop
  noHiddenMutableMemoryBecomesAuthoritativeUnproved : Prop
  noAcceptedFoldImplementation : Prop
  noPresentationBuilder : Prop
  noActiveViewComputation : Prop
  noStateCompressionRequirement : Prop
  noStateMutation : Prop
  noSemanticAuthorityPromotion : Prop

/-- Source obligations retained until state folding, presentation, and memory authority are separately checked. -/
inductive StateWorkingPresentationObligation where
  | noRequiredGlobalStateCompression
  | acceptedPresentationVersionedFoldShape
  | workingPresentationShape
  | activeViewDerivedFromRetainedStateAndHistoryUnproved
  | noHiddenMutableMemoryBecomesAuthoritativeUnproved
  | noAcceptedFoldImplementation
  | noPresentationBuilder
  | noActiveViewComputation
  | noStateCompressionRequirement
  | noStateMutation
  | noSemanticAuthorityPromotion
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
