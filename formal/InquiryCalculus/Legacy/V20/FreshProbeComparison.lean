import InquiryCalculus.Legacy.V20.ProbeTraces

/-! # v2.0 fresh-probe-before-comparison boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The predecessor retains fresh-return and historical-comparison coordinates without executing either. -/
structure FreshProbeComparisonSyntax (EnvironmentModelInterface : Type u) (ProbeContract : Type u)
    (CurrentInput : Type u) (FreshVersion : Type u) (FreshReturn : Type u) (HistoricalTrace : Type u)
    (FreshEvent : Type u) (Comparison : Type u) (MethodLevelControl : Type u)
    (ModelWording : Type u) (EnvironmentPersistence : Type u) where
  environmentModelInterface : EnvironmentModelInterface
  probeContract : ProbeContract
  currentInput : CurrentInput
  freshVersion : FreshVersion
  freshReturn : FreshReturn
  historicalTrace : HistoricalTrace
  freshEvent : FreshEvent
  comparison : Comparison
  methodLevelControl : MethodLevelControl
  modelWording : ModelWording
  environmentPersistence : EnvironmentPersistence
  interfacePermitsFreshReturnWithoutPriorInterpretationShape : Prop
  freshProbeBeforeTraceComparisonShape : Prop
  freshReturnShape : Prop
  historicalTraceComparisonShape : Prop
  methodLevelControlNotConstitutionalPrimitiveUnproved : Prop
  wordingContinuityNotEnvironmentPersistenceUnproved : Prop
  noFreshProbeExecution : Prop
  noHistoricalTraceComparison : Prop
  noInterfaceEvaluation : Prop
  noMethodPromotion : Prop
  noWordingPersistenceInference : Prop
  noSemanticAuthorityPromotion : Prop

/-- Source obligations retained until fresh probing and comparison behavior are separately checked. -/
inductive FreshProbeComparisonObligation where
  | interfacePermitsFreshReturnWithoutPriorInterpretationShape
  | freshProbeBeforeTraceComparisonShape
  | freshReturnShape
  | historicalTraceComparisonShape
  | methodLevelControlNotConstitutionalPrimitiveUnproved
  | wordingContinuityNotEnvironmentPersistenceUnproved
  | noFreshProbeExecution
  | noHistoricalTraceComparison
  | noInterfaceEvaluation
  | noMethodPromotion
  | noWordingPersistenceInference
  | noSemanticAuthorityPromotion
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
