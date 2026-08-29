import InquiryCalculus.Legacy.V20.QuestionRedundancy

/-! # v2.0 precision-not-improvement boundary -/
namespace InquiryCalculus.Legacy.V20

/-- The source law distinguishes precision comparison from improvement assessment. -/
inductive RepresentationComparisonKind where
  | precision
  | improvement
  deriving DecidableEq, Repr

/-- A local candidate precision comparison between two question coordinates. -/
structure PrecisionComparisonSyntax (B : Binding) (I : TypeInterpretation B) where
  coarser : CanonicalQuestionSyntax B I
  finer : CanonicalQuestionSyntax B I

/-- A local candidate improvement assessment between two active representations. -/
structure ImprovementAssessmentSyntax (B : Binding) (I : TypeInterpretation B) where
  incumbent : LiveCarrierToken B I
  candidate : LiveCarrierToken B I

/-- The two comparison kinds remain formally distinct before any improvement criterion is admitted. -/
theorem precision_kind_is_not_improvement_kind :
    RepresentationComparisonKind.precision ≠ RepresentationComparisonKind.improvement := by
  decide

/-- Protected behavior, cost, robustness, and preference require later typed criteria. -/
inductive PrecisionNotImprovementObligation where
  | noAutomaticMonotonicity
  | protectedBehaviorCriterion
  | lowerCostCriterion
  | betterRobustnessCriterion
  | globalRepresentationOrdering
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
