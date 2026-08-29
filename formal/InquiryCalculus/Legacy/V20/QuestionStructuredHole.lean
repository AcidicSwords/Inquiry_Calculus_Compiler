import InquiryCalculus.Legacy.V20.SolutionFibers

/-! # v2.0 question-as-structured-hole boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w

/-- A question-shaped open X-coordinate in a preserved relation with one fixed Y-coordinate. -/
structure StructuredHoleSyntax (X : Type u) (Y : Type v) (C : Type w) where
  relation : TernaryRelationSyntax X Y C
  fixed : Y
  target : TargetRegion C

/-- The hole's candidate lawful refillings are its solution fiber. -/
def StructuredHoleSyntax.completionFiber {X : Type u} {Y : Type v} {C : Type w}
    (hole : StructuredHoleSyntax X Y C) : X → Prop :=
  solutionFiberX hole.relation hole.fixed hole.target

/-- The completion fiber preserves exact target-witness membership in the retained relation. -/
theorem structuredHole_completionFiber_iff {X : Type u} {Y : Type v} {C : Type w}
    (hole : StructuredHoleSyntax X Y C) (x : X) :
    hole.completionFiber x ↔ ∃ c, hole.target.contains c ∧ hole.relation.holds x hole.fixed c := Iff.rfl

/-- Filling, valid completion, question semantics, and execution remain outside this law boundary. -/
inductive StructuredHoleObligation where
  | removalOfFilling
  | retainedRelationConstraint
  | lawfulRefillCriterion
  | validCompletion
  | questionSemantics
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
