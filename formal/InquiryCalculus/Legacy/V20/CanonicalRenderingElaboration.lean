import InquiryCalculus.Legacy.V20.PromptOperator

/-! # v2.0 canonical-rendering and elaboration boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The predecessor retains every named rendering coordinate without implementing rendering or elaboration. -/
structure CanonicalRenderingElaborationSyntax (OpenIR : Type u) (PromptIR : Type u) (Boundary : Type u)
    (NormalizedQuestion : Type u) (RelationSchema : Type u) (Referent : Type u) (Port : Type u)
    (Scope : Type u) (Applicability : Type u) (Orientation : Type u) (Presentation : Type u)
    (NegationUse : Type u) (DepartureWitness : Type u) (SemanticCoverage : Type u)
    (ExecutionCoverage : Type u) (Status : Type u) (ReturnFiber : Type u) (SelectedReturn : Type u)
    (Seed : Type u) (Recovery : Type u) (DischargeRoute : Type u) (ResolutionCriterion : Type u)
    (Comparator : Type u) (Annotation : Type u) (BackwardRelation : Type u) (FormulaNegation : Type u) where
  openIR : OpenIR
  promptIR : PromptIR
  boundary : Boundary
  normalizedQuestion : NormalizedQuestion
  relationSchemaIdentity : RelationSchema
  boundReferents : Referent
  openPortNamesAndTypes : Port
  scope : Scope
  applicability : Applicability
  boundaryOrientation : Orientation
  sourceDeterminationPresentationIdentityAndVersion : Presentation
  negationUseIdentity : NegationUse
  departureWitnessProvenance : DepartureWitness
  semanticCoverage : SemanticCoverage
  executionMaterializationCoverage : ExecutionCoverage
  status : Status
  useSpecificReturnFiber : ReturnFiber
  selectedReturn : SelectedReturn
  seed : Seed
  recoveryProvenance : Recovery
  dischargeRoute : DischargeRoute
  resolutionCriterion : ResolutionCriterion
  recurrentProbeComparator : Comparator
  rootOrRouteAnnotation : Annotation
  backwardRelation : BackwardRelation
  formulaNegation : FormulaNegation
  canonicalRenderingRelationShape : Prop
  partialElaborationRelationShape : Prop
  controlledRoundTripNormalizationUnproved : Prop
  freeProseHasNoRoundTripGuarantee : Prop
  preservesRelationSchemaIdentity : Prop
  preservesBoundReferents : Prop
  preservesOpenPortNamesAndTypes : Prop
  preservesScopeAndApplicability : Prop
  preservesProtectedBoundaryOrientation : Prop
  preservesSourceDeterminationPresentationIdentityAndVersion : Prop
  preservesNegationUseAndDepartureWitnessProvenance : Prop
  keepsSemanticAndExecutionCoverageDistinct : Prop
  preservesStatusWithoutStrengthening : Prop
  preservesReturnFiberSelectedReturnSeedAndRecoveryProvenance : Prop
  preservesDischargeRouteAndResolutionCriterion : Prop
  preservesRecurrentProbeComparator : Prop
  derivedInterrogativeRootsAreNotSemanticSpecies : Prop
  annotationErasesUnderNormalizationUnproved : Prop
  annotationCannotAlterProtectedCoordinatesUnproved : Prop
  preservesBackwardRelationKindUnproved : Prop
  noExistentialPreimageToWeakestSufficientStrengthening : Prop
  noContextualNegationUseSynthesis : Prop
  formulaNegationRemainsDistinct : Prop
  conservativeInterrogativeLoweringClaimUnproved : Prop
  noInterrogativeRuntimeOpcodeClaimUnproved : Prop
  noRendererImplementation : Prop
  noElaboratorImplementation : Prop
  noNormalizationProof : Prop
  noAnnotationEffect : Prop
  noSemanticAuthorityPromotion : Prop

/-- Source obligations retained until the rendering relations and classified claims are separately checked. -/
inductive CanonicalRenderingElaborationObligation where
  | canonicalRenderingRelationShape
  | partialElaborationRelationShape
  | controlledRoundTripNormalizationUnproved
  | freeProseHasNoRoundTripGuarantee
  | relationSchemaIdentity
  | boundReferents
  | openPortNamesAndTypes
  | scopeAndApplicability
  | protectedBoundaryOrientation
  | sourceDeterminationPresentationIdentityAndVersion
  | negationUseAndDepartureWitnessProvenance
  | semanticAndExecutionCoverageDistinct
  | statusWithoutStrengthening
  | returnFiberSelectedReturnSeedAndRecoveryProvenance
  | dischargeRoute
  | resolutionCriterion
  | recurrentProbeComparatorUnproved
  | derivedInterrogativeRootsNotSemanticSpecies
  | annotationErasureUnproved
  | annotationCannotAlterProtectedCoordinatesUnproved
  | backwardRelationKindPreservationUnproved
  | noExistentialPreimageStrengthening
  | noContextualNegationUseSynthesis
  | formulaNegationDistinct
  | conservativeInterrogativeLoweringClaimUnproved
  | noInterrogativeRuntimeOpcodeClaimUnproved
  | noRendererImplementation
  | noElaboratorImplementation
  | noNormalizationProof
  | noAnnotationEffect
  | noSemanticAuthorityPromotion
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
