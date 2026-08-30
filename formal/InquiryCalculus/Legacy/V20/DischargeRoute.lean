import InquiryCalculus.Legacy.V20.ActiveView

/-! # v2.0 Stage 4 discharge-route boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- Stage 4 retains the declared route shape for a single open port without dispatching it. -/
structure DischargeRouteSyntax (Question : Type u) (Port : Type u) (Mode : Type u)
    (Carrier : Type u) (Result : Type u) (Continuation : Type u) where
  question : Question
  port : Port
  declaredMode : Mode
  carrier : Carrier
  result : Result
  continuation : Continuation
  respectsDeclaredPortMode : Prop
  pureRouteIsDeterministicComputationUnproved : Prop
  generateRouteMayUseSemanticGeneratorUnproved : Prop
  probeRouteMayUseActualizableInteractionUnproved : Prop
  checkRouteMayUseAdmittedCheckerUnproved : Prop
  warrantRouteMayUseStandingEngineUnproved : Prop
  mixedQuestionMayUseDifferentPortRoutesUnproved : Prop
  noRouteDispatch : Prop
  noEvidenceExecution : Prop
  noAnswerConstruction : Prop
  noContinuationExecution : Prop
  noModeCollapse : Prop
  noUntypedCarrierOrResult : Prop
  noOpaqueContinuation : Prop

/-- Source obligations retained until route selection and every lowering relation are separately checked. -/
inductive DischargeRouteObligation where
  | declaredPortMode
  | pureDeterministicComputationUnproved
  | generateSemanticGeneratorUnproved
  | probeActualizableInteractionUnproved
  | checkAdmittedCheckerUnproved
  | warrantStandingEngineUnproved
  | mixedQuestionDifferentPortRoutesUnproved
  | noRouteDispatch
  | noEvidenceExecution
  | noAnswerConstruction
  | noContinuationExecution
  | noModeCollapse
  | noUntypedCarrierOrResult
  | noOpaqueContinuation
  | noTotalSolverOrAuthorityPromotion
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
