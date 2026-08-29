import InquiryCalculus.Legacy.V20.PrecisionNotImprovement

/-! # v2.0 relational sections boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w

/-- A locally typed ternary relation, not a total function or an executable program. -/
structure TernaryRelationSyntax (X : Type u) (Y : Type v) (C : Type w) where
  holds : X → Y → C → Prop

/-- The residual X-section obtained by fixing the Y coordinate. -/
def sectionX {X : Type u} {Y : Type v} {C : Type w}
    (relation : TernaryRelationSyntax X Y C) (fixed : Y) : X → C → Prop :=
  fun x c => relation.holds x fixed c

/-- The distinct residual Y-section obtained by fixing the X coordinate. -/
def sectionY {X : Type u} {Y : Type v} {C : Type w}
    (relation : TernaryRelationSyntax X Y C) (fixed : X) : Y → C → Prop :=
  fun y c => relation.holds fixed y c

/-- The X-section has exactly the source relation's fixed-Y incidence. -/
theorem sectionX_holds_iff {X : Type u} {Y : Type v} {C : Type w}
    (relation : TernaryRelationSyntax X Y C) (y : Y) (x : X) (c : C) :
    sectionX relation y x c ↔ relation.holds x y c := Iff.rfl

/-- The Y-section has exactly the source relation's fixed-X incidence. -/
theorem sectionY_holds_iff {X : Type u} {Y : Type v} {C : Type w}
    (relation : TernaryRelationSyntax X Y C) (x : X) (y : Y) (c : C) :
    sectionY relation x y c ↔ relation.holds x y c := Iff.rfl

/-- Functionality, symmetry, satisfaction, composition, and execution remain open. -/
inductive RelationalSectionObligation where
  | notGenerallySymmetric
  | totalFunctionality
  | relationSatisfaction
  | sectionComposition
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
