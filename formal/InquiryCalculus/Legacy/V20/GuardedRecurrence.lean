import InquiryCalculus.Legacy.V20.OperationalSemantics

/-! # v2.0 guarded-recurrence boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The three listed guarded progress-boundary forms remain distinct obligations. -/
inductive GuardedProgressBoundary where
  | actualProbe
  | finiteChangingBranch
  | bindingAuthorizedProgress
  deriving DecidableEq, Repr

/-- Guarded recurrence syntax retains progress forms and the invisible-recursion foil without enforcement. -/
structure GuardedRecurrenceSyntax (Cycle : Type u) (Binding : Type u) (ProgressState : Type u) where
  cycle : Cycle
  binding : Binding
  progressState : ProgressState
  progressBoundary : GuardedProgressBoundary
  potentiallyUnboundedInquiry : Prop
  corecursiveCycle : Prop
  actualProbeBoundary : Prop
  finiteBranchStateChanges : Prop
  bindingAuthorizedBoundaryHasExplicitSemantics : Prop
  pureInvisibleRecurrenceFoil : Prop
  rejectionAsNonproductiveUnproved : Prop
  productivityValidityUnproved : Prop
  noRecurrenceEvaluator : Prop

/-- Source obligations retained until productivity and rejection conditions are separately checked. -/
inductive GuardedRecurrenceObligation where
  | potentiallyUnboundedInquiryOnlyThroughGuardedRecurrence
  | productiveCycleCrossesProgressBoundary
  | actualProbeBoundary
  | finiteChangingBranchBoundary
  | bindingAuthorizedProgressBoundary
  | explicitProgressBoundarySemantics
  | pureInvisibleRecurrenceFoil
  | rejectionAsNonproductiveUnproved
  | productivityValidityUnproved
  | noUncheckedProgressAssertion
  | noRecurrenceEvaluatorOrRuntimeEnforcement
  | noAnswerSelectionOrTotalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
