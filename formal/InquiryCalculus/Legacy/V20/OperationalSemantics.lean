import InquiryCalculus.Legacy.V20.SequencingComposition

/-! # v2.0 operational-semantics boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The displayed transition shapes remain distinct predecessor obligations. -/
inductive OperationalTransitionShape where
  | returnResult
  | branchPossibility
  | probeRequest
  deriving DecidableEq, Repr

/-- The listed probe lifecycle stages preserve their source order as data. -/
inductive ProbeLifecycleStage where
  | validateApplicabilityAndAuthority
  | actualizeOperator
  | receiveRawReturnAndCoupledConfiguration
  | appendRawOccurrenceBeforeDecoding
  | supplyRawReturnToLowerHandler
  deriving DecidableEq, Repr

/-- Operational syntax retains configurations, transition shapes, and source event-spine constraints. -/
structure OperationalSemanticsSyntax (Term : Type u) (RuntimeState : Type u) (Value : Type u)
    (ProbeOperator : Type u) (RawReturn : Type u) (CoupledConfiguration : Type u)
    (EventHistory : Type u) (SourceAsk : Type u) where
  term : Term
  runtimeState : RuntimeState
  value : Value
  probeOperator : ProbeOperator
  rawReturn : RawReturn
  coupledConfiguration : CoupledConfiguration
  eventHistory : EventHistory
  sourceAsk : SourceAsk
  configurationShape : Prop
  returnTransition : OperationalTransitionShape
  branchTransition : OperationalTransitionShape
  probeTransition : OperationalTransitionShape
  authoritativeRuntimeState : Prop
  branchChangesRepresentedPossibility : Prop
  branchCreatesNoActualityEvent : Prop
  probeLifecycle : ProbeLifecycleStage → Prop
  eventSpineBeforeDecoding : Prop
  sourceLoweringNeedsEventLinkageAndDeclaredOutcome : Prop
  rawReturnImmutabilityClaimUnproved : Prop
  transitionValidityUnproved : Prop
  noExecutableRuntime : Prop

/-- Source obligations retained until runtime transition and raw-record claims are separately checked. -/
inductive OperationalSemanticsObligation where
  | runtimeConfigurationShape
  | authoritativeRuntimeStateDefinedLater
  | returnTransitionShape
  | branchTransitionShape
  | branchChangesPossibilityWithoutActuality
  | probeTransitionShape
  | validateApplicabilityAndDischargeAuthority
  | actualizeOperator
  | receiveRawReturnAndCoupledConfiguration
  | appendRawOccurrenceBeforeDecoding
  | lowerHandlerGetsRawReturn
  | sourceLoweringRequiresEventLinkageAndOutcome
  | ordinaryEventSpineNotEventOrderShortcut
  | rawReturnImmutabilityUnproved
  | noRawRecordMutation
  | transitionValidityUnproved
  | noExecutableRuntime
  | noAnswerSelectionOrTotalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
