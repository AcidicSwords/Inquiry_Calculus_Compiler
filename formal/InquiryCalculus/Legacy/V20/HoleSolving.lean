import InquiryCalculus.Legacy.V20.DependentReciprocalProgram

/-! # v2.0 hole-solving boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y

/-- The residual-relative result distinguishes a protected filler from a continuing separating inquiry. -/
inductive HoleSolveResult (FillerClass : Type u) (SeparatingRelation : Type v) where
  | protectedFillerClass : FillerClass → HoleSolveResult FillerClass SeparatingRelation
  | askSeparatingRelation : SeparatingRelation → HoleSolveResult FillerClass SeparatingRelation

/-- A hole-solving state retains its residual web, open carrier, solution set, and effectivity horizon. -/
structure HoleSolvingSyntax (ResidualWeb : Type u) (OpenCarrier : Type v) (Solution : Type w)
    (ProtectedClass : Type x) (EffectivityHorizon : Type y) where
  residualWeb : ResidualWeb
  openCarrier : OpenCarrier
  solutionSet : Solution → Prop
  protectedEquivalenceClass : Solution → ProtectedClass
  effectivityHorizon : EffectivityHorizon
  protectedQuotientSingleton : Prop
  survivingProtectedClasses : ProtectedClass → Prop
  recomputeMeetAfterReturnedRelation : Prop
  recurSubjectToEffectivityHorizon : Prop

/-- Source obligations retained until solution computation and relation execution are separately formalized. -/
inductive HoleSolvingObligation where
  | residualWebWithOpenCarrier
  | solutionSetRelativeToResidualWebAndOpenCarrier
  | protectedQuotientSingletonRequiredForFillerClass
  | separatingRelationForTwoSurvivingProtectedClasses
  | extendResidualWebByReturnedRelation
  | recomputeMeet
  | recurSubjectToEffectivityHorizon
  | noTotalSolutionPromotion
  | noResidualErasure
  | semanticAndExecutionCoverageRemainDistinct
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
