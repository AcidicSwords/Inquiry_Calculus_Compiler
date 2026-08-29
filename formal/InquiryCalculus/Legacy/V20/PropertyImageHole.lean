import InquiryCalculus.Legacy.V20.SolutionFieldWeb

/-! # v2.0 property-image-of-a-hole boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v

/-- A value belongs to a property's image precisely when some field member maps to it. -/
def propertyImage {X : Type u} {P : Type v} (property : X → P) (field : X → Prop) : P → Prop :=
  fun value => ∃ x, field x ∧ property x = value

/-- A property is forced when its image contains exactly the named value. -/
def propertyForced {X : Type u} {P : Type v} (property : X → P) (field : X → Prop) (value : P) : Prop :=
  ∀ candidate, propertyImage property field candidate ↔ candidate = value

/-- Image membership is only a witnessed mapped value, not an answer selection. -/
theorem propertyImage_iff {X : Type u} {P : Type v} (property : X → P) (field : X → Prop) (value : P) :
    propertyImage property field value ↔ ∃ x, field x ∧ property x = value := Iff.rfl

/-- Answer selection, filler uniqueness, and solver meanings remain open. -/
inductive PropertyImageHoleObligation where
  | selectedAnswer
  | literalFillerUniqueness
  | protectedClassUniqueness
  | forcednessWarrant
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20
