import InquiryCalculus.Legacy.V20.AuthoritativeAncestry

/-! # v2.0 canonical event-record boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The predecessor retains every canonical event coordinate without constructing, linking, or decoding an event. -/
structure EventRecordSyntax (StateRef : Type u) (SourceAsk : Type u) (Question : Type u)
    (BoundaryRef : Type u) (OperatorRef : Type u) (ReturnRef : Type u) (Grain : Type u)
    (Route : Type u) (Provenance : Type u) (BackendVersion : Type u) (BindingVersion : Type u)
    (ReplayMetadata : Type u) (SourceConfig : Type u) (AskOccurrence : Type u)
    (ResolutionPath : Type u) (Event : Type u) where
  preStateRef : StateRef
  sourceAsk : SourceAsk
  semanticQuestionOrProbeSchemaRef : Question
  activeBoundaryChartRef : BoundaryRef
  compiledOperatorOrPromptRef : OperatorRef
  rawReturnRef : ReturnRef
  postStateRef : StateRef
  representationGrain : Grain
  dischargeActualizationRoute : Route
  provenance : Provenance
  backendVersion : BackendVersion
  bindingVersion : BindingVersion
  replayMetadata : ReplayMetadata
  sourceConfig : SourceConfig
  askOccurrence : AskOccurrence
  resolutionPath : ResolutionPath
  event : Event
  canonicalEventRecordShape : Prop
  sourceAskDependentOptionShape : Prop
  directOrLegacyRuntimeProbeUsesNone : Prop
  sourceCompiledAskUsesSomeExactAskReference : Prop
  preservesPreStateConfigurationReference : Prop
  preservesExactSourceAskOccurrence : Prop
  preservesSemanticQuestionProbeSchemaReference : Prop
  preservesActiveBoundaryChartReference : Prop
  preservesExactCompiledOperatorPromptReference : Prop
  preservesRawReturnReference : Prop
  preservesPostStateConfigurationReference : Prop
  preservesRepresentationGrain : Prop
  preservesDischargeActualizationRoute : Prop
  preservesProvenanceBackendBindingAndReplayMetadata : Prop
  exactSourceEventLinkageShape : Prop
  endpointEqualityIsInsufficientForExactLinkageUnproved : Prop
  reciprocalActualityUsesOrdinaryEventSpineUnproved : Prop
  noEventConstruction : Prop
  noSourceLinkVerification : Prop
  noRawReturnDecoding : Prop
  noRouteExecution : Prop
  noReplayEngine : Prop
  noAcceptanceDecision : Prop
  noSemanticAuthorityPromotion : Prop

/-- Source obligations retained until event formation, linkage, decoding, and actuality are separately checked. -/
inductive EventRecordObligation where
  | canonicalEventRecordShape
  | sourceAskDependentOptionShape
  | directOrLegacyRuntimeProbeUsesNone
  | sourceCompiledAskUsesSomeExactAskReference
  | preStateConfigurationReference
  | exactSourceAskOccurrence
  | semanticQuestionProbeSchemaReference
  | activeBoundaryChartReference
  | exactCompiledOperatorPromptReference
  | rawReturnReference
  | postStateConfigurationReference
  | representationGrain
  | dischargeActualizationRoute
  | provenanceBackendBindingAndReplayMetadata
  | exactSourceEventLinkageShape
  | endpointEqualityInsufficientForExactLinkageUnproved
  | reciprocalActualityUsesOrdinaryEventSpineUnproved
  | noEventConstruction
  | noSourceLinkVerification
  | noRawReturnDecoding
  | noRouteExecution
  | noReplayEngine
  | noAcceptanceDecision
  | noSemanticAuthorityPromotion
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
