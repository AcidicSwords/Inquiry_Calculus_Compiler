import InquiryCalculus.Legacy.V20.QuestionSuccessionTypingClaim

/-! # v2.0 dependent-reciprocal-program boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y z

/-- The ordinary typed question stages of the reciprocal dependency graph. -/
inductive ReciprocalProgramStage where
  | useX
  | exteriorX
  | fiberX
  | returnX
  | recoveryX
  | seedXtoY
  | presentationY
  | useY
  | exteriorY
  | fiberY
  | returnY
  | recoveryY
  | gammaD
  deriving DecidableEq, Repr

/-- The declared mode of a reciprocal question port remains distinct from its answer carrier. -/
inductive ReciprocalQuestionMode where
  | pure
  | check
  | warrant
  | declaredElsewhere
  deriving DecidableEq, Repr

/-- Typed data retained by the dependent reciprocal macro. -/
structure DependentReciprocalProgramSyntax (Determination : Type u) (Presentation : Type v)
    (Source : Type w) (TaggedUse : Type x) (Orientation : Type y) (Coverage : Type z) where
  determination : Determination
  sourcePresentation : Presentation
  source : Source
  sourceOrientation : Orientation
  taggedUseIdentity : TaggedUse
  semanticCoverage : Coverage
  executionCoverage : Coverage
  sameUseFiber : Prop
  completeThreeValuedRecoveryProfile : Prop
  independentlyAdmittedReorientationSeed : Prop
  independentlyAdmittedReorientedPresentation : Prop
  stageGraph : ReciprocalProgramStage → Prop
  fiberMode : ReciprocalQuestionMode
  recoveryMode : ReciprocalQuestionMode
  presentationMode : ReciprocalQuestionMode
  gammaMode : ReciprocalQuestionMode
  finiteLiftPreservesPredecessorAnswersAndTags : Prop
  unknownWhenFamilyCoverageIsIncomplete : Prop
  allDependentRolesBeforeGamma : Prop
  transparentFirstOrderMacro : Prop

/-- Source obligations retained until all relation, coverage, and execution interfaces are separately formalized. -/
inductive DependentReciprocalProgramObligation where
  | ordinaryTypedQuestionFamilies
  | taggedApplicableNegationUse
  | sameUseFiberMembershipRelation
  | completeThreeValuedRecoveryProfileWithProvenance
  | recoveryNeverFunctionOfFiberAlone
  | exactSeedSupportAndWarrant
  | noUndeclaredPresentationPredicate
  | gammaTaggedCompatibilityIncompatibilityUnknownCarrier
  | questionModeNotAnswerConstructor
  | nestedAskNamedContinuationDependencyGraph
  | finiteLiftPreservesProofCarryingAnswersAndTags
  | incompleteFamilyCoverageUnknown
  | secondUseIndependentlyAdmitted
  | oneWaySuccessCannotFillSecondUse
  | gammaOnlyAfterEveryDependentRole
  | transparentMacroNotRuntimeOpcodeClosureOrOracle
  | noGenericInverseOrOrdinaryConverseSubstitution
  | noAnswerSelectionOrTotalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
