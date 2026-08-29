import InquiryCalculus.Legacy.V20.ProgramCoreGrammar

/-! # v2.0 source-safe Ask-lowering boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The declared evidence mode for one open source port. -/
inductive LoweringPortMode where
  | pure
  | generate
  | probe
  | check
  | warrant
  deriving DecidableEq, Repr

/-- One typed evidence payload, retaining its declared mode rather than a selected answer. -/
inductive PortEvidenceSyntax (PureEvidence : Type u) (GenerationEvidence : Type u)
    (ProbeEvidence : Type u) (CheckEvidence : Type u) (WarrantEvidence : Type u) where
  | pure : PureEvidence → PortEvidenceSyntax PureEvidence GenerationEvidence ProbeEvidence CheckEvidence WarrantEvidence
  | generate : GenerationEvidence → PortEvidenceSyntax PureEvidence GenerationEvidence ProbeEvidence CheckEvidence WarrantEvidence
  | probe : ProbeEvidence → PortEvidenceSyntax PureEvidence GenerationEvidence ProbeEvidence CheckEvidence WarrantEvidence
  | check : CheckEvidence → PortEvidenceSyntax PureEvidence GenerationEvidence ProbeEvidence CheckEvidence WarrantEvidence
  | warrant : WarrantEvidence → PortEvidenceSyntax PureEvidence GenerationEvidence ProbeEvidence CheckEvidence WarrantEvidence

/-- Probe evidence retains occurrence, port, operator, event, raw return, and route identities. -/
structure ProbeEvidenceSyntax (Occurrence : Type u) (Port : Type u) (ProbeOperator : Type u)
    (ActualEvent : Type u) (RawReturn : Type u) (ResolutionRoute : Type u) where
  occurrence : Occurrence
  port : Port
  operator : ProbeOperator
  event : ActualEvent
  rawReturn : RawReturn
  route : ResolutionRoute
  eventForOccurrence : Prop
  portForOccurrence : Prop
  operatorForEvent : Prop
  rawForEvent : Prop
  routeForPort : Prop

/-- Source-safe lowering retains the evidence bundle, resolver boundary, and source-continuation gate. -/
structure SourceSafeAskLoweringSyntax (SourceConfig : Type u) (Occurrence : Type u) (Port : Type u)
    (PureEvidence : Type u) (GenerationEvidence : Type u) (ProbeOperator : Type u)
    (ActualEvent : Type u) (RawReturn : Type u) (ResolutionRoute : Type u)
    (CheckEvidence : Type u) (WarrantEvidence : Type u) (Bundle : Type u) (Resolver : Type u)
    (Outcome : Type u) (Core : Type u) (AnswerSlot : Type u) (Continuation : Type u) where
  sourceConfiguration : SourceConfig
  askOccurrence : Occurrence
  sourceAnswerSlot : AnswerSlot
  portMode : Port → LoweringPortMode
  portEvidence : Port → PortEvidenceSyntax PureEvidence GenerationEvidence
    (ProbeEvidenceSyntax Occurrence Port ProbeOperator ActualEvent RawReturn ResolutionRoute)
    CheckEvidence WarrantEvidence
  dischargeBundle : Bundle
  resolver : Resolver
  resolverOutcome : Outcome
  core : Core
  sourceContinuation : Continuation
  evidenceFollowsDeclaredMode : Prop
  exactOpenPortCoverage : Prop
  probeFirstEntersOrdinaryEventSpine : Prop
  exactRouteProvenanceNotEventOrder : Prop
  resolverConsumesCompleteBundle : Prop
  resolverChecksCrossPortConstraintsAndSupport : Prop
  onlySupportedOutcomeMayInvokeSourceContinuation : Prop
  rawHandlerMayPreserveAndRouteOnly : Prop
  rawHandlerCannotFillSourceAnswerSlot : Prop
  coreUsesReturnBranchAndProbeOnly : Prop

/-- Source obligations retained until evidence, routing, resolver, and compilation behavior are separately checked. -/
inductive SourceSafeAskLoweringObligation where
  | fiveDeclaredPortEvidenceModes
  | pureGenerateCheckAndWarrantRetainTypedProvenance
  | generationCarriesNoActualityAuthority
  | probeEvidenceHasOccurrencePortOperatorEventRawAndRoute
  | portFollowsAcceptedLoweringNotEventOrder
  | operatorMatchesExactCompiledRequest
  | rawReturnHasImmutableEventIdentity
  | byteEqualityInsufficientWithoutEventIdentity
  | bundleAllowsZeroOneOrSeveralEvents
  | sharedEventsRequireCheckedMultiPortLowering
  | resolverChecksCrossPortConstraintsAndSupport
  | resolverDoesNotManufactureCartesianAnswer
  | coreCoversEachOpenPortExactlyOnce
  | probeEntersOrdinaryEventSpineFirst
  | onlySupportedMayInvokeSourceContinuation
  | rawHandlerCannotFillSourceAnswerSlotDirectly
  | singleProbePortIsSpecializationOnly
  | noExecutionOrCompilationClaim
  | noAnswerSelectionOrTotalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
