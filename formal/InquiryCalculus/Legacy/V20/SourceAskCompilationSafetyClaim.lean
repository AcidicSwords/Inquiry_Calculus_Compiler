import InquiryCalculus.Legacy.V20.SourceSafeAskLowering

/-! # v2.0 source-Ask-compilation safety claim boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The unproved law's source-bound premise and direct-runtime-Probe counterposition. -/
structure SourceAskCompilationSafetyClaimSyntax (SourceAsk : Type u) (Compilation : Type u)
    (LoweringEvidence : Type u) (RuntimeProbe : Type u) (Occurrence : Type u) where
  sourceAsk : SourceAsk
  compilation : Compilation
  loweringEvidence : LoweringEvidence
  directRuntimeProbe : RuntimeProbe
  occurrence : Occurrence
  compilationHasSourceSafeLowering : Prop
  directRuntimeProbeRemainsRuntimeTerm : Prop
  directRuntimeProbeNotLawfulOccurrenceLowering : Prop
  sourceSafeLoweringDefinitionDependency : Prop
  lawStatusIsUnproved : Prop

/-- Source obligations retained until the law's premise and conclusion can be independently elaborated. -/
inductive SourceAskCompilationSafetyClaimObligation where
  | everySourceAskCompilationClaim
  | sourceSafeLoweringDefinitionDependency
  | occurrenceSpecificSourceAsk
  | directRuntimeProbeRemainsRuntimeTerm
  | directRuntimeProbeNotLawfulLowering
  | compilerRelationNotYetElaborated
  | sourceSafeConclusionNotYetProved
  | noAxiomOrPlaceholderProof
  | noExecutionOrSemanticEvaluation
  | noAnswerSelectionOrTotalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
