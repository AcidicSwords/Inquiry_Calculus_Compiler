import InquiryCalculus.Legacy.V20.RelationalSections

/-! # v2.0 solution-fiber boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w

/-- A typed target output region, not an answer or a selection policy. -/
structure TargetRegion (C : Type u) where
  contains : C → Prop

/-- The X-solution fiber of a fixed Y-coordinate and target output region. -/
def solutionFiberX {X : Type u} {Y : Type v} {C : Type w}
    (relation : TernaryRelationSyntax X Y C) (fixed : Y) (target : TargetRegion C) : X → Prop :=
  fun x => ∃ c, target.contains c ∧ relation.holds x fixed c

/-- The fiber is exactly the target inverse-image incidence of the fixed X-section. -/
theorem solutionFiberX_iff {X : Type u} {Y : Type v} {C : Type w}
    (relation : TernaryRelationSyntax X Y C) (y : Y) (target : TargetRegion C) (x : X) :
    solutionFiberX relation y target x ↔ ∃ c, target.contains c ∧ sectionX relation y x c := Iff.rfl

/-- Predicate-valued relation case with no separate output witness. -/
def predicateSolutionFiber {X : Type u} {Y : Type v} (relation : X → Y → Prop) (fixed : Y) : X → Prop :=
  fun x => relation x fixed

/-- Answers, unique witnesses, satisfaction, and execution remain later obligations. -/
inductive SolutionFiberObligation where
  | targetRegionSemantics
  | inverseImageLaw
  | predicateRelationCorrespondence
  | completeAnswer
  | uniqueWitness
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
